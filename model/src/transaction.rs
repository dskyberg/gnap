/// Grant Request models.
///
/// All interaction with the server starts with a grant request.
///
use serde::{Serialize, Deserialize};
use serde_utils::vec_or_one::deser_one_as_vec;
use redis::{RedisWrite, ToRedisArgs};


/// AccessToken request flags.
///
/// A set of flags that indicate desired
/// attributes or behavior to be attached to the access token by the
/// AS.  This field is OPTIONAL.
/// The values of the "flags" field defined by this specification are as
/// follows:
///
/// "bearer"  If this flag is included, the access token being requested
///  is a bearer token.  If this flag is omitted, the access token is
///  bound to the key used by the client instance in this request, or
///  the key's most recent rotation.  Methods for presenting bound and
///  bearer access tokens are described in Section 7.2.
///
/// "split"  If this flag is included, the client instance is capable of
///  receiving a different number of tokens than specified in the token
///  request (Section 2.1), including receiving multiple access tokens
///  (Section 3.2.2) in response to any single token request
///  (Section 2.1.1) or a different number of access tokens than
///  requested in a multiple access token request (Section 2.1.2).  The
///  "label" fields of the returned additional tokens are chosen by the
///  AS.  The client instance MUST be able to tell from the token
///  response where and how it can use each of the access tokens.  [[
///  See issue #37 (https://github.com/ietf-wg-gnap/gnap-core-protocol/
///  issues/37) ]]

#[allow(proc_macro_derive_resolution_fallback)]


/// Flag values MUST NOT be included more than once.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccessTokenFlag {
    Bearer,
    Split,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GNAPStandardAccess {
    #[serde(rename = "type")]
    pub access_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datatypes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileges: Option<Vec<String>>,
}

/// Access Token portion of a grant request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessTokenRequest {
    /// Describes the rights that the
    /// client instance is requesting for one or more access tokens to be
    /// used at RS's.   This field is REQUIRED.  Section 8
    pub access: Vec<GNAPStandardAccess>,
    /// A unique name chosen by the client instance to refer
    /// to the resulting access token.  The value of this field is opaque
    /// to the AS.  If this field is included in the request, the AS MUST
    /// include the same label in the token response (Section 3.2).  This
    /// field is REQUIRED if used as part of a multiple access token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<AccessTokenFlag>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubjectFormatType {
    IssSubject,
    Opaque
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubjectAssertionType {
    IdToken,
    SAML2
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectRequest {
    pub formats: Option<Vec<SubjectFormatType>>,
    pub assertions: Option<Vec<SubjectAssertionType>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InteractStartMode {
    Redirect,
    App,
    UserCode
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InteractFinishMethodType {
    Redirect,
    Push
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractFinishRequest {
    pub method: InteractFinishMethodType,
    pub uri: String,
    pub nonce: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractRequest {
    pub start: Vec<InteractStartMode>,
    pub finish: Option<InteractFinishRequest>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantRequest {
    #[serde(deserialize_with = "deser_one_as_vec")]
    access_token: Vec<AccessTokenRequest>,
    subject: Option<SubjectRequest>,
    // We will only support client reference identifiers for now
    client: Option<String>,
    // We will only support user ref ids for now
    user: Option<String>,
    interact: Option<InteractRequest>
}


pub type InteractionStartModes = Vec<String>;
pub type InteractionFinishMethods = Vec<String>;
pub type KeyProofs = Vec<String>;
pub type SubjectFormats = Vec<String>;
pub type Assertions = Vec<String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionOptions {
    /// The location of the AS's
    /// grant request endpoint.  The location MUST be a URL [RFC3986] with
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
 impl ToRedisArgs for &TransactionOptions {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg_fmt(serde_json::to_string(self).expect("Can't serialize TransactionOptions as string"))
    }
}
