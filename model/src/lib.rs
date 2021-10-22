pub mod transaction;
pub mod oauth;
pub mod client;
pub mod oidc;
pub mod gnap;
pub mod resource;

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
