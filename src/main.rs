use dotenvy::dotenv;

#[actix_web::main]
async fn main() {
    dotenv().ok();

    similarium::run().await.unwrap()
}
