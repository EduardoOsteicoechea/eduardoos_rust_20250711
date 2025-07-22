use deadpool_postgres::{Pool,Client};
use serde::{Serialize,Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleAuthenticationRequest
{
    pub username:String,
    pub password:String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct SimpleAuthenticationResponse
{
    pub authenticated:bool,
    pub message:String
}

pub async fn authentication_simple_authentication
(
    pool: &Pool,
    username:String,
    password:String
)
-> Result<SimpleAuthenticationResponse, crate::ApiError>
{
    let database_client:Client = pool.get().await?;

    let sql: &str = r#"
        SELECT EXISTS(
            SELECT 1
            FROM 
                users_001
            WHERE
                username=$1
            AND
                password=$2
        );
    "#;

    let row = database_client.query_one(sql, &[&username, &password]).await?;

    let authenticated: bool = row.get(0);

    if authenticated {
        Ok(SimpleAuthenticationResponse{
            authenticated: true,
            message: String::new()
        })
    } else {
        Ok(SimpleAuthenticationResponse{
            authenticated: false,
            message: String::from("Incorrect username or password.")
        })
    }
}
