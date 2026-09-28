---
title: Independent pentest and cryptographic review for VIDA
date: 2026-09-22
status: complete
---

# Що це означає

Незалежний pentest виконує зовнішня application-security команда, яка не проєктувала перевірюваний код. Вона тестує release-like clients, Core/API boundaries, local storage, authentication/authorization, network behavior, package supply chain і бізнес-логіку, маючи source code, документацію, builds та test identities.

Crypto-review виконує команда з практичною експертизою у cryptographic protocols. Вона окремо перевіряє протокольну модель, key lifecycle, multi-device enrollment/revocation, E2EE calls/messages, recovery, metadata leakage, downgrade/replay і відповідність реалізації специфікації. Звичайний mobile pentest не замінює цю роботу.

# Хто може виконувати

- App Defense Alliance Authorized Lab може виконати незалежну mobile assessment; лабораторії мають пройти визначену програмою акредитацію та proficiency evaluation.
- CREST-accredited application-security provider може виконувати verification, що спирається на OWASP ASVS/MASVS.
- Спеціалізована cryptography practice виконує protocol/design/implementation review; публічні звіти NCC Group для WhatsApp E2EE backups і contacts демонструють цей тип scope, але не є автоматичним вибором vendor для VIDA.

# Навіщо VIDA

VIDA поєднує E2EE, recovery keys, offline replicas, multi-device revocation, calls, external AppPackages і складну authorization/sync business logic. Помилка в одному з цих шарів може не проявитися в unit tests або scanner. Незалежна команда додає інший threat model, ручну перевірку й reproducible evidence. Це зменшує ризик, але не гарантує відсутність вразливостей.

# Коли

Vendor selection і витрати зараз не потрібні. Доцільна точка рішення — після стабілізації security-critical vertical slice та threat model, але до public Release 1 із реальними приватними даними. Спочатку визначається scope; потім review; потім remediation і retest.

# Джерела

- [OWASP MASVS](https://mas.owasp.org/MASVS/)
- [OWASP assessment guidance](https://mas.owasp.org/MASVS/04-Assessment_and_Certification/)
- [OWASP mobile testing methodology](https://mas.owasp.org/MASTG/0x04b-Mobile-App-Security-Testing/)
- [NIST SP 800-115](https://csrc.nist.gov/pubs/sp/800/115/final)
- [App Defense Alliance authorized labs](https://www.appdefensealliance.org/certification/authorized-labs)
- [Example: WhatsApp E2EE backup assessment](https://www.nccgroup.com/research/public-report-whatsapp-end-to-end-encrypted-backups-security-assessment/)
