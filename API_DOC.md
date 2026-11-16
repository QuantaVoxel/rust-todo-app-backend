# Todo App REST API Documentation (v1)

Dokumentasi resmi endpoint API untuk **Auth Module** dan **User Module (Users, Profile, Settings)** pada Todo App v2 backend yang dibangun menggunakan framework **Salvo** dan ORM **SeaORM 2.0**.

---

## 1. Konvensi Umum

### Base URL
```
http://localhost:8080/api/v1
```

### Format Data & Header
- **Request & Response Body**: `application/json`
- **Autentikasi**: Header `Authorization: Bearer <access_token>` (wajib untuk endpoint terproteksi)
- **Format Tanggal/Waktu**: ISO 8601 UTC (`2026-11-16T14:25:00Z`)
- **Penamaan Field**: `snake_case`

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

## 4. Contoh Alur Lengkap (End-to-End Walkthrough)

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
