use crate::error::Error;
use ureq;
use serde::Serialize;

pub struct Config{
    pub webhook_url: String
}

//Serialize is a trait (function added to any type) that defines how to format this in a json, so this struct is now just drop in json
//I will be adding fields that contain other serialized structs for deeper json fields, it will be alyed out to match 

#[derive(Serialize)]
pub struct Message {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Embed>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<AllowedMentions>,
}

#[derive(Serialize)]
pub struct Embed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<Field>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Thumbnail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<Footer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

#[derive(Serialize)]
pub struct Author {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
}

#[derive(Serialize)]
pub struct Field {
    pub name: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline: Option<bool>,
}

#[derive(Serialize)]
pub struct Thumbnail {
    pub url: String,
}

#[derive(Serialize)]
pub struct Image {
    pub url: String,
}

#[derive(Serialize)]
pub struct Footer {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
}

#[derive(Serialize)]
pub struct Poll {
    pub question: Question,
    pub answers: Vec<Answer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_multiselect: Option<bool>,
}

#[derive(Serialize)]
pub struct Question {
    pub text: String,
}

#[derive(Serialize)]
pub struct Answer {
    pub poll_media: PollMedia,
}

#[derive(Serialize)]
pub struct PollMedia {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<Emoji>,
}

#[derive(Serialize)]
pub struct Emoji {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct AllowedMentions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
}

//curl -X POST "https://discord.com/api/webhooks/1472438204903854238/5TEUOghXAJMnppFzDR0ilOjWl6aWPEq7252HarJGWKPaO4APMxBPl9mxd1HxgMUjtzqY" \
//  -H "Content-Type: application/json" \
//  -d '{"content": "test message from terminal"}'


pub fn send(config: &Config, msg: &Message) -> Result<(),Error> {
//send_json handles the content type header
    match ureq::post(&config.webhook_url).send_json(msg){
        Ok(_) => return Ok(()),
        Err(e) => return Err(Error::NetworkFailure(e.to_string()))
    }
}
