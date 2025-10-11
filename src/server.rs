use tonic::{transport::Server, Request, Response, Status};
include!("./zkp_auth.rs");
use auth_server::{Auth, AuthServer};
use num_bigint::BigUint;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Default)]
pub struct AuthImpl {
    pub user_info: Mutex<HashMap<String, UserInfo>>,
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
        todo!()
    }

    async fn verify_authentication(
        &self,
        request: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        todo!()
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
