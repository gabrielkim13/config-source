//! # config-source
//!
//! Macro for deriving [config-rs](https://github.com/mehcode/config-rs)'s `config::Source` trait.
//!
//! ## Usage
//!
//! To derive `config::Source` for a struct, simply add the `#[derive(ConfigSource)]` attribute to
//! the struct:
//!
//! ```
//! #[derive(serde::Deserialize, config_source::ConfigSource, Clone, Debug)]
//! pub struct MyConfig {
//!     pub question: String,
//!     pub answer: u64,
//! }
//!
//! impl Default for MyConfig {
//!     fn default() -> Self {
//!         Self {
//!             question: String::from("The Ultimate Question of Life, the Universe, and Everything"),
//!             answer: 42,
//!         }
//!     }
//! }
//! ```
//!
//! Then, you can use the `config::Config` struct to load your configuration using a default value:
//!
//! ```
//! # #[derive(serde::Deserialize, config_source::ConfigSource, Clone, Debug)]
//! # pub struct MyConfig {
//! #     pub question: String,
//! #     pub answer: u64,
//! # }
//! #
//! # impl Default for MyConfig {
//! #     fn default() -> Self {
//! #         Self {
//! #             question: String::from("The Ultimate Question of Life, the Universe, and Everything"),
//! #             answer: 42,
//! #         }
//! #     }
//! # }
//! #
//! let config = config::Config::builder()
//!     .add_source(MyConfig::default()) // Default value as source!
//!     .add_source(config::File::with_name("my_config.toml").required(false))
//!     .add_source(config::Environment::with_prefix("MY_CONFIG").separator("__"))
//!     .build()
//!     .expect("Failed to build `MyConfig`");
//! ```

#![warn(missing_docs)]

mod config;

use proc_macro::TokenStream;

/// Derives `config::Source` for a struct.
#[proc_macro_derive(ConfigSource)]
pub fn config_source(input: TokenStream) -> TokenStream {
    config::derive(input)
}
