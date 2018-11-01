extern crate burgundy;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate uuid;
pub mod types;
pub use burgundy::Error;
pub use burgundy::Result;
mod get;
pub use self::get::ClubhouseGet;
mod put;
pub use self::put::ClubhousePut;
mod delete;
pub use self::delete::ClubhouseDelete;
mod post;
pub use self::post::ClubhousePost;

/// See https://clubhouse.io/api/rest/v2/
pub struct Clubhouse {
    domain: burgundy::Domain,
}

impl Clubhouse {
    pub fn new(token: &str) -> Self {
        let mut domain = burgundy::Domain::new(&"https://api.clubhouse.io/api/v2");

        let user_agent = format!("Clubhouse (Rust, Burgundy)/{}", env!("CARGO_PKG_VERSION"));
        domain.header(&"User-Agent", &user_agent.as_str());
        domain.query(&"token", &token);

        Self { domain }
    }

    pub fn get(&self) -> self::get::ClubhouseGet {
        self::get::ClubhouseGet {
            path: self.domain.get(),
        }
    }

    pub fn put(&self) -> self::put::ClubhousePut {
        self::put::ClubhousePut {
            path: self.domain.put(),
        }
    }

    pub fn delete(&self) -> self::delete::ClubhouseDelete {
        self::delete::ClubhouseDelete {
            path: self.domain.delete(),
        }
    }

    pub fn post(&self) -> self::post::ClubhousePost {
        self::post::ClubhousePost {
            path: self.domain.post(),
        }
    }
}
