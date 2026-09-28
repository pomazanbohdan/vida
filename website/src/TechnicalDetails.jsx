import { useEffect, useId, useState } from 'react';
import { ArrowRight, ArrowUpRight, CaretDown, Code, Stack, Check, FileText } from '@phosphor-icons/react';
import './technical.css';

let mermaidReady;
function engine() {
  mermaidReady ??= import('mermaid').then(({ default: mermaid }) => {
    mermaid.initialize({ startOnLoad:false, securityLevel:'strict', theme:'base', fontFamily:'Manrope, Arial, sans-serif', themeVariables:{primaryColor:'#f3ebe5',primaryTextColor:'#19202e',primaryBorderColor:'#c0937c',lineColor:'#9b6850',secondaryColor:'#f6f4f1',tertiaryColor:'#fff',fontSize:'16px'}, flowchart:{htmlLabels:false,useMaxWidth:true,curve:'linear'}, sequence:{useMaxWidth:true,wrap:true,actorMargin:45,messageMargin:35} });
    return mermaid;
  });
  return mermaidReady;
}
export function Diagram({ source, label }) {
  const id=useId().replace(/[^a-zA-Z0-9]/g,'');
  const [svg,setSvg]=useState(''),[error,setError]=useState(false);
  useEffect(()=>{let active=true; setSvg('');setError(false); engine().then(m=>m.render(`vida${id}`,source)).then(r=>{if(active)setSvg(r.svg.replace(/max-width: ([\d.]+)px;/,'width: $1px;'))}).catch(()=>{if(active)setError(true)});return()=>{active=false}},[source,id]);
  return <figure className="technical-diagram"><figcaption>{label}</figcaption>{error?<p role="alert">Не вдалося відобразити схему. Її текст доступний нижче.</p>:svg?<div className="diagram-scroll" role="img" aria-label={label} dangerouslySetInnerHTML={{__html:svg}}/>:<p className="diagram-loading" role="status">Готуємо схему…</p>}<details className="diagram-source"><summary>Текстова версія схеми</summary><pre>{source}</pre></details></figure>
}

