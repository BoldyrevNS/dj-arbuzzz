mod api;
mod config;
mod dto;
mod error;
mod infrastucture;
pub mod schema;
mod service;

pub use crate::api::AppState;
use crate::config::AppConfig;
use crate::infrastucture::database::pool::create_pool;

async fn server_start<'a>(addr: &str, state: AppState) {
    println!("🌐 Binding server to: {}", addr);

    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(l) => {
            println!("✓ Server bound successfully to {}", addr);
            l
        }
        Err(e) => {
            eprintln!("❌ Failed to bind to {}: {}", addr, e);
            panic!("Cannot bind to address");
        }
    };

    println!("🎵 Starting Axum server...");
    let router = api::routes::create_router(state);

    println!("✅ Server is ready and listening on {}", addr);

    if let Err(e) = axum::serve(listener, router.into_make_service()).await {
        eprintln!("❌ Server error: {}", e);
        panic!("Server crashed");
    }
}

pub async fn bootstrap() {
    println!("📋 Loading application configuration...");

    let config = match std::panic::catch_unwind(|| AppConfig::new()) {
        Ok(c) => {
            println!("✓ Configuration loaded");
            println!("  - App port: {}", c.app_port);
            c
        }
        Err(e) => {
            eprintln!("❌ Failed to load configuration: {:?}", e);
            panic!("Configuration error");
        }
    };

    println!("🔌 Connecting to database...");
    let db_pool = match create_pool(&config.db_config.url).await {
        Ok(pool) => {
            println!("✓ Database connected successfully");
            pool
        }
        Err(e) => {
            eprintln!("❌ Failed to connect to database: {}", e);
            eprintln!(
                "   DB URL: {}",
                config.db_config.url.split('@').last().unwrap_or("<hidden>")
            );
            panic!("Database connection failed");
        }
    };

    println!("🎯 Creating application state...");
    let app_state = AppState::new(config, db_pool);

    let addr = format!("0.0.0.0:{}", &app_state.config.app_port);
    println!("🚀 Starting server on {}...", addr);

    server_start(&addr, app_state).await;
}
