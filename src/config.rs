//! The typed `[machine.howick]` config sub-section.
//!
//! In a factory config, a Howick machine looks like:
//!
//! ```toml
//! [[machine]]
//! id     = "howick-1"
//! driver = "howick-frama"
//! [machine.identification]            # standard OPC-UA Machinery nameplate
//! manufacturer = "Howick"
//! model        = "FRAMA"
//! [machine.howick]                    # this struct — driver-specific
//! usb_mount       = "/mnt/usb_share"
//! usb_gadget_mode = true
//! coil_sensor     = true
//! ```
//!
//! The gateway hands the `[machine.howick]` table to this driver verbatim; the
//! gateway itself never parses these fields.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Howick-specific machine configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct HowickConfig {
    /// Directory the FRAMA reads CSVs from. On a Pi Zero USB gadget this is the
    /// mounted image; elsewhere just a watched folder.
    pub usb_mount: PathBuf,
    /// True on a Pi Zero 2W acting as USB mass storage — re-present after each write.
    pub usb_gadget_mode: bool,
    /// True when the coil load-cell is fitted and calibrated.
    pub coil_sensor: bool,
    /// Weight of the empty coil spool in kg.
    pub empty_spool_kg: f64,
    /// Steel consumed per metre of profile, kg/m (default calibrated for S8908).
    pub steel_kg_per_m: f64,
}

impl Default for HowickConfig {
    fn default() -> Self {
        Self {
            usb_mount: PathBuf::from("/mnt/usb_share"),
            usb_gadget_mode: false,
            coil_sensor: false,
            empty_spool_kg: 18.0,
            steel_kg_per_m: 0.74,
        }
    }
}

impl HowickConfig {
    /// Convert a raw load-cell reading (kg) to metres of steel remaining.
    /// Returns 0.0 once the coil is exhausted (or not fitted).
    pub fn coil_metres(&self, raw_weight_kg: f64) -> f64 {
        let steel_kg = raw_weight_kg - self.empty_spool_kg;
        if steel_kg <= 0.0 {
            return 0.0;
        }
        (steel_kg / self.steel_kg_per_m).max(0.0)
    }
}
