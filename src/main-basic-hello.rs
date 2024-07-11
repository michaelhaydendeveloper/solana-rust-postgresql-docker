use tokio_postgres::{NoTls, Error};
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let db_user = env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let db_password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let db_name = env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");
    let db_host = env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set");
    let db_port = env::var("POSTGRES_PORT").expect("POSTGRES_PORT must be set");

    let db_url = format!(
        "host={} user={} password={} dbname={} port={}",
        db_host, db_user, db_password, db_name, db_port
    );

    let (client, connection) = tokio_postgres::connect(&db_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows = client.query("SELECT 1 + 1", &[]).await?;

    for row in rows {
        let sum: i32 = row.get(0);
        println!("Hello, World! The result of 1 + 1 is {}", sum);
    }

    Ok(())
}
