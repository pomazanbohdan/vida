import { Diagram } from './TechnicalDetails.jsx';
const connection=`flowchart TB
 A[Endpoint A: власний ключ та EndpointId] --> B[EndpointId B і дані про адреси]
 B --> L[Відомі адреси або Address Lookup]
 L --> D[Спроба прямого шляху]
 D --> Q{Peer досяжний напряму?}
 Q -->|Так| P[Прямий QUIC канал]
 Q -->|Потрібна координація NAT| R[Relay: координація та резервне передавання]
 R --> H[Hole punching: перевірка адресних кандидатів]
 H -->|Прямий шлях доступний| P
 H -->|Прямий шлях недоступний| F[Зашифрований канал через relay]
 P --> T[Автентифіковане з’єднання між endpoints]
 F --> T
 T --> ALPN[Узгоджений ALPN: потрібний VIDA protocol]
 ALPN --> V[VIDA перевіряє права, схему та операцію]`;
export function IrohDetails(){return <section className="iroh-details section" id="iroh-protocol"><div className="wrap"><p className="eyebrow">ОКРЕМО ПРО IROH / ТРАНСПОРТНИЙ РІВЕНЬ</p><div className="section-heading"><h2>З’єднати пристрої.<br/> Захистити обмін.</h2><p>Iroh забезпечує канал.<br/> VIDA визначає значення даних.</p></div><p className="technical-lead">Iroh — бібліотека з’єднань між вузлами на основі QUIC, з автентифікацією, шифруванням, пошуком адрес і проходженням NAT. У VIDA її підключенням та мережевим життєвим циклом керує VidaNodeHost.</p><Diagram source={connection} label="Як Iroh встановлює канал: цільова direct-first політика VIDA"/>
 <div className="iroh-steps">
 <article><span>01 / АДРЕСАТ</span><h3>Ключ замість постійної IP-адреси</h3><p>EndpointId пов’язаний із публічним ключем вузла. IP-адреса може змінюватися; ідентичність endpoint лишається засобом перевірки, з ким встановлено канал. Persona та права Space перевіряє VIDA окремо.</p></article>
 <article><span>02 / ПОШУК ШЛЯХУ</span><h3>Відомі адреси або Address Lookup</h3><p>Щоб дістатися peer, потрібні актуальні дані про його мережеву доступність. Вони можуть бути вже відомі або отримані через налаштований lookup. Це пошук маршруту, а не каталог людей чи дозвіл читати їхні дані.</p></article>
 <article><span>03 / NAT TRAVERSAL</span><h3>Hole punching</h3><p>Домашні маршрутизатори часто блокують непрохані вхідні пакети. Peers узгоджують адресні кандидати та надсилають проби назустріч. Якщо це створює досяжний шлях, дані можуть іти безпосередньо.</p></article>
 <article><span>04 / ЗАХИЩЕНИЙ КАНАЛ</span><h3>QUIC та TLS</h3><p>Peers автентифікують одне одного та обмінюються зашифрованими даними. Relay пересилає трафік, не отримуючи відкритого вмісту. При цьому мережеві метадані не стають автоматично невидимими.</p></article>
 <article><span>05 / ПРИКЛАДНИЙ ПРОТОКОЛ</span><h3>ALPN обирає обробник</h3><p>Під час встановлення з’єднання узгоджується прикладний протокол. Для VIDA задано версійний шаблон <code>vida/&lt;capability&gt;/&lt;major&gt;</code>; Router спрямовує з’єднання до відповідного обробника. Це межа сумісності, а не grant доступу.</p></article>
 <article><span>06 / ЗМІНА МЕРЕЖІ</span><h3>Шлях може змінитися</h3><p>Iroh підтримує зміну мережевих шляхів. За появи прямого маршруту обмін може перейти з relay на нього. Якщо зв’язок втрачено, VIDA показує очікування й відновлює обмін за власними правилами синхронізації.</p></article>
 </div><div className="ecosystem-scenario"><div><h3>Політика VIDA</h3><ul><li>Native peers спочатку пробують доступний прямий шлях.</li><li>Для LAN або відомих адрес передбачено профіль без relay та зовнішнього lookup.</li><li>Для NAT-координації й fallback — керовані VIDA сумісні Iroh relay.</li><li>За відсутності доступного шляху операція лишається локально pending.</li></ul></div><div><h3>Що залишається за ядром</h3><p>Iroh не визначає членство у Space, право редагувати ресурс, конфлікти версій або підтвердження запису до бізнесу. VIDA перевіряє підпис операції, schema, причинні залежності та policy, а потім оновлює локальні дані.</p><p>Успішний транспортний канал ще не означає, що зміна синхронізована або прийнята бізнесом.</p></div></div><p className="contract-note">Схема показує цільову політику VIDA, а не обов’язкову послідовність кожного Iroh deployment: relay може брати участь у встановленні каналу й координації NAT. Для Web прямий шлях потребує окремо перевіреного browser-compatible adapter; його готовність не випливає з можливостей native Iroh.</p><div className="source-row"><a className="technical-source" href={import.meta.env.BASE_URL+"reference/iroh-transport.md"} target="_blank" rel="noreferrer">ADR-0005: Iroh у VIDA ↗</a><a className="technical-source" href="https://github.com/n0-computer/iroh" target="_blank" rel="noreferrer">Офіційний Iroh: з’єднання й API ↗</a><a className="technical-source" href="https://www.iroh.computer/blog/iroh-on-QUIC-multipath" target="_blank" rel="noreferrer">Iroh: relay, NAT і зміна шляху ↗</a></div></div></section>}
