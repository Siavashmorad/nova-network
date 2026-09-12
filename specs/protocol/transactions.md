# NOVA Transactions

**Status:** Protocol draft — transaction types are not yet implemented in the current repository

## 1. Transaction identity

A transaction MUST have a deterministic identifier derived from its canonical encoding and a domain-separated cryptographic hash.

## 2. Minimum native transfer fields

The planned native transfer transaction requires, at minimum:

- chain ID;
- sender address;
- recipient address;
- amount;
- nonce;
- fee parameters;
- transaction type/version;
- authorization/signature.

The exact byte representation and signature preimage remain to be implemented.

## 3. Validation

Validation MUST reject malformed encodings, wrong chain ID, invalid authorization, nonce mismatch, insufficient funds, arithmetic overflow, invalid fee values, and any transaction exceeding protocol size limits.

## 4. Replay protection

The chain ID is consensus-critical and MUST be included in the signed transaction domain so a valid transaction from one NOVA network cannot be replayed on another network.

## 5. Determinism

Transaction validity and resulting state changes MUST depend only on the transaction bytes and pre-state. Wall-clock time and local process state must not affect deterministic execution.

## 6. Fees

The fee model is not yet implemented. Before mainnet, the protocol must specify fee denomination, gas/compute accounting, maximum fee behavior, fee deduction ordering, failed-transaction charging, and any burn/reward allocation.
