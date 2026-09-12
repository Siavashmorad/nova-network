# NOVA Protocol Specification — Phase 1

This directory separates current implementation facts from future protocol requirements.

## Documents

- [Overview](./overview.md)
- [Canonical Encoding](./encoding.md)
- [Transactions](./transactions.md)
- [Blocks](./blocks.md)
- [State](./state.md)
- [Genesis](./genesis.md)
- [Execution](./execution.md)

## Normative status

Documents explicitly marked **draft** are design constraints, not evidence that the corresponding implementation exists.

Reference test vectors must be generated from executable implementations. They must not be fabricated from prose.

## Current implementation boundary

The current repository contains Rust foundations for types, Blake3 hashing, deterministic account state transitions, storage, and a minimal node. Consensus and the remaining production protocol layers are later milestones.