const standardApps=[
 {name:'Спілкування',type:'Стандартний пакет · Messenger',value:'Особисті розмови, командний чат і довготривалі обговорення в одному контексті.',resources:'Повідомлення, розмова, тема форуму, вкладення та пов’язані ресурси.',behavior:'Чат зберігає швидкий потік повідомлень. Форум групує обговорення за темами. Повідомлення може посилатися на нотатку, задачу або файл.',technical:'Підписані зміни проходять спільний SyncLog. Доставка адресату та прочитання мають окремі підтвердження. Великі вкладення передає BlobStore.',reuse:'CRM отримує контекст розмови з клієнтом, ОСББ — повідомлення й форум будинку, проєкт — командні обговорення.',boundary:'1:1 та групові аудіо-/відеодзвінки заплановано до 8 учасників. Media/E2EE-профіль ще потребує технічної перевірки.'},
 {name:'Знання',type:'Стандартний пакет · Notes / Knowledge',value:'Персональні та спільні документи, які не втрачають зв’язок із роботою.',resources:'Нотатка, розділ, документ, вкладення, relation і зворотне посилання.',behavior:'Rich text, історія змін та спільне редагування. У відкритій нотатці видно курсори інших учасників; синхронізовані зміни підсвічуються.',technical:'Зміни документа відокремлені від тимчасових cursor/presence-сигналів. Вибір конкретного CRDT та редактора ще відкритий.',reuse:'У проєкті — рішення й технічна документація; у CRM — матеріали про клієнта; в ОСББ — документи й інструкції.',boundary:'Повні табличні бази, формули та складні Notion-подібні database views не входять до першого релізу.'},
 {name:'Проєкти',type:'Стандартний пакет · Projects / Tasks',value:'Задачі, люди та матеріали в одному робочому середовищі.',resources:'Проєкт, задача, підзадача, виконавець, строк, статус та зв’язки з іншими ресурсами.',behavior:'Список і дошка, пов’язані нотатки, чати, теми форуму та файли. Особистий проєкт живе в Personal Space; командний — у явно вибраному Shared Space.',technical:'Зміна статусу є доменною операцією з перевіркою права й причинної основи. Несумісні непорівнювані прийняті зміни не приховуються автоматичним вибором переможця.',reuse:'CRM використовує задачі для подальших контактів; ОСББ — для робіт та відповідальних; бізнес — для супроводу запиту.',boundary:'Gantt, облік часу, бюджетування та складна аналітика не входять до першого релізу.'},
 {name:'Календар',type:'Стандартна можливість Core',value:'Особисті та спільні події без прив’язки до встановлення Project App.',resources:'Подія, часовий пояс, повторення, нагадування, запрошення та RSVP.',behavior:'Одноразові події, прості щоденні/щотижневі повторення та запрошення наявних VIDA Personas. Календар можна приховати в меню, зберігши саму можливість.',technical:'Події належать Personal або Shared Space. Доступ і доменні зміни проходять спільні контракти ядра; нагадування залежать від можливостей платформи.',reuse:'Проєкт пов’язує роботу з подією. Застосунок запису може використовувати календарний контекст через дозволений контракт.',boundary:'Зовнішня синхронізація календарів і обмін .ics відкладені. Подія в календарі сама по собі не доводить підтвердження бізнесом запису на прийом.'},
 {name:'Контакти',type:'Стандартний сервіс Core',value:'Контактна модель для спілкування, співпраці та бізнес-застосунків.',resources:'Канонічна Contact Card, персональні контактні відомості та зв’язки з ресурсами.',behavior:'Ручне створення картки; імпорт системних контактів до Personal Space лише за явним вибором користувача.',technical:'Картка контакту не є автоматичним grant. Зв’язок із людиною не відкриває її приватний простір і не надає прав бізнес-застосунку.',reuse:'CRM може посилатися на контакт через дозволене представлення, а власні поля клієнта описувати окремою схемою.',boundary:'Core не виконує deduplication контактів; експорт і двостороння provider sync відкладені.'},
 {name:'Файли',type:'Стандартна можливість Core / Space',value:'Один файл може бути пов’язаний із кількома повідомленнями, нотатками та задачами.',resources:'Resource ID, версії, права, маніфест і блоки вмісту.',behavior:'Окремий Files view у Space. Повторне посилання на файл не вимагає дублювати його payload.',technical:'SyncLog передає зміни та посилання; BlobStore окремо отримує перевірені encrypted chunks, відновлює перервані завантаження та керує утриманням локальних копій.',reuse:'Документи CRM, матеріали проєкту, вкладення до заявки ОСББ та файли бізнес-запиту використовують спільне сховище.',boundary:'Довільна синхронізація папок як у OneDrive не входить до першого релізу. Точний криптографічний формат blob ще потребує специфікації.'}
];

export function StandardApps(){const [selected,setSelected]=useState(0);const a=standardApps[selected];return <section className="standard-apps wrap section" id="standard-apps"><div className="section-heading"><div><p className="eyebrow">СТАНДАРТНИЙ НАБІР</p><h2>Можливості, з яких<br/> складається VIDA.</h2></div><p>Три базові пакети та спільні сервіси Core.<br/> Одна модель ресурсів, прав і синхронізації.</p></div><div className="detail-selector" aria-label="Стандартні застосунки">{standardApps.map((x,i)=><button key={x.name} aria-pressed={i===selected} onClick={()=>setSelected(i)}>{x.name}</button>)}</div><article className="standard-detail" aria-live="polite"><div className="standard-intro"><span className="technical-label">{a.type}</span><h3>{a.name}</h3><p>{a.value}</p><p className="contract-note">Messenger, Notes і Projects постачаються разом із VIDA; їх видимість та активацію в Personal Space обирає користувач. Календар, контакти й файли — спільні можливості, а не три додаткові optional-пакети.</p></div><dl className="technical-facts"><div><dt>Дані</dt><dd>{a.resources}</dd></div><div><dt>Поведінка</dt><dd>{a.behavior}</dd></div><div><dt>Реалізація</dt><dd>{a.technical}</dd></div><div><dt>Повторне використання</dt><dd>{a.reuse}</dd></div><div><dt>Межа першого релізу</dt><dd>{a.boundary}</dd></div></dl></article><Source file="v1-bundle.md" label="Погоджений набір першого релізу"/></section>}

