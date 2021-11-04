//! Grant Request models.
//!
//!All interaction with the server starts with a grant request.
//!
use serde::{Serialize, Deserialize};
use redis::{RedisWrite, ToRedisArgs};
use super::CachePath;
use uuid::Uuid;
use super::grant::GrantRequest;

//#[allow(proc_macro_derive_resolution_fallback)]

pub type InteractionStartModes = Vec<String>;
pub type InteractionFinishMethods = Vec<String>;
pub type KeyProofs = Vec<String>;
pub type SubjectFormats = Vec<String>;
pub type Assertions = Vec<String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionOptions {
    /// The location of the AS's
    /// grant request endpoint.  The location MUST be a URL [RFC3986](https://datatracker.ietf.org/doc/html/rfc3986) with
    ///  a scheme component that MUST be https, a host component, and
    ///  optionally, port, path and query components and no fragment
    ///  components.  This URL MUST match the URL the client instance used
    ///  to make the discovery request.
    pub grant_request_endpoint: String,

    /// A list of the AS's interaction start methods.  The values of this
    /// list correspond to the possible values for the interaction start
    /// section (Section 2.5.1) of the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction_start_modes_supported: Option<InteractionStartModes>,

    /// A list of the AS's interaction finish methods.  The values of this
    /// list correspond to the possible values for the method element of
    /// the interaction finish section (Section 2.5.2) of the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction_finish_methods_supported: Option<InteractionFinishMethods>,

    /// A list of the AS's supported key proofing mechanisms.  The values of
    /// this list correspond to possible values of the "proof" field of the key
    ///  section (Section 7.1) of the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_proofs_supported: Option<KeyProofs>,

    /// A list of the AS's supported subject identifier types.  The values
    /// of this list correspond to possible values of the subject identifier
    /// section (Section 2.2) of the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_formats_supported: Option<SubjectFormats>,

    /// A list of the AS's supported assertion formats.  The values of this
    /// list correspond to possible values of the subject assertion section
    /// (Section 2.2) of the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assertions_supported: Option<Assertions>,
}

impl TransactionOptions {
    pub fn new() -> Self {
        let start_modes = vec![
            "redirect".to_owned(),
            "app".to_owned(),
            "user_code".to_owned(),
        ];

        let finish_methods = vec!["redirect".to_owned(), "push".to_owned()];

        let key_proof_methods = vec![
            "httpsig".to_owned(),
            "mtls".to_owned(),
            "jwsd".to_owned(),
            "jws".to_owned(),
        ];

        let subject_formats = vec![
            "account".to_owned(),
            "aliases".to_owned(),
            "did".to_owned(),
            "email".to_owned(),
            "iss_sub".to_owned(),
            "opaque".to_owned(),
            "phone_number".to_owned(),
        ];

        let assertions = vec!["oidc".to_owned(), "saml2".to_owned()];

        Self {
            grant_request_endpoint: "localhost::8000/gnap/grant".to_owned(),
            interaction_start_modes_supported: Some(start_modes),
            interaction_finish_methods_supported: Some(finish_methods),
            key_proofs_supported: Some(key_proof_methods),
            subject_formats_supported: Some(subject_formats),
            assertions_supported: Some(assertions),
        }
    }
}

impl CachePath for TransactionOptions {
    fn cache_path() -> &'static str {
        "gnap:tx_options"
    }
}

 impl ToRedisArgs for &TransactionOptions {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg_fmt(serde_json::to_string(self).expect("Can't serialize TransactionOptions as string"))
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum GnapTransactionState {
    Start,
    Received,
    ClientVerified,
    ResourceOwnerVerified,

}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GnapTransaction {
    pub tx_id: String,
    pub state: GnapTransactionState,
    pub request: Option<GrantRequest>,
}

impl GnapTransaction {
    pub fn create_id() -> String {
        Uuid::new_v4().to_string()
    }

    pub fn new(request: Option<GrantRequest>) -> Self {
        Self{
            tx_id: Self::create_id(),
            state: GnapTransactionState::Received,
            request: request
        }
    }
}

impl CachePath for GnapTransaction {
    fn cache_path() -> &'static str {
        "gnap:tx"
    }
}

 impl ToRedisArgs for &GnapTransaction {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg_fmt(serde_json::to_string(self).expect("Can't serialize GnapTransaction as string"))
    }
}
