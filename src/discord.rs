use crate::error::Error;
use ureq;
use ureq::unversioned::multipart::{Form,Part};
use serde::Serialize;
use serde_json;


#[derive(Default)]
pub struct Config{
    pub webhook_url: String,
    pub wait: Option<bool>
}

#[derive(Serialize, Default)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_tags: Option<Vec<String>>,
    #[serde(skip)]
    pub thread_id: Option<String>,
    #[serde(skip)]
    pub attachments: Option<Vec<Attachment>>
    
}

#[derive(Serialize, Default)]
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

#[derive(Serialize, Default)]
pub struct Emoji {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Default)]
pub struct AllowedMentions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
}

pub enum FileData{
    Path(String),
    Bytes(Vec<u8>)
}

pub struct Attachment {
    pub filename: String,
    pub description: Option<String>,
    pub data: FileData
}

pub fn send(config: &Config, msg: &Message) -> Result<(),Error> {
//send_json handles the content type header
    let url = build_url(config, msg);

    match &msg.attachments {
        Some(attachments) => send_multipart(&url, msg,attachments),
        None => send_json(&url, msg)
    }

}

fn build_url(config: &Config, msg: &Message) -> String {
    let mut params = vec![];

    match config.wait{
        Some(true) => {params.push("wait=true".to_string());},
        Some(false)=> {params.push("wait=false".to_string());},
        _=> {}
    }

    match &msg.thread_id {
        Some(id) =>{params.push(format!("thread_id={}",id));},
        _=>{}
    }
    
    if params.is_empty(){
        return config.webhook_url.clone();
    }
    else{
        return format!("{}?{}",config.webhook_url, params.join("&"));
    }
}

fn send_json(url: &str, msg: &Message) -> Result<(),Error> {
    match ureq::post(url).send_json(msg){
        Ok(_) => return Ok(()),
        Err(e) => return Err(Error::NetworkFailure(e.to_string()))
    }
}
fn send_multipart(url: &str, msg: &Message, attachments: &Vec<Attachment>) -> Result<(),Error> {
    let json_payload = serde_json::to_string(msg).unwrap();
    let mut form = Form::new()
        .part(
            "payload_json", 
            Part::text(&json_payload)
                .mime_str("application/json")
                .map_err(|e| Error::NetworkFailure(e.to_string()))?
            );
    let names: Vec<String> = attachments.iter().enumerate().map(|(i,_)|format!("files[{}]", i)).collect();
    for (i, attachment) in attachments.iter().enumerate() {
        let part = match &attachment.data {
            FileData::Path(path) => Part::file(path)
                .map_err(|e| Error::InvalidInput(e.to_string()))?,
            FileData::Bytes(bytes) => Part::bytes(bytes)
        };
        let part = part.file_name(&attachment.filename);
        form = form.part(&names[i], part);
    }

    match ureq::post(url).send(form) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(Error::NetworkFailure(e.to_string()))
    }      
}