# Reviewer gate — approval policy r5

Verdict: PASS with one configuration-scope question.

- AD-31 no longer encodes a minimum approval level: package/schema supplies an optimal default and Space Admin selects the effective Core profile.
- AD-34 closes v1 profiles to single approver, sequential stages and threshold M-of-N; all-of is representable as M=N.
- Approvals bind to a revision/frontier, so editing protected data invalidates their applicability without deleting audit history.
- Admin configuration cannot inject resolver code, alter governance or disable exclusive-claim serialization.
- Auto acceptance removes a human step, not the authoritative decision or signed outcome receipt.
- Remaining question is configuration granularity: one mode per AppInstance or separate mode per operation/resource family.

