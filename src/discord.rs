use crate::error::Error;

pub struct Config{
    pub webhook_url: String,
    pub username: Option<String>,
    pub avatar_url: Option<String>
}
pub struct Message{
    pub body_text: String,
}

pub fn send(config: &Config, msg: &Message) -> Result<(),Error> {
    todo!();
}
