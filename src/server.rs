use tonic::{transport::Server, Code, Request, Response, Status};
include!("./zkp_auth.rs");
use auth_server::{Auth, AuthServer};
use num_bigint::BigUint;
use std::collections::HashMap;
use std::sync::Mutex;
use zkp_chaum_pedersen::ZKP;

#[derive(Debug, Default)]
pub struct AuthImpl {
    pub user_info: Mutex<HashMap<String, UserInfo>>,
    pub auth_id_to_user: Mutex<HashMap<String, String>>,
}

#[derive(Debug, Default)]
pub struct UserInfo {
    // registration
    pub user_name: String,
    pub y1: BigUint,
    pub y2: BigUint,

    // authentication challenge
    pub r1: BigUint,
    pub r2: BigUint,

    // verification
    pub c: BigUint,
    pub s: BigUint,
    pub session_id: String,
}

#[tonic::async_trait]
impl Auth for AuthImpl {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        println!("Processing register request: {:?}", request);

        let request = request.into_inner();
        let user_info = UserInfo {
            user_name: request.user.clone(),
            y1: BigUint::from_bytes_be(&request.y1),
            y2: BigUint::from_bytes_be(&request.y2),
            ..UserInfo::default()
        };
        let user_info_hashmap = &mut self.user_info.lock().unwrap();
        user_info_hashmap.insert(request.user.clone(), user_info);

        Ok(Response::new(RegisterResponse {}))
    }

    async fn create_authentication_challenge(
        &self,
        request: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        println!("Processing challenge request: {:?}", request);

        let request = request.into_inner();
        let user_name = request.user.clone();
        let user_info_hashmap = &mut self.user_info.lock().unwrap();

        if let Some(user_info) = user_info_hashmap.get_mut(&user_name) {
            user_info.r1 = BigUint::from_bytes_be(&request.r1);
            user_info.r2 = BigUint::from_bytes_be(&request.r2);

            let (_, _, _, q) = ZKP::get_constants();
            let c = ZKP::generate_random_number_below(&q);
            let auth_id = ZKP::generate_random_string(12);

            user_info.c = c.clone();

            let auth_id_to_user = &mut self.auth_id_to_user.lock().unwrap();
            auth_id_to_user.insert(auth_id.clone(), user_name);

            Ok(Response::new(AuthenticationChallengeResponse {
                auth_id,
                c: c.to_bytes_be(),
            }))
        } else {
            Err(Status::new(
                Code::NotFound,
                format!("User: {} not found in the database", user_name),
            ))
        }
    }

    async fn verify_authentication(
        &self,
        request: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        println!("Processing verification request: {:?}", request);

        let request = request.into_inner();
        let auth_id = request.auth_id.clone();
        let user_info_hashmap = &mut self.auth_id_to_user.lock().unwrap();

        if let Some(user_name) = user_info_hashmap.get(&auth_id) {
            let user_info_hashmap = &mut self.user_info.lock().unwrap();
            let user_info = user_info_hashmap.get_mut(user_name).unwrap();

            // verification
            let s = request.s.clone();
            let (g, h, p, q) = ZKP::get_constants();
            let zkp = ZKP { p, q, g, h };
            let verification = zkp.verify(
                &user_info.r1,
                &user_info.r2,
                &user_info.y1,
                &user_info.y2,
                &user_info.c,
                &BigUint::from_bytes_be(&s),
            );
            println!("verification: {}", verification);

            if verification {
                let session_id = ZKP::generate_random_string(12);
                user_info.session_id = session_id.clone();
                Ok(Response::new(AuthenticationAnswerResponse { session_id }))
            } else {
                Err(Status::new(
                    Code::PermissionDenied,
                    format!("AuthId: {} is not verified", auth_id),
                ))
            }
        } else {
            Err(Status::new(
                Code::NotFound,
                format!("AuthId: {} not found in the database", auth_id),
            ))
        }
    }
}

#[tokio::main]
async fn main() {
    let addr: String = "127.0.0.1:50051".to_string();
    let auth_impl = AuthImpl::default();

    println!("🚀 Starting server on {}...", addr);
    println!("📡 Server is ready to accept connections");

    match Server::builder()
        .add_service(AuthServer::new(auth_impl))
        .serve(addr.parse().expect("Invalid address"))
        .await
    {
        Ok(_) => println!("✅ Server stopped gracefully"), // never executed
        Err(e) => {
            eprintln!("❌ Failed to start server: {}", e);
            eprintln!("💡 Try using a different port or check if the address is available");
        }
    }
}
