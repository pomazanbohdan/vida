---
name: VIDA
description: Visual identity for the VIDA local-first super-app.
status: discovery
updated: 2026-09-25
sources:
  - ../../prds/prd-vida-2026-09-22/prd.md
  - ../../prfaq-vida.md
  - ../../../../docs/01-product/v1-replacement-bundle.md
colors: {}
typography: {}
rounded: {}
spacing: {}
components: {}
---

# VIDA visual identity — approved concept, detailed design pending

This document owns how VIDA looks. [EXPERIENCE.md](EXPERIENCE.md) owns screens, states and behavior. The user selected visual direction A, graphite with restrained terracotta, as the common concept for all Core surfaces and Apps in light and dark mode. Literal brand colors, type family, radii and spacing values remain implementation decisions; empty token maps above prevent an exploratory HTML example from becoming a design contract.

## Brand & Style

**[APPROVED — product-wide visual character.]** VIDA uses the selected [A — graphite and terracotta concept](.working/visual-theme-comparison-2026-09-25.html): calm, neutral, content-first surfaces with a restrained warm accent in both light and dark mode. Carry this one visual language across Messenger, Forum, Notes/Knowledge, Project, Files, Contacts, Calendar, settings and future Apps; individual Apps express their content and workflows, not an unrelated brand skin. The selected direction supports quiet personal notes and dense team projects without becoming either a toy-like messenger or an enterprise dashboard. This approves the concept, not the HTML example's literal hex values, font, icon set, corner radii or component dimensions.

**[APPROVED — mobile group-chat density only]** Prefer the more compact, content-first conversation treatment of [.working/chat-skill-v3.png](.working/chat-skill-v3.png) to the larger v2 mock. This selects relative information density and message hierarchy, not the example's violet palette, font, icon set or corner radii. The product-wide style is the later selected A direction above.

**[APPROVED — mobile composition]** The user selected [mobile chat direction 1](.working/mobile-chat-selected-2026-09-22.png) and approved the relative information density and block order of its [Knowledge](.working/mobile-knowledge-concept-2026-09-22.png) and [Project](.working/mobile-project-concept-2026-09-22.png) extensions. Carry forward their quiet content-first hierarchy and compact but readable grouping across the other Apps. Their blue palette is superseded by visual direction A; font, icon set, radius, exact spacing tokens and per-screen layout remain open.

## Colors

**[APPROVED — family and mode behavior.]** Use the graphite/terracotta family from candidate A across light and dark themes. Follow the operating-system light/dark setting by default and allow a manual theme override in settings. The side-by-side example establishes the intended relationship of neutral canvas, readable graphite ink and sparse terracotta emphasis, not final numeric colors. **[DEFERRED TO IMPLEMENTATION — tokens and proof.]** Validate literal palettes and contrast per platform and screen; define semantic status colors separately. A status remains distinguishable through text/icon/position when color is unavailable. Persona and Space cues must never suggest that separate Personas are one identity.

## Typography

**[DEFERRED TO IMPLEMENTATION]** Font families, scale, density and multilingual typography for the approved locale set. Rich-text content and UI chrome need clearly differentiated roles without hiding long names or critical status labels.

## Layout & Spacing

**[APPROVED — adaptive composition.]** Design compact mobile, medium/tablet and wide/desktop presentations with the same Space/resource semantics. Mobile shows one main pane; medium shows list and detail; a wide window may add a third contextual pane only while the main content stays usable. Resizing preserves identity, selected Space/resource and navigation return path. **[OPEN — measured tokens.]** Exact spacing scale, breakpoint values, pane minimums and density require platform prototypes. The layout keeps the active Persona, owning Space and current resource legible during cross-resource navigation.

In mobile chat, group messages and related-resource previews should be concise enough to preserve conversation context while retaining readable text and platform-sized touch targets. Keep the composer and main destinations visible; do not achieve compactness by shrinking actionable controls or hiding delivery labels. Exact spacing tokens remain open.

For the approved mobile Knowledge composition, place document identity and actions before the body, then linked task/context and discussion. For Project, place task identity and status before the description, subtasks, linked resources and discussion/action. These are relative hierarchies, not fixed component sizes or interaction specifications.

## Elevation & Depth

**[APPROVED — conflict hierarchy.]** A conflict starts as an inline resource indicator and opens a dedicated comparison surface; it must not block the entire application with a modal. **[OPEN — layer tokens.]** Elevation for menus, share preview, calls and transient notifications still needs review. Security-critical confirmation remains visually distinct from passive status.

## Shapes

**[OPEN]** Corner, border and iconography language. Shape must not be the sole indicator of access, sync or conflict state.

## Components

The initial component inventory is the Persona switcher, Space context header and picker, mobile primary bar/“Ще” entry, resource card/header, relation chip, conversation row, forum topic row, note editor, task row/board card, File card, Calendar event row/invitation preview, share preview, inline conflict indicator and dedicated comparison surface, status indicator, call controls and recovery prompt. Their interaction belongs to EXPERIENCE.md; visual specifications and tokens remain open.

For the group-chat concept, use distinct incoming/outgoing message structure and a compact, recognizable linked-resource preview. The accepted density is illustrated in [.working/chat-skill-comparison-2026-09-22.png](.working/chat-skill-comparison-2026-09-22.png); screenshots do not define final component tokens.

The current three-screen visual set and its generation brief are collected in [.working/mobile-direction-selected-2026-09-22.md](.working/mobile-direction-selected-2026-09-22.md). Keep document prose less dense than chat rows; keep task metadata and linked resources scan-friendly without turning every row into a card.

## Do's and Don'ts

- Do make sync, pending, conflict, private and shared states readable without color alone.
- Do support touch, keyboard, screen readers, dynamic type and translated text.
- Do not style “Збережено локально” so it can be mistaken for “Синхронізовано” or “Доставлено”.
- Do not let decorative density obscure the active Persona, Space, access scope or a destructive action.
- Do respect the system theme unless the person selects a manual override; do not force a brand-owned light or dark mode.
- Do let an inline conflict indicator lead to a dedicated comparison surface; do not trap routine work behind a global conflict modal.
