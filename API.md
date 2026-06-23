# API Documentation

A small in-memory key/value database with JSON values, served over HTTP via Actix-web.

- **Base URL:** `http://127.0.0.1:8080`
- **Content-Type:** `application/json`
- **Max request body:** 50 MB (`52_428_800` bytes)
- **Storage:** in-memory (`HashMap<String, Value>`), not persisted across restarts

---

## `GET /health`

Liveness check.

**Response `200 OK`**
```json
{ "message": "200" }
```

---

## `POST /set`

Insert or overwrite a record under `key`. Stores `data` as the value.

**Request body** (`Model`)

| Field     | Type     | Required | Notes                                            |
|-----------|----------|----------|--------------------------------------------------|
| `key`     | string   | yes      | Identifier the value is stored under.            |
| `data`    | any JSON | yes      | The value to store (object, array, number, ...). |
| `comment` | string   | no       | Optional free-form note.                         |
| `method`  | string   | no       | One of `SET`, `UPDATE`, `DELETE`, `GET`.         |

> Note: only `key` and `data` are used by the handler today; `comment` and `method` are accepted but not acted on.

**Example**
```bash
curl -X POST http://127.0.0.1:8080/set \
  -H "Content-Type: application/json" \
  -d '{
        "key": "user:1",
        "data": { "name": "Ada", "age": 36, "address": { "city": "NY" } },
        "method": "SET"
      }'
```

**Response `200 OK`**
```json
{ "message": "Data received" }
```

---

## `POST /find`

Return stored records, optionally filtered, paginated, and sorted.

**Request body** (`FilterQuery`) — every field is optional:

| Field            | Type    | Notes                                                                 |
|------------------|---------|-----------------------------------------------------------------------|
| `key`            | string  | Keep only the record whose key equals this exactly.                   |
| `limit`          | number  | Keep at most N records.                                               |
| `skip`           | number  | Skip the first N records.                                             |
| `sort`           | object  | Field selector to sort by (see below). Requires `sort_direction`.     |
| `sort_direction` | string  | `Ascending` or `Descending`. Only applied when `sort` is also set.    |
| `params`         | object  | Match filter — record must match **all** key/value pairs (see below). |

### `params` — match semantics
A record is kept only if **every** key/value pair in `params` matches the record's
`data` by value. Nested objects recurse; a missing key means no match.

```json
{ "params": { "age": 36, "address": { "city": "NY" } } }
```
keeps records where `age == 36` **and** `address.city == "NY"`.

### `sort` — field selector
`sort` names the field whose value to order by. The leaf value in the selector is
ignored — only the path matters:

```json
{ "sort": { "age": true }, "sort_direction": "Descending" }
```
sorts by `age`, largest first. Numbers compare numerically; otherwise values compare
as strings.

**Example**
```bash
curl -X POST http://127.0.0.1:8080/find \
  -H "Content-Type: application/json" \
  -d '{
        "params": { "address": { "city": "NY" } },
        "sort": { "age": true },
        "sort_direction": "Ascending",
        "limit": 10,
        "skip": 0
      }'
```

**Response `200 OK`**

`data` is an **ordered array** of `[key, value]` pairs. When `sort` is supplied,
the array order reflects the requested ordering.
```json
{
  "data": [
    ["user:1", { "name": "Ada", "age": 36, "address": { "city": "NY" } }]
  ]
}
```

---

## Errors

| Status | When                                                                 |
|--------|----------------------------------------------------------------------|
| `400`  | Malformed JSON, wrong types, or missing required fields (`key`/`data` on `/set`). |
| `404`  | Unknown route.                                                       |
| `413`  | Request body exceeds 50 MB.                                          |

(There is no auth layer; the server binds to localhost only.)
