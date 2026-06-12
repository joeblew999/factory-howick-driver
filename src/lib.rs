//! # factory-howick-driver
//!
//! The Howick FRAMA machine driver — the first driver in the `factory-` family.
//! Implements [`factory_machine_model::MachineDriver`], translating the standard
//! contract to the FRAMA's native interface (a cut-list CSV written to a USB
//! mount the machine reads). Everything Howick-specific lives here.
//!
//! The gateway composes this crate when a factory's config declares a machine
//! with `driver = "howick-frama"`.

pub mod config;
pub mod driver;
pub mod sensor;
pub mod usb_gadget;

pub use config::HowickConfig;
pub use driver::HowickFrama;

/// The driver `kind` string this crate handles — matches `driver = "..."` in config.
pub const KIND: &str = "howick-frama";
