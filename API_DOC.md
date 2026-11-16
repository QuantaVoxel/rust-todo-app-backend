# Todo App REST API Documentation (v1)

Dokumentasi resmi seluruh endpoint REST API (88 endpoint pada 20 modul) untuk Todo App v2 backend yang dibangun menggunakan framework **Salvo** dan ORM **SeaORM 2.0**. Modul mencakup Auth, Users & Profiles, Settings, Tasks Core, Subtasks, Notes, Comments, Attachments, Reminders, Recurring Tasks, Locations, Sharing, Votes, Klasifikasi (Tags, Categories, Labels), Audit History, Integrasi Eksternal, Koneksi Pengguna, Notifikasi, Log Aktivitas, serta Data Referensi (Lookups).

---

## 1. Konvensi Umum

### Base URL
```
http://localhost:8080/api/v1
```

### Format Data & Header
- **Request & Response Body**: `application/json` (kecuali file upload via `multipart/form-data`)
- **Autentikasi**: Header `Authorization: Bearer <access_token>` (wajib untuk endpoint terproteksi)
- **Format Tanggal/Waktu**: ISO 8601 UTC (`2026-11-16T14:25:00Z`)
- **Penamaan Field**: `snake_case`

### Interactive API Documentation & Explorer (Swagger UI & Scalar)
Aplikasi menyediakan UI interaktif untuk eksplorasi dan pengujian API secara langsung di browser:
- **Swagger UI**: `http://localhost:8080/swagger-ui/` (Interaktif dengan 'Try it out' dan otentikasi Bearer token).
- **Scalar API Reference**: `http://localhost:8080/scalar/` (Modern visual documentation alternatif).
- **OpenAPI 3.1 Specification (JSON)**: `http://localhost:8080/api-doc/openapi.json` (Dapat diimpor ke Postman, Insomnia, atau code generator).
- **Cakupan Rute**: Seluruh 88 rute API di 20 modul telah terdaftar lengkap (`System`, `Auth`, `Users`, `Profiles`, `Settings`, `Lookups`, `Tasks`, `Subtasks`, `Notes`, `Comments`, `Attachments`, `Reminders`, `Recurring Tasks`, `Locations`, `Sharing`, `Votes`, `Audit History`, `Integrations`, `Tags`, `Categories`, `Labels`, `Connections`, `Notifications`, `Activity Logs`).
- **Autentikasi di Swagger**: Klik tombol **Authorize** di pojok kanan atas Swagger UI, masukkan token JWT hasil dari login/register dengan format `Bearer <token>` (atau cukup `<token>`), lalu seluruh request terproteksi akan terotentikasi secara otomatis.

### Security Headers & CORS
Setiap response API menyertakan header keamanan dan konfigurasi CORS:
- `Access-Control-Allow-Origin: *`
- `Access-Control-Allow-Methods: GET, POST, PUT, PATCH, DELETE, OPTIONS`
- `Access-Control-Allow-Headers: authorization, content-type, x-requested-with, accept, origin`
- `X-Content-Type-Options: nosniff`
- `X-Frame-Options: DENY`
- `Referrer-Policy: strict-origin-when-cross-origin`
- `X-XSS-Protection: 1; mode=block`

### Rate Limiting
Sistem menerapkan pembatasan laju permintaan berbasis IP dengan header kuota otomatis (`X-RateLimit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Reset`):
- **General API**: `120 requests / menit` per IP.
- **Auth Endpoints (`/auth/login`, `/auth/register`)**: `20 requests / menit` per IP untuk proteksi brute-force.
- **Status Kode**: Jika melebihi kuota, server mengembalikan HTTP `429 Too Many Requests`.

### Static File Serving
File yang diunggah dapat diakses langsung oleh browser atau klien HTTP pada rute publik:
- **Base URL Asset**: `http://localhost:8080/uploads/{filename}` (atau `/api/v1/uploads/{filename}`)
- Mendukung streaming, caching, ETag, dan resume download via HTTP Range headers.

### Standar Format Response Berhasil
```json
{
  "data": { ... }
}
```
*(Catatan: Endpoint `/auth/register`, `/auth/login`, dan `/auth/refresh` merender objek token / user secara langsung pada root JSON).*

### Standar Format Response Error
```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Pesan deskripsi error",
    "details": [
      {
        "field": "email",
        "issue": "must be a valid email"
      }
    ]
  }
}
```

| HTTP Status | Error Code | Keterangan |
| :--- | :--- | :--- |
| `400` | `VALIDATION_ERROR` | Request body tidak valid atau format salah |
| `401` | `UNAUTHORIZED` | Token tidak ada, tidak valid, atau telah dicabut |
| `403` | `FORBIDDEN` | Pengguna tidak memiliki hak akses |
| `404` | `NOT_FOUND` | Rute atau resource tidak ditemukan |
| `405` | `METHOD_NOT_ALLOWED` | Metode HTTP tidak diizinkan |
| `409` | `CONFLICT` | Duplikasi data unik (email, username, dll.) |
| `429` | `RATE_LIMITED` | Terlalu banyak permintaan (rate limit) |
| `500` | `INTERNAL_ERROR` | Terjadi kesalahan pada server / database |

---

## 2. Auth Module

