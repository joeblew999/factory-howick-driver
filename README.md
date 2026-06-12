# factory-howick-driver

<https://github.com/joeblew999/factory-howick-driver>

The **Howick FRAMA** machine driver — the first driver in the `factory-` family.
It implements [`factory-machine-model`](https://github.com/joeblew999/factory-machine-model)'s
`MachineDriver` contract, translating the standard OPC-UA model to the FRAMA's
native interface: a cut-list CSV written to a USB mount the machine reads.
Everything Howick-specific lives here and nowhere else.

Part of the `factory-` family:

| Repo | Role |
|------|------|
| [factory-machine-model](https://github.com/joeblew999/factory-machine-model) | the contract this driver implements |
| [factory-gateway](https://github.com/joeblew999/factory-gateway) | the OPC-UA gateway that composes this driver |
| **factory-howick-driver** (this) | Howick FRAMA driver |
| [howick-rs](https://github.com/joeblew999/howick-rs) | the cut-list / CSV job payload format |

## How it maps to the OPC-UA standard

The gateway publishes this machine under the **OPC UA for Machinery** `Machines/`
folder; this driver supplies the contents:

```text
Machines/howick-1/
├── Identification/        Manufacturer="Howick" · Model="FRAMA" · …   ← from config nameplate
├── MachineryItemState     NotExecuting ⇄ Executing                    ← driver.state()
├── Telemetry/
│     PiecesProduced  (UInt32)                                         ← driver.poll_telemetry()
│     CoilRemaining   (Double, m)   — when the load-cell is fitted
└── JobOrderReceiver       ← gateway; delivers ISA-95 JobOrders to driver.run_job()
```

- **`run_job(job)`** — takes the ISA-95 `JobOrder`, pulls its payload (a Howick
  cut-list CSV — see [`howick-rs`](https://github.com/joeblew999/howick-rs)), and
  writes it to the FRAMA's USB input (re-presenting the USB gadget on a Pi Zero).
- **`state()`** — `Executing` while writing a job, else `NotExecuting`
  (OPC 40001-1 `MachineryItemState`).
- **`poll_telemetry()`** — `PiecesProduced`, plus `CoilRemaining` (load-cell kg →
  metres) when the coil sensor is configured.

## Config

The driver is selected and configured from the factory config's per-machine block:

```toml
[[machine]]
id     = "howick-1"
driver = "howick-frama"
[machine.identification]        # standard OPC-UA Machinery nameplate
manufacturer = "Howick"
model        = "FRAMA"
[machine.howick]                # this driver's typed config (HowickConfig)
usb_mount       = "/mnt/usb_share"
usb_gadget_mode = true
coil_sensor     = true
```

## Licence

MIT OR Apache-2.0.
