use backend_rust::bootstrap;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Initialize rustls crypto provider
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

    bootstrap().await
}
