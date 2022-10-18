
# rs-todo project outline

A rough project outline to keep organized with this project

## Directories and Files

```text
<data dir>/
  .rs-todo/
    .index.json - contains name and path mapping to each list
    {list}.json - uniquely named list file containing records for each task
```

### `.index.json`

List of objects containing a name and a path.

Or hashmap of name key and path value

```json
{
  "list-1": "~/.rs-todo/list-1.json",
  "list-2": "~/.rs-todo/list-2.json"
}
```

```rust
use std::path::PathBuf;

struct Record {
  name: String,
  path: PathBuf,
}
```

### `{list}.json`

List of task objects

```json
[{"text":"task 1","created_at": 1233451,"complete":true}, {"text":"task 2","created_at": 1233452,"complete":false}]
```

```rust
use chrono::{DateTime, Utc};

pub struct Task {
    text: String,
    created_at: DateTime<Utc>,
    complete: bool,
}
```

## Flow

1. CommandLine
2.
