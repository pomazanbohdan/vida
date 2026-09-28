---
id: ARCH-WINDOWS-OPTIONS-001
status: draft
last_updated: 2026-09-20
decision_refs:
  - ADR-0014
open_questions:
  - OQ-0008
  - OQ-0035
---

# Windows-клієнт VIDA: WinUI 3, Flutter або Tauri

Це історичне дослідження кандидатів. `ADR-0020` обрав Flutter Windows для Release 1; WinUI 3 + C# лишається можливою окремою нативною реалізацією після Release 1. Прийняті інваріанти: встановлюваний повноцінний клієнт, без browser/WebView shell; Messenger, Notes/Knowledge і Projects/Tasks корисні офлайн; Rust core/runtime визначає домен, права, операції, синхронізацію та Iroh ([ADR-0014](decisions/ADR-0014-native-only-vida-clients.md), [ADR-0020](decisions/ADR-0020-flutter-windows-in-release-1.md)).

## Що дає WinUI 3

Microsoft називає [WinUI 3](https://learn.microsoft.com/en-us/windows/apps/get-started/winui-get-started-overview) native XAML UI framework для Windows desktop на C# або C++; він не є браузерним UI. Отже, WinUI 3 усуває питання WebView для **основного інтерфейсу**, але C# залишається managed .NET-кодом, а не тотожним Rust/C++ binary. Потреби VIDA в Windows-специфічних взаємодіях, клавіатурі та screen reader можна перевіряти на [UI Automation/XAML](https://learn.microsoft.com/en-us/windows/apps/design/accessibility/accessibility), не вважати автоматично доведеними. [ItemsRepeater](https://learn.microsoft.com/en-us/windows/apps/develop/ui/controls/items-repeater) підтримує віртуалізовані колекції, проте не надає готову політику фокуса й вибору для складної таблиці задач. [RichEditBox](https://learn.microsoft.com/en-us/windows/apps/develop/ui/controls/rich-edit-box) підтримує форматований текст, але сам по собі не доводить відповідність schema-defined blocks/relations та міжплатформному документному формату VIDA; точну API/SDK версію треба перевірити в spike.

Пропонований для прототипу шлях: `WinUI/XAML → C# presentation adapter → версійований C ABI → Rust vida-sdk facade → vida-runtime/core/Iroh`. [.NET LibraryImport/P/Invoke](https://learn.microsoft.com/en-us/dotnet/standard/native-interop/pinvoke-source-generation) дає механізм виклику native library, а [Rust FFI](https://doc.rust-lang.org/nomicon/ffi.html) — C ABI; це **технічна можливість**, не готовий VIDA binding. Потрібно довести DTO/error ABI, ownership і звільнення пам'яті, callbacks/events, async/cancellation, threading, crash behavior, x64/ARM64 і сумісність версій (`OQ-0035`). Офіційний [UniFFI](https://mozilla.github.io/uniffi-rs/next/) не перелічує C# серед мов повної підтримки; не планувати його як готовий C# міст без окремого доказу. Локальний IPC/sidecar можливий лише за окремої потреби ізоляції чи роботи до входу користувача; він додає автентифікацію локального каналу, життєвий цикл і one-writer проблему, не є дефолтом.

WinUI renderer повинен тлумачити ті самі `AppPackage` schemas/UI ports/capability manifest, що й mobile renderer. Новий пакет без оновлення клієнта працює лише в межах уже підтримуваних native components; невідома mandatory capability відхиляється явно. Для schema-defined views потрібен явний component registry та runtime binding/templating, а не припущення, що довільний XAML можна безпечно виконувати з пакета; [WinUI data binding](https://learn.microsoft.com/en-us/windows/apps/develop/data-binding/data-binding-in-depth) має різні runtime/compile-time механізми. Пакет не отримує права завантажувати довільний C# assembly або обходити Rust authorization. Це архітектурний наслідок [ADR-0007](decisions/ADR-0007-declarative-app-packages.md), а не властивість WinUI.

«Ті самі UI ports» ще не є достатньою специфікацією: `OQ-0056` має визначити versioned semantics для полів, валідації, relations, rich blocks, fallback й accessibility з cross-shell fixtures. Так само `OQ-0012` має задати мінімальну матрицю **конкретних** офлайн-команд для кожного з трьох Apps. Без обох контрактів Flutter і WinUI можуть пройти загальне гасло «offline та AppPackage», але показати різні можливості; bake-off тоді нечесний.

## Порівняння для VIDA

| Критерій | Flutter Windows | WinUI 3 + C# | Tauri Windows |
|---|---|---|---|
| Основний UI | Flutter renderer; спільні Dart-компоненти з mobile, але Windows UX все одно адаптуємо | Windows XAML controls і окремий C# presentation layer; mobile UI не повторно використовується | HTML/CSS/JS у системному WebView; окремий UI-стек, без browser client VIDA |
| Міст до Rust | Flutter FFI/FRB — prototype gate | C ABI + .NET P/Invoke — prototype gate | Rust host напряму, UI→Rust IPC — security gate |
| Складний editor/grid | Перевірити mobile/desktop адаптацію, IME, keyboard, accessibility | Перевірити blocks/relations/undo та virtualization/focus/a11y на native controls | Перевірити DOM editor/grid і недовірений вміст у privileged WebView |
| Вартість підтримки | Найбільше повторне використання з Flutter mobile | Окрема C#/XAML UI реалізація, зате Windows-specific UX | Окрема web UI реалізація та XSS→IPC межа |
| Інсталяція/оновлення | Windows installer/updater обирається | [MSIX/unpackaged/package identity](https://learn.microsoft.com/en-us/windows/apps/package-and-deploy/packaging/) окремо від Windows App SDK runtime mode, [.NET runtime publish mode](https://learn.microsoft.com/en-us/dotnet/core/deploying/single-file/overview) і постачання Rust DLL; кожну вісь треба випробувати | Tauri installer і WebView2 distribution треба випробувати |

Це якісне порівняння з документації, **не** вимірювання продуктивності чи вартості. WinUI 3 не потребує C#-реалізації Iroh: мережевий вузол лишається в Rust. За `ADR-0020` Flutter Windows входить у Release 1; WinUI 3/C# розглядається пізніше лише за доказом, що desktop-native UX виправдовує окрему реалізацію.

## Спільний decision gate

Той самий release-build vertical slice на Windows-пристроях, одному Rust core та однакових даних/fixtures для кандидатів, які відповідають `OQ-0008`:

1. Messenger, Notes/Knowledge, Projects/Tasks: однаковий мінімальний набір offline read/local actions за майбутньою матрицею `OQ-0012`, durable pending після process kill, reconnect, conflict і received revocation без другого доменного engine в C# або Dart.
2. AppPackage із зовнішнього репозиторію: native form/view, permissions, unknown capability, однаковий operation envelope й error semantics; cross-shell UI-port fixtures за `OQ-0056`, Windows renderer не підміняє canonical schema.
3. Rich notes + task grid: IME, keyboard, screen reader, virtualization, 100k-item search і перехід повідомлення ↔ документ ↔ задача; заздалегідь погоджені UX/a11y/performance пороги.
4. Windows lifecycle: sleep, app exit/restart, activation з notification, single-instance/one-writer, signed install/update/rollback і відновлення vault; [Windows App SDK lifecycle](https://learn.microsoft.com/en-us/windows/apps/develop/launch/app-lifecycle) не обіцяє always-on при sleep або аварії. Окремо перевірити чотири deployment-вісі: package identity/install, Windows App SDK runtime, .NET runtime та Rust DLL/ABI; один прапорець «self-contained» не вирішує всі чотири.
5. Безпека: атакувальний пакет/вміст не може пройти поза Rust facade; секрети й локальний кеш перевіряються окремо. [OWASP Authorization](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html) вимагає deny-by-default і перевірки прав на кожну дію. [MASVS](https://mas.owasp.org/MASVS/) стосується mobile; він не є сертифікацією Windows UI.

Повноцінна Windows-поведінка є вимогою Release 1. Flutter Windows vertical slice має довести packaging, supported architectures, accessibility, keyboard/menu/tray, FFI, startup і memory; WinUI-specific UX може виправдати C#/XAML лише після Release 1. Package model, UI-port contract (`OQ-0056`) та числові gates залишаються відкритими.
