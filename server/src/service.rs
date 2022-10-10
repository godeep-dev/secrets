//! Service implementation

use async_trait::async_trait;
use service::*;

use crate::db::DbConn;

/// Secrets service implementation
#[derive(Debug, Clone)]
pub struct Service {
    /// DB connection
    db: DbConn,
}

impl Service {
    /// Instantiates a new [Service]
    pub fn new(db: DbConn) -> Self {
        Self { db }
    }
}

#[async_trait]
impl rpc::Handler for Service {
    async fn handle<R>(&self, receiver: R, request: R::Request) -> R::Response
    where
        R: rpc::Receiver,
    {
        let req = match receiver.decode_request::<String>(request).await {
            Ok(ok) => ok,
            Err(err) => return receiver.encode_err(err).await,
        };

        match req.method.as_str() {
            "status" => {
                // let _data = receiver.decode_payload::<(), Error>(&req.data).await;
                let res = self.status().await;
                return receiver.encode_response(res).await;
            }
            m => {
                // Invalid method => return an error response
                return receiver.encode_err(format!("Invalid method: {m}")).await;
            }
        }
    }
}

#[async_trait]
impl SecretsService for Service {
    /// Returns the API status
    async fn status(&self) -> Result<ServiceStatus, Error> {
        todo!()
    }

    /// Signup a new user
    async fn signup(&self, input: SignupInput) -> Result<LoginResponse, Error> {
        todo!()
    }

    /// Login a new user
    async fn login(&self, input: LoginInput) -> Result<LoginResponse, Error> {
        todo!()
    }

    /// Reads a user
    async fn user(&self, token: String, id: String) -> Result<User, Error> {
        todo!()
    }

    /// Deletes a user
    async fn delete_user(&self, token: String, id: String) -> Result<User, Error> {
        todo!()
    }

    /// Add an organization
    async fn add_organization(
        &self,
        token: String,
        organization: OrganizationInput,
    ) -> Result<Organization, Error> {
        todo!()
    }

    /// Reads an organization
    async fn organization(&self, token: String, id: String) -> Result<Organization, Error> {
        todo!()
    }

    /// Deletes an organization
    async fn delete_organization(&self, token: String, id: String) -> Result<Organization, Error> {
        todo!()
    }

    /// Add a project
    async fn add_project(&self, token: String, project: ProjectInput) -> Result<Project, Error> {
        todo!()
    }

    /// Reads a project
    async fn project(&self, token: String, id: String) -> Result<Project, Error> {
        todo!()
    }

    /// Deletes a project
    async fn delete_project(&self, token: String, id: String) -> Result<Project, Error> {
        todo!()
    }

    /// Adds a secret
    async fn add_secret(&self, token: String, secret: SecretInput) -> Result<Secret, Error> {
        todo!()
    }

    /// Reads a secret
    async fn secret(&self, token: String, id: String) -> Result<Secret, Error> {
        todo!()
    }

    /// Update a secret
    async fn update_secret(&self, token: String, secret: Secret) -> Result<Secret, Error> {
        todo!()
    }

    /// Deletes a secret
    async fn delete_secret(&self, token: String, id: String) -> Result<Secret, Error> {
        todo!()
    }
}
