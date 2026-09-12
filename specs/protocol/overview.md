# NOVA Protocol Specification — Overview

**Status:** Development foundation / non-normative where marked

## 1. Scope

This document defines the protocol specification boundary for NOVA Network and records what is currently implemented versus what must become consensus-critical in later milestones.

The repository currently provides Rust foundations for core protocol types, Blake3 hashing, deterministic account state transitions, durable storage primitives, and a minimal node core. Consensus, canonical wire encoding, Merkleized state, networking, RPC, validators, and production cryptography are not yet implemented in the current repository.

## 2. Design goals

NOVA is intended to become an independent Layer-1 with:

- deterministic state transition and block validity;
- deterministic finality under a Byzantine-fault-tolerant consensus protocol;
- predictable transaction fees;
- parallel transaction execution where conflicts can be detected deterministically;
- validator-based proof of stake;
- light-client support;
- mobile-first wallet and node-adjacent tooling.

These are target properties, not current performance or production claims.

## 3. Canonicality rules

Future consensus-critical data MUST have one canonical representation. JSON, map iteration order, platform-native serialization, locale-dependent formatting, and implementation-defined integer encodings MUST NOT be used directly for consensus hashing.

The canonical encoding specification must define, at minimum:

1. integer widths and byte order;
2. length-prefix rules;
3. enum discriminants;
4. optional-value representation;
5. list ordering;
6. address and hash byte order;
7. domain separation for cryptographic hashes;
8. rejection rules for non-canonical encodings.

Until that specification is implemented and tested, no mainnet compatibility claim should be made.

## 4. Current primitive types

The current Rust type foundation defines:

- `Hash`: 32-byte array;
- `Address`: 20-byte array;
- `Amount`: unsigned 128-bit integer;
- `Account`: `{ balance: Amount, nonce: u64 }`.

These definitions are implementation facts, not yet a complete wire-level protocol specification.

## 5. Current cryptographic primitive

The current crypto crate exposes deterministic Blake3 hashing over arbitrary bytes and returns a 32-byte digest. A production protocol must additionally specify domain separation, canonical input encoding, signature algorithms, key formats, address derivation, and verification rules.

## 6. Current state transition

The current state foundation uses a deterministic ordered map of addresses to accounts. Transfers validate the sender nonce and balance, perform checked arithmetic, increment the sender nonce, and update balances atomically with respect to the transfer operation.

The state transition currently exposes explicit errors for insufficient balance, nonce mismatch, and arithmetic overflow.

## 7. Planned protocol layers

The complete specification is expected to cover:

- `encoding.md` — canonical binary representation;
- `transactions.md` — transaction model and validity;
- `blocks.md` — block/header/body model;
- `state.md` — state model, commitments, and transition function;
- `genesis.md` — chain identity and genesis state;
- `execution.md` — deterministic execution and future parallel execution;
- consensus specifications — proposer selection, rounds, votes, finality, validator sets, and slashing;
- networking specifications — peer identity, transport, gossip, synchronization;
- economics — supply, issuance, fees, rewards, and governance.

## 8. Compatibility policy

A future implementation is conformant only if it produces the same consensus-critical bytes, hashes, validity decisions, state roots, receipts, and finality decisions for the same valid inputs.

Reference vectors must be generated only after the corresponding canonical implementation exists. No expected hash or root should be invented from an informal description.

## 9. Security status

NOVA is **NOT PRODUCTION READY**. This repository is a development foundation. No testnet, mainnet, TPS, latency, validator-count, or security guarantee should be inferred from the design targets in this document.
