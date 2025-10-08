use num_bigint::{BigUint, RandBigInt};
use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
pub struct ZKP {
    p: BigUint,
    q: BigUint,
    g: BigUint,
    h: BigUint,
}

impl ZKP {
    // g ** x mod p
    // output = n ** exp mod p
    pub fn exponentiate(n: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
        n.modpow(exponent, modulus)
    }

    // s = k - c * x mod q
    pub fn solve(&self, k: &BigUint, c: &BigUint, x: &BigUint) -> BigUint {
        if *k >= c * x {
            (k - c * x).modpow(&BigUint::from(1u32), &self.q)
        } else {
            &self.q - (c * x - k).modpow(&BigUint::from(1u32), &self.q)
        }
    }

    // unified formula (mathematically equivalent)
    pub fn solve_unified(&self, k: &BigUint, c: &BigUint, x: &BigUint) -> BigUint {
        let cx = c * x;
        if *k >= cx {
            // in case of k >= c*x: k - c*x mod q
            (k - &cx).modpow(&BigUint::from(1u32), &self.q)
        } else {
            // k < c*x: q - (c*x - k) mod q
            &self.q - (cx - k).modpow(&BigUint::from(1u32), &self.q)
        }
    }

    // cond1: r1 = g ** s * y1 ** c mod p
    // cond2: r2 = h ** s * y2 ** c mod p
    pub fn verify(
        &self,
        r1: &BigUint,
        r2: &BigUint,
        y1: &BigUint,
        y2: &BigUint,
        c: &BigUint,
        s: &BigUint,
    ) -> bool {
        let cond1 = *r1
            == (&self.g.modpow(s, &self.p) * y1.modpow(c, &self.p))
                .modpow(&BigUint::from(1u32), &self.p);
        let cond2 = *r2
            == (&self.h.modpow(s, &self.p) * y2.modpow(c, &self.p))
                .modpow(&BigUint::from(1u32), &self.p);
        cond1 && cond2
    }

    pub fn generate_random_below(limit: &BigUint) -> BigUint {
        let mut rng = rand::thread_rng();

        rng.gen_biguint_below(limit)
    }
}

// trait: cryptographic operation
pub trait CryptographicOperation {
    fn compute(&self, input: &BigUint) -> BigUint;
    fn name(&self) -> &str;
}

// implementation: exponentiation
#[derive(Debug)]
pub struct Exponentiation {
    pub base: BigUint,
    pub modulus: BigUint,
}

impl CryptographicOperation for Exponentiation {
    fn compute(&self, exponent: &BigUint) -> BigUint {
        self.base.modpow(exponent, &self.modulus)
    }

    fn name(&self) -> &str {
        "Exponentiation"
    }
}

impl Display for Exponentiation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "base={}, modulus={}", self.base, self.modulus)
    }
}

