# NOVA Blocks

**Status:** Protocol draft — block structures are not yet implemented in the current repository

## 1. Block purpose

A block commits to an ordered set of transactions and the resulting chain state. Block validity must be deterministic and independently verifiable by every full node.

## 2. Planned header commitments

The production design is expected to include commitments for:

- protocol version;
- chain height;
- epoch/validator-set version;
- proposer identity;
- parent block hash;
- state root;
- transaction root;
- receipt root;
- validator-set commitment;
- consensus round;
- protocol-defined timestamp/slot;
- execution/fee accounting;
- proposer authorization.

The exact fields, widths, order, and signing rules are not yet normative.

## 3. Block body

The block body will contain transactions in canonical order. Any transaction ordering used for execution or Merkleization MUST be identical across implementations.

## 4. Block identity

The block hash MUST be derived from canonical header bytes. The hash domain and treatment of the proposer signature must be explicitly defined.

## 5. Validity

A node must verify the parent relationship, header encoding, proposer authorization, transaction validity, execution result, commitments, and consensus proof before accepting a block as valid.

## 6. Performance targets

A future benchmark target may include approximately one-second block production and two-to-three-second deterministic finality, but these are engineering targets only. They are not measured results.
