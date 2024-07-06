use actix_web::{ web, App, Responder,HttpServer, HttpResponse };
use std::str::FromStr;
use web3::contract::{ Contract, Options };
use web3::types::{ BlockId, BlockNumber };
use web3::{ transports::Http, Web3, types::Address };

// use web3::contract::Options::;

// std::convert::From<u32>;

async fn index() -> impl Responder {
    "Hello world!"
}

async fn getNumber() -> impl Responder {

    let http = Http::new("http://localhost:8545").expect("Failed to create HTTP transport");

    let web3 = Web3::new(http);

    let contract_address = "YOUR_CONTRACT_ADDRESS";

    let contract_abi = include_bytes!("../abi.json");

    let contract = Contract::from_json(
        web3.eth(),
        web3::types::Address::from_str(contract_address).unwrap(),
        contract_abi,
    )
    .expect("Failed to create contract");

    let result = contract
    .query::<u32, _, _, _>("get", (), None, Options::default(), BlockId::Number(BlockNumber::Latest))
    .await;

    match result {
        Ok(value) => HttpResponse::Ok().body(format!("Stored value in contract: {}", value)),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error calling contract function: {}", err)),
    }

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}