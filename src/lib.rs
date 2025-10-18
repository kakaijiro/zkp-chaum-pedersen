use num_bigint::{BigUint, RandBigInt};
use rand::{distributions::Alphanumeric, Rng};
use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
pub struct ZKP {
    pub p: BigUint,
    pub q: BigUint,
    pub g: BigUint,
    pub h: BigUint,
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

    pub fn generate_random_number_below(limit: &BigUint) -> BigUint {
        let mut rng = rand::thread_rng();

        rng.gen_biguint_below(limit)
    }

    pub fn generate_random_string(size: usize) -> String {
        rand::thread_rng()
            .sample_iter(Alphanumeric)
            .take(size)
            .map(char::from)
            .collect()
    }

    pub fn get_constants() -> (BigUint, BigUint, BigUint, BigUint) {
        let p = BigUint::from_bytes_be(&hex::decode("B10B8F96A080E01DDE92DE5EAE5D54EC52C99FBCFB06A3C69A6A9DCA52D23B616073E28675A23D189838EF1E2EE652C013ECB4AEA906112324975C3CD49B83BFACCBDD7D90C4BD7098488E9C219A73724EFFD6FAE5644738FAA31A4FF55BCCC0A151AF5F0DC8B4BD45BF37DF365C1A65E68CFDA76D4DA708DF1FB2BC2E4A4371").unwrap());
        let q = BigUint::from_bytes_be(
            &hex::decode("F518AA8781A8DF278ABA4E7D64B7CB9D49462353").unwrap(),
        );
        let g = BigUint::from_bytes_be(&hex::decode("A4D1CBD5C3FD34126765A442EFB99905F8104DD258AC507FD6406CFF14266D31266FEA1E5C41564B777E690F5504F213160217B4B01B886A5E91547F9E2749F4D7FBD7D3B9A92EE1909D0D2263F80A76A6A24C087A091F531DBF0A0169B6A28AD662A4D18E73AFA32D779D5918D08BC8858F4DCEF97C2A24855E6EEB22B3B2E5").unwrap());

        let exp = BigUint::from_bytes_be(
            &hex::decode("8E73AFA32D779D5918D08BC8858F4DCEF97C2A24855E6EEB22B3B2E5").unwrap(),
        );
        let h = g.modpow(&exp.clone(), &p.clone());
        // let h = g.modpow(&ZKP::generate_random_number_below(&q), &p);

        (g, h, p, q)
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
        let k = ZKP::generate_random_number_below(&zkp.q);

        let c = ZKP::generate_random_number_below(&zkp.q);

        let y1 = ZKP::exponentiate(&zkp.g, &x, &zkp.p);
        let y2 = ZKP::exponentiate(&zkp.h, &x, &zkp.p);

        let r1 = ZKP::exponentiate(&zkp.g, &k, &zkp.p);
        let r2 = ZKP::exponentiate(&zkp.h, &k, &zkp.p);

        let s = zkp.solve(&k, &c, &x);

        let result = zkp.verify(&r1, &r2, &y1, &y2, &c, &s);
        assert!(result);
    }

    #[test]
    fn test_trivial_invalid_numbers() {
        let (g, h, p, q) = ZKP::get_constants();
        let zkp = ZKP { p, q, g, h };

        let r1 = BigUint::from(0u32);
        let r2 = BigUint::from(0u32);
        let y1 = BigUint::from(0u32);
        let y2 = BigUint::from(0u32);
        let c = BigUint::from(0u32);
        let s = BigUint::from(0u32);

        let result = zkp.verify(&r1, &r2, &y1, &y2, &c, &s);
        println!(
            "r1={}, r2={}, y1={}, y2={}, c={}, s={}, result={}",
            r1, r2, y1, y2, c, s, result
        );
        assert!(!result);
    }

