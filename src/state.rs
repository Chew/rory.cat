//! # App State
//!
//! AppState is useful for providing root-level data that is available to all routes in the same scope.
//! An application's State can be accessed with the web::Data<T> extractor where T is the type of state.
//! State is also accessible via middleware.
//!
//! State docs: https://actix.rs/docs/application

use crate::DbPool;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: DbPool
}
