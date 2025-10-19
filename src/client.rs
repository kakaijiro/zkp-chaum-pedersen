use std::io::stdin;
include!("./zkp_auth.rs");
use auth_client::AuthClient;
use num_bigint::BigUint;
use zkp_chaum_pedersen::ZKP;

fn read_input(prompt: &str) -> Result<String, std::io::Error> {
    println!("{}", prompt);
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;
    Ok(buf.trim().to_string())
}

#[tokio::main]
async fn main() {
    let (g, h, p, q) = ZKP::get_constants();
    let zkp = ZKP {
        p: p.clone(),
        q: q.clone(),
        g: g.clone(),
        h: h.clone(),
    };

    let mut client = match AuthClient::connect("http://127.0.0.1:50051").await {
        Ok(client) => client,
        Err(e) => {
            eprintln!("❌ Failed to connect to the server: {}", e);
            std::process::exit(1);
        }
    };
    println!("✅ Client connected to server");

    // Register
    let username = match read_input("Please enter username:") {
        Ok(name) => name,
        Err(e) => {
            eprintln!("❌ Failed to fetch username: {}", e);
            std::process::exit(1);
        }
    };

    let password_input = match read_input("Please enter password:") {
        Ok(input) => input,
        Err(e) => {
            eprintln!("❌ Failed to fetch password: {}", e);
            std::process::exit(1);
        }
    };
    let password = BigUint::from_bytes_be(password_input.as_bytes());

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
            std::process::exit(1);
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
            std::process::exit(1);
        }
    };

    // Verify authentication
    println!("========== verify authentication ==========");
    let password_input = match read_input("Please enter password to login:") {
        Ok(input) => input,
        Err(e) => {
            eprintln!("❌ Failed to fetch password: {}", e);
            std::process::exit(1);
        }
    };
    let password = BigUint::from_bytes_be(password_input.as_bytes());

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
            std::process::exit(1);
        }
    };

    println!(
        "✅ Authentication verified successfully. Session ID: {}",
        session_id
    );
}
