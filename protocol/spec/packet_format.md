# MatrixNet IPv8 Packet Format

## Overview

MatrixNet uses a custom binary packet format optimized for:

- 3D scene streaming
- Low overhead
- Extensibility

---

## Packet Structure

| Field        | Size        | Description |
|--------------|------------|------------|
| Version      | u8         | Protocol version |
| Packet Type  | u8         | Scene, Asset, Control |
| Flags        | u16        | Compression, encryption flags |
| Length       | u32        | Payload size |
| Source       | u128       | Source address |
| Destination  | u128       | Destination address |
| Payload      | bytes      | Encoded data |

---

## Packet Types

| Type | Name       |
|------|------------|
| 0x01 | Scene      |
| 0x02 | Mesh       |
| 0x03 | Texture    |
| 0x04 | Control    |

---

## Flags

| Bit | Meaning |
|-----|--------|
| 0   | Compressed |
| 1   | Encrypted |
| 2   | Fragmented |

---

## Notes

- Payload may be compressed (LZ4 or Zstd)
- Payload may be encrypted (AES-GCM)
- Large scenes are fragmented across multiple packets
