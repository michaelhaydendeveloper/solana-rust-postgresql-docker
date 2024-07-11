use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use tokio_postgres::NoTls;
use std::env;
use std::str::FromStr;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Load environment variables
    let db_user = env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let db_password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let db_name = env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");
    let db_host = env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set");
    let db_port = env::var("POSTGRES_PORT").expect("POSTGRES_PORT must be set");

    let db_url = format!(
        "host={} user={} password={} dbname={} port={}",
        db_host, db_user, db_password, db_name, db_port
    );

    // Connect to PostgreSQL
    let (client, connection) = tokio_postgres::connect(&db_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Create a table
    client.execute(
        "CREATE TABLE IF NOT EXISTS balances (id SERIAL PRIMARY KEY, public_key VARCHAR NOT NULL, balance BIGINT NOT NULL)",
        &[],
    ).await?;

    // Insert a dummy entry
    client.execute(
        "INSERT INTO balances (public_key, balance) VALUES ($1, $2)",
        &[&"DummyPublicKey", &0_i64],
    ).await?;

    // Query the dummy entry
    let rows = client.query("SELECT public_key, balance FROM balances", &[]).await?;

    for row in rows {
        let public_key: &str = row.get(0);
        let balance: i64 = row.get(1);
        println!("Public Key: {}, Balance: {}", public_key, balance);
    }

    // Connect to Solana
    let solana_url = "https://api.mainnet-beta.solana.com"; // Use the appropriate Solana cluster
    let rpc_client = RpcClient::new(solana_url);

    // Query the balance of a specific public key
    let pubkey = Pubkey::from_str("H3Av3R6y5zXYeq7MTZVG2J3eZ1gKVEFyR3F4FVeCjC2H")?;
    let balance = rpc_client.get_balance(&pubkey)?;

    println!("Hello, Solana! The balance of {} is {} lamports.", pubkey, balance);

    Ok(())
}
