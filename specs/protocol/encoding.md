# NOVA Canonical Encoding

**Status:** Specification draft — not yet implemented

## 1. Requirement

Consensus-critical objects MUST be serialized into one canonical byte sequence. Equivalent semantic objects MUST NOT have multiple valid encodings.

## 2. Prohibited inputs

The consensus hash function MUST NOT consume JSON, unordered map serialization, debug formatting, platform-native structs, floating-point values, or implementation-defined serialization.

## 3. Required rules

The final encoding specification must define:

- fixed-width integer sizes;
- unsigned integer byte order;
- byte-string length prefixes;
- list length and element ordering;
- enum discriminants;
- optional values;
- fixed-size hash/address encoding;
- maximum object sizes;
- rejection of trailing bytes where applicable;
- domain-separated hash preimages.

## 4. Canonical hash input

For each consensus object, the protocol must define an explicit domain tag and field order before hashing. A hash MUST be computed over canonical bytes only.

## 5. Test-vector requirement

Every consensus-critical encoding must have byte-level reference vectors containing input fields, encoded hexadecimal bytes, and resulting hash where applicable. Vectors are authoritative only after they are generated from the implemented canonical encoder.
