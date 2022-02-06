use users_database::UsersDatabase;
use users_service::{users::user_service_server::UserServiceServer, UserServiceInstance};
use tonic::transport::Server;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod users_database;
mod users_service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Enable async logging framework
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Initialize service
    let addr = "[::1]:50051".parse()?;
    let database = UsersDatabase::new().await;
    let user_service = UserServiceInstance::new(database.clone());

    // launch
    Server::builder()
        .add_service(UserServiceServer::new(user_service))
        .serve(addr)
        .await?;

    Ok(())
}
