//! Secrets service

#![deny(missing_docs)]

use serde::{Deserialize, Serialize};

pub mod error;
#[cfg(feature = "http")]
pub mod http;
pub mod traits;

use error::Result;

/// Re-export of the `async_trait` macro
pub use async_trait::async_trait;

// ---------------------------------------------------------------
// SERVICE DEFINITION
// ---------------------------------------------------------------

/// The [Service] is the interface between the server and client
#[async_trait]
pub trait Service: Clone + Send + Sync + 'static {
    /// Returns the API status
    async fn status(&self) -> Result<ServiceStatus>;

    /// Signup a new user
    async fn signup(&self, input: SignupInput) -> Result<LoginResponse>;

    /// Login a new user
    async fn login(&self, input: LoginInput) -> Result<LoginResponse>;

    /// Reads a user
    async fn user(&self, token: String, id: String) -> Result<User>;

    /// Deletes a user
    async fn delete_user(&self, token: String, id: String) -> Result<User>;

    /// Add an organization
    async fn add_organization(
        &self,
        token: String,
        organization: OrganizationInput,
    ) -> Result<Organization>;

    /// Reads an organization
    async fn organization(&self, token: String, id: String) -> Result<Organization>;

    /// Deletes an organization
    async fn delete_organization(&self, token: String, id: String) -> Result<Organization>;

    /// Add a project
    async fn add_project(&self, token: String, project: ProjectInput) -> Result<Project>;

    /// Reads a project
    async fn project(&self, token: String, id: String) -> Result<Project>;

    /// Deletes a project
    async fn delete_project(&self, token: String, id: String) -> Result<Project>;

    /// Adds a secret
    async fn add_secret(&self, token: String, secret: SecretInput) -> Result<Secret>;

    /// Reads a secret
    async fn secret(&self, token: String, id: String) -> Result<Secret>;

    /// Update a secret
    async fn update_secret(&self, token: String, secret: Secret) -> Result<Secret>;

    /// Deletes a secret
    async fn delete_secret(&self, token: String, id: String) -> Result<Secret>;
}

// ---------------------------------------------------------------
// STATUS
// ---------------------------------------------------------------

/// Service status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatus {}

// ---------------------------------------------------------------
// AUTH
// ---------------------------------------------------------------

/// Signup input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignupInput {
    /// Email
    pub email: String,
    /// Name
    pub name: String,
    /// Password
    pub password: String,
}

/// Login input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginInput {
    /// Email
    pub email: String,
    /// Password
    pub password: String,
}

/// Login response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    /// Token
    pub token: String,
    /// User
    pub user: User,
}

/// User
#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct User {
    /// ID
    pub id: String,
    /// Name
    pub name: String,
    /// Email
    pub email: String,
    /// Password
    pub password: String,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

// ---------------------------------------------------------------
// ORGANIZATIONS
// ---------------------------------------------------------------

/// Organization input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationInput {
    /// Name
    pub name: String,
}

/// Organization
#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct Organization {
    /// ID
    pub id: String,
    /// Name
    pub name: String,
}

impl PartialEq for Organization {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

// ---------------------------------------------------------------
// PROJECTS
// ---------------------------------------------------------------

/// Project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInput {
    /// Organization ID
    pub org_id: String,
    /// Name
    pub name: String,
}

/// Project
#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct Project {
    /// ID
    pub id: String,
    /// Name
    pub name: String,
    /// Organization
    pub organization: Organization,
}

impl PartialEq for Project {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

// ---------------------------------------------------------------
// SECRETS
// ---------------------------------------------------------------

/// Secret input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretInput {
    /// Organization ID
    pub org_id: String,
    /// Project ID
    pub project_id: Option<String>,
    /// Key
    pub key: String,
    /// Value
    pub value: String,
}

/// Secret
#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct Secret {
    /// ID
    pub id: String,
    /// Organization
    pub oeganization: Organization,
    /// Project
    pub project: Option<Project>,
    /// Key
    pub key: String,
    /// Value
    pub value: String,
}

impl PartialEq for Secret {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
