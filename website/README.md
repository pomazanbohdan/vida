# VIDA presentation website

Ukrainian presentation of VIDA for future users and contributors. Implements selected visual option 2, with an interactive frontend concept and source-grounded feature, application and architecture sections.

## Run

```powershell
npm install
npm run dev -- --host 127.0.0.1 --port 4173 --strictPort
npm run build
npm run test:sites
```

The Product Design starter emits `dist/client`, `dist/server/index.js` and `dist/.openai/hosting.json`. Hosting runtime files are preserved. No backend, collection of personal data or production messaging is included. Demo state resets on reload.

## Content sources

- `../docs/01-product/product-boundary.md`
- `../docs/01-product/v1-replacement-bundle.md`
- `../docs/01-product/composable-workspace-model.md`
- `../_bmad-output/planning-artifacts/architecture/architecture-vida-2026-09-19/ARCHITECTURE-SPINE.md`
- `../_bmad-output/planning-artifacts/ux-designs/ux-vida-2026-09-22/DESIGN.md`
- `../research/Новий Text Document (3).txt`, line 346: OSBB application scenario with requests, assignees, approvals and forum.
- User requested Projects, CRM, OSBB and business appointment applications in the presentation. Future modules do not imply first-release inclusion.

## Assets and visual provenance

`public/assets/personal-space.png` and `shared-space.png` were generated with built-in ImageGen from the selected mock. Briefs: neutral editorial illustrations of one person at a laptop and three colleagues collaborating, graphite and restrained terracotta palette, no text or logos. No CLI image API used.

Icons: [Phosphor React](https://github.com/phosphor-icons/react). Font: locally packaged Manrope via `@fontsource/manrope`. See dependency packages for licenses.

QA evidence and comparison are in `qa/`; these are not copied into the production client. See `design-qa.md` for scope and checks. Only `public/` assets are shipped.

## Technical presentation

`src/Ecosystem.jsx` connects the narrative from everyday capabilities through business composition, marketplace distribution, external platforms and infrastructure. `src/IrohDetails.jsx` separately explains endpoint identity, lookup, NAT traversal, encryption, ALPN and route changes. The user's correction excludes mailbox/offline postal-queue functionality from presentation content; historic reference documents retain their original wording. New source snapshots cover package distribution, product boundaries, Hosted Space and ADR-0005. Primary Iroh links appear alongside the explanation.

`src/TechnicalDetails.jsx` adds six standard capability descriptions, four business application breakdowns, shared-core responsibilities, schema-driven execution, synchronization evidence and schema evolution. Five Mermaid diagrams render as SVG with textual source and horizontal scrolling at their readable native size.

Content snapshots in `public/reference/` come from the v1 bundle, AppPackage runtime baseline, transport architecture, device sync session, operation finality and schema evolution contracts. They preserve the source status (including review/draft boundaries). CRM and booking resource examples are explanatory proposals, not finalized package schemas. Refresh these snapshots when the underlying contracts change.
