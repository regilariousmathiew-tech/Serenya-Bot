pub mod meta;
pub mod playback;
pub mod voice;

use crate::utils::Error;

/// Register all bot commands.
pub fn all_commands() -> Vec<poise::Command<crate::Data, Error>> {
    vec![
        meta::ping(),
        meta::about(),
        playback::play(),
        voice::join(),
        voice::leave(),
    ]
}
