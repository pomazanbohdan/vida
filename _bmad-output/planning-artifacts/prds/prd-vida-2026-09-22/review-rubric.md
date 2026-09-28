# PRD Quality Review — VIDA Release 1

## Overall verdict

PRD має сильну продуктову тезу, чесно окреслений scope, повний словник і послідовну систему UJ/FR/NFR, тому вже придатний як вхід для UX та архітектурного опрацювання. Водночас він ще не є green-light-to-build: ключові контракти синхронізації, числові NFR, release ownership і критерії зміни архітектури лишаються відкритими, а частина FR не має достатньої acceptance-деталізації.

## Decision-readiness — thin

Документ добре відділяє прийняті рішення від невирішених: §5 фіксує non-goals, §9 прямо називає release blockers, а §10 містить справжні відкриті питання. Це значно краще за приховані припущення.

Однак десять Open Questions включають не другорядні уточнення, а основу продукту: finality/receipt, CRDT, E2EE media stack, числові resource budgets, юридичну та security-відповідальність. PRD дозволяє почати UX і capability prototypes, але не дає безпечної підстави для implementation fan-out або публічного release commitment.

### Findings

- **[high]** Phase-blockers не мають disposition (§§9–10) — кожен blocker перелічений, але немає owner, decision artifact, deadline/revisit condition і чіткої межі, до якої роботи можна йти без нього. *Fix:* для OQ-1–OQ-10 додати owner, required evidence, цільовий downstream document і gate (`before architecture freeze`, `before implementation fan-out`, `before store submission`).
- **[high]** Немає затвердженого stop-loss/architecture-change правила (§10, OQ-9) — R-1 визнає надвеликий інтеграційний scope, але невдалий Iroh/CRDT/calls/AppPackage proof не змінює жодного рішення. *Fix:* сформулювати перевірні failure criteria і заздалегідь визначені наслідки: заміна stack, ізоляція capability або блокування release.

## Substance over theater — strong

Vision специфічне для VIDA: typed Resource graph, Persona/Space isolation, рівнозначні Devices, local-first sync і відкриті специфікації не можна без втрати сенсу переставити в типовий collaboration PRD. Journeys безпосередньо ведуть до поведінки продукту, а більшість FR містять конкретні перевірні наслідки. Addendum зберігає технічні механізми поза продуктовою оповіддю й явно маркує deferred decisions.

### Findings

- **[medium]** Частина NFR лишається назвою практики, а не release contract (§7 NFR-1, NFR-2, NFR-6, NFR-11) — “MASVS-aligned”, “окремий review gate”, “підтримують screen readers” і “актуальні store-policy checks” не визначають рівень, coverage або pass condition. *Fix:* назвати нормативні профілі/рівні, обов'язкові артефакти та pass/fail evidence; технічний спосіб лишити в addendum/architecture.

## Strategic coherence — adequate

§1 задає зрозумілу ставку: один пов'язаний local-first контекст для комунікації, знань і проєктної роботи без обов'язкового vendor authority. Messenger, Notes/Knowledge, Project, Files, Relations, Spaces і sync прямо підтримують цю ставку; SM-3 перевіряє саме connected-context journey, а counter-metrics захищають приватність і якість.

Стратегічна логіка слабшає на межі Release 1: повний Marketplace/external repositories, E2EE group calls, AppPackage runtime і 21 locale profile оголошені одночасно обов'язковими, але PRD не пояснює, чому кожен із цих дорогих контурів потрібен для доведення головної тези в першому публічному релізі.

### Findings

- **[high]** Release 1 scope не має внутрішньої пріоритизації (§§6.1, 9 R-1) — усі capabilities мають однаковий статус, хоча частина доводить core thesis, а частина розширює platform/distribution ambition. Це суперечить SM-C3. *Fix:* позначити thesis-critical release gates і окремі launch-completeness gates або додати причинне обґрунтування, чому calls, Marketplace, external repositories і всі locales є невіддільними від Release 1.
- **[medium]** Primary metrics переважно доводять conformance, а не продуктову цінність (§8) — лише SM-3 перевіряє user outcome, і його research method/sample ще відкриті. *Fix:* додати 1–2 privacy-preserving outcome metrics для особистого та shared workflow та зафіксувати sampling/evidence plan.

## Done-ness clarity — thin

Більшість FR має хоча б одну спостережувану поведінку, а §6.2 дає корисний наскрізний сценарій. Особливо сильні FR-7, FR-22–FR-28 і FR-31: вони описують failure behavior, а не тільки happy path.

