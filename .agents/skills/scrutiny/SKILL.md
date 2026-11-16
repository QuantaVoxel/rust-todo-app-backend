---
name: scrutiny
description: Comprehensive guide for request and data validation using the scrutiny crate in Rust and Salvo. Covers derive macros, 50+ validation rules, custom error messages, conditional validation, nested structs, deserialization integration, and mapping to API standard error responses.
version: 1.0.0
tags: [validation, scrutiny, salvo, input-validation, error-handling]
---

# Scrutiny Validation Guide

`scrutiny` is a high-performance, Laravel-inspired validation library for Rust that leverages `#[derive(Validate)]` and compile-time type introspection. It eliminates runtime string-rule parsing and provides standards-compliant format checks (RFC 5321 for email, WHATWG for URLs, RFC 4122 for UUIDs, ISO 8601 for dates).

---

## 1. Quick Setup & Imports

In `Cargo.toml`:
```toml
[dependencies]
scrutiny = "0.1.2"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

In your Rust module:
```rust
use scrutiny::Validate;
use scrutiny::traits::Validate as _; // Needed for .validate() method call
use scrutiny::error::{ValidationError, ValidationErrors};
use serde::Deserialize;
```

---

## 2. Basic Example

```rust
use scrutiny::Validate;
use scrutiny::traits::Validate as _;
use serde::Deserialize;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserDto {
    #[validate(required, email, bail)]
    pub email: Option<String>,

    #[validate(required, min = 3, max = 50)]
    pub username: Option<String>,

    #[validate(required, min = 8, confirmed)]
    pub password: Option<String>,

    pub password_confirmation: Option<String>,

    #[validate(nullable, url)]
    pub website: Option<String>,

    #[validate(required, in_list("user", "admin"))]
    pub role: Option<String>,
}

// Executing validation:
let dto: CreateUserDto = ...;
match dto.validate() {
    Ok(()) => println!("Valid!"),
    Err(errors) => {
        // errors is of type ValidationErrors
        for (field, err_list) in errors.field_errors() {
            for err in err_list {
                println!("{}: {}", field, err.message);
            }
        }
    }
}
```

---

## 3. Comprehensive Rule Reference

### Presence & Control
| Rule | Usage | Description |
|---|---|---|
| `required` | `#[validate(required)]` | Field must be `Some(...)` and non-empty. |
| `filled` | `#[validate(filled)]` | If present (`Some`), must not be empty. |
| `nullable` | `#[validate(nullable, email)]` | If `None`, subsequent checks pass. |
| `sometimes` | `#[validate(sometimes, email)]` | Skip rules if field is absent. |
| `bail` | `#[validate(required, email, bail)]` | Stop evaluating further rules for this field on first error. |
| `prohibited` | `#[validate(prohibited)]` | Field must NOT be present or must be empty. |
| `prohibited_if` | `#[validate(prohibited_if(field = "role", value = "user"))]` | Prohibited when condition matches. |
| `prohibited_unless` | `#[validate(prohibited_unless(field = "role", value = "admin"))]` | Prohibited unless condition matches. |

### Types & Standard Formats
| Rule | Usage | Standards / Notes |
|---|---|---|
| `email` | `#[validate(email)]` | RFC 5321 (via `email_address` parser, no fake regex). |
| `url` | `#[validate(url)]` | WHATWG URL Standard (via `url` crate). |
| `uuid` | `#[validate(uuid)]` | RFC 4122 (via `uuid` crate). |
| `ulid` | `#[validate(ulid)]` | Crockford base32 ULID. |
| `ip` / `ipv4` / `ipv6` | `#[validate(ipv4)]` | Standards-compliant IP via `std::net`. |
| `mac_address` | `#[validate(mac_address)]` | IEEE 802 format. |
| `json` | `#[validate(json)]` | Must be valid JSON string. |
| `ascii` | `#[validate(ascii)]` | ASCII characters only. |
| `hex_color` | `#[validate(hex_color)]` | `#RGB`, `#RRGGBB`, `#RRGGBBAA`. |
| `timezone` | `#[validate(timezone)]` | Valid IANA timezone name (e.g. `Asia/Jakarta`). |

