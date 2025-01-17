#![doc = include_str!("../README.md")]
#![doc(
  html_logo_url = "https://top.gg/favicon.png",
  html_favicon_url = "https://top.gg/favicon.png"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
mod snowflake;

cfg_if::cfg_if! {
  if #[cfg(feature = "api")] {
    mod client;
    mod error;
    mod http;
    mod util;

    /// Bot-related structs.
    pub mod bot;

    /// User-related structs.
    pub mod user;

    #[doc(inline)]
    pub use bot::{Filter, NewStats, Query};
    pub use client::Client;
    pub use error::{Error, Result};
    pub use snowflake::SnowflakeLike; // for doc purposes
  }
}

cfg_if::cfg_if! {
  if #[cfg(feature = "autoposter")] {
    mod autoposter;

    #[cfg_attr(docsrs, doc(cfg(feature = "autoposter")))]
    pub use autoposter::Autoposter;
  }
}

cfg_if::cfg_if! {
  if #[cfg(feature = "webhook")] {
    mod webhook;

    pub use webhook::*;
  }
}
