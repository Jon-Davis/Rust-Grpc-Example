use std::env;

use futures::stream::BoxStream;
use sqlx::sqlite::Sqlite as Profile;
use sqlx::sqlite::SqlitePoolOptions as PoolOptions;
use sqlx::sqlite::SqliteQueryResult;
use sqlx::Pool;
use tokio::sync::OnceCell;
use tracing::info;

static POOL: OnceCell<Pool<Profile>> = OnceCell::const_new();

#[derive(Clone, Debug)]
pub struct UsersDatabase {
    connection_pool: &'static Pool<Profile>,
}

impl UsersDatabase {
    /**
     * Initializes a new UserDatabase, for this example an in memory sqlite databases is opened and a USERS table is created
     */
    pub async fn new() -> Self {
        info!("Initializing sqlite database");
        
        // We need a 'static reference to the connection pool for sqlx and tonic to work together so we create a
        // initialize a 'static OnceCell that contains our connection pool. We then store a reference to the OnceCell rather
        // than the actual connection pool.
        let connection_pool = POOL.get_or_init(|| async {
            let connection_string = env::var("DB_CONNECTION_STRING")    // This will default to "sqlite::memory:" when running through cargo
                .expect("FATAL: DB_CONNECTION_STRING environment variable must be defined");
            let pool = PoolOptions::new()
                .connect(&connection_string)
                .await
                .expect("FATAL: Couldn't connect to database");
            // Initialize table because it's probably in memory
            let _initialize = sqlx::query("CREATE TABLE IF NOT EXISTS USERS(name TEXT PRIMARY KEY, greeting TEXT)")
                .execute(&pool)
                .await
                .expect("FATAL: Failed to initialize database");
            pool
        }).await;

        UsersDatabase { connection_pool }
    }

    /**
     * Returns a User struct, which is the entry in the database that matches the inputted name
     */
    pub async fn get_user(&self, user: &str) -> sqlx::Result<Option<User>> {
        sqlx::query_as::<_, User>("SELECT name, greeting FROM USERS WHERE UPPER(name) = UPPER(?)")
            .bind(user)
            .fetch_optional(self.connection_pool)
            .await
    }

    /**
     * Updates the greeting for a specific user in the database
     */
    pub async fn update_user(&self, user: &User) -> sqlx::Result<SqliteQueryResult> {
        sqlx::query("INSERT INTO USERS (name, greeting) VALUES (UPPER(?), UPPER(?)) ON CONFLICT(name) DO UPDATE SET greeting=excluded.greeting")
            .bind(&user.name)
            .bind(&user.greeting)
            .execute(self.connection_pool)
            .await
    }

    /**
     * Returns a stream of all Users in the database
     */
    pub fn get_all_users(&self) -> sqlx::Result<BoxStream<'static, sqlx::Result<User>>> {
        let result = sqlx::query_as::<_, User>("SELECT * FROM USERS").fetch(self.connection_pool);
        Ok(result.into())
    }
}

#[derive(sqlx::FromRow)]
pub struct User {
    pub name: String,
    pub greeting: String,
}