const businessApps=[
 {name:'Проєкти',base:'Messenger + Notes + Projects; Files і Calendar через Core.',special:'Власна структура проєкту, статуси, зв’язки задач із матеріалами та обговореннями.',entities:['Project — робочий контекст','Task — статус, виконавець, строк','Relation — зв’язок із нотаткою, темою чи файлом'],flow:'Створити задачу → перевірити доступ → зберегти операцію → оновити список/дошку → синхронізувати.',policy:'Особистий проєкт у Personal Space; для командної роботи — окремий Shared Space. Статуси перевіряються за правилами ресурсу.',sync:'Незалежні зміни поєднуються за контрактом типу. Несумісні непорівнювані прийняті статуси зберігаються як явний конфлікт.',gate:'Стандартний пакет першого релізу. Точний формат схем, алгоритми merge та протокол приймання ще проходять реалізаційні gates.'},
 {name:'CRM',base:'Контакти Core + Messenger + Notes + Projects; Files та Calendar за дозволеними контрактами.',special:'Клієнтські поля, етапи взаємодії та власні правила роботи бізнесу поверх базових інструментів.',entities:['Client — посилання на Contact Card і бізнес-поля','Interaction — контекст взаємодії','Follow-up — зв’язок із задачею або подією'],flow:'Відкрити клієнта → перейти до дозволеної розмови й матеріалів → створити задачу наступного контакту.',policy:'Бізнесова картка та особистий контакт не стають однією загальнодоступною базою. Кожен cross-app запит перевіряється від імені чинного актора.',sync:'Передаються дозволені операції бізнес-простору. Контактні зв’язки не відкривають приватний вміст іншого Space.',gate:'Напрям розвитку. Наведені сутності — пояснювальна декомпозиція, не затверджена CRM-специфікація чи готовий пакет.'},
 {name:'ОСББ',base:'Messenger і форум + Notes + Projects; Files та Contacts через Core.',special:'Заявки мешканців, категорії робіт, виконавці та погодження — власний домен будинку.',entities:['Request — заявка, категорія та стан','Assignment — відповідальний / пов’язана задача','Approval — рішення уповноваженого учасника'],flow:'Мешканець створює заявку → уповноважений учасник класифікує → призначає виконавця → погоджує результат.',policy:'Права мешканця, виконавця та керівника мають задаватися policy конкретного Space. Назви бізнес-ролей не надають Core-повноважень самі по собі.',sync:'Локальна заявка може чекати на прийняття. Реплікація не є погодженням роботи; outcome прив’язаний до конкретного запиту й ревізії.',gate:'Сценарій заявок, виконавців, погоджень і форуму є в обговореннях. Повний пакет, фінансові функції та рольова матриця ще не специфіковані.'},
 {name:'Запис до бізнесу',base:'Calendar + Contacts Core, Messenger, Notes і, за потреби, Projects.',special:'Послуги, доступність, заявка на час і рішення бізнесу — поверх стандартної комунікації й календаря.',entities:['Service — пропозиція бізнесу','AppointmentRequest — запит і бажаний час','Outcome — прийнято / відхилено / строк минув'],flow:'Користувач обирає послугу й час → створює запит → бізнес перевіряє доступність → видає підтвердження або відмову.',policy:'Власник ресурсу чи названий бізнес-процес підтверджує слот. Телефон, relay або перша репліка не можуть самі підтвердити бронювання.',sync:'Збережено локально ≠ доставлено бізнесу ≠ запис підтверджено. За відсутності потрібної authority запит залишається pending.',gate:'Майбутній напрям. Джерело істини доступності, інтеграція з City Portal/зовнішнім сервісом і правила бронювання ще відкриті.'}
];

