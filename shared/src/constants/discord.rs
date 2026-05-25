/*
shared/src/constants/discord.rs
Discord関連の定数設定
*/

// Discordの制限文字数
// 本文の制限
pub const CONTENT_LIMIT: usize = 2000;

// embedの制限
// タイトル
pub const EMBED_TITLE_LIMIT: usize = 256;
// 説明
pub const EMBED_DESCRIPTION_LIMIT: usize = 4096;
// フィールド名と値
pub const EMBED_FIELD_NAME_LIMIT: usize = 256;
pub const EMBED_FIELD_VALUE_LIMIT: usize = 1024;
// embed全体
pub const EMBED_TOTAL_LIMIT: usize = 6000;
// embedの数
pub const EMBED_COUNT_LIMIT: usize = 10;
