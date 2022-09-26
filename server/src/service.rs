//! Service implementation

use service::{error::Result, *};

use crate::db::DbConn;

/// Service implementation
#[derive(Debug, Clone)]
pub struct ServiceImpl {
    /// DB connection
    db: DbConn,
}

impl ServiceImpl {
    /// Instantiates a new [Service]
    pub fn new(db: DbConn) -> Self {
        Self { db }
    }
}

#[async_trait]
impl Service for ServiceImpl {
    async fn status(&self) -> Result<ServiceStatus> {
        Ok(ServiceStatus {})
    }

    async fn signup(&self, input: SignupInput) -> Result<LoginResponse> {
        todo!()
    }

    async fn login(&self, input: LoginInput) -> Result<LoginResponse> {
        todo!()
    }
}
