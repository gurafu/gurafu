use crate::gurafu::session::Session;

pub struct Client {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

impl Client {
    pub fn session(&self) -> Session {
        Session::new()
    }
}
