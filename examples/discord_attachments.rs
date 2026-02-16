use notification::discord;

fn main() {
    let config = discord::Config {
        webhook_url: "https://discord.com/api/webhooks/YOUR_WEBHOOK_ID/YOUR_WEBHOOK_TOKEN".to_string(),
        ..Default::default()
    };

    let msg = discord::Message {
        content: Some("Here are some attachments".to_string()),
        username: Some("Bot".to_string()),
        // file attachments are sent via multipart upload instead of JSON
        attachments: Some(vec![
            discord::Attachment {
                // the filename Discord will display for the download
                filename: "hello.txt".to_string(),
                // alt text shown for accessibility and in search
                description: Some("A text file from disk".to_string()),
                // Path reads a file from the filesystem at send time
                data: discord::FileData::Path("examples/hello.txt".to_string()),
            },
            discord::Attachment {
                filename: "generated.txt".to_string(),
                description: None,
                // Bytes sends raw data directly from memory
                data: discord::FileData::Bytes(b"This was generated in memory".to_vec()),
            },
        ]),
        ..Default::default()
    };

    match discord::send(&config, &msg) {
        Ok(()) => println!("discord_attachments: passed"),
        Err(e) => println!("discord_attachments: failed {:?}", e),
    }
}
