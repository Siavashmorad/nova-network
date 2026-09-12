# NOVA State Specification

**Status:** Current implementation record; consensus extensions are planned

## 1. State model

The current state implementation stores accounts in an ordered map keyed by a 20-byte `Address`.

An account contains:

| Field | Type | Meaning |
|---|---|---|
| `balance` | `u128` | Native account balance |
| `nonce` | `u64` | Sender transaction sequence |

An absent account is represented by the default account value: zero balance and zero nonce.

## 2. Transfer transition

For a transfer `(from, to, amount, nonce)`, the current implementation performs these checks in order:

1. Read the sender account.
2. Require `sender.nonce == nonce`.
3. Require `sender.balance >= amount`.
4. For a self-transfer, preserve the balance, increment the nonce, and commit the updated account.
5. Otherwise, subtract `amount` from the sender using checked arithmetic.
6. Add `amount` to the receiver using checked arithmetic.
7. Increment the sender nonce using checked arithmetic.
8. Commit both updated accounts.

Failures are represented by `NonceMismatch`, `InsufficientBalance`, or `Overflow`.

## 3. Determinism

The state transition uses explicit integer arithmetic and an ordered map. It does not depend on wall-clock time, randomness, locale, or thread scheduling.

The current implementation therefore provides a useful deterministic foundation, but it does not yet define a cryptographic state commitment.

## 4. State root — planned

A production NOVA protocol requires a canonical state commitment. The intended design is a Merkleized state tree with deterministic key ordering and Blake3-based hashing. The exact leaf format, internal-node format, empty-tree value, domain separation, and proof format MUST be specified before state roots become consensus-critical.

No state-root hash should be treated as normative until these rules are implemented and covered by reference vectors.

## 5. Atomicity requirement

A failed transfer MUST NOT partially modify state. The current transfer implementation calculates checked values before committing the sender and receiver updates, satisfying this requirement for the implemented transfer operation.

Future transaction types must preserve the same all-or-nothing state-transition property.

## 6. Future extensions

The complete state specification must define:

- account creation and deletion semantics;
- transaction fee accounting;
- native coin issuance;
- validator and delegation state;
- contract storage;
- state versioning;
- snapshots and pruning;
- Merkle proofs;
- genesis allocation;
- state synchronization;
- compatibility rules across protocol upgrades.
