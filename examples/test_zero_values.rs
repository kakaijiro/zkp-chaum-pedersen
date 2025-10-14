use num_bigint::BigUint;
use zkp_chaum_pedersen::ZKP;

fn main() {
    // ZKP setup (using small values for easier testing)
    let zkp = ZKP {
        p: BigUint::from(23u32), // prime
        q: BigUint::from(11u32), // subgroup size
        g: BigUint::from(4u32),  // generator
        h: BigUint::from(9u32),  // another generator
    };

    // Test case: all values are 0 except c (which is generated randomly)
    let r1 = BigUint::from(0u32);
    let r2 = BigUint::from(0u32);
    let y1 = BigUint::from(0u32);
    let y2 = BigUint::from(0u32);
    let c = BigUint::from(4u32); // non-zero challenge
    let s = BigUint::from(0u32);

    println!("=== Testing with zero values ===");
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
}
