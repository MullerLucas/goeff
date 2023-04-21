use hell_core::error::HellResult;

mod endpoints;
mod server;


#[tokio::main]
async fn main() -> HellResult<()> {
    println!("starting goeff-server ...");

    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    server::run_server().await?;

    println!("stopping goeff-server ...");

    Ok(())
}