const composition=`flowchart TB
 subgraph BASE[Стандартна основа]
  M[Messenger і форум]
  N[Notes і Knowledge]
  P[Projects і Tasks]
  C[Calendar, Contacts, Files]
 end
 subgraph CUSTOM[Домен конкретного бізнесу]
  S[Власні типи ресурсів і поля]
  W[Команди, форми та workflows]
 end
 BASE --> D[Версійні залежності та дозволені API]
 CUSTOM --> PKG[AppPackage]
 D --> PKG
 PKG --> I[AppInstance у вибраному Space]
 I --> CORE[Спільний Core: права, валідація, операції]
 CORE --> SYNC[Спільні SyncLog, Delivery і BlobStore]`;

export function AppComposition(){const [selected,setSelected]=useState(0);const a=businessApps[selected];return <section className="composition wrap section" id="composition"><div className="section-heading"><div><p className="eyebrow">ЯК СКЛАДАЮТЬСЯ ЗАСТОСУНКИ</p><h2>Стандартне + спеціалізоване<br/> + власне.</h2></div><p>Бізнес-застосунок описує свій домен<br/> і використовує готові можливості VIDA.</p></div><p className="technical-lead">CRM не повинна заново створювати чат, редактор чи синхронізацію. Вона підключає дозволені можливості стандартних пакетів, додає власні ресурси, поля й процеси та працює в межах спільного ядра.</p><Diagram source={composition} label="Схема 1. Композиція бізнес-застосунку"/><div className="composition-rules"><p><strong>AppPackage — опис.</strong> Версійні схеми, залежності, команди, workflows, UI та permission declarations.</p><p><strong>AppInstance — конкретна активація.</strong> Власні налаштування й ресурси у Space. Завантажити пакет і активувати його — різні дії.</p><p><strong>Залежність — обмежений контракт.</strong> Пакет отримує конкретні operations або data views, а не необмежений доступ до іншого застосунку.</p></div><div className="detail-selector" aria-label="Технічні деталі застосунків">{businessApps.map((x,i)=><button key={x.name} aria-pressed={i===selected} onClick={()=>setSelected(i)}>{x.name}</button>)}</div><article className="business-detail" aria-live="polite"><div className="business-title"><span className="technical-label">ТЕХНІЧНА ДЕКОМПОЗИЦІЯ</span><h3>{a.name}</h3></div><div className="business-columns"><dl className="technical-facts"><div><dt>Стандартна основа</dt><dd>{a.base}</dd></div><div><dt>Спеціалізована частина</dt><dd>{a.special}</dd></div><div><dt>Приклад процесу</dt><dd>{a.flow}</dd></div></dl><div><h4>Ресурси та зв’язки</h4><ul className="entity-list">{a.entities.map(e=><li key={e}><Code size={18}/>{e}</li>)}</ul><dl className="technical-facts"><div><dt>Права та прийняття</dt><dd>{a.policy}</dd></div><div><dt>Синхронізація</dt><dd>{a.sync}</dd></div></dl></div></div><p className="contract-note">{a.gate}</p></article><p className="scope-note">Один пакет може мати різні instances у різних Spaces. Складний застосунок може організовувати окремий Space зі стандартними залежностями; точна топологія та grants мають бути явними. Автор пакета не стає власником даних користувачів.</p><Source file="app-runtime.md" label="Контракт пакета та runtime"/></section>}

