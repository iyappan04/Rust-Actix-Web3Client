#[tokio::main]
async fn main() -> web3::Result<()> {

    let transport = web3::transports::Http::new("http://localhost:7545")?;

    let web3 = web3::Web3::new(transport);

    println!("Calling accounts.");

    let mut accounts = web3.eth().accounts().await?;

    println!("Accounts: {:?}", accounts[0]);

    for account in accounts {
        let balance = web3.eth().balance(account, None).await?;
        println!("Balance of {:?}: {}", account, balance);
    }
    
    Ok(())
}