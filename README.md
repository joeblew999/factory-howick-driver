# factory-howick-driver

<https://github.com/joeblew999/factory-howick-driver>

The **Howick FRAMA** machine driver — the first driver in the `factory-` family.
Runs at the edge (a Raspberry Pi wired to the machine), translates the machine's
native protocol to the standard machine model, and presents it to the gateway.
Everything Howick-specific lives here and nowhere else.

Part of the `factory-` family:

| Repo | Role |
|------|------|
| [factory-machine-model](https://github.com/joeblew999/factory-machine-model) | the contract this driver implements (`MachineDriver`) |
| [factory-gateway](https://github.com/joeblew999/factory-gateway) | the OPC-UA gateway this driver reports to |
| **factory-howick-driver** (this) | Howick FRAMA edge driver |
| [howick-rs](https://github.com/joeblew999/howick-rs) | the cut-list / CSV job payload format |

## What it does

- **`descriptor()`** — declares Howick identity + its two telemetry fields
  (`PiecesProduced`, `CoilRemaining`). Everything else (Status, Jobs) is generic.
- **`run_job()`** — writes the cut-list CSV to the USB-gadget mount the FRAMA reads.
- **`poll_telemetry()`** — reads the coil load-cell, converts kg → metres remaining.

Job payloads are Howick cut-list CSV — see
[`howick-rs`](https://github.com/joeblew999/howick-rs) for the format.

## Depends on

- [`factory-machine-model`](https://github.com/joeblew999/factory-machine-model)
  — implements its `MachineDriver` trait.

## Status — extraction in progress 🚧

Being carved out of [`opcua-howick`](https://github.com/joeblew999/opcua-howick)
(crate `howick-frama`). The driver impl already exists as
[`howick-frama/src/driver.rs`](https://github.com/joeblew999/opcua-howick/blob/feat/standard-machine-model/crates/howick-frama/src/driver.rs)
on branch `feat/standard-machine-model` — that `HowickFrama` impl is the proof the
machine fits the generic model. What moves here: that driver, the edge agent
(OPC-UA client + USB-gadget writer + coil sensor), and the Pi setup docs.