const coreGraph=`flowchart TB
 UI[Flutter: Android, iOS, Windows, Web] --> SDK[vida-sdk / binding facade]
 SDK --> RUNTIME[vida-runtime: інфраструктура й життєвий цикл]
 RUNTIME --> CORE[vida-core: правила продукту]
 CORE --> CONTRACTS[vida-contracts: ID, DTO, помилки, ports]
 RUNTIME --> CONTRACTS
 RUNTIME --> HOST[VidaNodeHost / Iroh]
 RUNTIME --> DEL[Передавання та повтори]
 RUNTIME --> LOG[SyncLog]
 RUNTIME --> BLOB[BlobStore]
 CORE --> MEMBER[SpaceMembership / policy]
 CONF[Conformance: специфікації та перевірні сценарії] -.-> CORE
 CONF -.-> RUNTIME
 CONF -.-> UI`;
const schemaGraph=`flowchart LR
 PKG[Версійний AppPackage] --> S[Схеми ресурсів і relations]
 PKG --> W[Команди та workflows]
 PKG --> V[Форми й представлення]
 S --> I[AppInstance у Space]
 W --> I
 V --> I
 I --> CMD[Намір користувача]
 CMD --> VALID[Core: schema + права + передумови]
 VALID --> OP[Підписана Operation]
 OP --> STORE[Атомарно: журнал + outbox]
 STORE --> VIEW[Локальна проєкція та UI]
 STORE --> SYNC[Синхронізація]`;
const syncGraph=`sequenceDiagram
 participant A as Пристрій A
 participant B as Пристрій B
 A->>A: Перевірка команди, підпис, local commit
 Note over A: Збережено локально
 A->>B: Iroh: direct-first / encrypted relay fallback
 A->>B: Device binding, grants, key epoch
 B-->>A: Перевірений control state та scope
 A->>B: Causal summary / frontier
 B-->>A: Відсутні operations і залежності
 A->>B: Підписані operations
 Note over A,B: Blob manifests і chunks — окремий потік
 B->>B: Підпис, схема, права, dependencies
 B->>B: Ідемпотентний запис / apply / projection
 B-->>A: Окремі receipts і frontiers
 Note over A,B: Authority outcome визначає policy ресурсу`;
const evolutionGraph=`flowchart LR
 P[Нова версія пакета] --> T[Перевірка походження і capabilities]
 T --> F[Migration preflight на копії]
 F --> G[Дозвіл активації за policy Space]
 G --> A[Атомарна активація schema + converters]
 A --> W[Нові writes у current schema]
 OLD[Старі дані з writer schema] --> C[Versioned converter]
 C --> READ[Поточне представлення без переписування історії]
 READ --> EDIT[Edit: defaults або missing-required]
 EDIT --> W`;

const schemaExample=`# Пояснювальний ескіз, НЕ формат маніфесту VIDA
AppPackage: ОСББ
  dependencies:
    Messenger: дозволена розмова та тема форуму
    Notes: дозволені документи
    Projects: створення і читання пов’язаної задачі
  resource schema: Request
    stable schema identity + version
    fields: category, description, state
    relations: reporter, assignee, task, attachments
  commands: createRequest, assign, submitForApproval
  workflow: подано → в роботі → очікує погодження
  UI: форма заявки, список, деталі
  permissions: операції від імені актора за policy Space

AppInstance:
  конкретний Space + instance identity + configuration
  активна schema version + дозволені dependency bindings`;

