use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::env;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let address = args
        .next()
        .unwrap_or_else(|| "11111111111111111111111111111111".to_string());
    let rpc_url = args
        .next()
        .or_else(|| env::var("SOLANA_RPC_URL").ok())
        .unwrap_or_else(|| "https://api.devnet.solana.com".to_string());

    let pubkey = Pubkey::from_str(&address)?;
    let client = RpcClient::new(rpc_url.clone());
    let balance = client.get_balance(&pubkey).await?;

    let sol = balance as f64 / 1_000_000_000.0;
    println!("RPC: {}", rpc_url);
    println!("地址: {}", pubkey);
    println!("余额: {} lamports ({:.9} SOL)", balance, sol);

    Ok(())
}
