use notification::discord;

fn main() {
    let config = discord::Config {
        webhook_url: "https://discord.com/api/webhooks/YOUR_WEBHOOK_ID/YOUR_WEBHOOK_TOKEN".to_string(),
        ..Default::default()
    };

    let msg = discord::Message {
        username: Some("Bot".to_string()),
        // embeds are rich cards that appear below the message content
        embeds: Some(vec![
            // a fully populated embed showing every available field
            discord::Embed {
                // title text displayed at the top of the embed
                title: Some("Full Embed".to_string()),
                // makes the title a clickable hyperlink
                url: Some("https://example.com".to_string()),
                // the main body text of the embed, supports Discord markdown
                description: Some("**bold** *italic* __underline__ ~~strike~~ `code`".to_string()),
                // the colored strip on the left side of the embed, as a decimal color value
                color: Some(15258703),
                // small author section at the very top of the embed
                author: Some(discord::Author {
                    // author display name (required)
                    name: "Author".to_string(),
                    // makes the author name a clickable link
                    url: Some("https://example.com".to_string()),
                    // small icon next to the author name
                    icon_url: Some("https://i.imgur.com/4M34hi2.png".to_string()),
                }),
                // key-value fields displayed in the embed body
                fields: Some(vec![
                    discord::Field {
                        // field label (required)
                        name: "Inline 1".to_string(),
                        // field content (required)
                        value: "val".to_string(),
                        // when true, fields sit side by side (up to 3 per row)
                        inline: Some(true),
                    },
                    discord::Field {
                        name: "Inline 2".to_string(),
                        value: "val".to_string(),
                        inline: Some(true),
                    },
                    discord::Field {
                        name: "Inline 3".to_string(),
                        value: "val".to_string(),
                        inline: Some(true),
                    },
                    discord::Field {
                        name: "Not inline".to_string(),
                        value: "val".to_string(),
                        // when false, the field takes the full width
                        inline: Some(false),
                    },
                    discord::Field {
                        name: "No inline field".to_string(),
                        value: "val".to_string(),
                        // None defaults to false (full width)
                        inline: None,
                    },
                ]),
                // small image in the top-right corner of the embed
                thumbnail: Some(discord::Thumbnail {
                    url: "https://i.imgur.com/4M34hi2.png".to_string(),
                }),
                // large image displayed below the embed body
                image: Some(discord::Image {
                    url: "https://i.imgur.com/4M34hi2.png".to_string(),
                }),
                // text and optional icon at the bottom of the embed
                footer: Some(discord::Footer {
                    // footer text (required)
                    text: "Footer with icon".to_string(),
                    // small icon next to the footer text
                    icon_url: Some("https://i.imgur.com/4M34hi2.png".to_string()),
                }),
                // ISO 8601 timestamp shown next to the footer
                timestamp: Some("2025-02-14T00:00:00.000Z".to_string()),
            },
            // a minimal embed — only title and color
            discord::Embed {
                title: Some("Minimal Embed".to_string()),
                color: Some(3066993),
                ..Default::default()
            },
            // an embed with just author, description, and footer (no icons)
            discord::Embed {
                description: Some("Author name only, footer no icon".to_string()),
                author: Some(discord::Author {
                    name: "Just a name".to_string(),
                    url: None,
                    icon_url: None,
                }),
                footer: Some(discord::Footer {
                    text: "Footer without icon".to_string(),
                    icon_url: None,
                }),
                ..Default::default()
            },
        ]),
        ..Default::default()
    };

    match discord::send(&config, &msg) {
        Ok(()) => println!("discord_embed: passed"),
        Err(e) => println!("discord_embed: failed {:?}", e),
    }
}
