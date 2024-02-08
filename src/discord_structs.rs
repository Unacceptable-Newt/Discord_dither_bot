use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
    pub bot: Option<bool>,
    pub system: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub banner: Option<String>,
    pub accent_color: Option<u32>,
    pub locale: Option<String>,
    pub verified: Option<bool>,
    pub email: Option<String>,
    pub flags: Option<u32>,
    pub premium_type: Option<u32>,
    pub public_flags: Option<u32>,
    pub avatar_decoration_data: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DiscordOverwrites {
    pub id: String,
    pub r#type: u32,
    pub allow: String,
    pub deny: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    pub id: String,
    pub channel_id: String,
    pub author: User,
    pub content: Option<String>,
    pub timestamp: String,
    pub edited_timestamp: Option<String>,
    pub tts: bool,
    pub mention_everyone: bool,
    pub mentions: Vec<User>,
    pub mention_roles: Vec<String>,//need to update with actual role object
    pub mention_channels: Option<Vec<String>>,// need to update with Channel message object
    pub attachments: Option<Vec<AttachmentObject>>,
    pub embeds: Option<Vec<String>>,//need to update with actual embeds object
    pub reactions: Option<Vec<String>>,//need to add reaactions object
    pub nonce: Option<u32>,
    pub webhook_id: Option<String>,
    pub r#type: u32,
    pub activity: Option<String>, //need to make message activity object
    pub application: Option<String>,// need to make application object
    pub application_id: Option<String>,
    pub message_reference: Option<String>, //need to make message refference object
    pub flags: Option<u32>,
    pub referenced_message: Option<Box<Message>>,
    pub interaction: Option<DiscordInteractionData>,//need to add message interaction object
    pub thread: Option<DiscordChannelObject>,
    pub components: Option<Vec<String>>,//need to add message components object
    pub sticker_items: Option<Vec<String>>,//need to add message sticker items object
    pub stickers: Option<Vec<String>>,//need to add sticker object
    pub position: Option<u32>,
    pub role_subscription_data: Option<String>,//need to add subscription object
    pub resolved: Option<ResolvedData>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DiscordChannelObject {
    pub id: String,
    pub r#type: u32,
    pub guild_id: Option<String>,
    pub position: Option<u32>,
    pub permission_overwrites: Option<DiscordOverwrites>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<String>,
    pub bitrate: Option<u32>,
    pub user_limit: Option<u32>,
    pub rate_limit_per_user: Option<u32>,
    pub recipients: Option<Vec<User>>,
    pub icon: Option<String>,
    pub owner_id: Option<String>,
    pub application_id: Option<String>,
    pub managed: Option<bool>,
    pub parent_id: Option<String>,
    pub last_pin_timestamp: Option<String>,
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<u32>,
    pub message: Option<u32>,
    pub member_count: Option<u32>,
    pub thread_metadata: Option<String>, //need to update with actual object later
    pub member: Option<String>, //need to update with actual object later
    pub default_auto_archive_duration: Option<u32>,
    pub permissions: Option<String>,
    pub flags: Option<u32>,
    pub total_message_sent: Option<u32>,
    pub available_tags: Option<String>, //need to update with tag options
    pub applied_tags: Option<Vec<String>>,
    pub default_reaction_emoji: Option<String>, //need to update with emoji object
    pub default_thread_rate_limit_per_user: Option<u32>,
    pub default_sort_order: Option<u32>,
    pub default_forum_layout: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Member {
    pub user: Option<User>,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub roles: Option<Vec<String>>,
    pub joined_at: Option<String>,
    pub premium_since: Option<String>,
    pub deaf: bool,
    pub mute: bool,
    pub flags: u32,
    pub pending: Option<bool>,
    pub permissions: Option<String>,
    pub communication_disabled_unit: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DiscordPing {
    pub id: String,
    pub application_id: String,
    pub r#type: u32,
    pub token: String,
    pub version: Option<u32>,
    pub entitlements: Vec<String>,
    pub user: Option<User>,
    pub member: Option<Member>,
    pub channel_id: Option<String>,
    pub app_permissions: Option<String>,
    pub guild_locale: Option<String>,
    pub guild_id: Option<String>,
    pub message: Option<String>, // update with actual object later
    pub channel: Option<DiscordChannelObject>,
    pub data: Option<DiscordInteractionData>,
    pub locale: Option<String>,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DiscordInteractionData {
    pub id: String,
    pub name: String,
    pub r#type: u32,
    pub options: Option<Vec<DiscordOptions>>,
    pub resolved: Option<ResolvedData>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ResolvedData {
    pub users: Option<HashMap<String, User>>,
    pub members: Option<HashMap<String, Member>>,
    pub roles: Option<HashMap<String, String>>,//need to add role structure later
    pub channels: Option<HashMap<String, DiscordChannelObject>>,
    pub messages: Option<HashMap<String, String>>,//need to implement message object
    pub attachments: Option<HashMap<String, AttachmentObject>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AttachmentObject {
    pub id: String,
    pub filename: String,
    pub description: Option<String>,
    pub content_type: Option<String>,
    pub size: u32,
    pub url: String,
    pub proxy_url: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub ephemeral: Option<bool>,
    pub duration_secs: Option<f64>,
    pub waveform: Option<String>,
    pub flags: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DiscordOptions {
    pub name: String,
    pub r#type: u32,
    pub value: Option<String>,
    pub options: Option<Vec<DiscordOptions>>,
    pub focused: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DiscordInteractionResponce {
    pub r#type: u8,
    pub data: DiscordData,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DiscordData {
    pub tts: bool,
    pub content: String,
    pub embeds: Vec<String>,// need to make embeds object for uses with embeding data
    pub allowed_mentions: Mentions,
}

    #[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Mentions {
    pub parse: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MessageWithAttachments {
    pub content: Option<String>,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub tts: Option<bool>,
    pub embeds: Option<Vec<Embeds>>,
    pub allowed_mentions: Option<AllowedMentions>,
    pub attachments: Option<Vec<PartialAttachmentObject>>,
    pub flags: Option<u32>,
    pub thread_name: Option<String>,
    pub applied_tags: Option<Vec<String>>,
}

impl MessageWithAttachments {
    pub fn new(file_name: String, username: String, user_snowflake: String) -> Self {
        let content = format!("here is your image <@{}>",user_snowflake);
        let title = format!("{}'s image",username);
        Self { 
            content: Some(content),
            username: None,
            avatar_url: None,
            tts: Some(false),
            embeds: Some(vec![
            Embeds {
                title: Some(title.clone()),
                r#type: None,
                description: None,
                url: None,
                timestamp: None,
                color: None,
                fotter: None,
                image: Some(EmbedImage { 
                    url: format!("attachment://{}",file_name),
                    proxy_url: None,
                    height: None,
                    width: None, 
                }),
                thumbnail: None,
                video: None,
                provider: None,
                author: None,
                fields: None,
                }]),
            allowed_mentions: None,
            attachments: Some(vec![
            PartialAttachmentObject {
                id: 0,
                description: title,
                filename: file_name,
                }]),
            flags: None,
            thread_name: None,
            applied_tags: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PartialAttachmentObject {
    pub id: u32,
    pub description: String,
    pub filename: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AllowedMentions {
    pub parse: Vec<String>,
    pub roles: Vec<String>,
    pub users: Vec<String>,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Embeds {
    title: Option<String>,
    r#type: Option<String>,
    description: Option<String>,
    url: Option<String>,
    timestamp: Option<String>,
    color: Option<u32>,
    fotter: Option<String>, // need to add fotter object
    image: Option<EmbedImage>,
    thumbnail: Option<EmbedThumbnail>,
    video: Option<String>, //need to add video object
    provider: Option<String>, // need to add provider object
    author: Option<EmbedAuthor>,
    fields: Option<Vec<String>>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EmbedThumbnail {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EmbedImage {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EmbedAuthor {
    pub name: String,
    pub url: Option<String>,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

impl DiscordInteractionResponce {
    pub fn new(name: Option<String>) -> Self {
        Self { r#type: 4, data: DiscordData {
            tts: false,
            content: match name {
                None => "Hello World!".to_string(),
                Some(s) => format!("Hello {}", s),
            },
            embeds: Vec::new(),
            allowed_mentions: Mentions { parse: Vec::new()}
        }
        }
    }

    pub fn later_responce() -> Self {
        Self { r#type: 5,
        data: DiscordData { 
            tts: false,
            content: "test".to_string(),
            embeds: Vec::new(),
            allowed_mentions: Mentions { parse: Vec::new() } 
        }
        }
    }
}
