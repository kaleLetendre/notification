# notification

A lightweight library for sending one way messages across multiple platforms.

## Supported Platforms

- **Discord** — webhooks
- **Email** — SMTP (coming soon)
- **Console** — stdout

## Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
notification = { git = "git@github.com:kaleLetendre/notification.git" }
```

All platforms are enabled by default. To only include what you need, disable defaults and pick specific features:
```toml
[dependencies]
notification = { git = "git@github.com:kaleLetendre/notification.git", default-features = false, features = ["discord"] }
```

## Features

| Feature   | Description                  | Dependencies    |
|-----------|------------------------------|-----------------|
| `console` | Print messages to stdout     | none            |
| `discord` | Send messages via webhooks   | `ureq`, `serde` |
| `email`   | Send messages via SMTP       | `lettre`        |

## Console

Print messages directly to stdout. No configuration needed. This is mostly just to test installs and model basic library structure.

```rust
use notification::console;

let msg = console::Message {
    body_text: "Hello from console!".to_string(),
};

match console::send(&msg) {
    Ok(()) => println!("sent"),
    Err(e) => println!("failed: {:?}", e),
}
```

## Discord

Send messages to Discord channels via webhooks. Supports text messages, embeds, polls, mentions, file attachments, threads, and message flags.

Create a webhook in Discord: Server Settings → Integrations → Webhooks → New Webhook → Copy URL.

### How it works

Each part of a Discord webhook message is represented as a typed struct. Optional fields default to `None` and are omitted from the JSON payload. Structs that are entirely optional fields implement `Default`, so you can set only the fields you care about and use `..Default::default()` for the rest.

Messages without attachments are sent as JSON. Messages with attachments are automatically sent as multipart form data. The `thread_id` and `wait` fields are appended as URL query parameters rather than included in the JSON body.

See the `examples/discord_*.rs` files for full working examples of each feature.

### Config

| Field | Type | Description |
|-------|------|-------------|
| `webhook_url` | `String` | The full webhook URL from Discord |
| `wait` | `Option<bool>` | When `true`, Discord confirms the message was received before responding. Appended as a query parameter |

### Message

| Field | Type | Description |
|-------|------|-------------|
| `content` | `Option<String>` | Text body of the message, up to 2000 characters |
| `username` | `Option<String>` | Overrides the webhook's default display name |
| `avatar_url` | `Option<String>` | Overrides the webhook's default avatar |
| `embeds` | `Option<Vec<Embed>>` | Rich embed cards displayed below the message |
| `poll` | `Option<Poll>` | Interactive poll users can vote on |
| `tts` | `Option<bool>` | Text-to-speech: reads the message aloud in the channel |
| `allowed_mentions` | `Option<AllowedMentions>` | Controls which @mentions actually send pings |
| `flags` | `Option<u32>` | Message flag bitfield. [See Discord docs for values](https://discord.com/developers/docs/resources/message#message-object-message-flags) |
| `thread_name` | `Option<String>` | Creates a new forum thread with this name |
| `applied_tags` | `Option<Vec<String>>` | Tag IDs to apply when creating a forum thread |
| `thread_id` | `Option<String>` | Sends the message into an existing thread. Appended as a query parameter |
| `attachments` | `Option<Vec<Attachment>>` | File uploads. Triggers multipart form data instead of JSON |

### Embed

| Field | Type | Description |
|-------|------|-------------|
| `title` | `Option<String>` | Title text at the top of the embed |
| `url` | `Option<String>` | Makes the title a clickable hyperlink |
| `description` | `Option<String>` | Main body text, supports Discord markdown |
| `color` | `Option<u32>` | Colored strip on the left side, as a decimal color value |
| `author` | `Option<Author>` | Small author section at the top |
| `fields` | `Option<Vec<Field>>` | Key-value fields in the embed body |
| `thumbnail` | `Option<Thumbnail>` | Small image in the top-right corner |
| `image` | `Option<Image>` | Large image below the embed body |
| `footer` | `Option<Footer>` | Text and icon at the bottom |
| `timestamp` | `Option<String>` | ISO 8601 timestamp shown next to the footer |

### Sub-structs

| Struct | Field | Type | Description |
|--------|-------|------|-------------|
| `Author` | `name` | `String` | Author display name |
| | `url` | `Option<String>` | Makes the author name a clickable link |
| | `icon_url` | `Option<String>` | Small icon next to the author name |
| `Field` | `name` | `String` | Field label |
| | `value` | `String` | Field content |
| | `inline` | `Option<bool>` | When `true`, fields sit side by side (up to 3 per row) |
| `Thumbnail` | `url` | `String` | Thumbnail image URL |
| `Image` | `url` | `String` | Large image URL |
| `Footer` | `text` | `String` | Footer text |
| | `icon_url` | `Option<String>` | Small icon next to the footer text |

### Poll

| Field | Type | Description |
|-------|------|-------------|
| `question` | `Question` | The question displayed at the top (`text` field) |
| `answers` | `Vec<Answer>` | List of choices. Each wraps a `PollMedia` with `text` and optional `emoji` |
| `duration` | `Option<u32>` | How long the poll stays open, in hours |
| `allow_multiselect` | `Option<bool>` | When `true`, users can select multiple answers |

### Emoji

| Field | Type | Description |
|-------|------|-------------|
| `id` | `Option<String>` | ID for custom server emojis |
| `name` | `Option<String>` | Unicode emoji character (e.g. `"🔥"`) |

### AllowedMentions

| Field | Type | Description |
|-------|------|-------------|
| `parse` | `Option<Vec<String>>` | Broad mention types to allow: `"users"`, `"roles"`, `"everyone"` |
| `roles` | `Option<Vec<String>>` | Specific role IDs allowed to be pinged |
| `users` | `Option<Vec<String>>` | Specific user IDs allowed to be pinged |

Get a user's ID by enabling Developer Mode in Discord (Settings → Advanced → Developer Mode), then right-clicking their name and selecting Copy User ID.

### Attachment

| Field | Type | Description |
|-------|------|-------------|
| `filename` | `String` | The filename Discord displays for the download |
| `description` | `Option<String>` | Alt text for accessibility and search |
| `data` | `FileData` | Either `FileData::Path("path/to/file")` to read from disk, or `FileData::Bytes(vec)` to send raw bytes from memory |

## Email (coming soon)

Send emails via SMTP. Works with any SMTP provider including Gmail, Outlook, AWS SES, and self-hosted servers.

```rust
use notification::email;

let config = email::Config {
    smtp_server: "smtp.gmail.com".to_string(),
    port: 587,
    username: "you@gmail.com".to_string(),
    password: "your_app_password".to_string(),
    from_address: "you@gmail.com".to_string(),
};

let msg = email::Message {
    to: vec!["recipient@example.com".to_string()],
    cc: None,
    bcc: None,
    subject: "Hello".to_string(),
    body_text: "Hello from Rust!".to_string(),
};

match email::send(&config, &msg) {
    Ok(()) => println!("sent"),
    Err(e) => println!("failed: {:?}", e),
}
```