const syncSteps=[
 ['Локальна дія','Core перевіряє команду, схему й доступний контрольний стан. Stable OperationId, causal base, Space і підпис пов’язують зміну з автором. Атомарний запис журналу та outbox дає статус «збережено локально».'],
 ['Канал і доступ','VidaNodeHost встановлює канал. Device binding, членство, grants/revocations і key epoch перевіряються до захищених даних. Успішний connect не надає прав читати Space.'],
 ['План обміну','Сторони обмінюються summaries/frontiers лише дозволених scope. Визначають відсутні operations та причинні залежності, замість пересилання всієї бази щоразу.'],
  ['Передача','Operations і великі файли передаються окремо. Підписи, схема та цілісність перевіряються; перервані передачі можуть продовжитися. Relay пересилає зашифрований трафік між підключеними пристроями.'],
 ['Приймання','Повторний OperationId не застосовується двічі. Брак causal dependencies або доказів чинних прав залишає candidate pending. Отримана операція не запускає заново вихідну бізнес-команду.'],
 ['Проєкція й результат','Після перевірки оновлюється похідне представлення. Для операцій з окремим погодженням потрібен outcome названої authority. Список, пошуковий індекс і лічильники — відновлювані проєкції.'],
 ['Підтвердження','Локальний запис, реплікація, доставка, доменне рішення та зовнішній ефект підтверджуються окремо. Повторне з’єднання відновлює обмін із frontiers, не гублячи pending намірів.']
];

function Source({file,label}){return <a className="technical-source" href={'/reference/'+file} target="_blank" rel="noreferrer"><FileText size={16}/>{label}<ArrowUpRight size={15}/></a>}
function TechTable({headers,rows}){return <div className="technical-table-scroll"><table className="technical-table"><thead><tr>{headers.map(h=><th key={h} scope="col">{h}</th>)}</tr></thead><tbody>{rows.map((r,i)=><tr key={i}>{r.map((c,j)=><td key={j}>{c}</td>)}</tr>)}</tbody></table></div>}

