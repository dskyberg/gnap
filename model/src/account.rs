//! User account model
//!
//! Account and claims related to identities.
//!
use redis::{RedisWrite, ToRedisArgs};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::CachePath;

/// Snail mail address and verification status
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct AccountAddress {
    pub country: String,
    pub locality: String,
    pub postal_code: String,
    pub region: String,
    pub street_address: String,
    pub formatted: Option<String>,
    pub primary: bool,
}

/// Email address and verification status
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct EmailAddress {
    pub address: String,
    pub verified: bool,
    pub primary: bool,
}

/// This should never be used.  Who even does binary gendr any more?
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(untagged)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
}

/// Phone number and verification status
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PhoneNumber {
    phone_number: String,
    verified: bool,
    primary: bool,
}

/// User/RO identity info
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Account {
    account_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<Vec<AccountAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    birthdate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<Vec<EmailAddress>>,
    family_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    gender: Option<Gender>,
    given_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    middle_name: Option<String>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<Vec<PhoneNumber>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    picture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zoneinfo: Option<String>,
}

impl CachePath for Account {
    fn cache_path() -> &'static str {
        "gnap:accounts"
    }
}

impl From<AccountRequest> for Account {
    fn from(ar: AccountRequest) -> Self {
        Self {
            account_id: Account::create_id(),
            address: ar.address,
            birthdate: ar.birthdate,
            email: ar.email,
            family_name: ar.family_name,
            gender: ar.gender,
            given_name: ar.given_name,
            locale: ar.locale,
            middle_name: ar.middle_name,
            name: ar.name,
            nickname: ar.nickname,
            phone: ar.phone,
            picture: ar.picture,
            preferred_username: ar.preferred_username,
            profile: ar.profile,
            tax_id: ar.tax_id,
            website: ar.website,
            zoneinfo: ar.zoneinfo,
        }
    }
}

impl Account {
    pub fn create_id() -> Uuid {
        Uuid::new_v4()
    }
}

impl ToRedisArgs for &Account {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg_fmt(serde_json::to_string(self).expect("Can't serialize Account as string"))
    }
}

/// Used for direct DB interaction
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct AccountRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<Vec<AccountAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    birthdate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<Vec<EmailAddress>>,
    family_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    gender: Option<Gender>,
    given_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    middle_name: Option<String>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<Vec<PhoneNumber>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    picture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zoneinfo: Option<String>,
}

impl AccountRequest {
    pub fn new(given: &str, family: &str) -> Self {
        AccountRequest {
            address: None,
            birthdate: None,
            email: None,
            family_name: family.to_owned(),
            gender: None,
            given_name: given.to_owned(),
            locale: None,
            middle_name: None,
            name: format!("{} {}", given, family),
            nickname: None,
            phone: None,
            picture: None,
            preferred_username: None,
            profile: None,
            tax_id: None,
            website: None,
            zoneinfo: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_it() {
        let ar = AccountRequest::new("John", "Smith");
        let acct = Account::from(ar);
        println!("{:?}", acct);
        assert!(true)
    }

    #[test]
    fn cache_path() {
        assert_eq!( Account::cache_path(), "gnap:accounts");
    }
}
