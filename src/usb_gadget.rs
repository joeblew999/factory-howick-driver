//! Writing a cut-list to the FRAMA's USB input.
//!
//! On a Pi Zero 2W in `g_mass_storage` gadget mode, writing the file to the
//! mounted image isn't enough — the host (the FRAMA) must be told the storage
//! changed. Off-gadget (Pi 5 / NUC / Mac) it's a plain file write.

use std::path::Path;
use std::time::Duration;

use crate::config::HowickConfig;

/// Write a cut-list CSV to the machine input directory; refresh the USB gadget
/// if configured.
pub async fn write_csv(config: &HowickConfig, filename: &str, csv: &[u8]) -> anyhow::Result<()> {
    let dest = config.usb_mount.join(filename);
    tokio::fs::create_dir_all(&config.usb_mount).await?;
    tokio::fs::write(&dest, csv).await?;
    tracing::info!(path = %dest.display(), bytes = csv.len(), "cut-list written to FRAMA input");

    if config.usb_gadget_mode {
        refresh_usb_gadget().await?;
    }
    Ok(())
}

/// Sync and re-present USB storage to the FRAMA (Pi Zero 2W gadget mode only).
async fn refresh_usb_gadget() -> anyhow::Result<()> {
    if let Err(e) = tokio::process::Command::new("sync").status().await {
        tracing::warn!("sync failed: {e}");
    }
    tokio::time::sleep(Duration::from_millis(200)).await;

    let gadget = "/sys/bus/platform/drivers/dwc2/dwc2/gadget/suspended";
    if Path::new(gadget).exists() {
        let _ = tokio::fs::write(gadget, "1").await;
        tokio::time::sleep(Duration::from_millis(300)).await;
        let _ = tokio::fs::write(gadget, "0").await;
        tracing::info!("USB gadget: storage re-presented to FRAMA");
    } else {
        let script = "/usr/local/bin/usb-refresh.sh";
        if Path::new(script).exists() {
            let _ = tokio::process::Command::new("sh").arg(script).status().await;
        }
    }
    Ok(())
}
