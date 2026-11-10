# 🚀 Exotica Launcher

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)](https://www.microsoft.com/windows)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

**Exotica** — это легковесный, стильный и безопасный лаунчер для Roblox, написанный на Rust. Он создан для тех, кто хочет выжать максимум производительности из своего ПК, особенно если игра установлена через Microsoft Store (Xbox).

## 🎯 Зачем нужен Exotica?
Версия Roblox из Microsoft Store часто работает нестабильно, не имеет гибких настроек и не дает возможности применять оптимизации. Exotica решает эту проблему:
* Обходит защиту UWP-приложений и запускает игру напрямую.
* Безопасно применяет разрешенные FastFlags для повышения FPS.
* Предлагает удобный интерфейс для управления профилями и очистки системы перед игрой.

## ✨ Ключевые возможности

*   **⚡ 3 Профиля оптимизации:**
    *   `FPS BOOST` — отключает тяжелые тени и траву для максимального FPS.
    *   `BALANCED` — золотая середина между картинкой и производительностью.
    *   `QUALITY` — для тех, у кого мощный ПК.
*   **🛡 Безопасность (Anti-Ban):** Лаунчер **не внедряется** в память игры и не использует запрещенные скрипты. Он работает только через официальный файл `ClientAppSettings.json`.
*   **🚀 Game Booster:** В один клик закрывает тяжелые фоновые процессы (Chrome, Discord, Telegram), освобождая драгоценные ресурсы процессора.
*   **📊 Мониторинг:** Отслеживание использования ОЗУ и проверка пинга до серверов.
*   **🎨 Современный UI:** Темная тема, плавные анимации, светящийся фон и настраиваемые цветовые акценты.

## 🚀 Как начать пользоваться

### Для обычных пользователей
1. Перейди в раздел **[Releases](../../releases)** на этом GitHub.
2. Скачай последний файл `exotica_launcher.exe`.
3. Запусти файл. Лаунчер сам найдет установленный Roblox (в том числе из Xbox Game Pass).

### Для разработчиков
Если ты хочешь собрать проект из исходников или внести свой вклад:

```bash
# 1. Клонируй репозиторий
git clone https://github.com/твой_ник/exotica-launcher.git
cd exotica-launcher

# 2. Собери оптимизированную версию (требуется установленный Rust и C++ Build Tools)
cargo build --release

# 3. Готовый .exe будет лежать в папке:
# target/release/exotica_launcher.exe