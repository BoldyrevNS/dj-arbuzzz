pub mod handlers;
pub mod routes;

use std::sync::Arc;

use crate::{
    config::AppConfig,
    infrastucture::{cache::client::Cache, database::pool::DbPool},
    service::{
        auth::sign_up_service::SignUpService, otp_service::OTPService, smtp_service::SMTPService,
    },
};

pub struct Services {
    pub sign_up_service: Arc<SignUpService>,
}

pub struct AppState {
    pub db_pool: Arc<DbPool>,
    pub config: AppConfig,
    pub services: Services,
}

impl AppState {
    pub fn new(config: AppConfig, db_pool: DbPool) -> Self {
        let otp_service = Arc::new(OTPService::new());
        let smtp_service = Arc::new(SMTPService::new(&config));
        let cache = Arc::new(Cache::new(&config.redis_config.url));
        let db_pool = Arc::new(db_pool);
        let sign_up_service = Arc::new(SignUpService::new(
            db_pool.clone(),
            cache.clone(),
            otp_service.clone(),
            smtp_service.clone(),
        ));

        let services = Services { sign_up_service };
        AppState {
            db_pool,
            config,
            services,
        }
    }
}
