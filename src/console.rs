use crate::error::Error;

pub struct Message{
    pub body: String
}

pub fn send(msg: &Message) -> Result<(),Error> {
    println!("{}",msg.body);
    return Ok(())
}
