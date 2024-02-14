use crate::Data;

pub mod general;
pub mod quote;
pub mod version;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
