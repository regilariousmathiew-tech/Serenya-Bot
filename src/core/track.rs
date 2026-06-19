use poise::serenity_prelude as serenity;

#[derive(Debug, Clone)]
pub struct Track {
    pub title: String,
    pub url: String,
    pub duration: Option<std::time::Duration>,
    pub requester_id: serenity::UserId,
    pub requester_name: String,
    pub source_type: SourceType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceType {
    Url,
    Search,
    Playlist,
}
