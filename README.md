# AI-PowerPoint

Редактор презентаций в браузере, устроенный как Microsoft PowerPoint, с русским интерфейсом,
помощником на локальной модели (Ollama-совместимый шлюз) и файлами пользователей на своём сервере.
Работает без интернета: всё загружается из папки приложения.

- **Установка на сервер** (nginx + PHP, без Rust): [studio/README.ru.md](studio/README.ru.md).
- **Что нового:** [studio/CHANGES.ru.md](studio/CHANGES.ru.md). Готовые сборки — в Releases
  (`ai-powerpoint-build-N-дата.zip`), их собирает GitHub Actions (`.github/workflows/studio.yml`).
- **Основа:** открытый проект [DeckCraft](https://github.com/storytold/slidecraft) команды ArtCraft
  (MIT OR Apache-2.0). Его описание — [docs/README-upstream.md](docs/README-upstream.md); правила
  работы с кодом — [AGENTS.md](AGENTS.md).

Что добавлено поверх DeckCraft:

- `apps/deckcraft-web/src/web/bridge.rs` — `window.aipowerpoint`: страница управляет редактором теми же
  командами, что канал управления и MCP; сохранение (Ctrl+S) уходит странице и ложится на сервер.
- `apps/deckcraft-web-boot` — загрузчик, который скачивает приложение частями с полосой загрузки.
- `crates/ui-egui/src/i18n.rs`, `crates/ui-egui/i18n/ru.tsv` — русский интерфейс.
- Фирменная, светлая и тёмная темы (`crates/ui-egui/src/theme.rs`), домик «На сайт», кнопка **ИИ** у выделения.
- `studio/` — панель помощника, PHP (`api/`), сборка папки для сервера с номерами сборок.

Лицензия: MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`, `NOTICE`, `ATTRIBUTION.md`).
