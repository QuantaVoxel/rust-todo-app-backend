# Todo App v2 — Production-Ready Rust Backend (Salvo 1.1 & SeaORM 2.0)

Backend REST API & Real-time Notification Server untuk **Todo App v2**, dibangun menggunakan **Rust (2024 Edition)**, **Salvo 1.1.2**, **SeaORM 2.0.4**, **PostgreSQL**, dan **Scrutiny Validation**.

---

## Fitur Utama

- **93 Endpoint di 20 Modul**: Auth (JWT Rotation & Instant Revocation), Users, Profiles, Settings, Tasks Core, Subtasks, Notes & History, Comments, Attachments (Multipart & URL), Reminders, Recurring Tasks, Locations, Sharing & Collaboration, Votes, Tags, Categories, Labels, Audit History, External Integrations, User Connections, Notifications, Activity Logs, dan Reference Lookups.
- **Real-time Notifications (SSE & WebSocket)**:
  - Server-Sent Events (`GET /api/v1/notifications/stream`) dengan keep-alive 15 detik.
  - Full-duplex WebSocket (`GET /api/v1/notifications/ws`) dengan server heartbeat 30 detik dan perintah interaktif (`ping`, `mark_read`).
- **Background Worker Otomatis**: Memproses pengingat jatuh tempo dan membuat instance *recurring task* baru secara berkala setiap 30 detik.
- **In-Memory Caching (Moka & ETag)**: Caching berkecepatan tinggi untuk data referensi (`/statuses`, `/priorities`, `/external-apps`, `/enums`) dan klasifikasi (`/tags`, `/categories`, `/labels`) dengan invalidasi otomatis saat modifikasi data.
- **Production Resilience & Observability**:
  - Graceful Shutdown (`SIGINT` / `SIGTERM`) dengan drain *in-flight requests* (25 detik).
  - HTTP Compression (`brotli`, `zstd`, `gzip`, `deflate`), Request Timeout (30 detik), CORS, Security Headers, dan Rate Limiting berbasis IP.
  - Readiness Probe (`GET /api/v1/health/ready`) dan Prometheus/JSON Metrics (`GET /api/v1/metrics`).
- **Interactive API Explorer**:
  - Swagger UI: `http://localhost:8080/swagger-ui/`
  - Scalar UI: `http://localhost:8080/scalar/`
  - OpenAPI 3.1 JSON: `http://localhost:8080/api-doc/openapi.json`

---

## Cara Menjalankan dengan Docker Compose (Direkomendasikan)

1. Jalankan PostgreSQL 16 dan Todo App Backend:
   ```bash
   docker compose up -d --build
   ```
2. Container otomatis menjalankan 33 migrasi database dan mengisi data demo (`SEED_DEMO_DATA=true`).
3. Periksa kesiapan layanan:
   ```bash
   curl http://localhost:8080/api/v1/health/ready
   ```

---

## Cara Menjalankan Secara Lokal (Cargo)

1. Salin konfigurasi environment:
   ```bash
   cp .env.example .env
   ```
2. Jalankan server (otomatis menerapkan migrasi database):
   ```bash
   cargo run
   ```
   *(Opsional)* Jalankan sekaligus mengisi data demo:
   ```bash
   cargo run -- --seed
   ```
3. Jalankan seluruh pengujian otomatis:
   ```bash
   cargo test
   ```

---

## Akun Demo (Seeder)

Jika dijalankan dengan `--seed`, `SEED_DEMO_DATA=true`, atau memanggil `POST /api/v1/seed/demo`:
- **Demo Account**: `demo_user` (`demo@todoapp.local`) | Password: `Password123!`
- **Collaborator Account**: `collab_user` (`collab@todoapp.local`) | Password: `Password123!`

Dokumentasi lengkap kontrak endpoint tersedia di [API_DOC.md](file:///home/venom/RustroverProjects/todo-app-v2/API_DOC.md) dan koleksi Postman di [postman_collection.json](file:///home/venom/RustroverProjects/todo-app-v2/postman_collection.json).
