//! Data Persistence
//!
//! Data persistence is managed via MongoDB. Caching is managed with Redis. The dao lib provides an
//! abstraction level between the REST handlers and the database or cache.
//!
//! [Service](service::Service) wraps persistence and caching so that items that
//! should be cached, can be managed appropriately.
//!
//! The dao lib defines a Service that encapsulates the [GnapDB](db::GnapDB) and
//! the [GnapCache](cache::GnapCache).
//!

pub mod cache;
pub mod db;
pub mod service;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
