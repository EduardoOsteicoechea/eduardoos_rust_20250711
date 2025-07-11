use axum::{
    routing::{get, post},
    Router,
    response::{Html,Json, IntoResponse},
    Extension,
};
use std::{net::SocketAddr, path::PathBuf, env};
use axum_server::tls_rustls::RustlsConfig;
use web_pages::page_home::retrieve_page_html_string;
use api::{
    check_database_exists,
    create_todo_tasks_table,
    read_all_todo_tasks,
    insert_todo_task,
    DatabaseCheckResult,
    ApiError,
    TodoTask,
    NewTodoTask,
};
use deadpool_postgres::{Pool,Manager,Config,Runtime};
use tokio_postgres::NoTls;
use std::fmt;
use serde_json;
use tower_http::services::ServeDir;
use axum::http::StatusCode;
use axum::handler::HandlerWithoutStateExt;






#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cert_path = PathBuf::from("cert.pem");
    let key_path = PathBuf::from("key.pem");

    let config = RustlsConfig::from_pem_file(cert_path, key_path)
        .await
        .expect("Failed to load TLS certificates");

    let database_environment = DatabaseEnvironmentModel::load_variables_from_environment();
    println!("Database environment model loaded.");
    
    let mut pg_config = tokio_postgres::Config::new();
    pg_config.host(database_environment.host);
    pg_config.user(database_environment.user);
    pg_config.password(database_environment.password);
    pg_config.dbname(database_environment.database_name);
    pg_config.port(database_environment.port);

    let mgr = Manager::new(pg_config, NoTls);

    let dbpool = Pool::builder(mgr)
        .runtime(Runtime::Tokio1)
        .build()?;

    let project_root = env::current_dir()
    .expect("Failed to get current working directory");
    let static_files_path = project_root.join("static");

    let not_found_service = handler_404.into_service();
    let static_files_service = ServeDir::new(static_files_path)
        .not_found_service(not_found_service);

    let app = Router::new()
        .route("/", get(page_home_handler))
        .route("/api/database_exists",get(database_exists_handler))
        .route("/api/create_todo_tasks_table",get(create_todo_tasks_table_handler))
        .route("/api/view_all_todo_tasks",get(view_all_todo_tasks_route_handler))
        .route("/api/create_todo_task",post(create_todo_task_route_handler))
        .layer(Extension(dbpool))
        .fallback_service(static_files_service);

    let addr = SocketAddr::from(([0, 0, 0, 0], 443));

    println!("listening on https://{}", addr);

    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|e| {
            eprintln!("Server error: {}", e);
            std::process::exit(1);
        });

    Ok(())
}

struct DatabaseEnvironmentModel{
    host:String,
    user:String,
    password:String,
    database_name:String,
    port:u16
}

impl DatabaseEnvironmentModel{
    fn load_variables_from_environment() -> Self {
        let host = env::var("DATABASEHOST")
            .expect("DATABASEHOST environment variable not set");
        let user = env::var("DATABASEUSER")
            .expect("DATABASEUSER environment variable not set");
        let password = env::var("DATABASEPASSWORD")
            .expect("DATABASEPASSWORD environment variable not set");
        let database_name = env::var("DATABASENAME")
            .expect("DATABASENAME environment variable not set");
        let database_port_as_string = env::var("DATABSEPORT").unwrap_or_else(|_| "5432".to_string());
        let port:u16 = database_port_as_string.parse()
            .expect("DATABASEPORT environment variable is not a valid u16");

        DatabaseEnvironmentModel{
            host,
            user,
            password,
            database_name,
            port
        }
    }
}


async fn page_home_handler() -> Html<String>{
    let a = retrieve_page_html_string().await;
    Html(a)
}

async fn database_exists_handler(Extension(pool):Extension<Pool>,)->Json<DatabaseCheckResult>{
    let db_name_to_check = "eduardoosserver".to_string();
    match api::check_database_exists(&pool,&db_name_to_check).await{
        Ok(result)=>Json(result),
        Err(e)=>{
            eprintln!("Database check error for '{}': {}",db_name_to_check,e);
            Json(DatabaseCheckResult{
                database_name: db_name_to_check,
                exists:false,
                message:format!("Failed to check database existence: {}",e)
            })
        }
    }
}

async fn create_todo_tasks_table_handler(Extension(pool): Extension<Pool>) -> Json<serde_json::Value>{
    match api::create_todo_tasks_table(&pool).await{
        Ok(_)=>{
            println!("Table 'todo_tasks' creation/check initiated successfully.");
            Json(serde_json::json!({
                "status":"success",
                "message":"Table 'todo_tasks' created or already existing."
            }))
        },
        Err(e)=>{
            eprintln!("Erro creating the table: {}",e);
            Json(serde_json::json!({
                "status":"error",
                "message":format!("failed to create/check the 'todo_tasks' table: {}",e)
            }))
        }
    }
}


async fn view_all_todo_tasks_route_handler(Extension(pool): Extension<Pool>) -> Json<serde_json::Value> {
    match api::read_all_todo_tasks(&pool).await {
        Ok(tasks) => {
            println!("Successfully retrieved {} todo tasks.", tasks.len());
            Json(serde_json::to_value(tasks).unwrap_or_else(|e| {
                eprintln!("Error serializing tasks to JSON: {}", e);
                serde_json::json!({
                    "status": "error",
                    "message": format!("Failed to serialize tasks: {}", e)
                })
            }))
        },
        Err(e) => {
            eprintln!("Error reading all todo tasks: {}", e);
            Json(serde_json::json!({
                "status": "error",
                "message": format!("Failed to retrieve todo tasks: {}", e)
            }))
        }
    }
}

async fn create_todo_task_route_handler(
    Extension(pool): Extension<Pool>,
    Json(new_task_data): Json<NewTodoTask>,
) -> Json<serde_json::Value> {
    match api::insert_todo_task(&pool, new_task_data).await {
        Ok(new_id) => {
            println!("Successfully inserted new task with ID: {}", new_id);
            Json(serde_json::json!({
                "status": "success",
                "message": "Task created successfully.",
                "id": new_id
            }))
        },
        Err(e) => {
            eprintln!("Error inserting new task: {}", e);
            Json(serde_json::json!({
                "status": "error",
                "message": format!("Failed to create task: {}", e)
            }))
        }
    }
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404 Not Found")
}