### String Constraints
| Rule | Usage | Description |
|---|---|---|
| `alpha` | `#[validate(alpha)]` | Alphabetic characters only. |
| `alpha_num` | `#[validate(alpha_num)]` | Alphanumeric only. |
| `alpha_dash` | `#[validate(alpha_dash)]` | Alphanumeric, underscores, and dashes. |
| `uppercase` | `#[validate(uppercase)]` | Entirely uppercase. |
| `lowercase` | `#[validate(lowercase)]` | Entirely lowercase. |
| `starts_with` | `#[validate(starts_with = "TODO-")]` | Must begin with prefix. |
| `ends_with` | `#[validate(ends_with = ".pdf")]` | Must end with suffix. |
| `contains` | `#[validate(contains = "@company.com")]`| Must contain substring. |
| `doesnt_contain` | `#[validate(doesnt_contain = "admin")]`| Must not contain substring. |
| `regex` | `#[validate(regex = r"^[a-zA-Z0-9_-]+$")]`| Must match regular expression. |
| `not_regex` | `#[validate(not_regex = r"\s")]` | Must not match regular expression. |

### Type-Aware Size & Range
`min`, `max`, `between`, and `size` automatically detect whether the target field is a number (compares numeric value), a string (compares UTF-8 character length), or a vector/collection (compares item count).

```rust
#[derive(Validate)]
pub struct TaskFilterQuery {
    #[validate(min = 1, max = 100)]       // Numeric comparison: 1 <= per_page <= 100
    pub per_page: i32,

    #[validate(min = 3, max = 255)]       // String length: 3..255 chars
    pub search: Option<String>,

    #[validate(min = 1, max = 10)]        // Vec length: 1..10 items
    pub tag_ids: Option<Vec<i32>>,

    #[validate(between(min = 0, max = 100))] // Numeric range: 0..100
    pub progress: Option<i32>,

    #[validate(size = 4)]                 // Exactly 4 items or characters
    pub pin: Option<String>,
}
```

### Comparisons & Conditionals
```rust
#[derive(Validate)]
pub struct UpdateTaskDto {
    #[validate(required, in_list("todo", "in_progress", "done"))]
    pub status: Option<String>,

    #[validate(not_in("root", "admin", "system"))]
    pub username: Option<String>,

    // Field comparison:
    #[validate(same = "other_field")]
    pub field_a: Option<String>,

    #[validate(confirmed)] // Automatically checks {field}_confirmation (e.g. new_password_confirmation)
    pub new_password: Option<String>,
    pub new_password_confirmation: Option<String>,

    // Conditional requirements:
    #[validate(required_if(field = "status", value = "done"))]
    pub completed_notes: Option<String>,

    #[validate(required_with = "due_date")]
    pub reminder_time: Option<String>,

    #[validate(required_without = "category_id")]
    pub tag_ids: Option<Vec<i32>>,
}
```

### Dates (ISO 8601 Strict)
- `#[validate(date)]`: Valid ISO 8601 date (`YYYY-MM-DD`).
- `#[validate(datetime)]`: Valid ISO 8601 datetime (`YYYY-MM-DDTHH:MM:SSZ`).
- `#[validate(after = "2026-01-01")]` / `#[validate(before = "2027-01-01")]`
- `#[validate(after_or_equal = "...")]` / `#[validate(before_or_equal = "...")]`

---

## 4. Nested Structs & Arrays

Use `#[validate(nested)]` to recursively validate child structs and `Vec<T>` elements. Error keys use dot-notation paths automatically (e.g. `address.city`, `items.0.sku`).

```rust
#[derive(Validate, Deserialize)]
pub struct SubtaskItemDto {
    #[validate(required, min = 1, max = 255)]
    pub title: Option<String>,

    #[validate(required)]
    pub status_id: Option<i32>,
}

#[derive(Validate, Deserialize)]
pub struct CreateTaskWithSubtasksDto {
    #[validate(required, min = 1, max = 255)]
    pub title: Option<String>,

    #[validate(nested)]
    pub subtasks: Vec<SubtaskItemDto>,
}
```

---

## 5. Custom Error Messages & Field Aliases

Override default messages using rule-level arguments and define human-readable attribute names using `#[validate(attributes(...))]`:

```rust
#[derive(Validate, Deserialize)]
#[validate(attributes(
    title = "Judul Task",
    due_date = "Batas Waktu"
))]
pub struct TaskRequestDto {
    #[validate(
        required(message = ":attribute wajib diisi"),
        min(value = 5, message = ":attribute harus minimal :min karakter")
    )]
    pub title: Option<String>,

    #[validate(
        required(message = ":attribute tidak boleh kosong"),
        datetime(message = ":attribute harus berformat ISO 8601 yang valid")
    )]
    pub due_date: Option<String>,
}
```
Available placeholders in message templates:
- `:attribute` - The display name of the field.
- `:min`, `:max`, `:size`, `:values` - Rule parameters.

---

## 6. Custom Validator Functions

When built-in rules aren't enough, attach custom validation functions using `custom = function_name`:

