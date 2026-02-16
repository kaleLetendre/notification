#[cfg(feature = "console")]
pub mod console;

#[cfg(feature = "discord")]
pub mod discord;

#[cfg(feature = "email")]
pub mod email;


pub mod error;
