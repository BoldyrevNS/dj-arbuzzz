# Локальная разработка

## Требования
- Rust (nightly)
- Node.js 22+, pnpm
- PostgreSQL 17
- Redis 7
- Docker (опционально)

## Быстрый старт с Docker

### 1. Запустите инфраструктуру (PostgreSQL + Redis)
```bash
docker-compose up -d postgres redis
```

### 2. Создайте `.env` файл
```bash
cp .env.example .env
```

Отредактируйте `.env`, установите минимальные значения:
```env
# Database
POSTGRES_USER=djarbuzzz
POSTGRES_PASSWORD=djarbuzzz_password
POSTGRES_DB=djarbuzzz_db
DATABASE_URL=postgresql://djarbuzzz:djarbuzzz_password@localhost:5432/djarbuzzz_db

# Redis
REDIS_URL=redis://localhost:6379

# Secrets (для dev можно оставить дефолтные)
JWT_SECRET=dev-secret-key
RESTORE_SECRET=dev-restore-secret
SIGN_UP_SECRET=dev-signup-secret
ACCESS_SECRET=dev-access-secret
REFRESH_SECRET=dev-refresh-secret

# SMTP (для тестирования можно оставить пустыми)
SMTP_HOST=
SMTP_PORT=587
SMTP_LOGIN=
SMTP_PASSWORD=
SMTP_FROM=

# Music API (опционально)
MUSIC_API_TOKEN=
MUSIC_API_URL=https://api.vk.com/method/audio

# Paths
SONGS_PATH=./songs
```

### 3. Запустите миграции
```bash
docker-compose up migrations
```

### 4. Запустите Rust backend
```bash
cd server
cargo run
# Backend будет доступен на http://localhost:8080
# WebSocket: ws://localhost:8080/api/v1/ws/ws
```

### 5. Запустите Nuxt frontend
```bash
cd client
pnpm install
pnpm dev
# Frontend будет доступен на http://localhost:3000
```

### 6. Откройте браузер
```
http://localhost:3000
```

В консоли браузера должны увидеть:
```
[WebSocket] Connecting to: ws://localhost:3000/ws
[WebSocket] Connected
```

В консоли backend:
```
[WebSocket] Connection attempt
[WebSocket] Connection established
```

## WebSocket в локальной разработке

Frontend автоматически проксирует запросы `/ws` на backend через Nitro devProxy (настроено в `nuxt.config.ts`):
- Frontend: `ws://localhost:3000/ws` 
- → Backend: `ws://localhost:8080/api/v1/ws/ws`

## Проверка WebSocket вручную

### С помощью websocat
```bash
# Установка
brew install websocat

# Подключение к backend напрямую
websocat ws://localhost:8080/api/v1/ws/ws

# Через frontend dev server
websocat ws://localhost:3000/ws
```

### С помощью curl
```bash
curl -i -N \
  -H "Connection: Upgrade" \
  -H "Upgrade: websocket" \
  -H "Sec-WebSocket-Version: 13" \
  -H "Sec-WebSocket-Key: test" \
  http://localhost:8080/api/v1/ws/ws
```

## Troubleshooting

### Backend не стартует
- Проверьте, что PostgreSQL и Redis запущены: `docker-compose ps`
- Проверьте переменные окружения в `.env`
- Проверьте логи: `cargo run` покажет ошибки

### WebSocket не подключается
- Убедитесь, что backend запущен на порту 8080
- Проверьте консоль браузера для ошибок
- Проверьте логи backend (должны быть сообщения `[WebSocket] Connection attempt`)
- Попробуйте подключиться напрямую: `websocat ws://localhost:8080/api/v1/ws/ws`

### Миграции не применяются
```bash
# Сброс БД
docker-compose down -v
docker-compose up -d postgres redis
docker-compose up migrations
```

### CORS ошибки
Backend настроен для работы с `localhost:3000` в dev режиме.

## Production vs Development

| Компонент | Development | Production |
|-----------|-------------|------------|
| Frontend | `localhost:3000` | `https://djarbuzzz-music.ru` |
| Backend | `localhost:8080` | `http://backend:8080` (внутри Docker) |
| WebSocket | `/ws` → proxy на backend | `/ws` → nginx → backend |
| PostgreSQL | `localhost:5432` | `postgres:5432` |
| Redis | `localhost:6379` | `redis:6379` |

## Команды Docker

```bash
# Запустить всё
docker-compose up -d

# Остановить всё
docker-compose down

# Пересобрать backend
docker-compose build backend
docker-compose up -d backend

# Пересобрать frontend
docker-compose build frontend
docker-compose up -d frontend

# Логи
docker-compose logs -f backend
docker-compose logs -f frontend

# Удалить всё включая volumes
docker-compose down -v
```
