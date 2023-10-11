//! # Sonoff mini R3
//!
//! This crate provides a high-level client for official Sonoff mini R3 DIY API.
//!
//! Note that before using this library you should enter your device into DIY mode. More details on
//! how to do that can be found in [official documentation](https://sonoff.tech/diy-developer/).
//! Also you may need to read [API documentation](https://sonoff.tech/diy-developer/) which is used
//! to implement this lib.
//!
//! Currently library provides limited amount of features:
//! - fetching device info (only few attributes)
//! - setting startup position
//! - setting current switch position
//!
//!
//! Note that doscovery via mDNS is not supported, so you should know IP address of your device.
//! Port is 8081 by default (just try it, should work).
//!
//! Example:
//! ```ignore
//! use sonoff_minir3::Client;
//!
//! let client = Client::new("192.168.1.75", 8081);
//!
//!
//! // Fetch device's info
//! let got = client.fetch_info().await;
//!
//! assert_eq!(
//!     got.unwrap(),
//!     Info {
//!         switch: SwitchPosition::Off,
//!         startup: StartupPosition::Off
//!     }
//! )
//!
//!
//! // Set startup position
//! client.set_startup_position(StartupPosition::Stay).await;
//!
//!
//! // Set current switch position
//! client.set_switch_position(SwitchPosition::On).await;
//! ```
mod client;
mod models;

pub use client::*;
pub use models::*;
