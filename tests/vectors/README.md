# NOVA Reference Test Vectors

Phase 2 establishes byte-level fixtures only for primitives that are actually implemented and independently specified.

## Current fixture

`blake3-official.json` contains the first 8 official unkeyed BLAKE3 vectors (32-byte default output), using the deterministic repeating-byte input rule from the upstream BLAKE3 test-vector corpus.

## Not yet generated

Transaction, signature, address, Merkle-root, block-header, and block-hash vectors are intentionally absent until NOVA defines and implements the corresponding canonical encoders. The Phase 1 specification explicitly says those exact field widths, orderings, signature preimages, and hash domains are not yet normative. Generating expected bytes before the encoder exists would make the vectors assumptions rather than reference artifacts.

## Acceptance rule

A vector becomes authoritative only when:

1. the input is fully specified;
2. canonical bytes are produced by the NOVA implementation;
3. an independent verifier reproduces the expected output;
4. CI executes the vector test on every relevant change.

No private production key material belongs in the repository. Any signing vectors must use clearly documented deterministic test keys and never a wallet seed or real credential.
