//! Coil load-cell reading.
//!
//! A load cell + HX711 ADC under the coil spool reports the coil weight. A small
//! userspace helper on the Pi writes the latest kg to `/tmp/coil_weight_kg`; this
//! module reads that (or the `COIL_WEIGHT_KG` env override for dev). Returns
//! `None` when no source is available, so a missing sensor never overwrites the
//! last real reading with a spurious zero.

/// Read the current coil weight in kg, or `None` if no sensor source is present.
pub fn read_weight_kg() -> Option<f64> {
    if let Ok(val) = std::env::var("COIL_WEIGHT_KG") {
        if let Ok(kg) = val.trim().parse::<f64>() {
            return Some(kg);
        }
    }
    if let Ok(contents) = std::fs::read_to_string("/tmp/coil_weight_kg") {
        if let Ok(kg) = contents.trim().parse::<f64>() {
            return Some(kg);
        }
    }
    None
}
