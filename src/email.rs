use crate::error::Error;

pub struct Config{
    pub smtp_server: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from_address: String
}
pub struct Message{
    pub to: Vec<String>,
    pub cc: Option<Vec<String>>,
    pub bcc: Option<Vec<String>>,
    pub subject: String,
    pub body_text: String
}

pub fn send(config: &Config, msg:&Message) -> Result<(),Error> {
    todo!();
} 
