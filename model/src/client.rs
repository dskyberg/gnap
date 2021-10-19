use jsonwebtoken::Algorithm;
use serde::{Serialize, Deserialize};
use errors::GnapError;
use crate::oauth::{AcrValueType, ApplicationType, GrantType, ResponseType, SubjectType};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Client {
    pub redirect_uris: Vec<String>,
    pub client_name: String,
    pub contacts: Option<Vec<String>>,
    pub application_type: Option<ApplicationType>,
    pub response_types: Option<Vec<ResponseType>>,
    pub grant_types: Option<Vec<GrantType>>,
    pub client_uri: Option<String>,
    pub policy_uri: Option<String>,
    pub tos_uri: Option<String>,
    pub jwks_uri: Option<String>,
    pub logo_uri: Option<String>,
    pub subjec_type: Option<SubjectType>,
    pub sector_identifier_uri: Option<String>,
    pub subject_type: Option<String>,
    pub id_token_signed_response_alg: Option<Algorithm>,
    pub id_token_encrypted_response_enc: Option<String>,
    pub userinfo_signed_response_alg: Option<Algorithm>,
    pub userinfo_encrypted_response_alg: Option<Algorithm>,
    pub userinfo_encrypted_response_enc: Option<String>,
    pub request_object_signing_alg: Option<Algorithm>,
    pub request_object_encryption_alg: Option<Algorithm>,
    pub request_object_encryption_enc: Option<String>,
    pub token_endpoint_auth_method: Option<String>,
    pub token_endpoint_auth_signing_alg: Option<Algorithm>,
    pub default_max_age: Option<String>,
    pub require_auth_time: Option<String>,
    pub default_acr_values: Option<Vec<AcrValueType>>,
    pub initiate_login_uri: Option<String>,
    pub request_uris: Option<Vec<String>>,
}

/// Client defined by OIDC
///  This needs to be updated properly for GNAP, but should work for now.
impl Client {
    pub fn new(redirect_uris: Vec<String>, client_name: String) -> Self {
        Self {
            redirect_uris,
            client_name,
            contacts: None,
            application_type: None,
            response_types: None,
            grant_types: None,
            client_uri: None,
            policy_uri: None,
            tos_uri: None,
            jwks_uri: None,
            logo_uri: None,
            subjec_type: None,
            sector_identifier_uri: None,
            subject_type: None,
            id_token_signed_response_alg: None,
            id_token_encrypted_response_enc: None,
            userinfo_signed_response_alg: None,
            userinfo_encrypted_response_alg: None,
            userinfo_encrypted_response_enc: None,
            request_object_signing_alg: None,
            request_object_encryption_alg: None,
            request_object_encryption_enc: None,
            token_endpoint_auth_method: None,
            token_endpoint_auth_signing_alg: None,
            default_max_age: None,
            require_auth_time: None,
            default_acr_values: None,
            initiate_login_uri: None,
            request_uris: None,
        }
    }

    /// Validate a request body against openid-connect-registration-1_0
    pub fn validate_request(&self) -> Result<(), GnapError> {
        Ok(())
    }
}
