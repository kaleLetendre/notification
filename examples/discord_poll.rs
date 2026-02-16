use notification::discord;

fn main() {
    let config = discord::Config {
        webhook_url: "https://discord.com/api/webhooks/YOUR_WEBHOOK_ID/YOUR_WEBHOOK_TOKEN".to_string(),
        ..Default::default()
    };

    let msg = discord::Message {
        content: Some("Poll example".to_string()),
        username: Some("Bot".to_string()),
        // interactive poll that users can vote on directly in Discord
        poll: Some(discord::Poll {
            // the question displayed at the top of the poll
            question: discord::Question {
                text: "Best paradigm?".to_string(),
            },
            // list of choices users can vote for
            answers: vec![
                discord::Answer {
                    poll_media: discord::PollMedia {
                        // the answer text (required)
                        text: "DOD".to_string(),
                        // optional emoji shown next to the answer
                        emoji: Some(discord::Emoji {
                            // id is for custom server emojis, None for built-in unicode
                            id: None,
                            // unicode emoji character
                            name: Some("🔥".to_string()),
                        }),
                    },
                },
                discord::Answer {
                    poll_media: discord::PollMedia {
                        text: "OOP".to_string(),
                        emoji: Some(discord::Emoji {
                            id: None,
                            name: Some("💀".to_string()),
                        }),
                    },
                },
                discord::Answer {
                    poll_media: discord::PollMedia {
                        text: "Functional".to_string(),
                        // no emoji for this answer
                        emoji: None,
                    },
                },
            ],
            // how long the poll stays open, in hours
            duration: Some(1),
            // when true, users can select multiple answers
            allow_multiselect: Some(true),
        }),
        ..Default::default()
    };

    match discord::send(&config, &msg) {
        Ok(()) => println!("discord_poll: passed"),
        Err(e) => println!("discord_poll: failed {:?}", e),
    }
}