Проте acceptance coverage нерівномірний. Деякі FR є одним реченням без наслідків; числові performance/reliability bounds відкладені; кілька ключових термінів залежать від ще не визначеного protocol contract.

### Findings

- **[high]** П'ять FR не мають перевірних наслідків (§4: FR-11, FR-15, FR-17, FR-20, FR-33) — story generation муситиме самостійно вигадувати permissions, offline behavior, error states і acceptance. *Fix:* додати мінімум один позитивний та, де релевантно, один failure/permission consequence для кожного.
- **[high]** Performance і operational gates відсутні (§7 NFR-10; §10 OQ-4) — feature-complete gate залежить від чисел, яких PRD ще не має, тому “done” для startup, memory, battery, sync latency, crash/ANR і call quality невизначене. *Fix:* до architecture freeze затвердити baseline devices/networks, measurement method і числові release thresholds; до того позначити PRD explicitly not implementation-ready.
- **[medium]** SM-1 заявляє ширше покриття, ніж його сценарій доводить (§6.2, §8 SM-1) — vertical slice не перевіряє, зокрема, Contacts, Forums, Marketplace/repositories, schema migration, localization, diagnostics і export, але SM-1 каже “Validates FR-1–FR-36”. *Fix:* звузити mapping SM-1 до реально покритих FR або розширити release-gate suite окремими mandatory scenarios.

## Scope honesty — adequate

§5 є змістовним Non-goals section; §9 не маскує інтеграційні та операційні ризики; усі чотири inline `[ASSUMPTION]` мають roundtrip в §11. Документ чесно не обіцяє absolute anonymity, permanent mobile-online, recall експортованих копій або “exactly once”.

Водночас scope формально закритий, а механізм реагування на недоведені ключові можливості лишається Open Question. Це не прихований scope, але поки не чесний executable commitment.

### Findings

- **[medium]** Mandatory scope не має правила перегляду після capability proofs (§6.1; §9 R-1/R-2; §10 OQ-9) — документ одночасно визнає невизначеність stack і вимагає весь scope. *Fix:* задокументувати, які рішення є invariant product promise, а які можуть змінити форму після proof без порушення PRD.

## Downstream usability — adequate

Глосарій широкий і корисний; FR-1–FR-36, UJ-1–UJ-5 та NFR-1–NFR-12 без пропусків; domain nouns здебільшого стабільні. Addendum добре відділяє implementation choices, а Risks/Open Questions дають архітектурі конкретний backlog рішень.

Для UX і high-level architecture документ придатний зараз. Для epics/stories він потребує закриття acceptance gaps і точнішої прив'язки release evidence до вимог.

### Findings

- **[medium]** Traceability до release evidence надто груба (§§6.2, 8) — один vertical slice і широкі SM ranges не дозволяють source-extract, який gate доводить кожен FR/NFR. *Fix:* не створюючи повної matrix, додати до кожного release-gate scenario список FR/NFR IDs або до кожної Feature — короткий “release evidence” блок.
- **[low]** UJ-5 не має названого протагоніста (§2.3) — “Команда” не несе індивідуального контексту, на відміну від Олени й Андрія. *Fix:* переписати як сесію названої людини в команді та зберегти group-call контекст.

## Shape fit — strong

Для consumer/multi-stakeholder chain-top PRD обрана правильна форма: named journeys, glossary, grouped capabilities, globally stable FR IDs, cross-cutting NFRs, explicit non-goals, risks, open questions і assumptions. Документ не перетворений на architecture spec; технічні рішення винесені в addendum. Рівень формальності виправданий тим, що PRD має живити UX, architecture і stories для трьох платформ.

## Mechanical notes

- FR-1–FR-36, UJ-1–UJ-5 і NFR-1–NFR-12 є унікальними та без пропусків.
- Усі чотири inline `[ASSUMPTION]` відображені в §11; зайвих index entries не знайдено.
- UJ-1–UJ-4 мають named protagonists; UJ-5 потребує імені.
- Cross-reference `§6.2 → FR-1–FR-36` у SM-1 семантично завеликий, хоча синтаксично валідний.
- Addendum послідовно використовує ті самі ключові терміни; явного glossary drift, що змінює значення, не знайдено.
- Frontmatter `status: draft` відповідає наявним phase-blockers і не повинен ставати `final` до їх disposition.
