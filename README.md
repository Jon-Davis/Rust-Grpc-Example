# Rust Asynchronous Grpc Sample Project
This is an example project implemented in rust. Grpc is handled using Tonic and the backend uses sqlx.

This example project includes:
1. A grpc procedure that returns a greeting to the user. Example: Given "John Doe" as a name, the procedure would return "Hello John Doe!".
2. A grpc procedure that updates a greeting for a user. Example: "John Doe" could have their greeting changed to "Hi".
3. A grpc procedure that returns a stream of all Users and greetings greeting currently in the database.

The project uses an in memory SQLite database as POC for other databases.