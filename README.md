# entanglement-net

Binary wire protocol and networking layer for the MMO server infrastructure.

## Overview

`entanglement-net` defines the zero-copy binary protocol used between server and clients over UDP. All message types are `#[repr(C, packed)]` structs with `to_wire()`/`from_wire()` endian conversion, designed for direct memory mapping without serialization overhead.

### Precision model

The server simulates physics in **f64** (double precision) for accuracy over large world coordinates. When sending state to clients, positions and velocities are converted to **f32** (single precision) via `world_to_wire()`:

```
Server simulation (f64) → subtract shard origin → cast to f32 → EntityMoveCompact (32 bytes)
```

The shard origin subtraction keeps coordinates small (relative to shard center), minimizing f32 precision loss. The client receives f32 values and uses them directly for rendering and client-side prediction.

### What the client sends

The client sends **PlayerMove** (24 bytes) to the server:

| Field | Type | Description |
|-------|------|-------------|
| `input_sequence` | u32 | Monotonic sequence for server ack / client reconciliation |
| `estimated_server_tick` | u32 | Client's estimate of current server tick |
| `move_x` | f32 | Movement input X axis (-1.0 to 1.0) |
| `move_z` | f32 | Movement input Z axis (-1.0 to 1.0) |
| `orientation` | f32 | Player facing direction (radians) |
| `buttons` | u32 | Bitfield for actions (jump, attack, etc.) |

These are **input vectors**, not positions — the server applies them to the f64 simulation authoritatively.

### What the server sends

The server broadcasts **EntityMoveCompact** (32 bytes per entity) in batches:

```
UDP segment: [MsgHeader 6B] [EntityMoveBatch: server_tick 4B] [EntityMoveCompact₁ 32B] ... [EntityMoveCompactₙ 32B]
```

| Field | Type | Description |
|-------|------|-------------|
| `entity_id` | u32 | Entity identifier |
| `x, y, z` | f32 | Position (shard-relative, origin subtracted) |
| `orientation` | f32 | Facing direction |
| `vx, vy, vz` | f32 | Velocity vector |

The server also sends **StateAck** (44 bytes) per-player as authoritative correction:

| Field | Type | Description |
|-------|------|-------------|
| `input_sequence_acked` | u32 | Last processed input sequence |
| `server_tick` | u32 | Current server tick |
| `tick_delta_us` | u32 | Tick duration (for client timing) |
| `x, y, z` | f32 | Authoritative position |
| `vx, vy, vz` | f32 | Authoritative velocity |
| `hp` | u32 | Health points |
| `stamina` | f32 | Stamina value |

The client uses `input_sequence_acked` to reconcile its prediction buffer — replaying un-acked inputs on top of the server's authoritative state.

## Schema

Message types are defined in `schemas/messages.toml` and code-generated:

```bash
cargo run -p entanglement-net-codegen
```

## Tests

### Wire format regression test (headless)

Deterministic test that catches any change to the wire representation — intentional or accidental. This is critical before implementing delta compression or position quantization, which could silently alter simulation data.

```bash
cargo test -p entanglement-net --test wire_snapshot
```

**What it does:** Simulates 500 entities in deterministic circular motion for 120 ticks, capturing wire-format snapshots at 5 checkpoints. On first run it generates a golden reference file (`tests/golden/wire_snapshot.bin`). On subsequent runs it compares against it and reports:

- **Simulation (f64):** Must be **identical** — test fails if physics change. This means game logic has diverged.
- **Wire format (f32):** Reports per-field deltas if changed — expected when adding compression. Shown as a table with max/mean delta per field so you can evaluate the precision trade-off.

**Example output when wire format changes:**
```
Simulation ✓ IDENTICAL
Wire format ⚠ 1200 / 2500 values changed
Field          Max Δ        Mean Δ       Affected
x              0.001953     0.000488     1200
vz             0.000244     0.000061     800
```

**To accept changes:** Delete `tests/golden/wire_snapshot.bin` and re-run to regenerate.

### Full-stack wire capture test (live)

