use notification::discord;

fn main() {
    let config = discord::Config {
        webhook_url: "https://discord.com/api/webhooks/YOUR_WEBHOOK_ID/YOUR_WEBHOOK_TOKEN".to_string(),
        // when true, Discord waits for the message to be confirmed before responding
        wait: Some(true),
    };

    let msg = discord::Message {
        content: Some("This link won't preview: https://example.com".to_string()),
        username: Some("Bot".to_string()),
        // bitfield of message flags. 4 = suppress embeds (link previews won't show)
        flags: Some(4),
        ..Default::default()
    };

    match discord::send(&config, &msg) {
        Ok(()) => println!("discord_flags: passed"),
        Err(e) => println!("discord_flags: failed {:?}", e),
    }
}
