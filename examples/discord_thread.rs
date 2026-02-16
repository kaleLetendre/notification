use notification::discord;

fn main() {
    let config = discord::Config {
        webhook_url: "https://discord.com/api/webhooks/YOUR_WEBHOOK_ID/YOUR_WEBHOOK_TOKEN".to_string(),
        ..Default::default()
    };

    // create a new forum thread with tags
    let msg = discord::Message {
        content: Some("First post in the new thread".to_string()),
        username: Some("Bot".to_string()),
        // creates a new thread in a forum channel with this name
        thread_name: Some("New Discussion Thread".to_string()),
        // forum tag IDs to apply to the new thread (found in channel settings)
        applied_tags: Some(vec!["TAG_ID".to_string()]),
        ..Default::default()
    };

    match discord::send(&config, &msg) {
        Ok(()) => println!("discord_thread (new): passed"),
        Err(e) => println!("discord_thread (new): failed {:?}", e),
    }

    // post into an existing thread by its ID
    let msg = discord::Message {
        content: Some("Replying to an existing thread".to_string()),
        username: Some("Bot".to_string()),
        // sends the message into this thread instead of the main channel. appended as a query param
        thread_id: Some("THREAD_ID".to_string()),
        ..Default::default()
    };

    match discord::send(&config, &msg) {
        Ok(()) => println!("discord_thread (existing): passed"),
        Err(e) => println!("discord_thread (existing): failed {:?}", e),
    }
}
