//! Service traits

use serde::{de::DeserializeOwned, Serialize};

use crate::*;

/// Service method
pub trait ServiceMethod {
    /// Method ID
    const ID: &'static str;

    /// Method parameters type
    type Params: Serialize + DeserializeOwned;

    /// Return value type
    type RetValue: Serialize + DeserializeOwned;
}

// ---------------------------------------------------------------
// Service implementation
//
// NB: This could be derived by a macro on the trait
// ---------------------------------------------------------------

/// Method 'status'
pub struct StatusSrvMethod;

impl ServiceMethod for StatusSrvMethod {
    const ID: &'static str = "status";

    type Params = ();

    type RetValue = ServiceStatus;
}

impl StatusSrvMethod {
    /// Converts a set of inputs to parameters
    pub fn to_params() -> <Self as ServiceMethod>::Params {}
}