// generic function: process operation
pub fn process_operation<T: CryptographicOperation + Display>(
    operation: &T,
    input: &BigUint,
) -> BigUint {
    println!("Processing {}: {}", operation.name(), operation);
    operation.compute(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modulo_arithmetic() {
        let zkp = ZKP {
            p: BigUint::from(23u32),
            q: BigUint::from(11u32),
            g: BigUint::from(4u32),
            h: BigUint::from(9u32),
        };

        // case 1: k >= c * x
        let k1 = BigUint::from(7u32);
        let c1 = BigUint::from(2u32);
        let x1 = BigUint::from(3u32);
        // k1 - c1 * x1 = 7 - 2*3 = 7 - 6 = 1
        let s1 = zkp.solve(&k1, &c1, &x1);
        println!("Case 1: k={}, c={}, x={}, s={}", k1, c1, x1, s1);

        // case 2: k < c * x
        let k2 = BigUint::from(3u32);
        let c2 = BigUint::from(2u32);
        let x2 = BigUint::from(3u32);
        // k2 - c2 * x2 = 3 - 2*3 = 3 - 6 = -3
        // -3 mod 11 = 8
        let s2 = zkp.solve(&k2, &c2, &x2);
        println!("Case 2: k={}, c={}, x={}, s={}", k2, c2, x2, s2);

        // manual calculation
        let manual_s2 = zkp.q - &(&c2 * &x2 - &k2);
        println!("Manual calculation: q - (c*x - k) = {}", manual_s2);

        assert_eq!(s1, BigUint::from(1u32));
        assert_eq!(s2, BigUint::from(8u32));
        assert_eq!(s2, manual_s2);
    }

    #[test]
    fn test_unified_formula() {
        let zkp = ZKP {
            p: BigUint::from(23u32),
            q: BigUint::from(11u32),
            g: BigUint::from(4u32),
            h: BigUint::from(9u32),
        };

        // case 1: k >= c * x
        let k1 = BigUint::from(7u32);
        let c1 = BigUint::from(2u32);
        let x1 = BigUint::from(3u32);

        let s1_original = zkp.solve(&k1, &c1, &x1);
        let s1_unified = zkp.solve_unified(&k1, &c1, &x1);
        println!(
            "Case 1 - Original: {}, Unified: {}",
            s1_original, s1_unified
        );

        // case 2: k < c * x
        let k2 = BigUint::from(3u32);
        let c2 = BigUint::from(2u32);
        let x2 = BigUint::from(3u32);

        let s2_original = zkp.solve(&k2, &c2, &x2);
        let s2_unified = zkp.solve_unified(&k2, &c2, &x2);
        println!(
            "Case 2 - Original: {}, Unified: {}",
            s2_original, s2_unified
        );

        // both results are the same
        assert_eq!(s1_original, s1_unified);
        assert_eq!(s2_original, s2_unified);
    }

    #[test]
    fn test_trait_import_example() {
        // use imported trait
        let base = BigUint::from(4u32);
        let modulus = BigUint::from(23u32);
        let exponent = BigUint::from(6u32);

        let exp_op = Exponentiation { base, modulus };

        // use Display trait
        println!("Exponentiation: {}", exp_op);

        // use Debug trait
        println!("Debug: {:?}", exp_op);

        // use custom trait
        let result = exp_op.compute(&exponent);
        println!("Result: {}", result);

        assert_eq!(result, BigUint::from(2u32));
    }

    #[test]
    fn test_pointer_comparison() {
        let a = BigUint::from(5u32);
        let b = BigUint::from(5u32);
        let ref_a = &a;
        let ref_b = &b;

        // compare values
        assert_eq!(ref_a, ref_b); // true - values are the same

        // compare pointers
        assert_ne!(ref_a as *const BigUint, ref_b as *const BigUint); // false - different addresses

        // compare references
        let ref_a2 = &a;
        assert_eq!(ref_a as *const BigUint, ref_a2 as *const BigUint); // true - same address

        println!("ref_a address: {:p}", ref_a);
        println!("ref_b address: {:p}", ref_b);
        println!("ref_a2 address: {:p}", ref_a2);
    }

    #[test]
    fn test_toy_example_with_random_numbers() {
        let zkp = ZKP {
            p: BigUint::from(23u32),
            q: BigUint::from(11u32),
            g: BigUint::from(4u32),
            h: BigUint::from(9u32),
        };

        let x = BigUint::from(6u32);
        let k = ZKP::generate_random_below(&zkp.q);

        let c = ZKP::generate_random_below(&zkp.q);

        let y1 = ZKP::exponentiate(&zkp.g, &x, &zkp.p);
        let y2 = ZKP::exponentiate(&zkp.h, &x, &zkp.p);

        let r1 = ZKP::exponentiate(&zkp.g, &k, &zkp.p);
        let r2 = ZKP::exponentiate(&zkp.h, &k, &zkp.p);

        let s = zkp.solve(&k, &c, &x);

        let result = zkp.verify(&r1, &r2, &y1, &y2, &c, &s);
        assert!(result);
    }

    #[test]
    fn test_toy_example() {
        let zkp = ZKP {
            p: BigUint::from(23u32),
            q: BigUint::from(11u32),
            g: BigUint::from(4u32),
            h: BigUint::from(9u32),
        };

        let x = BigUint::from(6u32);
        let k = BigUint::from(7u32);

        let c = BigUint::from(4u32);

        let y1 = ZKP::exponentiate(&zkp.g, &x, &zkp.p);
        let y2 = ZKP::exponentiate(&zkp.h, &x, &zkp.p);
        assert_eq!(y1, BigUint::from(2u32));
        assert_eq!(y2, BigUint::from(3u32));

        let r1 = ZKP::exponentiate(&zkp.g, &k, &zkp.p);
        let r2 = ZKP::exponentiate(&zkp.h, &k, &zkp.p);
        assert_eq!(r1, BigUint::from(8u32));
        assert_eq!(r2, BigUint::from(4u32));

        let s = zkp.solve(&k, &c, &x);
        assert_eq!(s, BigUint::from(5u32));

        let result = zkp.verify(&r1, &r2, &y1, &y2, &c, &s);
        assert!(result);

        // fake secret
        let x_fake = BigUint::from(7u32);
        let s_fake = zkp.solve(&k, &c, &x_fake);
        let result_fake = zkp.verify(&r1, &r2, &y1, &y2, &c, &s_fake);
        assert!(!result_fake);
    }

    #[test]
    fn test_toy_example2() {
        let zkp = ZKP {
            p: BigUint::from(47u32),
            q: BigUint::from(23u32),
            g: BigUint::from(16u32),
            h: BigUint::from(17u32),
        };

        let x = BigUint::from(300u32);
        let k = BigUint::from(100u32);

        let c = BigUint::from(200u32);

        let y1 = ZKP::exponentiate(&zkp.g, &x, &zkp.p);
        let y2 = ZKP::exponentiate(&zkp.h, &x, &zkp.p);

        let r1 = ZKP::exponentiate(&zkp.g, &k, &zkp.p);
        let r2 = ZKP::exponentiate(&zkp.h, &k, &zkp.p);

        let s = zkp.solve(&k, &c, &x);

        let result = zkp.verify(&r1, &r2, &y1, &y2, &c, &s);
        assert!(result);
    }
}
