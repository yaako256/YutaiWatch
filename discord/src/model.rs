/*
discord/src/model.rs
discordに送信する型の定義
*/
use serde::Serialize;

// --- 実際に送信する構造体 ---
#[derive(Debug, Serialize)]
pub struct WebhookPayload {
  pub content: Option<String>,
  pub embeds: Vec<DiscordEmbed>,
}

// --- enbed型定義 ---
#[derive(Debug, Serialize, Clone)]
pub struct DiscordEmbed {
  pub title: String,
  pub description: String,
  pub url: Option<String>,
  pub color: Option<u32>,
  pub fields: Vec<EmbedField>,
  pub footer: Option<EmbedFooter>,
  pub timestamp: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct EmbedField {
  pub name: String,
  pub value: String,
  pub inline: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct EmbedFooter {
  pub text: String,
}
