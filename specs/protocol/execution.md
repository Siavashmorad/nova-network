# NOVA Execution

**Status:** Protocol draft — execution engine is not yet implemented in the current repository

## 1. State transition function

Execution is modeled as a deterministic function:

`post_state = F(pre_state, ordered_transactions, protocol_parameters)`

For identical inputs, every conforming node MUST produce identical state, receipts, fees, and state commitment.

## 2. Native execution

The first execution milestone should implement native transfers using checked arithmetic and explicit nonce validation, preserving the deterministic semantics already established by the current state foundation.

## 3. Parallel execution target

A future parallel executor may speculatively execute transactions concurrently and record read/write sets. Conflicting transactions must be deterministically detected, ordered, and re-executed or rolled back according to protocol rules.

Parallel execution MUST NOT change the result compared with canonical sequential execution.

## 4. Atomicity

A transaction either commits its complete state transition or commits no state changes. Intermediate speculative state must never become consensus-visible unless the transaction passes final validation.

## 5. Future smart contracts

EVM and/or WASM execution are future milestones. Their deterministic runtime versions, gas schedule, host functions, memory limits, and serialization rules must be specified before they become consensus-critical.

## 6. Receipts

A production execution layer must define receipt status, gas/compute usage, logs/events, fee accounting, and receipt commitment. These fields are currently absent from the repository's protocol implementation.
