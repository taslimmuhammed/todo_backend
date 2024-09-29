use dotenvy::dotenv;
use std::env;
use todo::run;
#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_uri = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    println!("{}", database_uri);
    run(database_uri).await;
}