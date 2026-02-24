use backend_rust::bootstrap;

#[tokio::main]
async fn main() {
    println!("🚀 Starting DJ Arbuzzz Backend...");

    dotenvy::dotenv().ok();
    println!("✓ Environment variables loaded");

    // Initialize rustls crypto provider
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    println!("✓ TLS provider initialized");

    println!("⚙️  Starting bootstrap...");
    bootstrap().await;

    eprintln!("❌ Bootstrap exited unexpectedly");
}
