//! Generated client and server

use rpc::transports::json::JsonTransport;

// -------------------------------------------------
// SERVER
// -------------------------------------------------

// -------------------------------------------------
// CLIENT
// -------------------------------------------------

/// Client
pub struct Client {
    /// RPC client
    rpc_client: rpc::Client<JsonTransport>,
    /// Token
    token: Option<String>,
}

impl Client {
    /// Instantiates a new [Client]
    pub const fn new() -> Self {
        let sender = JsonTransport::new();
        let rpc_client = rpc::Client::new(sender);
        Self {
            rpc_client,
            token: None,
        }
    }

    /// Authenticates the client
    pub fn authenticate(&mut self, token: impl AsRef<str>) {
        self.token = Some(token.as_ref().to_string());
    }

    /// Deuthenticates the client
    pub fn deauthenticate(&mut self) {
        self.token = None;
    }
}

// NB : The client API could be derived via a macro parameterized
// by the trait definining the service interface
impl Client {
    /// Returns the service status
    pub async fn status(&self) -> Result<ServiceStatus, Error> {
        let request = rpc::Request::new("status", self.token.clone(), ());
        self.rpc_client
            .call::<(), ServiceStatus, Error>(request)
            .await
    }

    /// Signup a new user
    pub async fn signup(&self, input: SignupInput) -> Result<LoginResponse, Error> {
        let request = rpc::Request::new("signup", self.token.clone(), input);
        self.rpc_client
            .call::<SignupInput, LoginResponse, Error>(request)
            .await
    }

    /// Login a new user
    pub async fn login(&self, input: LoginInput) -> Result<LoginResponse, Error> {
        let request = rpc::Request::new("login", self.token.clone(), input);
        self.rpc_client
            .call::<LoginInput, LoginResponse, Error>(request)
            .await
    }

    /// Reads a user
    pub async fn user(&self, id: String) -> Result<User, Error> {
        let request = rpc::Request::new("user", self.token.clone(), id);
        self.rpc_client.call::<String, User, Error>(request).await
    }

    /// Deletes a user
    pub async fn delete_user(&self, id: String) -> Result<User, Error> {
        let request = rpc::Request::new("delete_user", self.token.clone(), id);
        self.rpc_client.call::<String, User, Error>(request).await
    }

    /// Add an organization
    pub async fn add_organization(
        &self,
        organization: OrganizationInput,
    ) -> Result<Organization, Error> {
        let request = rpc::Request::new("add_organization", self.token.clone(), organization);
        self.rpc_client
            .call::<OrganizationInput, Organization, Error>(request)
            .await
    }

    /// Reads an organization
    pub async fn organization(&self, id: String) -> Result<Organization, Error> {
        let request = rpc::Request::new("organization", self.token.clone(), id);
        self.rpc_client
            .call::<String, Organization, Error>(request)
            .await
    }

    /// Deletes an organization
    pub async fn delete_organization(&self, id: String) -> Result<Organization, Error> {
        let request = rpc::Request::new("delete_organization", self.token.clone(), id);
        self.rpc_client
            .call::<String, Organization, Error>(request)
            .await
    }

    /// Add a project
    pub async fn add_project(&self, project: ProjectInput) -> Result<Project, Error> {
        let request = rpc::Request::new("add_project", self.token.clone(), project);
        self.rpc_client
            .call::<ProjectInput, Project, Error>(request)
            .await
    }

    /// Reads a project
    pub async fn project(&self, id: String) -> Result<Project, Error> {
        let request = rpc::Request::new("project", self.token.clone(), id);
        self.rpc_client
            .call::<String, Project, Error>(request)
            .await
    }

    /// Deletes a project
    pub async fn delete_project(&self, id: String) -> Result<Project, Error> {
        let request = rpc::Request::new("delete_project", self.token.clone(), id);
        self.rpc_client
            .call::<String, Project, Error>(request)
            .await
    }

    /// Adds a secret
    pub async fn add_secret(&self, secret: SecretInput) -> Result<Secret, Error> {
        let request = rpc::Request::new("add_secret", self.token.clone(), secret);
        self.rpc_client
            .call::<SecretInput, Secret, Error>(request)
            .await
    }

    /// Reads a secret
    pub async fn secret(&self, id: String) -> Result<Secret, Error> {
        let request = rpc::Request::new("secret", self.token.clone(), id);
        self.rpc_client.call::<String, Secret, Error>(request).await
    }

    /// Update a secret
    pub async fn update_secret(&self, secret: Secret) -> Result<Secret, Error> {
        let request = rpc::Request::new("update_secret", self.token.clone(), secret);
        self.rpc_client.call::<Secret, Secret, Error>(request).await
    }

    /// Deletes a secret
    pub async fn delete_secret(&self, id: String) -> Result<Secret, Error> {
        let request = rpc::Request::new("delete_secret", self.token.clone(), id);
        self.rpc_client.call::<String, Secret, Error>(request).await
    }
}
