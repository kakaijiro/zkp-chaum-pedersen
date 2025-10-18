use std::io::stdin;
include!("./zkp_auth.rs");
use auth_client::AuthClient;
use num_bigint::BigUint;
use zkp_chaum_pedersen::ZKP;

#[tokio::main]
async fn main() {
    let mut buf = String::new();
    let (g, h, p, q) = ZKP::get_constants();
    let zkp = ZKP {
        p: p.clone(),
        q: q.clone(),
        g: g.clone(),
        h: h.clone(),
    };

    let mut client = AuthClient::connect("http://127.0.0.1:50051").await.unwrap();
    println!("✅ Client connected to server");

    // Register
    println!("Please enter username:");
    stdin().read_line(&mut buf).expect("Failed to read line");
    let username = buf.trim().to_string();
    buf.clear();

    println!("Please enter password:");
    stdin().read_line(&mut buf).expect("Failed to read line");
    let password = BigUint::from_bytes_be(buf.trim().as_bytes());
    buf.clear();

    let y1 = ZKP::exponentiate(&zkp.g, &password, &zkp.p);
    let y2 = ZKP::exponentiate(&zkp.h, &password, &zkp.p);

    let request = RegisterRequest {
        user: username.clone(),
        y1: y1.to_bytes_be(),
        y2: y2.to_bytes_be(),
    };
    let response = client.register(request).await;
    match response {
        Ok(resp) => {
            println!("✅ User registered successfully: {:?}", resp);
        }
        Err(e) => {
            println!("❌ Error registering user: {:?}", e);
            return;
        }
    }

    // Create authentication challenge
    let k = ZKP::generate_random_number_below(&zkp.q);
    let r1 = ZKP::exponentiate(&zkp.g, &k, &zkp.p);
    let r2 = ZKP::exponentiate(&zkp.h, &k, &zkp.p);

    let request = AuthenticationChallengeRequest {
        user: username,
        r1: r1.to_bytes_be(),
        r2: r2.to_bytes_be(),
    };
    let response = client.create_authentication_challenge(request).await;

    let (auth_id, c) = match response {
        Ok(resp) => {
            let inner = resp.into_inner();
            let auth_id = inner.auth_id.clone();
            let c = inner.c.clone();
            println!(
                "✅ Authentication challenge created successfully: {:?}",
                inner
            );
            (auth_id, c)
        }
        Err(e) => {
            println!("❌ Error creating authentication challenge: {:?}", e);
            return;
        }
    };

    // Verify authentication
    println!("========== verify authentication ==========");
    println!("Please enter password to login:");
    stdin().read_line(&mut buf).expect("Failed to read line");
    let password = BigUint::from_bytes_be(buf.trim().as_bytes());
    buf.clear();

    let c_biguint = BigUint::from_bytes_be(&c);
    let s = zkp.solve(&k, &c_biguint, &password);

    let request = AuthenticationAnswerRequest {
        auth_id,
        s: s.to_bytes_be(),
    };

    let response = client.verify_authentication(request).await;

    let session_id = match response {
        Ok(resp) => {
            let inner = resp.into_inner();
            inner.session_id
        }
        Err(e) => {
            println!("❌ Error verifying authentication: {:?}", e);
            return;
        }
    };

    println!(
        "✅ Authentication verified successfully. Session ID: {}",
        session_id
    );
}
