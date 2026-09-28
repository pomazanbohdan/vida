# Reviewer gate — per-process approval r6

Verdict: PASS.

- Approval policy is scoped to a named process/operation family inside an AppInstance, not globally to the whole AppInstance.
- One AppInstance can therefore use auto approval for booking, sequential approval for permits and M-of-N for committee decisions.
- Package/schema still supplies the optimal default for each process; Space Admin may override it without changing package code.
- Per-process configuration cannot replace Core resolver, revision binding, governance or exclusive-claim serialization.
- OQ-0071 is fully resolved; exact manifest/configuration field names remain implementation detail.

