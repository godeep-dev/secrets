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

    /// Reads a user
    async fn user(&self, token: String, id: String) -> Result<User> {
        todo!()
    }

    /// Deletes a user
    async fn delete_user(&self, token: String, id: String) -> Result<User> {
        todo!()
    }

    /// Add an organization
    async fn add_organization(
        &self,
        token: String,
        organization: OrganizationInput,
    ) -> Result<Organization> {
        todo!()
    }

    /// Reads an organization
    async fn organization(&self, token: String, id: String) -> Result<Organization> {
        todo!()
    }

    /// Deletes an organization
    async fn delete_organization(&self, token: String, id: String) -> Result<Organization> {
        todo!()
    }

    /// Add a project
    async fn add_project(&self, token: String, project: ProjectInput) -> Result<Project> {
        todo!()
    }

    /// Reads a project
    async fn project(&self, token: String, id: String) -> Result<Project> {
        todo!()
    }

    /// Deletes a project
    async fn delete_project(&self, token: String, id: String) -> Result<Project> {
        todo!()
    }

    /// Adds a secret
    async fn add_secret(&self, token: String, secret: SecretInput) -> Result<Secret> {
        todo!()
    }

    /// Reads a secret
    async fn secret(&self, token: String, id: String) -> Result<Secret> {
        todo!()
    }

    /// Update a secret
    async fn update_secret(&self, token: String, secret: Secret) -> Result<Secret> {
        todo!()
    }

    /// Deletes a secret
    async fn delete_secret(&self, token: String, id: String) -> Result<Secret> {
        todo!()
    }
}