    #[test]
    fn test_zero_values_with_nonzero_challenge() {
        // ZKP setup (using small values for easier testing)
        let zkp = ZKP {
            p: BigUint::from(23u32), // prime
            q: BigUint::from(11u32), // subgroup size
            g: BigUint::from(4u32),  // generator
            h: BigUint::from(9u32),  // another generator
        };

        // Test case: all values are 0 except c (which is non-zero)
        let r1 = BigUint::from(0u32);
        let r2 = BigUint::from(0u32);
        let y1 = BigUint::from(0u32);
        let y2 = BigUint::from(0u32);
        let c = BigUint::from(4u32); // non-zero challenge
        let s = BigUint::from(0u32);

        println!("=== Testing with zero values and non-zero challenge ===");
        println!(
            "r1={}, r2={}, y1={}, y2={}, c={}, s={}",
            r1, r2, y1, y2, c, s
        );

        // Calculate what the verification should check:
        // cond1: r1 == (g^s * y1^c) mod p
        // cond2: r2 == (h^s * y2^c) mod p

        let g_s = zkp.g.modpow(&s, &zkp.p);
        let y1_c = y1.modpow(&c, &zkp.p);
        let h_s = zkp.h.modpow(&s, &zkp.p);
        let y2_c = y2.modpow(&c, &zkp.p);

        println!("g^s mod p = {}", g_s);
        println!("y1^c mod p = {}", y1_c);
        println!("h^s mod p = {}", h_s);
        println!("y2^c mod p = {}", y2_c);

        let cond1_calc = (g_s.clone() * y1_c.clone()).modpow(&BigUint::from(1u32), &zkp.p);
        let cond2_calc = (h_s.clone() * y2_c.clone()).modpow(&BigUint::from(1u32), &zkp.p);

        println!("(g^s * y1^c) mod p = {}", cond1_calc);
        println!("(h^s * y2^c) mod p = {}", cond2_calc);

        let cond1 = r1 == cond1_calc;
        let cond2 = r2 == cond2_calc;

        println!(
            "cond1: r1 == (g^s * y1^c) mod p => {} == {} = {}",
            r1, cond1_calc, cond1
        );
        println!(
            "cond2: r2 == (h^s * y2^c) mod p => {} == {} = {}",
            r2, cond2_calc, cond2
        );

        let result = zkp.verify(&r1, &r2, &y1, &y2, &c, &s);
        println!("Final verification result: {}", result);

        // This should be TRUE because:
        // - g^0 mod p = 1, h^0 mod p = 1
        // - 0^c mod p = 0 for any c > 0
        // - 1 * 0 mod p = 0
        // - So both conditions become: 0 == 0, which is true
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

    #[test]
    fn test_1024bit_constants() {
        let p = BigUint::from_bytes_be(&hex::decode("B10B8F96A080E01DDE92DE5EAE5D54EC52C99FBCFB06A3C69A6A9DCA52D23B616073E28675A23D189838EF1E2EE652C013ECB4AEA906112324975C3CD49B83BFACCBDD7D90C4BD7098488E9C219A73724EFFD6FAE5644738FAA31A4FF55BCCC0A151AF5F0DC8B4BD45BF37DF365C1A65E68CFDA76D4DA708DF1FB2BC2E4A4371").unwrap());
        let q = BigUint::from_bytes_be(
            &hex::decode("F518AA8781A8DF278ABA4E7D64B7CB9D49462353").unwrap(),
        );
        let g = BigUint::from_bytes_be(&hex::decode("A4D1CBD5C3FD34126765A442EFB99905F8104DD258AC507FD6406CFF14266D31266FEA1E5C41564B777E690F5504F213160217B4B01B886A5E91547F9E2749F4D7FBD7D3B9A92EE1909D0D2263F80A76A6A24C087A091F531DBF0A0169B6A28AD662A4D18E73AFA32D779D5918D08BC8858F4DCEF97C2A24855E6EEB22B3B2E5").unwrap());
        let h = g.modpow(&ZKP::generate_random_number_below(&q), &p);

        let zkp = ZKP {
            p: p.clone(),
            q: q.clone(),
            g: g.clone(),
            h: h.clone(),
        };

        let x = ZKP::generate_random_number_below(&zkp.q);
        let k = ZKP::generate_random_number_below(&zkp.q);

        let c = ZKP::generate_random_number_below(&zkp.q);

        let y1 = ZKP::exponentiate(&zkp.g, &x, &zkp.p);
        let y2 = ZKP::exponentiate(&zkp.h, &x, &zkp.p);

        let r1 = ZKP::exponentiate(&zkp.g, &k, &zkp.p);
        let r2 = ZKP::exponentiate(&zkp.h, &k, &zkp.p);

        let s = zkp.solve(&k, &c, &x);

        let result = zkp.verify(&r1, &r2, &y1, &y2, &c, &s);
        assert!(result);
    }

    #[test]
    fn test_verify_sample_data() {
        // ZKP setup (using small values for easier testing)
        let zkp = ZKP {
            p: BigUint::from(23u32), // prime
            q: BigUint::from(11u32), // subgroup size
            g: BigUint::from(4u32),  // generator
            h: BigUint::from(9u32),  // another generator
        };

        println!("=== Successful Cases ===");

        // Successful case 1: correct secret value and random value
        let x1 = BigUint::from(6u32); // secret value
        let k1 = BigUint::from(7u32); // random value
        let c1 = BigUint::from(4u32); // challenge

        let y1_1 = ZKP::exponentiate(&zkp.g, &x1, &zkp.p); // g^x mod p
        let y2_1 = ZKP::exponentiate(&zkp.h, &x1, &zkp.p); // h^x mod p
        let r1_1 = ZKP::exponentiate(&zkp.g, &k1, &zkp.p); // g^k mod p
        let r2_1 = ZKP::exponentiate(&zkp.h, &k1, &zkp.p); // h^k mod p
        let s1 = zkp.solve(&k1, &c1, &x1); // s = k - c*x mod q

        println!("Successful case 1:");
        println!("  x={}, k={}, c={}", x1, k1, c1);
        println!("  y1={}, y2={}", y1_1, y2_1);
        println!("  r1={}, r2={}", r1_1, r2_1);
        println!("  s={}", s1);

        let result1 = zkp.verify(&r1_1, &r2_1, &y1_1, &y2_1, &c1, &s1);
        println!("  Verification result: {}", result1);
        assert!(result1);

        // Successful case 2: with different values
        let x2 = BigUint::from(3u32);
        let k2 = BigUint::from(5u32);
        let c2 = BigUint::from(2u32);

        let y1_2 = ZKP::exponentiate(&zkp.g, &x2, &zkp.p);
        let y2_2 = ZKP::exponentiate(&zkp.h, &x2, &zkp.p);
        let r1_2 = ZKP::exponentiate(&zkp.g, &k2, &zkp.p);
        let r2_2 = ZKP::exponentiate(&zkp.h, &k2, &zkp.p);
        let s2 = zkp.solve(&k2, &c2, &x2);

        println!("Successful case 2:");
        println!("  x={}, k={}, c={}", x2, k2, c2);
        println!("  y1={}, y2={}", y1_2, y2_2);
        println!("  r1={}, r2={}", r1_2, r2_2);
        println!("  s={}", s2);

        let result2 = zkp.verify(&r1_2, &r2_2, &y1_2, &y2_2, &c2, &s2);
        println!("  Verification result: {}", result2);
        assert!(result2);

        println!("\n=== Failed Cases ===");

        // Failed case 1: calculate s with wrong secret value
        let x_fake = BigUint::from(7u32); // wrong secret value
        let s_fake = zkp.solve(&k1, &c1, &x_fake);

        println!("Failed case 1: calculate s with wrong secret value");
        println!("  Correct x={}, Wrong x={}", x1, x_fake);
        println!("  Correct s={}, Wrong s={}", s1, s_fake);

        let result_fake1 = zkp.verify(&r1_1, &r2_1, &y1_1, &y2_1, &c1, &s_fake);
        println!("  Verification result: {}", result_fake1);
        assert!(!result_fake1);

        // Failed case 2: use wrong r1
        let r1_wrong = BigUint::from(1u32); // wrong r1

        println!("Failed case 2: use wrong r1");
        println!("  Correct r1={}, Wrong r1={}", r1_1, r1_wrong);

        let result_fake2 = zkp.verify(&r1_wrong, &r2_1, &y1_1, &y2_1, &c1, &s1);
        println!("  Verification result: {}", result_fake2);
        assert!(!result_fake2);

        // Failed case 3: use wrong r2
        let r2_wrong = BigUint::from(1u32); // wrong r2

        println!("Failed case 3: use wrong r2");
        println!("  Correct r2={}, Wrong r2={}", r2_1, r2_wrong);

        let result_fake3 = zkp.verify(&r1_1, &r2_wrong, &y1_1, &y2_1, &c1, &s1);
        println!("  Verification result: {}", result_fake3);
        assert!(!result_fake3);

        // Failed case 4: use wrong c
        let c_wrong = BigUint::from(1u32); // wrong c

        println!("Failed case 4: use wrong c");
        println!("  Correct c={}, Wrong c={}", c1, c_wrong);

        let result_fake4 = zkp.verify(&r1_1, &r2_1, &y1_1, &y2_1, &c_wrong, &s1);
        println!("  Verification result: {}", result_fake4);
        assert!(!result_fake4);

        // Failed case 5: use wrong y1
        let y1_wrong = BigUint::from(1u32); // wrong y1

        println!("Failed case 5: use wrong y1");
        println!("  Correct y1={}, Wrong y1={}", y1_1, y1_wrong);

        let result_fake5 = zkp.verify(&r1_1, &r2_1, &y1_wrong, &y2_1, &c1, &s1);
        println!("  Verification result: {}", result_fake5);
        assert!(!result_fake5);

        // Failed case 6: use wrong y2
        let y2_wrong = BigUint::from(1u32); // wrong y2

        println!("Failed case 6: use wrong y2");
        println!("  Correct y2={}, Wrong y2={}", y2_1, y2_wrong);

        let result_fake6 = zkp.verify(&r1_1, &r2_1, &y1_1, &y2_wrong, &c1, &s1);
        println!("  Verification result: {}", result_fake6);
        assert!(!result_fake6);

        println!("\n=== Test Completed ===");
        println!("All test cases worked as expected!");
    }
}

// The hexadecimal value of the prime is:

//    p = B10B8F96 A080E01D DE92DE5E AE5D54EC 52C99FBC FB06A3C6
//        9A6A9DCA 52D23B61 6073E286 75A23D18 9838EF1E 2EE652C0
//        13ECB4AE A9061123 24975C3C D49B83BF ACCBDD7D 90C4BD70
//        98488E9C 219A7372 4EFFD6FA E5644738 FAA31A4F F55BCCC0
//        A151AF5F 0DC8B4BD 45BF37DF 365C1A65 E68CFDA7 6D4DA708
//        DF1FB2BC 2E4A4371

//    The hexadecimal value of the generator is:

//    g = A4D1CBD5 C3FD3412 6765A442 EFB99905 F8104DD2 58AC507F
//        D6406CFF 14266D31 266FEA1E 5C41564B 777E690F 5504F213
//        160217B4 B01B886A 5E91547F 9E2749F4 D7FBD7D3 B9A92EE1
//        909D0D22 63F80A76 A6A24C08 7A091F53 1DBF0A01 69B6A28A
//        D662A4D1 8E73AFA3 2D779D59 18D08BC8 858F4DCE F97C2A24
//        855E6EEB 22B3B2E5

//    The generator generates a prime-order subgroup of size:

//    q = F518AA87 81A8DF27 8ABA4E7D 64B7CB9D 49462353
