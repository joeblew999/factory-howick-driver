# fixtures

Real Howick **cut-list CSV** samples — the format this driver delivers to a FRAMA.
Used by the driver's tests as realistic payloads.

| File | What |
|------|------|
| `T1.csv` | a roof-truss frameset (`FRAMESET,T1`), profile S8908 |
| `W1.csv` | a wall frameset (`FRAMESET,W1`) |

Format (Howick FrameCAD export): a header (`UNIT,MILLIMETRE` · `PROFILE,...`),
then `FRAMESET,<name>`, then one `COMPONENT,...` row per member listing operations
(`DIMPLE`, `LIP_CUT`, `WEB`, ...) at their millimetre offsets.

These are generic format examples. Customer- or project-specific exports live in
the private [factory-customers](https://github.com/joeblew999/factory-customers) repo.
