use anyhow::Result;
use crm::pb::{
    user_service_server::{UserService, UserServiceServer},
    CreateUserRequest, GetUsersRequest, User,
};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct UserServer;

#[tonic::async_trait]
impl UserService for UserServer {
    async fn get_user(&self, request: Request<GetUsersRequest>) -> Result<Response<User>, Status> {
        let req = request.into_inner();
        println!("request: {:?}", req);
        Ok(Response::new(User::new(1, "Alice", "alice@qq.com")))
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<User>, Status> {
        let input = request.into_inner();
        println!("request: {:?}", input);
        let user = User::new(1, &input.name, &input.email);
        Ok(Response::new(user))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = UserServer;
    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(UserServiceServer::new(greeter))
        .serve(addr)
        .await?;
    Ok(())
}
