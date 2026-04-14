# IPv8 Addressing Model

## Overview

MatrixNet uses 128-bit addresses to identify nodes.

---

## Address Structure

| Segment     | Size | Description |
|-------------|------|------------|
| Region ID   | 32b  | Network region |
| Zone ID     | 32b  | Sub-network |
| Node ID     | 64b  | Unique node |

---

## Example

0xAABBCCDD-11223344-5566778899AABBCC

---

## Types

- Static (server-like nodes)
- Ephemeral (AI agents, temporary clients)
- Virtual (scene instances)

---

## Features

- Supports massive scale
- Allows spatial mapping (future)
- Compatible with identity rotation
