# Digital Signature Protocol

## Purpose
Provide a secure, auditable, and interoperable digital-signature protocol for signing and verifying documents and transactions in this project.

## Scope
Applies to all components requiring non-repudiable signatures: backend services, contracts, and legal artifacts.

## Actors
- Signer: entity (human or service) that creates signatures.
- Verifier: entity that checks signatures before accepting data.
- Key Manager: system/service that generates, stores, rotates, and revokes keys.

## Key Types and Algorithms
- Primary: ECDSA over P-256 with SHA-256 (recommended).
- Secondary (optional): RSA-PSS with SHA-256 for legacy interoperability.

## Key Management
- Keys MUST be generated in a hardware-backed keystore (HSM or KMS) where available.
- Private keys MUST never leave the KMS/HSM in plaintext.
- Key IDs (`kid`) MUST be used to reference keys in signatures.
- Rotate keys on a regular schedule and publish new public keys before retiring old ones.

## Signing Flow
1. Prepare canonical payload (use deterministic JSON canonicalization or application-specific canonical form).
2. Compute digest with SHA-256.
3. Request signature from Key Manager using the key ID and digest.
4. Attach signature, signer ID, `kid`, algorithm, and timestamp to the signed artifact.

## Verification Flow
1. Extract signature metadata (`kid`, algorithm, timestamp) and signed payload.
2. Retrieve public key for `kid` from trusted key registry.
3. Verify signature using the declared algorithm and SHA-256 digest.
4. Check timestamp and revocation status of `kid`.

## Signature Format
- Use JWS (JSON Web Signature) compact or JSON serialization for API flows.
- Store or transmit signatures together with metadata: `{signature, kid, alg, signer, timestamp}`.

## Timestamping and Non-Repudiation
- Include an ISO 8601 UTC timestamp in each signature.
- Optionally anchor critical documents to a trusted timestamping service for stronger non-repudiation.

## Revocation and Audit
- Maintain a revocation list or use OCSP for real-time status checks.
- Log signing and verification events with immutable audit records.

## Security Considerations
- Enforce least-privilege access to signing endpoints.
- Protect signing endpoints with strong auth and rate limits.
- Monitor for anomalous signing activity.

## Example (JSON)
{
  "payload": {...},
  "signature": "<base64url>",
  "kid": "key-2026-02",
  "alg": "ES256",
  "signer": "service-A",
  "timestamp": "2026-02-28T12:00:00Z"
}