export function ImplementationDetails(){const [step,setStep]=useState(0);return <section className="implementation section" id="implementation"><div className="wrap"><div className="section-heading"><div><p className="eyebrow">ТЕХНІЧНИЙ РОЗБІР</p><h2>Від схеми застосунку<br/> до узгоджених даних.</h2></div><p>Схеми нижче пояснюють цільову реалізацію.<br/> Вони не означають, що runtime уже реалізовано.</p></div><nav className="technical-toc" aria-label="Технічні розділи"><a href="#core-details">01 · Спільне ядро</a><a href="#schema-details">02 · Схеми даних</a><a href="#sync-details">03 · Синхронізація</a><a href="#evolution-details">04 · Конфлікти й версії</a></nav>
 <article className="technical-chapter" id="core-details"><span className="chapter-number">01 / СПІЛЬНЕ ЯДРО</span><h3>Одна реалізація правил.<br/> Кілька інтерфейсів.</h3><p className="technical-lead">Спільне ядро — це headless-логіка VIDA, відокремлена від екранів і конкретної операційної системи. Воно визначає, що означає ресурс, хто може його змінити та як перевірити операцію. Flutter показує інтерфейс; Rust core/runtime забезпечують спільну поведінку.</p><Diagram source={coreGraph} label="Схема 2. Межі спільного ядра та інфраструктури"/><TechTable headers={['Компонент','Відповідальність','Межа']} rows={[
 ['vida-core','Семантика ресурсів, команд, прав та доменних передумов.','Не володіє UI та платформними дозволами ОС.'],
 ['vida-runtime','Життєвий цикл спільної інфраструктури й реалізації зовнішніх ports.','Не дозволяє адаптерам змінювати домен без перевірених операцій.'],
 ['vida-contracts / SDK','ID, DTO, помилки, interfaces/ports та binding до клієнта.','Версійна сумісність; внутрішній API не стає публічним автоматично.'],
 ['VidaNodeHost','Iroh endpoints, Router, discovery, relay та реакція на зміну мережі.','Transport identity не замінює Persona, membership або права.'],
 ['SpaceMembership','Підписані grants/revokes, ролі, scope і key epochs.','Доступ не залежить від того, який пристрій першим з’єднався.'],
 ['SyncLog / передавання','Перевірена історія, causal repair, outbox, повтори й доставка.','ACK доставки не означає доменного прийняття.'],
 ['BlobStore','Encrypted manifests, перевірені chunks, докачування, pin/GC.','Не визначає Space policy чи порядок повідомлень.'],
 ['Conformance Plane','Специфікації, схеми, golden vectors та сценарії сумісності.','Наявність тестового сценарію ще не є виконаним proof.']
 ]}/><Source file="transport-architecture.md" label="Межі компонентів та шлях зміни"/></article>
 <article className="technical-chapter" id="schema-details"><span className="chapter-number">02 / SCHEMA-DRIVEN ЗАСТОСУНКИ</span><h3>Схема описує дані й поведінку.<br/> Runtime надає виконання.</h3><p className="technical-lead">Йдеться не лише про графічні діаграми. Кожен пакет має машинозчитувані схеми ресурсів: поля, зв’язки, команди, workflows, декларації прав і UI/data ports. Клієнт відображає їх через підтримувані primitives, а ядро повторно перевіряє кожну зміну.</p><Diagram source={schemaGraph} label="Схема 3. Як декларативний пакет стає працюючим AppInstance"/><div className="schema-columns"><div><h4>Що описує пакет</h4><dl className="technical-facts"><div><dt>Типи та поля</dt><dd>Stable SchemaId/ContractId, версія, поля та правила валідності. Формат серіалізації ще не зафіксовано остаточно.</dd></div><div><dt>Зв’язки</dt><dd>Посилання на ресурси, а не дублікати чатів, задач чи файлів. Cross-app зв’язок використовує версійний typed contract і перевірені права.</dd></div><div><dt>Команди та процеси</dt><dd>Дозволені переходи й операції на підтримуваних можливостях runtime. Нова declaration сама по собі не додає новий виконуваний primitive.</dd></div><div><dt>Форми й представлення</dt><dd>UI показує дані та доступні дії; прихована кнопка не замінює перевірку доступу в Core.</dd></div></dl></div><div className="schema-example"><span>Приклад домену ОСББ</span><pre>{schemaExample}</pre></div></div><p className="contract-note">Для iOS v1 погоджено декларативний пакет на вже вбудованих capabilities. Завантажуваний Rhai/Wasm/JavaScript executor не входить до цього профілю. Остаточний manifest, hook API, signing і migration ABI ще потребують специфікації.</p><Source file="app-runtime.md" label="AppPackage, AppInstance та capability contract"/></article>
 <article className="technical-chapter" id="sync-details"><span className="chapter-number">03 / СИНХРОНІЗАЦІЯ</span><h3>Синхронізуються перевірені зміни.<br/> Права й результати залишаються явними.</h3><p className="technical-lead">Пристрої рівноправні: немає «головного телефону» або ноутбука-арбітра. Iroh переносить дані, а VIDA перевіряє їх значення. Прямий канал — перший вибір; encrypted relay — резервний. Browser direct-path лишається окремим прототипним gate.</p><Diagram source={syncGraph} label="Схема 4. Пряма сесія між двома авторизованими пристроями"/><p className="contract-note">Це пояснювальна послідовність із review-контуру сесії, а не затверджений wire protocol. Exact encoding, authority topology та деякі merge-правила ще відкриті.</p><div className="sync-stepper" aria-label="Етапи синхронізації">{syncSteps.map(([name],i)=><button key={name} aria-pressed={i===step} onClick={()=>setStep(i)}><span>{i+1}</span>{name}</button>)}</div><div className="sync-explanation" aria-live="polite"><b>Крок {step+1}. {syncSteps[step][0]}</b><p>{syncSteps[step][1]}</p></div><h4 className="table-heading">П’ять різних доказів — п’ять різних значень</h4><TechTable headers={['Вісь стану','Що має бути доведено','Приклад запису до бізнесу']} rows={[
 ['Збережено локально','Атомарний commit операції, журналу, outbox та потрібних локальних даних.','Запит переживе перезапуск клієнта.'],
 ['Синхронізовано','Підписаний applied-at-frontier receipt за Replication Policy; типово від першої незалежної дозволеної durable replica.','Є підтверджена репліка; це ще не бронювання.'],
 ['Доставлено','Чинний пристрій адресата розшифрував, перевірив і durable-зберіг точну операцію.','Запит отримала сторона бізнесу.'],
 ['Доменне рішення','Named Resource Authority видала outcome для RequestId + revision + frontier.','Бізнес підтвердив або відхилив запис.'],
 ['Зовнішній ефект','Окремий доказ виконання зовнішньої дії за її контрактом.','Інтеграція застосувала зміну; доставка не запускає її двічі.']
 ]}/><div className="data-planes"><h4>Різні дані — різні шляхи</h4><p><strong>Зміни ресурсів:</strong> підписані operations, SyncLog та repair.</p><p><strong>Файли:</strong> encrypted manifests і verified chunks через BlobStore.</p><p><strong>Курсори й присутність:</strong> тимчасові сигнали, що не стають історією документа.</p><p><strong>Права й ключі:</strong> контрольні операції membership, grants/revocations і key epochs.</p></div><div className="source-row"><Source file="device-sync.md" label="Контур sync-сесії"/><Source file="operation-finality.md" label="Докази, receipts і finality"/></div></article>
 <article className="technical-chapter" id="evolution-details"><span className="chapter-number">04 / КОНФЛІКТИ ТА ЕВОЛЮЦІЯ</span><h3>Зміни зберігають контекст.<br/> Оновлення — історію.</h3><div className="conflict-cases"><article><h4>Незалежні правки</h4><p>Зміни різних частин тексту поєднуються за контрактом. Конкретний CRDT/editor pair ще потребує вибору та перевірки.</p></article><article><h4>Несумісні прийняті гілки</h4><p>Для непорівнюваних змін статусу чи заміни файла зберігаються обидва варіанти та явний конфлікт. Немає winner за годинником чи порядком доставки.</p></article><article><h4>Явне рішення</h4><p>Уповноважена людина створює нову операцію з причинною основою обох гілок. Права перевіряються знову; нова непорівнювана гілка може відкрити конфлікт повторно.</p></article></div><Diagram source={evolutionGraph} label="Схема 5. Версії схем і безпечна активація змін"/><TechTable headers={['Ситуація','Поведінка']} rows={[
 ['Додано optional поле або safe default','Старий запис читається через converter; наступний edit створює write у current schema.'],
 ['Нове required поле без default','Збереження блокується до введення значення. Початковий запис не губиться.'],
 ['Старий клієнт не розуміє нову схему','Safe round-trip або явний read-only/update-required для affected AppInstance; жодного тихого видалення невідомих полів.'],
 ['Preflight міграції не пройдено','Попередня активна версія лишається чинною. Це відмова до активації, а не rollback після неї.'],
 ['Міграція локальної бази','Оновлює фізичне сховище одного пристрою, але не приймає нову schema всього Shared Space.']
 ]}/><p className="contract-note">Еволюція схем описана чернеткою контракту. Exact migration authority, mixed-version window та recovery після активації ще відкриті. Історичні operations зберігають власну schema/version; довільний downgrade після активації не входить до baseline.</p><Source file="schema-evolution.md" label="Чернетка еволюції схем"/></article>
 <aside className="technical-boundary"><h4>Що вже визначено, а що ще треба реалізувати</h4><p>Прийнято архітектурні межі, модель пакетів, розділення доказів і принципи збереження конфліктів. Конкретні wire formats, CRDT/editor, storage provider, media stack, concurrent authority mechanics і повні бізнес-пакети потребують прототипів, рішень та conformance-доказів.</p></aside></div></section>}
