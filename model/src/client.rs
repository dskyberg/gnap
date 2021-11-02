use crate::oauth::{AcrValueType, ApplicationType, GrantType, ResponseType, SubjectType};
use redis::{RedisWrite, ToRedisArgs};
use jsonwebtoken::Algorithm;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use errors::GnapError;
use super::CachePath;


#[derive(Deserialize, Clone, Debug)]
pub struct GnapClientRequest {
    pub redirect_uris: Vec<String>,
    pub client_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GnapClient {
    pub client_id: Uuid,
    pub redirect_uris: Vec<String>,
    pub client_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_type: Option<ApplicationType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_types: Option<Vec<ResponseType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_types: Option<Vec<GrantType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwks_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subjec_type: Option<SubjectType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sector_identifier_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token_signed_response_alg: Option<Algorithm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token_encrypted_response_enc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userinfo_signed_response_alg: Option<Algorithm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userinfo_encrypted_response_alg: Option<Algorithm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userinfo_encrypted_response_enc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_object_signing_alg: Option<Algorithm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_object_encryption_alg: Option<Algorithm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_object_encryption_enc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_endpoint_auth_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_endpoint_auth_signing_alg: Option<Algorithm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_max_age: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_auth_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_acr_values: Option<Vec<AcrValueType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiate_login_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_uris: Option<Vec<String>>,
}

/// Client defined by OIDC
///  This needs to be updated properly for GNAP, but should work for now.
impl GnapClient {
    pub fn new(redirect_uris: Vec<String>, client_name: String) -> Self {
        Self {
            client_id: GnapClient::create_id(),
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

    pub fn create_id() -> Uuid {
        Uuid::new_v4()
    }
}
impl CachePath for GnapClient {
    fn cache_path() -> &'static str {
        "gnap:clients"
    }
}

impl ToRedisArgs for &GnapClient {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg_fmt(serde_json::to_string(self).expect("Can't serialize GnapClient as string"))
    }
}

