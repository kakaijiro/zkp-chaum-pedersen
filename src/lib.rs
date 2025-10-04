use num_bigint::BigUint;

// g ** x mod p
// output = n ** exp mod p
pub fn exponentiate(n: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
    return n.modpow(exponent, modulus);
}

// s = k - c * x mod q
pub fn solve(k: &BigUint, c: &BigUint, x: &BigUint, q: &BigUint) -> BigUint {
    if *k >= c * x {
        return (k - c * x).modpow(&BigUint::from(1u32), q);
    }
    else {
        return q - (c * x - k).modpow(&BigUint::from(1u32), q);
    }
}

// cond1: r1 = g ** s * y1 ** c mod p
// cond2: r2 = h ** s * y2 ** c mod p
pub fn verify(r1: &BigUint, r2: &BigUint, y1: &BigUint, y2: &BigUint, g: &BigUint, h: &BigUint, c: &BigUint, s: &BigUint,p: &BigUint) -> bool {
    let cond1 = *r1 == (g.modpow(s, p) * y1.modpow(c, p)).modpow(&BigUint::from(1u32), p);
    let cond2 = *r2 == (h.modpow(s, p) * y2.modpow(c, p)).modpow(&BigUint::from(1u32), p);
    return cond1 && cond2;
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_toy_example(){
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

        let result = verify(&r1, &r2, &y1, &y2, &g, &h, &c, &s, &p);
        assert!(result);

        // fake secret
        let x_fake = BigUint::from(7u32);
        let s_fake = solve(&k, &c, &x_fake, &q);
        let result_fake = verify(&r1, &r2, &y1, &y2, &g, &h, &c, &s_fake, &p);
        assert!(!result_fake);

    }
}