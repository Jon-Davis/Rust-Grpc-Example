use self::users::{
    user_service_server::UserService, GetAllRequest, GetGreetingRequest, GetGreetingResponse, UpdateUserResponse, User,
};
use crate::users_database::{self, UsersDatabase};
use futures::{stream::BoxStream, StreamExt};
use tonic::{Request, Response, Status};
use tracing::info;

// This macro places the generated rust definitions from the protofile into the users module
pub mod users {
    tonic::include_proto!("users"); 
}

type GrpcResult<T> = Result<Response<T>, Status>; // Shorthand type definition for the return type of Grpc methods

#[derive(Clone)]
pub struct UserServiceInstance {
    database: UsersDatabase,
}

impl UserServiceInstance {
    pub fn new(database: UsersDatabase) -> Self {
        Self { database }
    }
}

#[tonic::async_trait]
impl UserService for UserServiceInstance {
    type GetAllUsersStream = BoxStream<'static, Result<User, Status>>;

    /**
     * Returns a String to the caller that greets the user given their name. By default the service will say "HELLO {User}!"
     */
    async fn get_greeting(&self, request: Request<GetGreetingRequest>) -> GrpcResult<GetGreetingResponse> {
        let user = &request.get_ref().name;
        info!("Received get user request for user {}", user);
        let response = self.database
            .get_user(user).await
            .map_err(|e| Status::internal(e.to_string()))?
            .map(|usr| format!("{} {}!", usr.greeting, usr.name))
            .unwrap_or(format!("HELLO {}!", user.to_uppercase()));
        Ok(Response::new(GetGreetingResponse { response }))
    }

    /**
     * Updates the greeting for a given user, after which calls to get a greeting for that user will say "{greeting} {user}", instead of "HELLO {user}!".
     */
    async fn update_user(&self, request: Request<users::User>) -> GrpcResult<UpdateUserResponse> {
        let user: users_database::User = request.get_ref().to_owned().into();
        info!("Received update user request for user {}, with the greeting of {}", user.name, user.greeting);
        let _response = self.database
            .update_user(&user).await
            .map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(UpdateUserResponse {}))
    }

    /**
     * Streams out all of the users to the requester
     */
    async fn get_all_users(&self, _request: Request<GetAllRequest>) -> GrpcResult<Self::GetAllUsersStream> {
        let users = self.database
            .get_all_users()
            .map_err(|e| Status::internal(e.to_string()))?
            .filter_map(|user| async move {
                match user {
                    Ok(u) => Some(Ok(u.into())),
                    _ => None,
                }
            })
            .boxed();
        Ok(Response::new(users))
    }
}

// Convert from a Database User entry into a Grpc User Message
impl From<users_database::User> for users::User {
    fn from(user: users_database::User) -> Self {
        User {
            name: user.name,
            greeting: user.greeting,
        }
    }
}
// Convert from a Grpc User message into a Database User entry
impl From<users::User> for users_database::User {
    fn from(user: users::User) -> Self {
        users_database::User {
            name: user.name,
            greeting: user.greeting,
        }
    }
}
