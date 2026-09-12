# NOVA Genesis

**Status:** Protocol draft — genesis is not yet implemented as a production specification

## 1. Chain identity

Genesis MUST uniquely define a NOVA network through a chain ID and a genesis block/header commitment. Nodes MUST reject blocks and transactions from incompatible chain identities.

## 2. Genesis state

Genesis state must deterministically initialize all required accounts and protocol configuration. The canonical representation must define account ordering and exact byte encoding before a genesis state root can be normative.

## 3. Initial supply

Any native-coin initial supply is a protocol parameter and MUST be explicitly represented in genesis configuration/state. It MUST NOT exist only as an unused constant in application code.

## 4. Validators

If the genesis network starts with validators, the initial validator set, voting power/stake, consensus keys, and activation rules must be encoded deterministically in genesis.

## 5. Reproducibility

Independent implementations given the same genesis configuration MUST compute the same genesis state commitment and genesis block identifier.

## 6. Genesis files

The future repository should provide machine-readable genesis artifacts plus human-readable documentation, with CI tests that regenerate and compare the canonical commitments.
