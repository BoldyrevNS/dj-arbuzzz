pub mod handlers;
pub mod routes;

use std::sync::Arc;

use crate::{
    config::AppConfig,
    infrastucture::{
        cache::client::Cache, database::pool::DbPool,
        repositories::users_repository::UsersRepository,
    },
    service::{
        auth::{
            auth_service::AuthService, restore_service::RestoreService,
            sign_up_service::SignUpService,
        },
        otp_service::OTPService,
        smtp_service::SMTPService,
        token_service::TokenService,
    },
};

pub struct Services {
    pub sign_up_service: Arc<SignUpService>,
    pub restore_service: Arc<RestoreService>,
    pub auth_service: Arc<AuthService>,
}

pub struct AppState {
    pub db_pool: Arc<DbPool>,
    pub config: Arc<AppConfig>,
    pub services: Services,
}

impl AppState {
    pub fn new(config: AppConfig, db_pool: DbPool) -> Self {
        let config = Arc::new(config);
        let otp_service = Arc::new(OTPService::new());
        let smtp_service = Arc::new(SMTPService::new(&config));
        let cache = Arc::new(Cache::new(&config.redis_config.url));
        let db_pool = Arc::new(db_pool);

        let users_repository = Arc::new(UsersRepository::new(db_pool.clone()));
        let token_service = Arc::new(TokenService::new(config.clone()));

        let sign_up_service = Arc::new(SignUpService::new(
            cache.clone(),
            otp_service.clone(),
            smtp_service.clone(),
            users_repository.clone(),
            token_service.clone(),
        ));

        let restore_service = Arc::new(RestoreService::new(
            cache.clone(),
            otp_service.clone(),
            smtp_service.clone(),
            users_repository.clone(),
            token_service.clone(),
        ));

        let auth_service = Arc::new(AuthService::new(
            token_service.clone(),
            users_repository.clone(),
        ));

        let services = Services {
            sign_up_service,
            restore_service,
            auth_service,
        };

        AppState {
            db_pool,
            config,
            services,
        }
    }
}
