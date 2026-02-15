use notify::discord;

fn main() {
    let config = discord::Config {
        webhook_url: "https://discord.com/api/webhooks/1472438204903854238/5TEUOghXAJMnppFzDR0ilOjWl6aWPEq7252HarJGWKPaO4APMxBPl9mxd1HxgMUjtzqY".to_string(),
    };

    // test 1: everything - all structs, all fields populated
    let msg = discord::Message {
        content: Some("test 1: full coverage".to_string()),
        username: Some("Booba".to_string()),
        avatar_url: Some("https://cdn.truyen-hentai.com/the-perfect-boob-window-for-a-loving-and-tight?interlace=1&quality=90&type=jpeg&url=https%3A%2F%2Fs3.truyen-hentai.com%2F67e5db02d75b0522461783.jpg&width=400&sign=XqydYOQVspl-VfukueYztObNPrlOt2MZpOw_EZYAEjE".to_string()),
        embeds: Some(vec![
            discord::Embed {
                title: Some("Full Embed".to_string()),
                url: Some("https://example.com".to_string()),
                description: Some("**bold** *italic* __underline__ ~~strike~~ `code`".to_string()),
                color: Some(15258703),
                author: Some(discord::Author {
                    name: "Author".to_string(),
                    url: Some("https://example.com".to_string()),
                    icon_url: Some("https://i.imgur.com/4M34hi2.png".to_string()),
                }),
                fields: Some(vec![
                    discord::Field {
                        name: "Inline 1".to_string(),
                        value: "val".to_string(),
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
                        inline: Some(false),
                    },
                    discord::Field {
                        name: "No inline field".to_string(),
                        value: "val".to_string(),
                        inline: None,
                    },
                ]),
                thumbnail: Some(discord::Thumbnail {
                    url: "https://i.imgur.com/4M34hi2.png".to_string(),
                }),
                image: Some(discord::Image {
                    url: "https://i.imgur.com/4M34hi2.png".to_string(),
                }),
                footer: Some(discord::Footer {
                    text: "Footer with icon".to_string(),
                    icon_url: Some("https://i.imgur.com/4M34hi2.png".to_string()),
                }),
                timestamp: Some("2025-02-14T00:00:00.000Z".to_string()),
            },
            discord::Embed {
                title: Some("Minimal Embed".to_string()),
                url: None,
                description: None,
                color: Some(3066993),
                author: None,
                fields: None,
                thumbnail: None,
                image: None,
                footer: None,
                timestamp: None,
            },
            discord::Embed {
                title: None,
                url: None,
                description: Some("Author name only, footer no icon".to_string()),
                color: None,
                author: Some(discord::Author {
                    name: "Just a name".to_string(),
                    url: None,
                    icon_url: None,
                }),
                fields: None,
                thumbnail: None,
                image: None,
                footer: Some(discord::Footer {
                    text: "Footer without icon".to_string(),
                    icon_url: None,
                }),
                timestamp: None,
            },
        ]),
        poll: None,
        tts: None,
        allowed_mentions: Some(discord::AllowedMentions {
            parse: Some(vec!["users".to_string(), "roles".to_string()]),
            roles: None,
            users: None,
        }),
    };
    match discord::send(&config, &msg) {
        Ok(()) => println!("test 1 full coverage: passed"),
        Err(e) => println!("test 1 full coverage: failed {:?}", e),
    }

    // test 2: embed only no content
    let msg = discord::Message {
        content: None,
        username: Some("Booba".to_string()),
        avatar_url: None,
        embeds: Some(vec![
            discord::Embed {
                title: Some("Embed only".to_string()),
                url: None,
                description: Some("No content field".to_string()),
                color: Some(3447003),
                author: None,
                fields: None,
                thumbnail: None,
                image: None,
                footer: None,
                timestamp: None,
            },
        ]),
        poll: None,
        tts: None,
        allowed_mentions: None,
    };
    match discord::send(&config, &msg) {
        Ok(()) => println!("test 2 embed only: passed"),
        Err(e) => println!("test 2 embed only: failed {:?}", e),
    }

    // test 3: poll with multiselect, mixed emoji types
    let msg = discord::Message {
        content: Some("test 3: poll".to_string()),
        username: Some("Booba".to_string()),
        avatar_url: None,
        embeds: None,
        poll: Some(discord::Poll {
            question: discord::Question {
                text: "Best paradigm?".to_string(),
            },
            answers: vec![
                discord::Answer {
                    poll_media: discord::PollMedia {
                        text: "DOD".to_string(),
                        emoji: Some(discord::Emoji {
                            id: None,
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
                        emoji: None,
                    },
                },
            ],
            duration: Some(1),
            allow_multiselect: Some(true),
        }),
        tts: None,
        allowed_mentions: None,
    };
    match discord::send(&config, &msg) {
        Ok(()) => println!("test 3 poll: passed"),
        Err(e) => println!("test 3 poll: failed {:?}", e),
    }

    // test 4: ping Kale
    let msg = discord::Message {
        content: Some("<@472626052749852672> hey from rust!".to_string()),
        username: Some("Booba".to_string()),
        avatar_url: None,
        embeds: None,
        poll: None,
        tts: None,
        allowed_mentions: Some(discord::AllowedMentions {
            parse: None,
            roles: None,
            users: Some(vec!["472626052749852672".to_string()]),
        }),
    };
    match discord::send(&config, &msg) {
        Ok(()) => println!("test 4 ping Kale: passed"),
        Err(e) => println!("test 4 ping Kale: failed {:?}", e),
    }
}