use notification::discord;

fn main() {
    let config = discord::Config {
        webhook_url: "https://discord.com/api/webhooks/YOUR_WEBHOOK_ID/YOUR_WEBHOOK_TOKEN".to_string(),
        ..Default::default()
    };

    let msg = discord::Message {
        // use <@USER_ID> syntax in content to mention a user
        content: Some("<@USER_ID> hey from rust!".to_string()),
        username: Some("Bot".to_string()),
        // controls which @mentions in the content actually send pings
        allowed_mentions: Some(discord::AllowedMentions {
            // broad mention types to allow: "users", "roles", "everyone". None disables broad parsing
            parse: None,
            // specific role IDs that are allowed to be pinged
            roles: None,
            // specific user IDs that are allowed to be pinged
            users: Some(vec!["USER_ID".to_string()]),
        }),
        ..Default::default()
    };

    match discord::send(&config, &msg) {
        Ok(()) => println!("discord_mentions: passed"),
        Err(e) => println!("discord_mentions: failed {:?}", e),
    }
}