Modul autentikasi menggunakan **bcrypt** untuk hashing kata sandi dan **JWT (HMAC-SHA256)** dengan pemisahan token type (`access` vs `refresh`) serta fitur **Refresh Token Rotation** dan **Instant Revocation**.

### 2.1 Register Akun Baru
Mendaftarkan akun user baru sekaligus membuat data profil utama.

- **Method**: `POST`
- **Endpoint**: `/auth/register`
- **Auth**: Public

#### Aturan Validasi Input
| Field | Tipe | Wajib | Aturan |
| :--- | :--- | :--- | :--- |
| `username` | String | **Ya** | Min 3, maks 50 karakter, unik |
| `email` | String | **Ya** | Format email valid, unik |
| `password` | String | **Ya** | Min 8 karakter |
| `first_name` | String | **Ya** | Min 1, maks 255 karakter |
| `last_name` | String | **Ya** | Min 1, maks 255 karakter |
| `phone_number`| String | **Ya** | Min 6, maks 20 karakter |
| `avatar_url` | String | Tidak | Format URL valid |

#### Request Body
```json
{
  "username": "budi",
  "email": "budi@mail.com",
  "password": "Password123!",
  "first_name": "Budi",
  "last_name": "Santoso",
  "phone_number": "+62812345678",
  "avatar_url": "https://cdn.example.com/avatars/budi.png"
}
```

#### Response `201 CREATED`
```json
{
  "user_id": 1,
  "username": "budi",
  "email": "budi@mail.com",
  "first_name": "Budi",
  "last_name": "Santoso",
  "phone_number": "+62812345678",
  "created_at": "2026-11-16T14:25:00Z"
}
```

#### Response `409 CONFLICT` (Email/Username sudah digunakan)
```json
{
  "error": {
    "code": "CONFLICT",
    "message": "Email atau username sudah digunakan",
    "details": null
  }
}
```

---

### 2.2 Login
Autentikasi kredensial user untuk memperoleh Access Token (berlaku 1 jam) dan Refresh Token (berlaku 30 hari).

- **Method**: `POST`
- **Endpoint**: `/auth/login`
- **Auth**: Public

#### Request Body
```json
{
  "email": "budi@mail.com",
  "password": "Password123!"
}
```

