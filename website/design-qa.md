# VIDA presentation — design QA

final result: passed

## Target and scope

- Source: `qa/reference.png`, selected option 2; original at `C:/Users/pomaz/.codex/generated_images/01a0da4a-648b-79b2-8a28-2732c9e28c61/exec-0de37be0-4154-4125-b645-d1aa03e6bbf0.png`.
- Implementation: `http://127.0.0.1:4173/`, initial chat state, light theme.
- Desktop CSS viewport: 1440 × 1024; devicePixelRatio approximately 1. The IAB capture is 1425 × 1013 pixels; native capture differs slightly from CSS dimensions.
- Source pixels: 1122 × 1402. Comparison renders both images at the same panel width to normalize their different original widths. Only overlapping hero content is judged in the viewport comparison.
- Evidence: `qa/comparison-final.png` combines source and implementation in the same image. `qa/desktop-hero.png` supplies readable focused evidence for type, controls, chat content and spacing.
- Additional viewport captures: `qa/applications.png`, `qa/architecture.png`, `qa/mobile-hero.png`, `qa/mobile-apps.png`, `qa/mobile-320.png`.
- Mobile CSS viewports tested: 390 × 844 and 320 × 800. No horizontal overflow; section scroll-width check passed at 320.
- `qa/desktop-full.png` is exploratory only: the IAB full-page capture showed stitching artifacts and is excluded from visual acceptance.

## Comparison history

1. Initial combined comparison `qa/comparison-hero.png`: P2 — the demo was too short and internal text was too small compared with the selected image. Increased desktop demo height from 658 to 780px and adjusted message, sidebar, heading and resource typography.
2. Recaptured `qa/desktop-hero.png`, then compared in `qa/comparison-final.png`: hero proportions and content hierarchy corrected. The large headline, neutral canvas, graphite sidebar, terracotta CTA, linked note/task and platform row preserve the selected direction.
3. Narrow-screen safeguards: responsive headline sizing, wrapped menu and shortened architecture rows. Verified at 320px: document scrollWidth equals viewport width; no overflowing main section.

## Required fidelity surfaces

- Typography: locally bundled Manrope provides readable Ukrainian; headline wrapping and hierarchy follow the source. Exact generated-image glyph shapes are not a font contract.
- Spacing/layout: split hero, product panel, personal/shared pairing and technology strip retained. Larger feature, applications, architecture and roadmap sections intentionally complete the original content request and the user's later applications request.
- Colors: neutral white surfaces, graphite typography/sidebar and restrained terracotta actions retained; dark architecture section extends the same family.
- Image quality: both personal/shared illustrations are generated raster assets, loaded successfully, correctly cropped and not stretched. Interface demo is real HTML controls. Icons use the Phosphor library rather than handmade icon drawings.
- Copy: planned feature scope distinguished from shipped capabilities. Projects marked first release; CRM, OSBB and business appointments marked future ecosystem directions. Technical gates are visible without implying implemented or verified guarantees.

## Interaction and runtime checks

- Header anchors, applications anchor and mobile menu verified; choosing a mobile link closes the menu.
- Demo chat message submitted and displayed locally; no network submission or persistence is implied.
- Knowledge and project tabs, linked resources and task completion checkbox verified.
- Rust core and app-package architecture controls update the explanation.
- Contribution CTA points to the actual repository remote; `git ls-remote` resolves its HEAD.
- Two content images loaded successfully.
- Browser console: zero warnings/errors captured in the reviewed session.
- Vite production build passed; Sites worker suite passed 4/4.

## Follow-up polish (P3)

- Generated reference typography and illustration details are approximated rather than pixel-identical.
- Small ornamental leaf flourish in the reference footer is omitted; the CTA retains the terracotta surface, hierarchy and working contribution link.
- Demo is illustrative frontend state, not the VIDA application implementation.
- Mobile adaptation is assessed for usability; the source supplies no separate mobile website frame.

## Implementation checklist

- [x] Selected visual implemented and compared
- [x] Required features and applications represented
- [x] Desktop/mobile interactions checked
- [x] No unresolved P0/P1/P2 findings
- [x] Local preview left running
- [ ] Publish through Sites when requested

## Technical expansion — 2026-09-26

### Ecosystem continuation

- Added a six-stage presentation guide, marketplace distribution, City Portal platform integration, infrastructure and a separate Iroh protocol chapter.
- User correction overrides historical delivery documents: no mailbox/offline postal queue is presented. Local pending changes and transport relays remain distinct.
- Nine diagrams rendered; browser alert count zero. Journey anchors all resolve. New diagrams use the existing SVG renderer and local horizontal scrolling.
- Iroh explanation distinguishes upstream connectivity from VIDA direct-first policy and browser implementation gates; primary upstream references are linked in the section.
- Fixed inherited paragraph styling inside Mermaid labels to prevent clipped multiline text.
- Desktop evidence: `qa/ecosystem-platform.png`, `qa/iroh-final.png`.
- At 320 CSS pixels, document scrollWidth is 320; 390px ecosystem layout also passed. The viewport override was reset and the original preview tab remains open on Iroh.
- Sites worker tests passed 4/4 after adding the ecosystem and Iroh chapters.

- Added six standard capability panels, four business decompositions, five SVG diagrams and separate core/schema/sync/evolution chapters. The selected option 2 visual direction remains unchanged; these sections are an intentional content extension.
- Browser rendered all five diagrams without alert fallbacks or console errors. Calendar selection and keyboard activation of CRM, OSBB, business booking and sync confirmation step were verified through resulting DOM state.
- IAB pointer actions sometimes scrolled to controls without activating them; keyboard activation verified the actual handlers. Pointer automation is not counted as successful interaction evidence for those controls.
- At 390 and 320 CSS pixels, no page-level horizontal overflow. Wide diagrams and tables use local scrolling. Diagrams retain native text scale instead of shrinking to unreadable mobile labels.
- P2 corrected: hiding heading line breaks joined adjacent words on mobile; explicit spaces now preserve word boundaries. Oversized sequence rendering and shrunken flow labels corrected by preserving SVG natural widths.
- Evidence: `qa/technical-business.png`, `qa/technical-mobile-320.png`, `qa/technical-schema-final.png`. Earlier `technical-sync.png` and `technical-mobile.png` are intermediate captures, excluded from final visual acceptance.
- Source links expose local document snapshots; schema examples and unresolved protocol decisions are explicitly labeled.
