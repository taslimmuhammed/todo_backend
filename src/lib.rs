use sea_orm::Database;
mod routes;
mod database;
mod utils;
pub async fn run(database_uri:String){
    let database = match Database::connect(database_uri).await{
        Ok(db)=> db,
        Err(e)=>{
            println!("Error Connecting to database, reason :- {e}");
            std::process::exit(1);
        }
    };
    println!("Starting Server at localhost:3000");
    let app = routes::create_routes(database);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}