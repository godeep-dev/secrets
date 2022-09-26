//! Secrets service

#![deny(missing_docs)]

use serde::{Deserialize, Serialize};

pub mod error;
#[cfg(feature = "http")]
pub mod http;

use error::Result;

/// Re-export of the `async_trait` macro
pub use async_trait::async_trait;

// ---------------------------------------------------------------
// DEFINITION
// ---------------------------------------------------------------

/// The [Service] is the interface between the server and client
#[async_trait]
pub trait Service {
    /// Returns the API status
    async fn status(&self) -> Result<ServiceStatus>;

    /// Signup a new user
    async fn signup(&self, input: SignupInput) -> Result<LoginResponse>;

    /// Login a new user
    async fn login(&self, input: LoginInput) -> Result<LoginResponse>;
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

/// Organization
#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct Organization {
    /// ID
    pub id: u32,
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
#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct Project {
    /// ID
    pub id: u32,
    /// Name
    pub name: String,
}

impl PartialEq for Project {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

// ---------------------------------------------------------------
// SECRETS
// ---------------------------------------------------------------

/// Secret
#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct Secret {
    /// ID
    pub id: u32,
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
