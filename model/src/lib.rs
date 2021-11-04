use uuid::Uuid;
use errors::GnapError;
pub mod transaction;
pub mod grant;
pub mod oauth;
pub mod client;
pub mod oidc;
pub mod gnap;
pub mod resource;
pub mod account;

/// CachePath ensures each model type that will be cached provides a
/// consistent path to cache objects
pub trait CachePath {
    fn cache_path() -> &'static str;
}

/// GnapID ensures any model that contains an "id", such as "client_id"
/// is generated and parsed in a consistent manner
pub trait GnapID {
    fn parse_id(&self) -> Result<Uuid, GnapError>;
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    #[test]
    fn it_works() {
        let my_uuid = Uuid::new_v4();
        println!("{}", my_uuid);
        assert_eq!(2 + 2, 4);
    }
}
