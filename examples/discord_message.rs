use notification::discord;

fn main() {
    let config = discord::Config {
        // the full webhook URL from Discord (Server Settings → Integrations → Webhooks)
        webhook_url: "https://discord.com/api/webhooks/YOUR_WEBHOOK_ID/YOUR_WEBHOOK_TOKEN".to_string(),
        ..Default::default()
    };

    let msg = discord::Message {
        // the text body of the message, up to 2000 characters
        content: Some("Hello from the message example!".to_string()),
        // overrides the webhook's default display name in Discord
        username: Some("Bot".to_string()),
        // overrides the webhook's default avatar with this image URL
        avatar_url: Some("https://i.imgur.com/4M34hi2.png".to_string()),
        // text-to-speech: when true, the message is read aloud to users in the channel
        tts: Some(false),
        ..Default::default()
    };

    match discord::send(&config, &msg) {
        Ok(()) => println!("discord_message: passed"),
        Err(e) => println!("discord_message: failed {:?}", e),
    }
}
