// Core shared types and utilities

pub mod errors;
pub mod result;
pub mod types;

pub use errors::{ApiError, ApplicationError, DomainError, InfrastructureError, UseCaseError};
pub use result::Result;
pub use types::*;
