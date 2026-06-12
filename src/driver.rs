//! `HowickFrama` — the [`MachineDriver`] implementation for Howick FRAMA
//! roll-formers. Everything Howick-specific lives here; the gateway only ever
//! sees the standard `factory-machine-model` contract.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use factory_machine_model::{
    Identification, JobOrder, MachineDescriptor, MachineDriver, MachineryItemState, Telemetry,
    TelemetryField, Value, ValueKind,
};

use crate::config::HowickConfig;
use crate::{sensor, usb_gadget};

/// Telemetry BrowseNames exposed under `Machines/<id>/Telemetry/`.
pub const PIECES_PRODUCED: &str = "PiecesProduced";
pub const COIL_REMAINING: &str = "CoilRemaining";

/// Driver for one Howick FRAMA wired to this host.
pub struct HowickFrama {
    machine_id: String,
    identification: Identification,
    config: HowickConfig,
    pieces_produced: AtomicU64,
    running: AtomicBool,
}

impl HowickFrama {
    /// Build from a machine id, its standard nameplate, and the typed Howick config.
    pub fn new(machine_id: impl Into<String>, identification: Identification, config: HowickConfig) -> Self {
        Self {
            machine_id: machine_id.into(),
            identification,
            config,
            pieces_produced: AtomicU64::new(0),
            running: AtomicBool::new(false),
        }
    }
}

impl MachineDriver for HowickFrama {
    fn descriptor(&self) -> MachineDescriptor {
        MachineDescriptor {
            machine_id: self.machine_id.clone(),
            kind: "howick-frama".to_owned(),
            identification: self.identification.clone(),
            telemetry: vec![
                TelemetryField::new(PIECES_PRODUCED, ValueKind::UInt, None),
                TelemetryField::new(COIL_REMAINING, ValueKind::Double, Some("m")),
            ],
        }
    }

    async fn state(&self) -> MachineryItemState {
        if self.running.load(Ordering::Relaxed) {
            MachineryItemState::Executing
        } else {
            MachineryItemState::NotExecuting
        }
    }

    async fn run_job(&self, job: &JobOrder) -> anyhow::Result<()> {
        let payload = job
            .payload()
            .ok_or_else(|| anyhow::anyhow!("job {} carries no cut-list payload", job.job_order_id))?;
        let filename = format!("{}.csv", job.job_order_id);

        self.running.store(true, Ordering::Relaxed);
        let result = usb_gadget::write_csv(&self.config, &filename, payload).await;
        self.running.store(false, Ordering::Relaxed);
        result?;

        self.pieces_produced.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    async fn poll_telemetry(&self) -> anyhow::Result<Telemetry> {
        let mut t = Telemetry::new();
        t.insert(
            PIECES_PRODUCED.to_owned(),
            Value::UInt(self.pieces_produced.load(Ordering::Relaxed)),
        );
        if self.config.coil_sensor
            && let Some(kg) = sensor::read_weight_kg()
        {
            t.insert(COIL_REMAINING.to_owned(), Value::Double(self.config.coil_metres(kg)));
        }
        Ok(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn driver() -> HowickFrama {
        let mut cfg = HowickConfig::default();
        cfg.usb_mount = std::env::temp_dir().join("factory-howick-test");
        HowickFrama::new("howick-1", Identification::new("Howick", "FRAMA"), cfg)
    }

    #[test]
    fn descriptor_is_standard_plus_howick_telemetry() {
        let d = driver().descriptor();
        assert_eq!(d.kind, "howick-frama");
        assert_eq!(d.identification.manufacturer, "Howick");
        let names: Vec<&str> = d.telemetry.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(names, [PIECES_PRODUCED, COIL_REMAINING]);
    }

    #[tokio::test]
    async fn runs_a_job_and_counts_a_piece() {
        let d = driver();
        assert_eq!(d.state().await, MachineryItemState::NotExecuting);
        let job = JobOrder::with_payload("T1-1", "CutListCsv", b"UNIT,MILLIMETRE\n".to_vec());
        d.run_job(&job).await.unwrap();
        let t = d.poll_telemetry().await.unwrap();
        assert_eq!(t.get(PIECES_PRODUCED), Some(&Value::UInt(1)));
        assert!(!t.contains_key(COIL_REMAINING), "no coil node when sensor off");
    }
}
