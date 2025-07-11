// ~/rust-http-server-80/api/src/lib.rs

use deadpool_postgres::{Pool, Client, PoolError};
use serde::{Serialize, Deserialize};
use tokio_postgres::Error as PgError; // PgError is tokio_postgres::Error
use std::fmt;
use chrono::{DateTime,Utc};


// Define a custom error type to encompass all possible errors
#[derive(Debug)]
pub enum ApiError {
    Pool(PoolError),
    Database(PgError),
    // You can add other error types here if your API grows, e.g., IoError, SerializationError
}

// Implement From traits to convert from underlying error types to ApiError
impl From<PoolError> for ApiError {
    fn from(err: PoolError) -> Self {
        ApiError::Pool(err)
    }
}

impl From<PgError> for ApiError {
    fn from(err: PgError) -> Self {
        ApiError::Database(err)
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Pool(e) => write!(f, "Connection Pool Error: {}", e),
            ApiError::Database(e) => write!(f, "Database Error: {}", e),
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseCheckResult {
    pub database_name: String,
    pub exists: bool,
    pub message: String,
}

pub async fn check_database_exists(pool: &Pool, db_name: &str) -> Result<DatabaseCheckResult, ApiError> {
    let client: Client = pool.get().await?; // The `?` operator will now convert PoolError to ApiError::Pool

    let query = "SELECT EXISTS (SELECT 1 FROM pg_database WHERE datname = $1)";
    let row = client.query_one(query, &[&db_name]).await?; // The `?` operator will now convert PgError to ApiError::Database
    let exists: bool = row.get(0);
    let message = if exists {
        format!("Database '{}' exists.", db_name)
    } else {
        format!("Database '{}' does not exist.", db_name)
    };

    Ok(DatabaseCheckResult {
        database_name: db_name.to_string(),
        exists,
        message,
    })
}

pub async fn create_todo_tasks_table(pool: &Pool) -> Result<(), ApiError> {
    // Acquire a client connection from the pool.
    // The `?` operator will automatically convert `sqlx::Error` (e.g., PoolError) into `ApiError`.
    let client: Client = pool.get().await?;

    // SQL query to create the 'todo_tasks' table if it doesn't already exist.
    // We use TIMESTAMPTZ for timestamps to store time with timezone information,
    // which is generally recommended for robust applications.
    let create_table_query = r#"
        CREATE TABLE IF NOT EXISTS todo_tasks (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            executor VARCHAR(255),
            importance INTEGER NOT NULL CHECK (importance >= 1 AND importance <= 5),
            category VARCHAR(255),
            description TEXT,
            creation_timestamp TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            expected_end_timestamp TIMESTAMPTZ,
            stops_number INTEGER NOT NULL DEFAULT 0,
            last_stop_timestamp TIMESTAMPTZ,
            continuations_number INTEGER NOT NULL DEFAULT 0,
            last_continuation_timestamp TIMESTAMPTZ,
            last_update TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
    "#;

    // Execute the SQL query.
    // The `?` operator will automatically convert `sqlx::Error` (e.g., PgError) into `ApiError`.
    client.execute(create_table_query, &[]).await?;

    // Log success or return Ok(())
    println!("Table 'todo_tasks' checked/created successfully.");
    Ok(())
}





#[derive(Debug, Serialize, Deserialize)]
pub struct TodoTask {
    pub id: i32,
    pub name: String,
    pub executor: Option<String>,
    pub importance: i32,
    pub category: Option<String>,
    pub description: Option<String>,
    pub creation_timestamp: DateTime<Utc>,
    pub expected_end_timestamp: Option<DateTime<Utc>>,
    pub stops_number: i32,
    pub last_stop_timestamp: Option<DateTime<Utc>>,
    pub continuations_number: i32,
    pub last_continuation_timestamp: Option<DateTime<Utc>>,
    pub last_update: DateTime<Utc>,
}

/// Reads all rows from the 'todo_tasks' table.
///
/// # Arguments
/// * `pool` - A reference to the PostgreSQL connection pool.
///
/// # Returns
/// `Ok(Vec<TodoTask>)` containing all tasks if successful.
/// `Err(ApiError)` if there was an error connecting to the database or executing the query.
pub async fn read_all_todo_tasks(pool: &Pool) -> Result<Vec<TodoTask>, ApiError> {
    let client: Client = pool.get().await?;

    let query = "SELECT
        id,
        name,
        executor,
        importance,
        category,
        description,
        creation_timestamp,
        expected_end_timestamp,
        stops_number,
        last_stop_timestamp,
        continuations_number,
        last_continuation_timestamp,
        last_update
        FROM todo_tasks ORDER BY id ASC"; // Order by ID for consistent results

    let rows = client.query(query, &[]).await?; // Execute the query with no parameters

    let tasks: Vec<TodoTask> = rows.into_iter().map(|row| {
        TodoTask {
            id: row.get("id"),
            name: row.get("name"),
            executor: row.get("executor"),
            importance: row.get("importance"),
            category: row.get("category"),
            description: row.get("description"),
            creation_timestamp: row.get("creation_timestamp"),
            expected_end_timestamp: row.get("expected_end_timestamp"),
            stops_number: row.get("stops_number"),
            last_stop_timestamp: row.get("last_stop_timestamp"),
            continuations_number: row.get("continuations_number"),
            last_continuation_timestamp: row.get("last_continuation_timestamp"),
            last_update: row.get("last_update"),
        }
    }).collect();

    Ok(tasks)
}






/// Struct for inserting a new TodoTask, excluding auto-generated/default fields.
#[derive(Debug, Serialize, Deserialize)]
pub struct NewTodoTask {
    pub name: String,
    pub executor: Option<String>,
    pub importance: i32,
    pub category: Option<String>,
    pub description: Option<String>,
    pub expected_end_timestamp: Option<DateTime<Utc>>,
}


/// Inserts a new task into the 'todo_tasks' table.
///
/// # Arguments
/// * `pool` - A reference to the PostgreSQL connection pool.
/// * `new_task` - The `NewTodoTask` struct containing the data for the new task.
///
/// # Returns
/// `Ok(i32)` the ID of the newly inserted task if successful.
/// `Err(ApiError)` if there was an error connecting to the database or executing the query.
pub async fn insert_todo_task(pool: &Pool, new_task: NewTodoTask) -> Result<i32, ApiError> {
    let client: Client = pool.get().await?;

    // SQL query to insert a new task.
    // We explicitly list columns to insert into, and use RETURNING id to get the generated ID.
    let insert_query = r#"
        INSERT INTO todo_tasks (
            name,
            executor,
            importance,
            category,
            description,
            expected_end_timestamp
        ) VALUES (
            $1, $2, $3, $4, $5, $6
        )
        RETURNING id;
    "#;

    // Execute the insert query with the provided task data as parameters.
    let row = client.query_one(
        insert_query,
        &[
            &new_task.name,
            &new_task.executor,
            &new_task.importance,
            &new_task.category,
            &new_task.description,
            &new_task.expected_end_timestamp,
        ],
    ).await?;

    let new_id: i32 = row.get("id");
    println!("Inserted new task with ID: {}", new_id);
    Ok(new_id)
}
