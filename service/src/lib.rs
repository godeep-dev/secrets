//! Secrets service

#![deny(missing_docs)]

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub mod gen;

// ---------------------------------------------------------------
// SERVICE DEFINITION
// ---------------------------------------------------------------

/// The secrets service is the interface between the server and client
#[async_trait]
pub trait SecretsService {
    /// Returns the API status
    async fn status(&self) -> Result<ServiceStatus, Error>;

    /// Signup a new user
    async fn signup(&self, input: SignupInput) -> Result<LoginResponse, Error>;

    /// Login a new user
    async fn login(&self, input: LoginInput) -> Result<LoginResponse, Error>;

    /// Reads a user
    async fn user(&self, token: String, id: String) -> Result<User, Error>;

    /// Deletes a user
    async fn delete_user(&self, token: String, id: String) -> Result<User, Error>;

    /// Add an organization
    async fn add_organization(
        &self,
        token: String,
        organization: OrganizationInput,
    ) -> Result<Organization, Error>;

    /// Reads an organization
    async fn organization(&self, token: String, id: String) -> Result<Organization, Error>;

    /// Deletes an organization
    async fn delete_organization(&self, token: String, id: String) -> Result<Organization, Error>;

    /// Add a project
    async fn add_project(&self, token: String, project: ProjectInput) -> Result<Project, Error>;

    /// Reads a project
    async fn project(&self, token: String, id: String) -> Result<Project, Error>;

    /// Deletes a project
    async fn delete_project(&self, token: String, id: String) -> Result<Project, Error>;

    /// Adds a secret
    async fn add_secret(&self, token: String, secret: SecretInput) -> Result<Secret, Error>;

    /// Reads a secret
    async fn secret(&self, token: String, id: String) -> Result<Secret, Error>;

    /// Update a secret
    async fn update_secret(&self, token: String, secret: Secret) -> Result<Secret, Error>;

    /// Deletes a secret
    async fn delete_secret(&self, token: String, id: String) -> Result<Secret, Error>;
}

// ---------------------------------------------------------------
// ERROR
// ---------------------------------------------------------------

/// Service error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    /// Message
    pub message: String,
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Self { message }
    }
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
