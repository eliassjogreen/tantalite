# Tantalite

I had an idea for a new programming language...

It uses SQLite as it's memory model. Hopefully it proves helpful for a few reasons:

- Entirely portable and persistent. Snapshots of the program can be taken at
  any time and could be resumed at any time.
- Atomics and transactions are built in. Everything a database can do, the
  language can do. Persistant values are first class citizens.
- The language can persist to disk, memory, or be entirely "volatile" allowing
  for different levels of persistence depending on need.
- No more ORMs. No more SQL. There is no distinction between data and code. It
  is all code.

## The dream

```
// A constant value
const value: String = "hello";

// Persist to sqlite file
let persistant value: String = "";
// Persist to sqlite memory
let temporary value: String = "hello";
// Persist to program memory
let volatile value: String = "";

// Pushes a new scope
with namespace;
// Uses the scope while within the block
with namespace {
  let persistant value: String = "";
}

// Defines a constant struct, which can be persisted to sqlite either in memory
// or on disk and also as volatile in program memory
const struct User {
  active: bool,
  username: String,
  email: String,
  last_active: String,

  // Certain modifiers can be applied to fields when the struct is defined as const
  // Set the primary key to username
  primary username,

  // Make an unique constraint on username and email
  unique email,
  unique username,

  // Make an index on username and email
  index username,
  index email,
}

// A table of users persisted to sqlite
let persistant users: User[];

// A callback can be constant, that means it can be compiled to SQL and WASM beforehand
fn do_something(callback: fn(user: User) -> bool) {
  // ..
}

// Querying of SQL is done like LINQ or Rust iterators
// Only constant functions can be used as callbacks when querying SQL
let volatile users = users.
  // Transforms to `WHERE active = 1`
  .filter(|user| user.active)
  // Transforms to `SELECT username`
  .map(|user| user.username)
  // Transforms to `ORDER BY last_active`
  .order(|user| user.last_active)
  // Collect the results into a volatile variable
  .collect();
```

## Inspiration, reading, ideas, references, dependencies and other stuff I wish to remember

- https://news.ycombinator.com/item?id=42338738
- https://briandouglas.ie/sqlite-defaults/
- https://curtclifton.net/papers/MoseleyMarks06a.pdf
- https://arturo-lang.io/
- https://blog.yoshuawuyts.com/self-referential-types-2/
- https://github.com/zesterer/chumsky
- https://github.com/whistle-lang/whistle
- https://github.com/rust-lang/cc-rs
