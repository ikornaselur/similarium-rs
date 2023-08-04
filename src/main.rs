use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    similarium::run().await
}
