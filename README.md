# Rust Asynchronous Grpc Sample Project
This is an example project implemented in rust. Grpc is handled using tonic and the backend uses sqlx.

This example project includes:
1. An endpoint that returns a greeting to the user. 
    - Example: Given "John Doe" as a name, the procedure would return "Hello John Doe!".
2. An endpoint that updates a greeting for a user. 
    - Example: "John Doe" could have their greeting changed to "Hi".
3. An endpoint that returns a stream of all Users and greetings currently in the database.

The project uses an in memory SQLite database as POC for other databases.
