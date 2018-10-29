extern crate burgundy;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate uuid;
mod post;
pub use self::post::ClubhousePost;
mod put;
pub use self::put::ClubhousePut;
mod get;
pub use self::get::ClubhouseGet;
mod delete;
pub use self::delete::ClubhouseDelete;

pub mod types;

/// See https://clubhouse.io/api/rest/v2/
pub struct Clubhouse {
    domain: burgundy::Domain,
}

impl Clubhouse {
    pub fn new(token: String) -> Self {
        let mut domain = burgundy::Domain::new(&"https://api.clubhouse.io/api/v2");

        let user_agent = format!("Clubhouse (Rust, Burgundy)/{}", env!("CARGO_PKG_VERSION"));
        domain.header(&"User-Agent", &user_agent.as_str());
        domain.query(&"token", &token);

        Self { domain }
    }

    pub fn post(&self) -> self::post::ClubhousePost {
        self::post::ClubhousePost {
            path: self.domain.post(),
        }
    }

    pub fn put(&self) -> self::put::ClubhousePut {
        self::put::ClubhousePut {
            path: self.domain.put(),
        }
    }

    pub fn get(&self) -> self::get::ClubhouseGet {
        self::get::ClubhouseGet {
            path: self.domain.get(),
        }
    }

    pub fn delete(&self) -> self::delete::ClubhouseDelete {
        self::delete::ClubhouseDelete {
            path: self.domain.delete(),
        }
    }
}