#### Response `200 OK`
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOjEsInVzZXJuYW1lIjoiYnVkaSIsInRva2VuX3R5cGUiOiJhY2Nlc3MiLCJleHAiOjE3NjM1NjM1MDAsImlhdCI6MTc2MzU1OTkwMCwianRpIjoiMTAyOTM4NDctNTY3OC00M2ExLWJjZGYtMDEyMzQ1Njc4OWFiIn0...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOjEsInVzZXJuYW1lIjoiYnVkaSIsInRva2VuX3R5cGUiOiJyZWZyZXNoIiwiZXhwIjoxNzY2MTUxOTAwLCJpYXQiOjE3NjM1NTk5MDAsImp0aSI6ImZhYmNkZWYwLTEyMzQtNTY3OC05MGFiLWMxZDJlM2Y0ZzVoNiJ9...",
  "expires_in": 3600,
  "user": {
    "user_id": 1,
    "username": "budi"
  }
}
```

#### Response `401 UNAUTHORIZED` (Kredensial salah)
```json
{
  "error": {
    "code": "UNAUTHORIZED",
    "message": "Email atau kata sandi salah",
    "details": null
  }
}
```

---

### 2.3 Refresh Token
Memperbarui Access Token yang telah kedaluwarsa menggunakan Refresh Token yang valid. Fitur **Refresh Token Rotation** secara otomatis mencabut refresh token lama dan menerbitkan refresh token baru.

- **Method**: `POST`
- **Endpoint**: `/auth/refresh`
- **Auth**: Public

#### Request Body
```json
{
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

#### Response `200 OK`
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "expires_in": 3600,
  "user": {
    "user_id": 1,
    "username": "budi"
  }
}
```

#### Response `401 UNAUTHORIZED` (Token tidak valid / telah dicabut)
```json
{
  "error": {
    "code": "UNAUTHORIZED",
    "message": "Token telah dicabut (revoked)",
    "details": null
  }
}
```

---

### 2.4 Logout
Mencabut Refresh Token dan sesi Bearer token agar tidak dapat digunakan kembali.

- **Method**: `POST` (juga mendukung `DELETE`)
- **Endpoint**: `/auth/logout`
- **Auth**: Public / Optional Bearer Token

#### Headers (Opsional)
```http
Authorization: Bearer <access_token>
```

#### Request Body (Opsional)
```json
{
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

#### Response `200 OK`
```json
{
  "data": "Logout berhasil"
}
```

---

### 2.5 Change Password
Mengganti kata sandi akun login.

- **Method**: `POST`
- **Endpoint**: `/auth/change-password`
- **Auth**: **Bearer Token (Wajib)**

#### Headers
```http
Authorization: Bearer <access_token>
```

#### Request Body
```json
{
  "old_password": "Password123!",
  "new_password": "NewSecretPassword456!",
  "new_password_confirmation": "NewSecretPassword456!"
}
```

#### Response `200 OK`
```json
{
  "data": "Kata sandi berhasil diubah"
}
```

#### Response `400 BAD_REQUEST` (Password lama salah)
```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Kata sandi lama tidak sesuai",
    "details": null
  }
}
```

---

## 3. User Module (Users, Profile, Settings)

Semua endpoint pada modul ini **wajib** menyertakan header:
```http
Authorization: Bearer <access_token>
```

---

### 3.1 Get Profile & Akun Saya
Mengambil rincian akun login beserta objek profil lengkap.

- **Method**: `GET`
- **Endpoint**: `/users/me`
- **Auth**: Bearer Token

#### Response `200 OK`
```json
{
  "data": {
    "user_id": 1,
    "username": "budi",
    "email": "budi@mail.com",
    "created_at": "2026-11-16T14:25:00Z",
    "updated_at": "2026-11-16T14:25:00Z",
    "profile": {
      "profile_id": 1,
      "user_id": 1,
      "first_name": "Budi",
      "last_name": "Santoso",
      "avatar_url": "https://cdn.example.com/avatars/budi.png",
      "phone_number": "+62812345678"
    }
  }
}
```

---

### 3.2 Update Akun Saya
Memperbarui informasi kredensial login (`username` dan/atau `email`).

- **Method**: `PATCH`
- **Endpoint**: `/users/me`
- **Auth**: Bearer Token

#### Request Body (Partial Update)
```json
{
  "username": "budi_santoso",
  "email": "budi.baru@mail.com"
}
```

#### Response `200 OK`
```json
{
  "data": {
    "user_id": 1,
    "username": "budi_santoso",
    "email": "budi.baru@mail.com",
    "created_at": "2026-11-16T14:25:00Z",
    "updated_at": "2026-11-16T14:35:00Z",
    "profile": {
      "profile_id": 1,
      "user_id": 1,
      "first_name": "Budi",
      "last_name": "Santoso",
      "avatar_url": "https://cdn.example.com/avatars/budi.png",
      "phone_number": "+62812345678"
    }
  }
}
```

#### Response `409 CONFLICT` (Username/Email telah dipakai user lain)
```json
{
  "error": {
    "code": "CONFLICT",
    "message": "Username sudah digunakan",
    "details": null
  }
}
```

---

### 3.3 Hapus Akun Saya
Menghapus akun user beserta seluruh data turunannya secara permanen (`ON DELETE CASCADE`).

- **Method**: `DELETE`
- **Endpoint**: `/users/me`
- **Auth**: Bearer Token

#### Response `200 OK`
```json
{
  "data": "Akun berhasil dihapus"
}
```

---

### 3.4 Cari Pengguna (Search Users)
Mencari user lain berdasarkan username atau email untuk keperluan berbagi task (*sharing*) atau pertemanan (*connections*). Hasil pencarian tidak menyertakan pengguna yang sedang login.

- **Method**: `GET`
- **Endpoint**: `/users`
- **Auth**: Bearer Token

#### Query Parameters
| Parameter | Tipe | Wajib | Keterangan |
| :--- | :--- | :--- | :--- |
| `search` | String | Tidak | Kata kunci pencarian nama atau email (contoh: `?search=siti`) |

#### Contoh Request
```http
GET /api/v1/users?search=siti
Authorization: Bearer <access_token>
```

#### Response `200 OK`
```json
{
  "data": [
    {
      "user_id": 2,
      "username": "siti_nur",
      "email": "siti@mail.com"
    },
    {
      "user_id": 5,
      "username": "siti_rahma",
      "email": "rahma.siti@mail.com"
    }
  ]
}
```

---

### 3.5 Ambil Profil Saya
Mengambil rincian profil milik user yang login. Endpoint ini dijamin selalu mengembalikan objek profil (tidak menghasilkan 404).

- **Method**: `GET`
- **Endpoint**: `/users/me/profile`
- **Auth**: Bearer Token

#### Response `200 OK`
```json
{
  "data": {
    "profile_id": 1,
    "user_id": 1,
    "first_name": "Budi",
    "last_name": "Santoso",
    "avatar_url": "https://cdn.example.com/avatars/budi.png",
    "phone_number": "+62812345678"
  }
}
```

---

### 3.6 Upsert Profil Saya
Membuat atau memperbarui (*upsert*) data profil (relasi 1:1).

- **Method**: `PUT`
- **Endpoint**: `/users/me/profile`
- **Auth**: Bearer Token

#### Aturan Validasi Input
| Field | Tipe | Wajib | Aturan |
| :--- | :--- | :--- | :--- |
| `first_name` | String | Tidak | Maks 255 karakter |
| `last_name` | String | Tidak | Maks 255 karakter |
| `avatar_url` | String | Tidak | Format URL valid |
| `phone_number`| String | Tidak | Maks 20 karakter |

#### Request Body
```json
{
  "first_name": "Budi",
  "last_name": "Pratama",
  "avatar_url": "https://cdn.example.com/avatars/budi-new.png",
  "phone_number": "+6281299998888"
}
```

#### Response `200 OK`
```json
{
  "data": {
    "profile_id": 1,
    "user_id": 1,
    "first_name": "Budi",
    "last_name": "Pratama",
    "avatar_url": "https://cdn.example.com/avatars/budi-new.png",
    "phone_number": "+6281299998888"
  }
}
```

---

### 3.7 Ambil Daftar Pengaturan User
Mengambil seluruh konfigurasi key-value yang tersimpan untuk user yang sedang login.

- **Method**: `GET`
- **Endpoint**: `/users/me/settings`
- **Auth**: Bearer Token

#### Response `200 OK`
```json
{
  "data": [
    {
      "setting_id": 1,
      "user_id": 1,
      "setting_key": "theme",
      "setting_value": "dark"
    },
    {
      "setting_id": 2,
      "user_id": 1,
      "setting_key": "language",
      "setting_value": "id"
    },
    {
      "setting_id": 3,
      "user_id": 1,
      "setting_key": "email_notifications",
      "setting_value": "true"
    }
  ]
}
```

---

### 3.8 Upsert Nilai Pengaturan
Menyimpan atau memperbarui nilai pengaturan. Operasi ini bersifat idempoten karena pasangan `(user_id, setting_key)` bersifat unik.

- **Method**: `PUT`
- **Endpoint**: `/users/me/settings/{setting_key}`
- **Auth**: Bearer Token

#### Path Parameters
| Parameter | Tipe | Keterangan |
| :--- | :--- | :--- |
| `setting_key` | String | Kunci pengaturan (contoh: `theme`, `email_notifications`, `timezone`) |

#### Request Body
```json
{
  "setting_value": "dark"
}
```

#### Response `200 OK`
```json
{
  "data": {
    "setting_id": 1,
    "user_id": 1,
    "setting_key": "theme",
    "setting_value": "dark"
  }
}
```

---

### 3.9 Hapus Pengaturan
Menghapus sebuah pengaturan berdasarkan `setting_key`.

- **Method**: `DELETE`
- **Endpoint**: `/users/me/settings/{setting_key}`
- **Auth**: Bearer Token

#### Path Parameters
| Parameter | Tipe | Keterangan |
| :--- | :--- | :--- |
| `setting_key` | String | Kunci pengaturan yang akan dihapus |

#### Contoh Request
```http
DELETE /api/v1/users/me/settings/theme
Authorization: Bearer <access_token>
```

#### Response `200 OK`
```json
{
  "data": "Pengaturan berhasil dihapus"
}
```

#### Response `404 NOT_FOUND` (Pengaturan tidak ditemukan)
```json
{
  "error": {
    "code": "NOT_FOUND",
    "message": "Pengaturan 'theme' tidak ditemukan",
    "details": null
  }
}
```

---

## 4. Tasks Core Module

Semua endpoint task membutuhkan header:
```http
Authorization: Bearer <access_token>
```

Aturan Hak Akses:
- **Owner** (`tasks.user_id = user_id`): Akses penuh (baca, ubah, hapus, arsip/unarsip, ubah sharing).
- **Shared Edit** (`permission = "edit"`): Baca, ubah status/prioritas/progress.
- **Shared Comment** (`permission = "comment"`): Baca dan komentar.
- **Shared View** (`permission = "view"`): Hanya membaca.

---

### 4.1 Daftar Task (List Tasks with Filter & Pagination)
Mengambil daftar task milik user atau yang dibagikan kepada user.

- **Method**: `GET`
- **Endpoint**: `/tasks`
- **Auth**: Bearer Token

#### Query Parameters
| Parameter | Tipe | Default | Keterangan |
| :--- | :--- | :--- | :--- |
| `scope` | String | `all` | `owned` (hanya milik sendiri), `shared` (hanya dibagikan), atau `all` |
| `archived` | Bool | `false` | `false` (task aktif), `true` (task diarsipkan) |
| `status_id` | Integer | - | Filter berdasarkan status ID |
| `priority_id` | Integer | - | Filter berdasarkan prioritas ID |
| `tag_id` | Integer | - | Filter berdasarkan tag ID |
| `due_before` | DateTime | - | Rentang due date sebelum (ISO 8601) |
| `due_after` | DateTime | - | Rentang due date setelah (ISO 8601) |
| `search` | String | - | Pencarian kata kunci pada judul & deskripsi |
| `sort` | String | `-task_id` | `due_date`, `-due_date`, `created_at`, `-created_at`, `priority`, `-priority` |
| `page` | Integer | `1` | Nomor halaman (mulai dari 1) |
| `per_page` | Integer | `20` | Jumlah data per halaman (1–100) |

#### Contoh Request
```http
GET /api/v1/tasks?status_id=1&search=laporan&page=1&per_page=10
Authorization: Bearer <access_token>
```

#### Response `200 OK`
```json
{
  "data": [
    {
      "task_id": 42,
      "user_id": 1,
      "title": "Siapkan laporan bulanan",
      "description": "Kumpulkan data penjualan Q3",
      "status_id": 1,
      "status": "pending",
      "priority_id": 3,
      "priority": "high",
      "progress_percentage": 40,
      "due_date": "2026-11-20T09:00:00Z",
      "archived_at": null,
      "created_at": "2026-11-16T14:00:00Z",
      "updated_at": "2026-11-16T14:30:00Z"
    }
  ],
  "meta": {
    "page": 1,
    "per_page": 10,
    "total": 1,
    "total_pages": 1
  }
}
```

---

### 4.2 Buat Task Baru
Membuat task baru. Pembuat otomatis menjadi owner task.

- **Method**: `POST`
- **Endpoint**: `/tasks`
- **Auth**: Bearer Token

#### Request Body
```json
{
  "title": "Siapkan laporan bulanan",
  "description": "Kumpulkan data penjualan Q3",
  "status_id": 1,
  "priority_id": 3,
  "progress_percentage": 0,
  "due_date": "2026-11-20T09:00:00Z",
  "tag_ids": [1, 2]
}
```

#### Response `201 CREATED`
```json
{
  "task_id": 42,
  "user_id": 1,
  "title": "Siapkan laporan bulanan",
  "description": "Kumpulkan data penjualan Q3",
  "status_id": 1,
  "status": "pending",
  "priority_id": 3,
  "priority": "high",
  "progress_percentage": 0,
  "due_date": "2026-11-20T09:00:00Z",
  "archived_at": null,
  "created_at": "2026-11-16T14:00:00Z",
  "updated_at": "2026-11-16T14:00:00Z"
}
```

---

### 4.3 Detail Task
Mengambil detail informasi task berdasarkan ID.

- **Method**: `GET`
- **Endpoint**: `/tasks/{task_id}`
- **Auth**: Bearer Token

#### Response `200 OK`
```json
{
  "data": {
    "task_id": 42,
    "user_id": 1,
    "title": "Siapkan laporan bulanan",
    "description": "Kumpulkan data penjualan Q3",
    "status_id": 1,
    "status": "pending",
    "priority_id": 3,
    "priority": "high",
    "progress_percentage": 0,
    "due_date": "2026-11-20T09:00:00Z",
    "archived_at": null,
    "created_at": "2026-11-16T14:00:00Z",
    "updated_at": "2026-11-16T14:00:00Z"
  }
}
```

---

### 4.4 Update Task (Partial Update)
Mengubah field pada task (`title`, `description`, `status_id`, `priority_id`, `due_date`).
Perubahan `status_id` otomatis mencatat riwayat ke `task_history`. Perubahan `priority_id` otomatis mencatat riwayat ke `task_priority_history`.

- **Method**: `PATCH`
- **Endpoint**: `/tasks/{task_id}`
- **Auth**: Bearer Token

#### Request Body
```json
{
  "status_id": 2,
  "priority_id": 4,
  "due_date": "2026-11-25T17:00:00Z"
}
```

#### Response `200 OK`
```json
{
  "data": {
    "task_id": 42,
    "user_id": 1,
    "title": "Siapkan laporan bulanan",
    "description": "Kumpulkan data penjualan Q3",
    "status_id": 2,
    "status": "in_progress",
    "priority_id": 4,
    "priority": "urgent",
    "progress_percentage": 0,
    "due_date": "2026-11-25T17:00:00Z",
    "archived_at": null,
    "created_at": "2026-11-16T14:00:00Z",
    "updated_at": "2026-11-16T14:40:00Z"
  }
}
```

---

### 4.5 Hapus Task
Menghapus task. Hanya pemilik (*owner*) yang dapat menghapus task. Seluruh relasi anak (subtask, notes, comments, reminders, dll.) terhapus otomatis via cascade.

- **Method**: `DELETE`
- **Endpoint**: `/tasks/{task_id}`
- **Auth**: Bearer Token (Owner saja)

#### Response `200 OK`
```json
{
  "data": "Task berhasil dihapus"
}
```

#### Response `403 FORBIDDEN` (Bila bukan owner)
```json
{
  "error": {
    "code": "FORBIDDEN",
    "message": "Hanya pemilik task yang dapat menghapus task",
    "details": null
  }
}
```

---

### 4.6 Arsipkan Task
Mengarsipkan task aktif (`archived_at` diisi timestamp saat ini).

- **Method**: `POST`
- **Endpoint**: `/tasks/{task_id}/archive`
- **Auth**: Bearer Token (Owner saja)

#### Response `200 OK`
```json
{
  "data": {
    "task_id": 42,
    "archived_at": "2026-11-16T14:45:00Z"
  }
}
```

---

### 4.7 Batalkan Arsip Task
Mengaktifkan kembali task yang diarsipkan (`archived_at` diset `null`).

- **Method**: `POST`
- **Endpoint**: `/tasks/{task_id}/unarchive`
- **Auth**: Bearer Token (Owner saja)

#### Response `200 OK`
```json
{
  "data": {
    "task_id": 42,
    "archived_at": null
  }
}
```

---

### 4.8 Update Progress Task
Mengubah persentase kemajuan task (0–100).

- **Method**: `PUT`
- **Endpoint**: `/tasks/{task_id}/progress`
- **Auth**: Bearer Token (Owner atau Edit)

#### Request Body
```json
{
  "progress_percentage": 75
}
```

#### Response `200 OK`
```json
{
  "data": {
    "task_id": 42,
    "progress_percentage": 75
  }
}
```

---

## 5. Data Referensi (Lookup Module)

Endpoint referensi bersifat public / read-only untuk mengambil data enumerasi dan lookup master.

### 5.1 Daftar Status Task
- **Method**: `GET`
- **Endpoint**: `/statuses`
- **Auth**: Public

#### Response `200 OK`
```json
{
  "data": [
    { "status_id": 1, "name": "pending" },
    { "status_id": 2, "name": "in_progress" },
    { "status_id": 3, "name": "completed" },
    { "status_id": 4, "name": "cancelled" }
  ]
}
```

---

### 5.2 Daftar Prioritas Task
- **Method**: `GET`
- **Endpoint**: `/priorities`
- **Auth**: Public

#### Response `200 OK`
```json
{
  "data": [
    { "priority_id": 1, "name": "low", "rank_order": 1 },
    { "priority_id": 2, "name": "medium", "rank_order": 2 },
    { "priority_id": 3, "name": "high", "rank_order": 3 },
    { "priority_id": 4, "name": "urgent", "rank_order": 4 }
  ]
}
```

---

### 5.3 Daftar External Apps
- **Method**: `GET`
- **Endpoint**: `/external-apps`
- **Auth**: Public

#### Response `200 OK`
```json
{
  "data": [
    { "app_id": 1, "name": "Trello" },
    { "app_id": 2, "name": "Jira" },
    { "app_id": 3, "name": "GitHub" },
    { "app_id": 4, "name": "Slack" }
  ]
}
```

---

### 5.4 Daftar Nilai Enum
- **Method**: `GET`
- **Endpoint**: `/enums`
- **Auth**: Public

#### Response `200 OK`
```json
{
  "data": {
    "connection_status": ["pending", "accepted", "blocked"],
    "frequency": ["daily", "weekly", "monthly", "yearly"],
    "notification_type": ["reminder", "comment", "share", "system"],
    "permission": ["view", "comment", "edit"],
    "vote_type": ["up", "down"]
  }
}
```

---

## 6. Subtasks Module

Subtask menggunakan `status_id` yang sama dengan referensi `task_statuses`.

| Method | Endpoint | Deskripsi |
| :--- | :--- | :--- |
| `GET` | `/tasks/{task_id}/subtasks` | Daftar subtask milik task |
| `POST` | `/tasks/{task_id}/subtasks` | Tambah subtask baru |
| `PATCH` | `/subtasks/{subtask_id}` | Ubah judul (`title`) atau status (`status_id`) |
| `DELETE` | `/subtasks/{subtask_id}` | Hapus subtask |

#### Request Body `POST /tasks/{task_id}/subtasks`
```json
{
  "title": "Kumpulkan bukti transaksi",
  "status_id": 1
}
```

---

## 7. Notes & Note History Module

Catatan internal pada task. Setiap perubahan teks note otomatis dicatat ke riwayat `task_notes_history`.

| Method | Endpoint | Deskripsi |
| :--- | :--- | :--- |
| `GET` | `/tasks/{task_id}/notes` | Daftar catatan pada task |
| `POST` | `/tasks/{task_id}/notes` | Tambah catatan baru |
| `PATCH` | `/notes/{note_id}` | Ubah teks catatan |
| `DELETE` | `/notes/{note_id}` | Hapus catatan |
| `GET` | `/notes/{note_id}/history` | Riwayat versi isi catatan sebelumnya |

#### Request Body `POST /tasks/{task_id}/notes`
```json
{
  "content": "Data penjualan ada di folder Google Drive Q3"
}
```

---

## 8. Comments Module

Komentar kolaborasi pada task (membutuhkan izin `comment` atau lebih tinggi).

| Method | Endpoint | Deskripsi |
| :--- | :--- | :--- |
| `GET` | `/tasks/{task_id}/comments` | Daftar komentar pada task |
| `POST` | `/tasks/{task_id}/comments` | Kirim komentar baru |
| `PATCH` | `/comments/{comment_id}` | Ubah isi komentar (penulis saja) |
| `DELETE` | `/comments/{comment_id}` | Hapus komentar (penulis atau task owner) |

#### Request Body `POST /tasks/{task_id}/comments`
```json
{
  "comment": "Sudah saya periksa, data valid."
}
```

---

## 9. Attachments Module

Lampiran dokumen atau file pendukung pada task. File fisik disimpan secara persisten di direktori server `./uploads/` dan dapat diakses publik melalui URL yang dihasilkan (`/uploads/{uuid}.{ext}`).

| Method | Endpoint | Deskripsi |
| :--- | :--- | :--- |
| `GET` | `/tasks/{task_id}/attachments` | Daftar lampiran file pada task |
| `POST` | `/tasks/{task_id}/attachments` | Unggah file lampiran (`multipart/form-data` atau JSON `file_url`) |
| `DELETE` | `/attachments/{attachment_id}` | Hapus lampiran |

#### Contoh 1: Unggah File Fisik (`multipart/form-data`)
```bash
curl -X POST http://localhost:8080/api/v1/tasks/1/attachments \
  -H "Authorization: Bearer <access_token>" \
  -F "file=@/path/to/dokumen.pdf"
```

#### Contoh 2: Daftarkan URL File Eksternal (`application/json`)
```json
{
  "file_url": "https://cdn.example.com/files/laporan.pdf",
  "file_type": "application/pdf"
}
```

#### Response `201 CREATED`
```json
{
  "data": {
    "attachment_id": 5,
    "task_id": 1,
    "file_url": "/uploads/550e8400-e29b-41d4-a716-446655440000.pdf",
    "file_type": "application/pdf",
    "uploaded_at": "2026-11-17T00:10:00Z"
  }
}
```

---

## 10. Reminders Module

Pengingat waktu untuk task. **Background Worker** aplikasi secara berkala memantau tabel ini (interval 30 detik); saat `reminder_time` tercapai, worker otomatis membuat notifikasi di `user_notifications` dan menandai `notification_sent = true`.

| Method | Endpoint | Deskripsi |
| :--- | :--- | :--- |
| `GET` | `/tasks/{task_id}/reminders` | Daftar pengingat task |
| `POST` | `/tasks/{task_id}/reminders` | Pasang pengingat baru |
| `PATCH` | `/reminders/{reminder_id}` | Ubah waktu pengingat (`reminder_time`) |
| `DELETE` | `/reminders/{reminder_id}` | Hapus pengingat |

#### Request Body `POST /tasks/{task_id}/reminders`
```json
{
  "reminder_time": "2026-11-20T08:00:00Z"
}
```

---

## 11. Recurring Tasks Module

Aturan pengulangan task berulang (relasi 1:1). **Background Worker** aplikasi memantau `next_due_date`; saat jatuh tempo, worker otomatis membuat instansi task baru, menduplikasi tag yang terpasang, mencatat log ke `activity_logs`, mengirim notifikasi ke user, dan memajukan `next_due_date` sesuai frekuensi (`daily`, `weekly`, `monthly`, `yearly`).

| Method | Endpoint | Deskripsi |
| :--- | :--- | :--- |
| `GET` | `/tasks/{task_id}/recurrence` | Ambil aturan perulangan task |
| `PUT` | `/tasks/{task_id}/recurrence` | Buat atau ubah aturan perulangan |
| `DELETE` | `/tasks/{task_id}/recurrence` | Hapus aturan perulangan |

#### Request Body `PUT /tasks/{task_id}/recurrence`
```json
{
  "frequency": "weekly",
  "next_due_date": "2026-11-27T09:00:00Z"
}
```
*(Nilai `frequency`: `daily`, `weekly`, `monthly`, `yearly`)*.

---

## 12. Locations Module

Master data lokasi koordinat dan relasi lokasinya dengan task.

| Method | Endpoint | Deskripsi |
| :--- | :--- | :--- |
| `GET` | `/locations` | Daftar master lokasi |
| `POST` | `/locations` | Buat lokasi (idempoten berdasarkan lat/long) |
| `GET` | `/tasks/{task_id}/locations` | Daftar lokasi milik task |
| `PUT` | `/tasks/{task_id}/locations/{location_id}` | Tautkan lokasi ke task |
| `DELETE` | `/tasks/{task_id}/locations/{location_id}` | Lepas tautan lokasi dari task |

#### Request Body `POST /locations`
```json
{
  "location_name": "Kantor Pusat Makassar",
  "latitude": -5.1477,
  "longitude": 119.4327
}
```

---

## 13. Sharing & Collaboration Module

Pengelolaan kolaborasi dan hak akses task kepada user lain (hanya owner task yang dapat mengelola).

| Method | Endpoint | Deskripsi |
| :--- | :--- | :--- |
| `GET` | `/tasks/{task_id}/shares` | Daftar user yang diberi izin akses |
| `PUT` | `/tasks/{task_id}/shares/{user_id}` | Beri atau ubah level izin (upsert) |
| `DELETE` | `/tasks/{task_id}/shares/{user_id}` | Cabut izin akses user |

#### Request Body `PUT /tasks/{task_id}/shares/{user_id}`
```json
{
  "permission": "edit"
}
```
*(Nilai `permission`: `view`, `comment`, `edit`)*.

---

## 14. Votes Module

Sistem voting komunitas/tim pada task. Satu user hanya memiliki satu vote per task (`up` atau `down`).

| Method | Endpoint | Deskripsi |
| :--- | :--- | :--- |
| `GET` | `/tasks/{task_id}/votes` | Ringkasan vote (`up`, `down`, `my_vote`) |
| `PUT` | `/tasks/{task_id}/vote` | Beri / perbarui vote user login |
| `DELETE` | `/tasks/{task_id}/vote` | Hapus vote user login |

#### Request Body `PUT /tasks/{task_id}/vote`
```json
{
  "vote_type": "up"
}
```

---

## 15. Klasifikasi: Tags, Categories, Labels

Manajemen klasifikasi dan penautannya ke task.

### 15.1 Tags
- `GET /tags` — Daftar tag
- `POST /tags` — Buat tag baru (`name` unik)
- `PATCH /tags/{tag_id}` — Ubah nama tag
- `DELETE /tags/{tag_id}?force=true` — Hapus tag (gunakan `?force=true` jika masih ditautkan ke task)
- `PUT /tasks/{task_id}/tags/{tag_id}` — Pasang tag ke task
- `DELETE /tasks/{task_id}/tags/{tag_id}` — Lepas tag dari task

### 15.2 Categories
- `GET /categories` — Daftar kategori
- `POST /categories` — Buat kategori (`name`, `description`)
- `PATCH /categories/{category_id}` — Ubah kategori
- `DELETE /categories/{category_id}?force=true` — Hapus kategori
- `PUT /tasks/{task_id}/categories/{category_id}` — Pasang kategori ke task
- `DELETE /tasks/{task_id}/categories/{category_id}` — Lepas kategori dari task

### 15.3 Labels
- `GET /labels` — Daftar label
- `POST /labels` — Buat label (`name` unik)
- `PATCH /labels/{label_id}` — Ubah nama label
- `DELETE /labels/{label_id}?force=true` — Hapus label
- `PUT /tasks/{task_id}/labels/{label_id}` — Pasang label ke task (otomatis mencatat ke `task_labels_history`)
- `DELETE /tasks/{task_id}/labels/{label_id}` — Lepas label dari task (otomatis mencatat ke `task_labels_history`)

---

## 16. Audit History Module (Read-only)

Riwayat perubahan status, prioritas, dan label dicatat otomatis oleh server dan bersifat read-only.

| Method | Endpoint | Deskripsi |
| :--- | :--- | :--- |
| `GET` | `/tasks/{task_id}/history/status` | Riwayat perubahan status (`task_history`) |
| `GET` | `/tasks/{task_id}/history/priority` | Riwayat perubahan prioritas (`task_priority_history`) |
| `GET` | `/tasks/{task_id}/history/labels` | Riwayat pemasangan & pelepasan label (`task_labels_history`) |

---

## 17. Integrasi Eksternal Module

Menghubungkan task lokal dengan task pada aplikasi eksternal (Trello, Jira, GitHub, Slack).

| Method | Endpoint | Deskripsi |
| :--- | :--- | :--- |
| `GET` | `/tasks/{task_id}/integrations` | Daftar integrasi task |
| `POST` | `/tasks/{task_id}/integrations` | Hubungkan task ke aplikasi lain |
| `DELETE` | `/integrations/{integration_id}` | Putuskan integrasi |

#### Request Body `POST /tasks/{task_id}/integrations`
```json
{
  "app_id": 1,
  "external_app_task_id": "TRELLO-9812"
}
```

---

## 18. Koneksi Antar-User Module

Hubungan pertemanan/koneksi antar pengguna.

| Method | Endpoint | Deskripsi |
| :--- | :--- | :--- |
| `GET` | `/connections?status=` | Daftar koneksi user (`pending`, `accepted`, `blocked`) |
| `POST` | `/connections` | Kirim permintaan koneksi baru |
| `PATCH` | `/connections/{connection_id}` | Terima (`accepted`) atau blokir (`blocked`) |
| `DELETE` | `/connections/{connection_id}` | Hapus koneksi |

#### Request Body `POST /connections`
```json
{
  "target_user_id": 7
}
```

---

## 19. Notifikasi Module

Notifikasi sistem, pengingat, komentar, dan sharing. Dibuat otomatis oleh sistem.

| Method | Endpoint | Deskripsi |
| :--- | :--- | :--- |
| `GET` | `/notifications?is_read=` | Daftar notifikasi user login |
| `GET` | `/notifications/unread-count` | Jumlah notifikasi belum dibaca |
| `PATCH` | `/notifications/{notification_id}` | Tandai sudah dibaca (`{"is_read": true}`) |
| `POST` | `/notifications/read-all` | Tandai seluruh notifikasi sudah dibaca |
| `DELETE` | `/notifications/{notification_id}` | Hapus notifikasi |

---

## 20. Activity Logs Module (Read-only)

Log riwayat audit aktivitas user login (misal: `task.create`, `task.update`, `task.delete`).

- **Method**: `GET`
- **Endpoint**: `/activity-logs`
- **Auth**: Bearer Token
- **Query Parameters**: `entity_type`, `entity_id`, `action`, `from`, `to`, `page`, `per_page`.

#### Response `200 OK`
```json
{
  "data": [
    {
      "log_id": 101,
      "user_id": 1,
      "action": "task.create",
      "entity_type": "task",
      "entity_id": 42,
      "timestamp": "2026-11-16T14:00:00Z"
    }
  ],
  "meta": {
    "page": 1,
    "per_page": 20,
    "total": 1,
    "total_pages": 1
  }
}
```

---

## 21. Contoh Alur Lengkap (End-to-End Walkthrough)

### Langkah 1: Registrasi User Baru
```bash
curl -X POST http://localhost:8080/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "budi",
    "email": "budi@mail.com",
    "password": "Password123!",
    "first_name": "Budi",
    "last_name": "Santoso",
    "phone_number": "+62812345678"
  }'
```

### Langkah 2: Login untuk Memperoleh Token
```bash
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "budi@mail.com",
    "password": "Password123!"
  }'
```
Simpan `access_token` dari respons JSON.

### Langkah 3: Ambil Data Akun & Profil Lengkap
```bash
curl -X GET http://localhost:8080/api/v1/users/me \
  -H "Authorization: Bearer <access_token>"
```

### Langkah 4: Set Pengaturan Tema
```bash
curl -X PUT http://localhost:8080/api/v1/users/me/settings/theme \
  -H "Authorization: Bearer <access_token>" \
  -H "Content-Type: application/json" \
  -d '{ "setting_value": "dark" }'
```

### Langkah 5: Cari User Lain
```bash
curl -X GET "http://localhost:8080/api/v1/users?search=siti" \
  -H "Authorization: Bearer <access_token>"
```

### Langkah 6: Logout
```bash
curl -X POST http://localhost:8080/api/v1/auth/logout \
  -H "Authorization: Bearer <access_token>" \
  -H "Content-Type: application/json" \
  -d '{ "refresh_token": "<refresh_token>" }'
```
