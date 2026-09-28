---
title: VIDA localization selection and the absence of a Google top-10 list
date: 2026-09-22
status: complete
---

# Висновок

Google не публікує універсальний рейтинг «топ-10 мов для застосунку». Google Play підтримує багато локалей, а language recommendations у Play Console формуються для конкретного застосунку лише за наявності достатніх даних: install distribution/growth, category preferences, conversion rate і potential market. Тому попередній список із десяти європейських мов не можна називати «топ-10 Google».

# Рішення для VIDA

- Українська є обов'язковою source/product locale.
- Російська свідомо не входить до початкового набору.
- Решта дев'ять локалей не фіксуються як вигаданий глобальний рейтинг.
- До появи Play Console data набір визначається окремим market decision: цільові країни → addressable users → продуктова релевантність → RTL/font/layout cost → human QA capacity.
- Після появи достатніх даних набір переглядається за app-specific recommendations і conversion/install evidence із Play Console.

# Незалежні орієнтири

- AppTweak оцінює найбільші ринки завантажень 2025 так: India, United States, Brazil, Indonesia, Mexico, China, Turkey, Philippines, Germany і United Kingdom. Це аргумент за English, Hindi, Brazilian Portuguese, Indonesian, Spanish, Turkish і German, але країна не завжди дорівнює одній мові.
- AppMagic для non-game Apps показує downloads-лидерів India, USA, Indonesia, Brazil, China, Pakistan, Morocco, Russia, Mexico і Bangladesh, а revenue-лидерів — USA, China, Japan, Germany, UK, France, Brazil, South Korea, Canada й Australia. Після виключення Russia це створює явний trade-off між охопленням (Hindi/Indonesian/Arabic/Bengali/Spanish) і monetization (Japanese/German/French/Korean).
- World Bank показує сильне недопредставлення багатьох великих мов у цифровому контенті; частка web pages не є коректним proxy для потенційної аудиторії застосунку.
- Crowdin рекомендує починати з мов поточних/цільових користувачів, а не з максимальної кількості; Phrase підкреслює, що localization включає UI, assets і культурні очікування, а не лише переклад рядків.

# Затверджений об'єднаний набір

Користувач затвердив не вибір між balanced-global та Europe-first, а їх об'єднання. Release 1 має 18 мов: Ukrainian, English, Spanish, Portuguese, Hindi, Indonesian, Arabic, German, French, Japanese, Korean, Turkish, Chinese, Polish, Italian, Romanian, Czech і Dutch. Regional profiles додають `es-419`/`es-ES`, `pt-BR`/`pt-PT`, `zh-Hans`/`zh-Hant`, разом 21 початковий locale profile. Російська виключена.

Chinese language support не означає автоматичного mainland-China launch: distribution/compliance лишається окремим контуром.

# Офіційні джерела

- [Google Play: Translate and localize your app](https://support.google.com/googleplay/android-developer/answer/9844778): перелік підтримуваних мов і app-specific language recommendations.
- [Google Play: country targeting](https://support.google.com/googleplay/android-developer/answer/7550024): availability визначається окремо за країнами/регіонами.
- [Android localization](https://developer.android.com/guide/topics/resources/localization): platform localization model і checklist.
- [AppTweak: app downloads by country 2025](https://www.apptweak.com/en/reports/app-downloads-by-country): незалежна оцінка download markets.
- [AppMagic: Mobile Market Landscape 2026](https://appmagic.rocks/files/view/upload/Reports/EN_MobileMarkeLandscape2026.pdf): download та revenue markets для Apps.
- [World Bank: Digital Progress and Trends Report 2025](https://documents1.worldbank.org/curated/en/099112525160593874/pdf/P505350-d12bf50c-2e65-4863-822b-8fb9e69bda4b.pdf): population/content language imbalance.
- [Crowdin mobile-app localization guide](https://crowdin.com/blog/mobile-app-localization-guide): target-market-first prioritization.
- [Phrase mobile-app localization guide](https://phrase.com/blog/posts/mobile-app-localization-why-and-how/): localization scope beyond translation.

# Відкрите рішення

Набір затверджено. Після запуску можна додавати regional variants і нові мови за фактичними app-specific даними, не вилучаючи погоджену Release-1 базу без окремого рішення.
