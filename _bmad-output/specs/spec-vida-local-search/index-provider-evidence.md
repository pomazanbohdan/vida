# Local index provider evidence and prototype gates

This is technical evidence, not a choice of SQLite FTS5 or any other search provider. Evaluate candidates against `SEARCH-F01–F11` on Android, iOS and Windows with the same authorized corpus.

| Risk | Primary-source observation | VIDA prototype proof |
|---|---|---|
| Deleted tokens and snippets | [SQLite FTS5 secure-delete](https://www.sqlite.org/fts5.html#the_secure_delete_configuration_option) documents that default updates/deletes can leave old full-text entries recoverable until merge; FTS and SQLite core secure-delete address different attackers. [OWASP MASVS-STORAGE-1](https://mas.owasp.org/MASVS/controls/MASVS-STORAGE-1/) requires secure storage of sensitive data. | Treat index postings, snippets, query suggestions, caches and temporary files as sensitive. Prove UI/API read denial at revocation immediately; inspect local-at-rest artifacts and backup/restore behavior separately. FTS options alone do not prove VIDA ACL revocation. |
| Crash/rebuild drift | [SQLite FTS5 external-content pitfalls](https://www.sqlite.org/fts5.html#external_content_table_pitfalls) say triggers do not backfill old rows; `rebuild` is available for applicable table modes, not contentless tables. | Compare committed canonical Resource set with index after crash, retry, migration and rebuild. Never make the index authoritative or expose an uncommitted projection. |
| Multilingual matching | [SQLite FTS5 tokenizer documentation](https://www.sqlite.org/fts5.html#tokenizers) describes `unicode61` and trigram behavior; a built-in tokenizer name does not prove correct Arabic/CJK search. | Fixture Ukrainian/Latin, Arabic with marks and RTL, Japanese/Korean/Chinese, mixed scripts, punctuation and short terms; check result navigation and accessible highlighting. |
| Backup restoration | [Android Auto Backup](https://developer.android.com/identity/data/autobackup) includes database directories among configurable backup domains. A restored local index may be older than current grants or keys. | Before exposing restored results, revalidate account/keys/grants and rebuild or purge affected derived index segments; test device transfer and backup exclusions for the chosen profile. |

## Selection boundary

- Candidate comparison records storage/key boundary, revocation-to-no-read behavior, restore/rebuild correctness, locale corpus results, index/update resource cost and release/dependency status. Numeric budgets come from the separate platform baseline gate, not this document.
- Indexing File bytes, OCR, attachment previews and ContactCard field depth remain explicit scope questions. A Relation or attachment never grants target read; the search provider cannot weaken that rule.
- A provider passes only when the same negative privacy and authorization fixtures pass on all Release-1 clients; documentation and synthetic benchmark speed alone are insufficient.
