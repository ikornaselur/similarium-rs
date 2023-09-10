use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> Result<(), similarium::SimilariumError> {
    dotenv().ok();
    similarium::api::run().await
}
