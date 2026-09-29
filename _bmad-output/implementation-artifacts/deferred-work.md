- source_spec: `C:/project/vida/_bmad-output/implementation-artifacts/spec-oq-0022-0024-controller-recovery-proof.md`
  summary: Define how an offline enrollment approval obtains a trustworthy expiry time.
  evidence: A trusted signer controls `approved_at`; a separate time witness would determine whether a post-expiry backdate is possible under the product threat model.
- source_spec: `C:/project/vida/_bmad-output/implementation-artifacts/spec-oq-0022-0024-controller-recovery-proof.md`
  summary: Decide whether a trusted Device's direct signed recovery-key rotation needs an independently verifiable preparation proof.
  evidence: `History::append` accepts a signed Rotate without local export readback; only a separate authority or receipt protocol could prove remote custody.
