use num_bigint::BigUint;

#[derive(Debug, Clone)]
pub struct VerificationParams<'a> {
    pub r1: &'a BigUint,
    pub r2: &'a BigUint,
    pub y1: &'a BigUint,
    pub y2: &'a BigUint,
    pub g: &'a BigUint,
    pub h: &'a BigUint,
    pub c: &'a BigUint,
    pub s: &'a BigUint,
    pub p: &'a BigUint,
}

// g ** x mod p
// output = n ** exp mod p
pub fn exponentiate(n: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
    n.modpow(exponent, modulus)
}

// s = k - c * x mod q
pub fn solve(k: &BigUint, c: &BigUint, x: &BigUint, q: &BigUint) -> BigUint {
    if *k >= c * x {
        (k - c * x).modpow(&BigUint::from(1u32), q)
    } else {
        q - (c * x - k).modpow(&BigUint::from(1u32), q)
    }
}

// unified formula (mathematically equivalent)
pub fn solve_unified(k: &BigUint, c: &BigUint, x: &BigUint, q: &BigUint) -> BigUint {
    let cx = c * x;
    if *k >= cx {
        // in case of k >= c*x: k - c*x mod q
        (k - &cx).modpow(&BigUint::from(1u32), q)
    } else {
        // k < c*x: q - (c*x - k) mod q
        q - (cx - k).modpow(&BigUint::from(1u32), q)
    }
}

// cond1: r1 = g ** s * y1 ** c mod p
// cond2: r2 = h ** s * y2 ** c mod p
pub fn verify(params: &VerificationParams) -> bool {
    let cond1 = *params.r1
        == (params.g.modpow(params.s, params.p) * params.y1.modpow(params.c, params.p))
            .modpow(&BigUint::from(1u32), params.p);
    let cond2 = *params.r2
        == (params.h.modpow(params.s, params.p) * params.y2.modpow(params.c, params.p))
            .modpow(&BigUint::from(1u32), params.p);
    cond1 && cond2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modulo_arithmetic() {
        let q = BigUint::from(11u32);

        // case 1: k >= c * x
        let k1 = BigUint::from(7u32);
        let c1 = BigUint::from(2u32);
        let x1 = BigUint::from(3u32);
        // k1 - c1 * x1 = 7 - 2*3 = 7 - 6 = 1
        let s1 = solve(&k1, &c1, &x1, &q);
        println!("Case 1: k={}, c={}, x={}, s={}", k1, c1, x1, s1);

        // case 2: k < c * x
        let k2 = BigUint::from(3u32);
        let c2 = BigUint::from(2u32);
        let x2 = BigUint::from(3u32);
        // k2 - c2 * x2 = 3 - 2*3 = 3 - 6 = -3
        // -3 mod 11 = 8
        let s2 = solve(&k2, &c2, &x2, &q);
        println!("Case 2: k={}, c={}, x={}, s={}", k2, c2, x2, s2);

        // manual calculation
        let manual_s2 = &q - &(&c2 * &x2 - &k2);
        println!("Manual calculation: q - (c*x - k) = {}", manual_s2);

        assert_eq!(s1, BigUint::from(1u32));
        assert_eq!(s2, BigUint::from(8u32));
        assert_eq!(s2, manual_s2);
    }

    #[test]
    fn test_unified_formula() {
        let q = BigUint::from(11u32);

        // case 1: k >= c * x
        let k1 = BigUint::from(7u32);
        let c1 = BigUint::from(2u32);
        let x1 = BigUint::from(3u32);

        let s1_original = solve(&k1, &c1, &x1, &q);
        let s1_unified = solve_unified(&k1, &c1, &x1, &q);
        println!(
            "Case 1 - Original: {}, Unified: {}",
            s1_original, s1_unified
        );

        // case 2: k < c * x
        let k2 = BigUint::from(3u32);
        let c2 = BigUint::from(2u32);
        let x2 = BigUint::from(3u32);

        let s2_original = solve(&k2, &c2, &x2, &q);
        let s2_unified = solve_unified(&k2, &c2, &x2, &q);
        println!(
            "Case 2 - Original: {}, Unified: {}",
            s2_original, s2_unified
        );

        // both results are the same
        assert_eq!(s1_original, s1_unified);
        assert_eq!(s2_original, s2_unified);
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
    fn test_toy_example() {
        let g = BigUint::from(4u32);
        let h = BigUint::from(9u32);
        let p = BigUint::from(23u32);
        let q = BigUint::from(11u32);

        let x = BigUint::from(6u32);
        let k = BigUint::from(7u32);

        let c = BigUint::from(4u32);

        let y1 = exponentiate(&g, &x, &p);
        let y2 = exponentiate(&h, &x, &p);
        assert_eq!(y1, BigUint::from(2u32));
        assert_eq!(y2, BigUint::from(3u32));

        let r1 = exponentiate(&g, &k, &p);
        let r2 = exponentiate(&h, &k, &p);
        assert_eq!(r1, BigUint::from(8u32));
        assert_eq!(r2, BigUint::from(4u32));

        let s = solve(&k, &c, &x, &q);
        assert_eq!(s, BigUint::from(5u32));

        let params = VerificationParams {
            r1: &r1,
            r2: &r2,
            y1: &y1,
            y2: &y2,
            g: &g,
            h: &h,
            c: &c,
            s: &s,
            p: &p,
        };
        let result = verify(&params);
        assert!(result);

        // fake secret
        let x_fake = BigUint::from(7u32);
        let s_fake = solve(&k, &c, &x_fake, &q);
        let params_fake = VerificationParams {
            r1: &r1,
            r2: &r2,
            y1: &y1,
            y2: &y2,
            g: &g,
            h: &h,
            c: &c,
            s: &s_fake,
            p: &p,
        };
        let result_fake = verify(&params_fake);
        assert!(!result_fake);
    }

    #[test]
    fn test_toy_example2() {
        let g = BigUint::from(16u32);
        let h = BigUint::from(17u32);
        let p = BigUint::from(47u32);
        let q = BigUint::from(23u32);

        let x = BigUint::from(300u32);
        let k = BigUint::from(100u32);

        let c = BigUint::from(200u32);

        let y1 = exponentiate(&g, &x, &p);
        let y2 = exponentiate(&h, &x, &p);

        let r1 = exponentiate(&g, &k, &p);
        let r2 = exponentiate(&h, &k, &p);

        let s = solve(&k, &c, &x, &q);

        let params = VerificationParams {
            r1: &r1,
            r2: &r2,
            y1: &y1,
            y2: &y2,
            g: &g,
            h: &h,
            c: &c,
            s: &s,
            p: &p,
        };
        let result = verify(&params);
        assert!(result);
    }
}
