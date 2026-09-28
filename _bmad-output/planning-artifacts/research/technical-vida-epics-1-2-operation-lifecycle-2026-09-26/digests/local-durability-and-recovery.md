# Digest: local durability and recovery

Checked 2026-09-26. Scope: Epic 1 local Persona/Note and Epic 2 Web local storage.

- SQLite's atomic transaction and WAL `synchronous=FULL` offer a candidate crash/power-loss commit boundary; not yet a selected VIDA dependency or measured Android guarantee. [SQLite](https://www.sqlite.org/pragma.html#pragma_synchronous)
- IndexedDB `durability: strict` is a browser API hint, not an immutable backup; origin storage can be cleared and best-effort storage evicted. [MDN transaction](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction), [MDN storage](https://developer.mozilla.org/en-US/docs/Web/API/Storage_API/Storage_quotas_and_eviction_criteria)
- Separate recovery secret, encrypted bundle and encrypted content copy are needed to distinguish authority restoration from data restoration. [OWASP](https://cheatsheetseries.owasp.org/cheatsheets/Key_Management_Cheat_Sheet.html), [Signal](https://support.signal.org/hc/en-us/articles/9708267671322-Signal-Secure-Backups)
- W3C explicitly warns that non-extractable WebCrypto keys do not guarantee underlying hardware protection and same-origin hostile script may use them. [W3C](https://www.w3.org/TR/WebCryptoAPI/)

Inference: `saved locally` is only defensible after one atomic storage commit; browser origin data must be described as local, not off-device backed-up.