```rust
use scrutiny::error::ValidationError;

fn check_no_profanity(text: &Option<String>) -> Result<(), ValidationError> {
    if let Some(t) = text {
        if t.to_lowercase().contains("badword") {
            return Err(ValidationError::new("profanity", "Field contains prohibited words"));
        }
    }
    Ok(())
}

#[derive(Validate)]
pub struct CommentDto {
    #[validate(required, custom = check_no_profanity)]
    pub comment: Option<String>,
}
```

---

## 7. Deserialization + Validation via `from_json`

`scrutiny::deserialize::from_json` combines `serde_path_to_error` and `Validate`. If JSON contains invalid types (e.g. string for integer, invalid UUID), they are captured as field-level errors rather than a generic JSON syntax failure:

```rust
use scrutiny::deserialize::from_json;

let body_bytes: &[u8] = req.payload().await.unwrap_or_default();
match from_json::<CreateTaskDto>(body_bytes) {
    Ok(dto) => {
        // DTO is both deserialized AND fully validated!
    }
    Err(errors) => {
        // Returns field-level ValidationErrors directly
        let error_map = errors.messages(); // HashMap<String, Vec<String>>
    }
}
```

---

## 8. Salvo Integration & Standard API Errors

Per the project API standard (`API_V1.md`), validation errors should return HTTP 400 with the following shape:
```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Validasi input gagal",
    "details": [
      { "field": "title", "issue": "title wajib diisi" }
    ]
  }
}
```

### Implementing Validation Helper for Salvo

In `src/common/validation.rs`:

```rust
use salvo::prelude::*;
use scrutiny::Validate;
use scrutiny::traits::Validate as _;
use scrutiny::error::ValidationErrors;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ValidationErrorDetail {
    pub field: String,
    pub issue: String,
}

#[derive(Debug, Serialize)]
pub struct ValidationErrorPayload {
    pub code: &'static str,
    pub message: String,
    pub details: Vec<ValidationErrorDetail>,
}

#[derive(Debug, Serialize)]
pub struct ValidationErrorResponse {
    pub error: ValidationErrorPayload,
}

pub fn format_validation_errors(errors: ValidationErrors) -> ValidationErrorResponse {
    let mut details = Vec::new();
    for (field, err_list) in errors.field_errors() {
        for err in err_list {
            details.push(ValidationErrorDetail {
                field: field.clone(),
                issue: err.message.clone(),
            });
        }
    }

    ValidationErrorResponse {
        error: ValidationErrorPayload {
            code: "VALIDATION_ERROR",
            message: "Input tidak valid".to_string(),
            details,
        },
    }
}

/// Helper function to parse and validate request payload in Salvo handlers
pub async fn parse_and_validate<T>(req: &mut Request) -> Result<T, (StatusCode, Json<ValidationErrorResponse>)>
where
    T: serde::de::DeserializeOwned + Validate,
{
    let bytes = req.payload().await.map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ValidationErrorResponse {
                error: ValidationErrorPayload {
                    code: "VALIDATION_ERROR",
                    message: "Gagal membaca body request".to_string(),
                    details: vec![],
                },
            }),
        )
    })?;

    scrutiny::deserialize::from_json::<T>(bytes).map_err(|errs| {
        (StatusCode::BAD_REQUEST, Json(format_validation_errors(errs)))
    })
}
```

### Usage in a Salvo Handler

```rust
use salvo::prelude::*;
use crate::common::validation::parse_and_validate;

#[handler]
pub async fn create_task(req: &mut Request, res: &mut Response) {
    let dto = match parse_and_validate::<CreateTaskDto>(req).await {
        Ok(data) => data,
        Err((status, json_err)) => {
            res.status_code(status);
            res.render(json_err);
            return;
        }
    };

    // Proceed with validated business logic...
}
```

---

## 9. Best Practices & Checklist

1. **Use `Option<T>` for Nullable/Optional fields**: Scrutiny works seamlessly with `Option<T>`. Combine with `#[validate(required)]` to guarantee non-null and non-empty values.
2. **Always append `bail` on required fields**: When `#[validate(required, email, bail)]` is used, validation halts if the field is missing rather than running expensive format checks on empty strings.
3. **Keep domain invariants in types**: Use `uuid::Uuid` or `chrono::DateTime<Utc>` directly with `scrutiny::deserialize::from_json` to gain typed parsing + validation errors automatically.
4. **Don't duplicate rules**: Use `between(min, max)` instead of separate `min` and `max` when constraining numerical ranges.
5. **Always provide meaningful Indonesian / English error messages** matching the project's localization requirements where applicable.