Validates the complete server→network→client pipeline by capturing actual EntityMoveCompact data received by a bot during a live benchmark. Lives in [mmo-client-test](https://github.com/Jaisiero/mmo-client-test).

**Capture a baseline** (requires running shard + bots):
```powershell
$env:CLIENT_CAPTURE_FILE="baseline.bin"
$env:CLIENT_BOT_COUNT="500"; $env:CLIENT_DURATION_SECS="75"
cargo run --release
```

**Compare after wire format changes:**
```powershell
$env:CLIENT_CAPTURE_FILE="current.bin"
$env:CLIENT_VERIFY_FILE="baseline.bin"
cargo run --release
```

Bot 0 records every received EntityMoveCompact (decoded to f64 world coordinates, sorted by server_tick and entity_id). The comparison reports:

- **Matched/missing/extra ticks and entities** — detects connectivity or timing differences.
- **Per-field deltas** (x, y, z, orientation, vx, vy, vz) with max/mean statistics — same format as the headless test.

```
═══ Wire Capture Comparison ═══
  Ticks:    842 matched, 0 missing, 3 extra
  Entities: 421000 matched, 0 missing, 0 extra
  Values:   ✓ ALL IDENTICAL (421000 entity-ticks compared)
```

> **Note:** Live captures are not perfectly deterministic across runs (network timing, tick alignment). Small numbers of missing/extra ticks are normal. The key signal is whether *matched* entity-tick values show systematic deltas — that indicates a wire format change.

### Roundtrip tests

Verifies every message type survives `to_wire()` → `from_wire()` bit-exactly, including struct sizes:

```bash
cargo test -p entanglement-net --test roundtrip
```

## Delta Encoding (Client-Facing)

Client-facing delta encoding reduces bandwidth for `EntityMoveCompact` broadcasts
by grouping entities that share the same changed-field pattern and sending only
the changed fields.

### Wire format — `ENTITY_MOVE_DELTA_BATCH` (0x0109)

```
MsgHeader:    6B  (standard: msg_type + length + sequence)
server_tick:  4B  (u32, current server tick)
bitfield:     1B  (u8, which of 7 fields follow per entity)
N × entry:    (4 + popcount(bitfield) × 4) bytes each
  entity_id:  4B  (u32, always present)
  [fields...]:    only changed fields, in bit order, f32 little-endian

Bitfield bits:
  0: x       1: y       2: z       3: orientation
  4: vx      5: vy      6: vz
```

- **Stride per entity**: `4 + popcount(bitfield) × 4` bytes
- **All entities in a batch share the same bitfield** — the server groups entities
  by change pattern and emits one batch per pattern.
- **Keyframe interval**: every N ticks (default 60, ~0.5s at 120Hz), staggered
  per client via `(server_tick + player_eid) % interval`, a full
  `EntityMoveCompact` batch is sent to recover from UDP loss.

### Global bitfield history (V4 architecture)

Instead of per-session baseline HashMaps, the server maintains a global
ring buffer of per-entity bitfields:

1. **Pre-tick**: compare each entity's current state against previous tick →
   compute 7-bit changed-field bitfield, store in ring slot `tick % 8`.
2. **Per-client send**: cumulative-OR recent bitfields based on the client's
   LOD skip gap (1–4 ticks) to determine which fields changed since last send.
3. **Batch grouping**: entities are grouped by identical cumulative bitfield
   pattern → one `ENTITY_MOVE_DELTA_BATCH` per pattern.

This eliminates O(sessions × entities) hash lookups, replacing them with
O(entities) bitfield pre-computation + O(entities) byte lookups per session.

### Configuration

| Environment variable | Default | Description |
|---|---|---|
| `SHARD_CLIENT_DELTA_ENCODING` | `true` | Enable/disable client delta encoding |
| `SHARD_CLIENT_DELTA_KEYFRAME_INTERVAL` | `60` | Ticks between full keyframes |

## Delta Encoding (Inter-Shard)

The `messages::delta` module provides bitfield-based delta encoding for `IntershardEntityUpdate` messages, reducing inter-shard bandwidth by only transmitting fields that changed since the last sent baseline.

### Wire format

```
entity_id:  u32  (4B, always present)
seq:        u16  (2B, monotonic counter for stale-packet detection)
bitfield:   u16  (2B, which fields follow)
[fields...]:      only changed fields, in field order, little-endian

Bitfield bits:
  0: x (f32)    1: y (f32)     2: z (f32)     3: orientation (f32)
  4: vx (f32)   5: vy (f32)    6: vz (f32)    7: hp (u32)
  8: combat_state (u8)
```

- **Header**: 8 bytes (entity_id + seq + bitfield).
- **Min payload**: 8 bytes (header only, all fields unchanged — typically skipped entirely).
- **Max payload**: 41 bytes (all 9 fields changed). Full `IntershardEntityUpdate` is 40 bytes + 6B msg header = 46 bytes.
- **Message type**: `INTERSHARD_ENTITY_UPDATE_DELTA` (0x0314).

### API

```rust
use entanglement_net::messages::delta;

// Sender: compute which fields changed, encode delta
let bitfield = delta::compute_bitfield(&baseline, &current);
let len = delta::encode(entity_id, seq, bitfield, &current, &mut buf);

// Receiver: decode delta, mutating baseline in-place
if let Some((entity_id, seq)) = delta::decode(payload, &mut baseline) {
    // baseline now contains the updated state
}
```

### Benchmark results (500 bots, dual-shard, boundary PvP)

~80% of inter-shard entity updates only change 1 field (x position), yielding ~72% theoretical bandwidth savings. Measured A/B comparison:

| Metric | Delta ON | Delta OFF | Improvement |
|--------|----------|-----------|-------------|
| inter_us avg (α) | 108 µs | 125 µs | **-14%** |
| inter_us avg (β) | 103 µs | 119 µs | **-13%** |
| tick_us avg (α) | 1,772 µs | 1,871 µs | **-5%** |
| tick_us avg (β) | 1,660 µs | 1,747 µs | **-5%** |

## Crate structure

```
entanglement-net/
├── schemas/messages.toml        # Message definitions (source of truth)
├── entanglement-net/            # Core library crate
│   ├── src/messages.rs          # Generated wire types (#[repr(C, packed)])
│   ├── src/session.rs           # Session management
│   ├── tests/roundtrip.rs       # Message roundtrip tests
│   ├── tests/wire_snapshot.rs   # Deterministic regression test
│   └── tests/golden/            # Golden reference files
├── entanglement-net-codegen/    # Code generator
└── include/entanglement_net.h   # C/C++ header (also generated)
```
