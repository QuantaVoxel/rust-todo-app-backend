# Todo App API Documentation

Versi 1.0 · REST API · Berdasarkan skema database ternormalisasi (`todo_app_normalized.dbml`)

## 1. Pendahuluan

API ini mengelola task, subtask, catatan, komentar, lampiran, pengingat, berbagi task, voting, klasifikasi (tag, kategori, label), riwayat perubahan, koneksi antar-user, dan notifikasi.

### Konvensi umum

| Item | Aturan |
| --- | --- |
| Base URL | `https://api.example.com/api/v1` |
| Format | JSON (`Content-Type: application/json`) |
| Autentikasi | `Authorization: Bearer <access_token>` (kecuali endpoint auth) |
| Waktu | ISO 8601 UTC, contoh `2026-11-20T09:00:00Z` |
| Penamaan field | `snake_case`, sama dengan nama kolom database |
| ID | Integer |

### Pagination

Endpoint list mendukung `page` (default 1) dan `per_page` (default 20, maks 100).

```json
{
  "data": [],
  "meta": { "page": 1, "per_page": 20, "total": 135, "total_pages": 7 }
}
```

### Format error

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "title wajib diisi",
    "details": [{ "field": "title", "issue": "required" }]
  }
}
```

| HTTP | Code | Arti |
| --- | --- | --- |
| 400 | `VALIDATION_ERROR` | Input tidak valid |
| 401 | `UNAUTHORIZED` | Token tidak ada / kedaluwarsa |
| 403 | `FORBIDDEN` | Tidak punya izin atas resource |
| 404 | `NOT_FOUND` | Resource tidak ada |
| 409 | `CONFLICT` | Duplikat (email, vote, tag, dll.) |
| 429 | `RATE_LIMITED` | Terlalu banyak request |
| 500 | `INTERNAL_ERROR` | Kesalahan server |

### Aturan akses task

Sebuah task bisa diakses oleh **owner** (`tasks.user_id`) atau user yang ada di `Task_Sharing`. Level izin: `view` (baca), `comment` (baca + komentar), `edit` (baca + komentar + ubah). Hanya owner yang boleh menghapus task, mengubah sharing, dan mengarsipkan.

## 2. Autentikasi

| Method | Endpoint | Deskripsi |
| --- | --- | --- |
| POST | `/auth/register` | Daftar akun baru |
| POST | `/auth/login` | Login, mengembalikan token |
| POST | `/auth/refresh` | Perbarui access token |
| POST | `/auth/logout` | Cabut refresh token |
| POST | `/auth/change-password` | Ganti password |

**POST /auth/register**

```json
{ "username": "budi", "email": "budi@mail.com", "password": "min8karakter" }
```

Response `201`:

```json
{ "user_id": 1, "username": "budi", "email": "budi@mail.com", "created_at": "2026-11-16T08:00:00Z" }
```

Error: `409 CONFLICT` jika email atau username sudah dipakai. Password disimpan sebagai `password_hash` dan tidak pernah dikembalikan di response.

**POST /auth/login**

```json
{ "email": "budi@mail.com", "password": "min8karakter" }
```

Response `200`:

```json
{ "access_token": "eyJ...", "refresh_token": "eyJ...", "expires_in": 3600, "user": { "user_id": 1, "username": "budi" } }
```

## 3. Users, Profile, Settings

| Method | Endpoint | Deskripsi |
| --- | --- | --- |
| GET | `/users/me` | Data user login (termasuk profil) |
| PATCH | `/users/me` | Ubah `username` / `email` |
| DELETE | `/users/me` | Hapus akun beserta datanya |
| GET | `/users?search=` | Cari user lain (username/email), untuk sharing & koneksi |
| GET | `/users/me/profile` | Ambil profil |
| PUT | `/users/me/profile` | Buat/ubah profil (relasi 1:1) |
| GET | `/users/me/settings` | Daftar pengaturan |
| PUT | `/users/me/settings/{setting_key}` | Set nilai pengaturan (upsert) |
| DELETE | `/users/me/settings/{setting_key}` | Hapus pengaturan |

**PUT /users/me/profile**

```json
{ "first_name": "Budi", "last_name": "Santoso", "avatar_url": "https://cdn.example.com/a.png", "phone_number": "+6281234567" }
```

**PUT /users/me/settings/theme**

```json
{ "setting_value": "dark" }
```

Kombinasi `(user_id, setting_key)` unik, sehingga PUT bersifat idempoten.

## 4. Data Referensi (Lookup)

| Method | Endpoint | Deskripsi |
| --- | --- | --- |
| GET | `/statuses` | Daftar `Task_Statuses` |
| GET | `/priorities` | Daftar `Task_Priorities` (urut `rank_order`) |
| GET | `/external-apps` | Daftar `External_Apps` |
| GET | `/enums` | Nilai enum: `vote_type`, `frequency`, `permission`, `connection_status`, `notification_type` |

Contoh response `GET /priorities`:

```json
{ "data": [ { "priority_id": 1, "name": "low", "rank_order": 1 }, { "priority_id": 2, "name": "high", "rank_order": 2 } ] }
```

## 5. Tasks

### Objek Task

```json
{
  "task_id": 42,
  "user_id": 1,
  "title": "Siapkan laporan bulanan",
  "description": "Kumpulkan data penjualan",
  "status_id": 2,
  "status": "in_progress",
  "priority_id": 3,
  "priority": "high",
  "progress_percentage": 40,
  "due_date": "2026-11-20T09:00:00Z",
  "archived_at": null,
  "created_at": "2026-11-16T08:00:00Z",
  "updated_at": "2026-11-16T08:30:00Z"
}
```

### Endpoint

| Method | Endpoint | Deskripsi |
| --- | --- | --- |
| GET | `/tasks` | List task milik + dibagikan ke user |
| POST | `/tasks` | Buat task |
| GET | `/tasks/{task_id}` | Detail task |
| PATCH | `/tasks/{task_id}` | Ubah sebagian field |
| DELETE | `/tasks/{task_id}` | Hapus task (owner saja) |
| POST | `/tasks/{task_id}/archive` | Arsipkan (set `archived_at`) |
| POST | `/tasks/{task_id}/unarchive` | Batalkan arsip (`archived_at = null`) |
| PUT | `/tasks/{task_id}/progress` | Ubah persentase progress |

### Query parameter GET /tasks

| Parameter | Tipe | Deskripsi |
| --- | --- | --- |
| `status_id` | int | Filter status |
| `priority_id` | int | Filter prioritas |
| `tag_id` / `category_id` / `label_id` | int | Filter klasifikasi |
| `due_before` / `due_after` | datetime | Rentang due date |
| `archived` | bool | `false` (default) hanya task aktif, `true` hanya arsip |
| `scope` | string | `owned`, `shared`, atau `all` (default) |
| `search` | string | Cari di `title` dan `description` |
| `sort` | string | `due_date`, `created_at`, `priority`, `-` di depan untuk descending |
| `include` | string | Pisahkan koma: `subtasks,tags,labels,categories,reminders,location` |

### POST /tasks

Request:

```json
{
  "title": "Siapkan laporan bulanan",
  "description": "Kumpulkan data penjualan",
  "status_id": 1,
  "priority_id": 3,
  "due_date": "2026-11-20T09:00:00Z",
  "tag_ids": [1, 4],
  "category_ids": [2],
  "label_ids": [5]
}
```

Aturan validasi: `title` wajib (maks 255), `status_id` wajib dan harus ada di `Task_Statuses`, `priority_id` opsional, `progress_percentage` default 0 dan harus 0–100. `user_id` diambil dari token, bukan dari body.

Response `201`: objek Task lengkap.

### PATCH /tasks/{task\_id}

Field yang boleh diubah: `title`, `description`, `status_id`, `priority_id`, `due_date`. Saat `status_id` berubah, server otomatis menulis baris ke `Task_History`. Saat `priority_id` berubah, server menulis ke `Task_Priority_History`. `updated_at` diperbarui otomatis.

```json
{ "status_id": 3 }
```

### PUT /tasks/{task\_id}/progress

```json
{ "progress_percentage": 100 }
```

Error `400` bila nilai di luar 0–100.

## 6. Subtasks

| Method | Endpoint | Deskripsi |
| --- | --- | --- |
| GET | `/tasks/{task_id}/subtasks` | Daftar subtask |
| POST | `/tasks/{task_id}/subtasks` | Buat subtask |
| PATCH | `/subtasks/{subtask_id}` | Ubah `title` / `status_id` |
| DELETE | `/subtasks/{subtask_id}` | Hapus subtask |

```json
{ "title": "Ambil data Oktober", "status_id": 1 }
```

Subtask memakai `Task_Statuses` yang sama dengan task induk.

## 7. Notes

| Method | Endpoint | Deskripsi |
| --- | --- | --- |
| GET | `/tasks/{task_id}/notes` | Daftar catatan |
| POST | `/tasks/{task_id}/notes` | Tambah catatan |
| PATCH | `/notes/{note_id}` | Ubah isi catatan (otomatis menulis `Task_Notes_History`) |
| DELETE | `/notes/{note_id}` | Hapus catatan |
| GET | `/notes/{note_id}/history` | Riwayat perubahan isi catatan |

```json
{ "content": "Data penjualan ada di folder Q3" }
```

Item history:

```json
{ "history_id": 7, "note_id": 3, "old_content": "...", "new_content": "...", "updated_at": "2026-11-16T09:00:00Z" }
```

## 8. Comments

| Method | Endpoint | Deskripsi |
| --- | --- | --- |
| GET | `/tasks/{task_id}/comments` | Daftar komentar (terbaru di bawah) |
| POST | `/tasks/{task_id}/comments` | Tambah komentar (butuh izin `comment` atau lebih) |
| PATCH | `/comments/{comment_id}` | Ubah komentar (penulis saja) |
| DELETE | `/comments/{comment_id}` | Hapus komentar (penulis atau owner task) |

```json
{ "comment": "Sudah saya cek, aman." }
```

Response menyertakan `user_id` penulis dan `created_at`.

## 9. Attachments

| Method | Endpoint | Deskripsi |
| --- | --- | --- |
| GET | `/tasks/{task_id}/attachments` | Daftar lampiran |
| POST | `/tasks/{task_id}/attachments` | Unggah lampiran (`multipart/form-data`, field `file`) |
| DELETE | `/attachments/{attachment_id}` | Hapus lampiran |

Response `201`:

```json
{ "attachment_id": 9, "task_id": 42, "file_url": "https://cdn.example.com/f/abc.pdf", "file_type": "application/pdf", "uploaded_at": "2026-11-16T09:10:00Z" }
```

Batas ukuran file ditentukan server (disarankan 10 MB). `file_type` diisi otomatis dari MIME type.

## 10. Reminders

| Method | Endpoint | Deskripsi |
| --- | --- | --- |
| GET | `/tasks/{task_id}/reminders` | Daftar pengingat |
| POST | `/tasks/{task_id}/reminders` | Buat pengingat |
| PATCH | `/reminders/{reminder_id}` | Ubah `reminder_time` |
| DELETE | `/reminders/{reminder_id}` | Hapus pengingat |

```json
{ "reminder_time": "2026-11-19T08:00:00Z" }
```

`notification_sent` default `false` dan hanya diubah oleh sistem saat notifikasi terkirim. Waktu pengingat harus di masa depan.

## 11. Recurring Tasks

Satu task hanya punya satu aturan pengulangan (relasi 1:1).

| Method | Endpoint | Deskripsi |
| --- | --- | --- |
| GET | `/tasks/{task_id}/recurrence` | Ambil aturan |
| PUT | `/tasks/{task_id}/recurrence` | Buat / ubah aturan |
| DELETE | `/tasks/{task_id}/recurrence` | Hapus aturan |

```json
{ "frequency": "weekly", "next_due_date": "2026-11-23T09:00:00Z" }
```

Nilai `frequency`: `daily`, `weekly`, `monthly`, `yearly`.

## 12. Locations

Lokasi disimpan sekali di `Locations`, lalu dihubungkan ke task lewat `Task_Location`.

| Method | Endpoint | Deskripsi |
| --- | --- | --- |
| GET | `/locations` | Daftar lokasi |
| POST | `/locations` | Buat lokasi (jika koordinat sama, mengembalikan yang sudah ada) |
| GET | `/tasks/{task_id}/locations` | Lokasi milik task |
| PUT | `/tasks/{task_id}/locations/{location_id}` | Hubungkan lokasi ke task |
| DELETE | `/tasks/{task_id}/locations/{location_id}` | Lepas lokasi dari task |

```json
{ "location_name": "Kantor Makassar", "latitude": -5.1477, "longitude": 119.4327 }
```

## 13. Sharing

Hanya owner task yang dapat mengelola sharing.

| Method | Endpoint | Deskripsi |
| --- | --- | --- |
| GET | `/tasks/{task_id}/shares` | Daftar user yang diberi akses |
| PUT | `/tasks/{task_id}/shares/{user_id}` | Beri / ubah izin (upsert) |
| DELETE | `/tasks/{task_id}/shares/{user_id}` | Cabut akses |

```json
{ "permission": "edit" }
```

Nilai `permission`: `view`, `comment`, `edit`. Memberi izin ke diri sendiri menghasilkan `400`. User yang diberi akses menerima notifikasi bertipe `share`.

## 14. Votes

Satu user hanya boleh punya satu vote per task.

| Method | Endpoint | Deskripsi |
| --- | --- | --- |
| GET | `/tasks/{task_id}/votes` | Ringkasan vote |
| PUT | `/tasks/{task_id}/vote` | Beri / ubah vote user login |
| DELETE | `/tasks/{task_id}/vote` | Hapus vote user login |

```json
{ "vote_type": "up" }
```

Response `GET /tasks/{task_id}/votes`:

```json
{ "up": 5, "down": 1, "my_vote": "up" }
```

## 15. Klasifikasi: Tags, Categories, Labels

Tiga sistem ini punya pola endpoint yang sama.

### Tags

| Method | Endpoint | Deskripsi |
| --- | --- | --- |
| GET | `/tags` | Daftar tag |
| POST | `/tags` | Buat tag (`name` unik) |
| PATCH | `/tags/{tag_id}` | Ubah nama |
| DELETE | `/tags/{tag_id}` | Hapus tag |
| PUT | `/tasks/{task_id}/tags/{tag_id}` | Pasang tag ke task |
| DELETE | `/tasks/{task_id}/tags/{tag_id}` | Lepas tag dari task |

### Categories

| Method | Endpoint | Deskripsi |
| --- | --- | --- |
| GET | `/categories` | Daftar kategori |
| POST | `/categories` | Buat kategori (`name`, `description`) |
| PATCH | `/categories/{category_id}` | Ubah kategori |
| DELETE | `/categories/{category_id}` | Hapus kategori |
| PUT | `/tasks/{task_id}/categories/{category_id}` | Pasang ke task |
| DELETE | `/tasks/{task_id}/categories/{category_id}` | Lepas dari task |

### Labels

| Method | Endpoint | Deskripsi |
| --- | --- | --- |
| GET | `/labels` | Daftar label |
| POST | `/labels` | Buat label (`name` unik) |
| PATCH | `/labels/{label_id}` | Ubah nama |
| DELETE | `/labels/{label_id}` | Hapus label |
| PUT | `/tasks/{task_id}/labels/{label_id}` | Pasang label (menulis `Task_Labels_History`) |
| DELETE | `/tasks/{task_id}/labels/{label_id}` | Lepas label (menulis `Task_Labels_History`) |

Contoh body POST `/tags`:

```json
{ "name": "urgent" }
```

Pasangan `PUT` bersifat idempoten karena tabel penghubung memakai composite primary key. Menghapus tag/kategori/label yang sedang dipakai mengembalikan `409` kecuali memakai `?force=true`, yang juga melepas semua asosiasinya.

## 16. Riwayat Perubahan (Read-only)

Riwayat ditulis otomatis oleh server dan tidak dapat diubah lewat API.

| Method | Endpoint | Deskripsi |
| --- | --- | --- |
| GET | `/tasks/{task_id}/history/status` | Riwayat `Task_History` |
| GET | `/tasks/{task_id}/history/priority` | Riwayat `Task_Priority_History` |
| GET | `/tasks/{task_id}/history/labels` | Riwayat `Task_Labels_History` |

Contoh item riwayat status:

```json
{ "history_id": 12, "task_id": 42, "old_status_id": 1, "new_status_id": 2, "changed_at": "2026-11-16T08:30:00Z" }
```

## 17. Integrasi Eksternal

| Method | Endpoint | Deskripsi |
| --- | --- | --- |
| GET | `/tasks/{task_id}/integrations` | Daftar integrasi task |
| POST | `/tasks/{task_id}/integrations` | Hubungkan ke task aplikasi lain |
| DELETE | `/integrations/{integration_id}` | Putuskan integrasi |

```json
{ "app_id": 2, "external_app_task_id": "TRELLO-8841" }
```

Pasangan `(app_id, external_app_task_id)` unik; duplikat menghasilkan `409`.

## 18. Koneksi Antar-User

| Method | Endpoint | Deskripsi |
| --- | --- | --- |
| GET | `/connections?status=` | Daftar koneksi user login |
| POST | `/connections` | Kirim permintaan koneksi |
| PATCH | `/connections/{connection_id}` | Terima atau blokir (penerima saja) |
| DELETE | `/connections/{connection_id}` | Hapus koneksi |

```json
{ "target_user_id": 7 }
```

Server menyimpan pasangan terurut (`user_id_1 < user_id_2`) sehingga hubungan A–B dan B–A tidak bisa duplikat. Nilai `status`: `pending`, `accepted`, `blocked`.

## 19. Notifikasi

| Method | Endpoint | Deskripsi |
| --- | --- | --- |
| GET | `/notifications?is_read=` | Daftar notifikasi user login |
| GET | `/notifications/unread-count` | Jumlah belum dibaca |
| PATCH | `/notifications/{notification_id}` | Tandai dibaca (`{"is_read": true}`) |
| POST | `/notifications/read-all` | Tandai semua sudah dibaca |
| DELETE | `/notifications/{notification_id}` | Hapus notifikasi |

Objek notifikasi:

```json
{ "notification_id": 31, "message": "Task 'Laporan' dibagikan kepada Anda", "notification_type": "share", "is_read": false, "created_at": "2026-11-16T09:20:00Z" }
```

Nilai `notification_type`: `reminder`, `comment`, `share`, `system`. Notifikasi dibuat oleh sistem, bukan lewat POST dari klien.

## 20. Activity Logs

| Method | Endpoint | Deskripsi |
| --- | --- | --- |
| GET | `/activity-logs` | Log aktivitas user login |

Parameter: `entity_type`, `entity_id`, `action`, `from`, `to`, serta pagination.

```json
{ "log_id": 501, "user_id": 1, "action": "task.update", "entity_type": "task", "entity_id": 42, "timestamp": "2026-11-16T08:30:00Z" }
```

Log bersifat read-only dan ditulis otomatis. Kolom `entity_type` + `entity_id` bersifat polymorphic (tanpa foreign key), jadi klien perlu menangani kasus entitas yang sudah terhapus.

## 21. Aturan Penghapusan (Cascade)

Menghapus sebuah task ikut menghapus: subtask, notes (beserta history-nya), comments, attachments, reminders, recurrence, asosiasi lokasi/tag/kategori/label, sharing, votes, integrasi, dan riwayat. Disarankan menetapkan `ON DELETE CASCADE` pada FK anak tersebut. Menghapus user menghapus seluruh task miliknya, profil, pengaturan, koneksi, dan notifikasinya.

## 22. Rate Limit & Keamanan

| Aspek | Rekomendasi |
| --- | --- |
| Rate limit | 100 request/menit per user; header `X-RateLimit-Remaining` |
| Token | Access token 1 jam, refresh token 30 hari |
| Password | Hash dengan bcrypt/argon2, tidak pernah dikembalikan |
| Otorisasi | Selalu cek kepemilikan/izin per task di server |
| Transport | HTTPS wajib |

## 23. Ringkasan Endpoint

| Grup | Jumlah endpoint |
| --- | --- |
| Auth | 5 |
| Users / Profile / Settings | 9 |
| Lookup | 4 |
| Tasks | 8 |
| Subtasks | 4 |
| Notes | 5 |
| Comments | 4 |
| Attachments | 3 |
| Reminders | 4 |
| Recurring | 3 |
| Locations | 5 |
| Sharing | 3 |
| Votes | 3 |
| Tags / Categories / Labels | 18 |
| History | 3 |
| Integrasi | 3 |
| Koneksi | 4 |
| Notifikasi | 5 |
| Activity logs | 1 |
