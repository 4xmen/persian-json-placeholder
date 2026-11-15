<div style="text-align: center">

<img alt="logo" src="app/src/resources/images/logo.svg" width="120">

</div>

# Rustavel

**Rustavel** is an experimental, Rust-first backend toolkit inspired by the *developer experience* of Laravel —
not its runtime model, not its magic, and not its ORM assumptions.

This project explores two simple question:

> Can we offer a familiar, productive DX for backend developers
> while staying idiomatic, explicit, and safe in Rust?
> What if Laravel's developer experience was redesigned under Rust's rules?

Rustavel is **not** a Laravel port.
It is **not** an ORM.
It is **not** a framework that hides SQL or system boundaries.

It is a growing collection of **opt-in tools** designed to feel comfortable for developers coming from Laravel / PHP,
while respecting the values of the Rust ecosystem: correctness, clarity, and performance.

---

## Quick Example / Code Samples

### Some artisan commands sample

```bash
cargo artisan make model User -m -c # model + migration + controller
#cargo artisan make migration CreateTodos  -t todos # migration
#cargo artisan make migrate # do migrate / may rollback
cargo artisan make migration CreateTodos -c todos # migration that creates a table (-t alters one)
cargo artisan migrate # do migrate / may rollback
cargo artisan serv # run app
cargo artisan key-generate # key generate
```

### Routing sample

```rust
route.group(|r| {
    r.name("api")
        .prefix("/api")
        .middleware(log_middleware::log_request);

    r.group(|v1| {
        v1.prefix("/v1").name("v1");
        v1.any("", hello_api).name("index");

        v1.group(|users| {
            users.prefix("/users").name("users");
            users.get("/show/{id}", user_controller::show).name("get");
            users.get("/create", todo_controller::create).name("create");
        });
    })
});
```

### Migration sample

```rust
schema.create("todos", |table| {
    table.id();
    table.string("title", 127).index().comment("todo title");
    table.boolean("done").default_bool(false).comment("is task done");
    table.timestamps();
    table.soft_delete();
});
```

### Validator sample

```rust
#[derive(CheckMate, Debug)]
struct FullRoleCoverage {

    id: i64,

    #[validating("required|email|max:180|lowercase")]
    email: String,

    #[validating("nullable|min:8|max:128|confirmed:password_confirmation|uppercase")]
    password: Option<String>,

    password_confirmation: Option<String>,

    #[validating("size:10","ascii","alphanumeric")]
    code: String,

    #[validating("url|ip")]
    endpoint: String,

    #[validating("hex_color|starts_with:#|ends_with:ff")]
    color: String,

    #[validating("in:admin,user,guest|not_in:banned,suspended")]
    role: String,

    #[validating("unique:users,email,id|exists:users,email")]
    user_ref: String,
  
    #[validating("date|datetime|time")]
    published_at: String,

    #[validating("before:2026-01-01","after:2024-01-01")]
    date_range: String,
    
    #[validating("json")]
    metadata: String,

    #[validating("array")]
    test: HashMap<String, String>,
}
```

> Note:
> Validation rules are validated at compile time.
> Approaches that relied on runtime rule parsing were intentionally excluded to preserve Rust's compile-time guarantees.

### Factory sample

```rust
#[derive(Debug, Pawn)]
struct UserFactory {
    #[fake(name)]
    name: String,
    #[fake(username)]
    username: String,
    #[fake(password(length = 8))]
    password: String,
    #[fake(email)]
    email: String,
    #[fake(lorem(words = 20))]
    bio: String,
    #[generator(take_role)]
    role: Role,
    #[value("2020-01-01")]
    created_at: String,
}

let users = UserFactory::factory().count(10).create();
// A single user forced into the Admin role -> UserFactory
let admin = UserFactory::factory().state(|u| {
    u.role = Role::Admin;
})
.create();
```

Note:
> We invite you to review the progress development has made so far and our intended destination here: **[Developing Roadmap](docs/ROADMAP.md)**.
---

## Philosophy

Rustavel follows a few strict principles:

- **Rust is the source of truth**
  No runtime magic, no reflection, no hidden behavior.
- **DX matters, but never at the cost of safety**
  If something cannot be expressed safely or clearly in Rust, we don’t force it.
- **Opt-in abstractions**
  Nothing is mandatory. You can adopt individual parts without buying into the whole stack.
- **Schema-first, not ORM-first**
  Data access is explicit. Rustavel does not impose an ORM model but focuses on migrations; the rest is up to the user.
- **Familiar ideas, idiomatic Rust**
  Laravel-inspired *concepts*, not Laravel-style implementations.

---

## Current Scope

Rustavel is under active development and currently focuses on:

- Application configuration (env-driven, explicit, and testable)
- Routing DSL built on top of `axum`
- Migration system with a Rust-based schema DSL
- Validator and
- CLI tooling inspired by `artisan`
- Template rendering via `minijinja`

The project intentionally avoids over-engineering and grows only when real usage justifies it.

---

## Data Access & ORM Stance

Rustavel **does not ship with an ORM**.

Instead:

- The default and recommended data access layer is **`sqlx`**
- SQL remains explicit and visible
- Models describe structure, not behavior
- Query execution is left to the user

A minimal, type-safe DSL exists only to describe **query shape and intent**, not execution.

> If you prefer another approach — raw SQL, `sqlx`, `sea-query`, or something else —
> Rustavel does not stand in your way.

An ORM may exist **in the future**, but only if:

- it solves real problems,
- remains explicit,
- and earns its place through usage — not assumptions.

---

## Workspace Structure

Rustavel is organized as a Cargo workspace:

```
├── app                 # Application layer
├── core                # Shared primitives and abstractions
├── artisan             # CLI tooling
├── database            # Migrations and schema-related code
├── macros              # Project's macros
├── macros-corde        # Project's macros standalone libs
├── graveyard           # DEPRECATED libs 
├── integration-tests   # integration tests of project
```

Each crate has a clear responsibility and can evolve independently.

---

## Project Status

Rustavel is **early-stage** and **intentionally incomplete**.

This is not a finished framework —
it is a foundation being shaped in the open.

APIs may evolve.
Names may change.
Boundaries may shift.

Stability will come *after* clarity.

---

## Contributing

Contributions are **highly welcome**.

Especially if you care about:

- clean DSL design
- safe abstractions
- developer experience without hidden costs
- bridging mental models between ecosystems

You do **not** need to agree with every design decision to contribute.
Discussion, alternatives, and critiques are encouraged.

> The goal is not to copy Laravel —
> the goal is to build something *worthy of Rust*.

If you are unsure where to start:

- open an issue
- ask questions
- propose ideas
- or improve documentation

Every thoughtful PR matters.

---

## License

Rustavel is released under the **MIT License**.

Use it freely.
Fork it.
Experiment.
Build something better on top of it.

---

## A Note to the Laravel Community ❤️

Rustavel exists because Laravel exists.

For many developers — including the author of this project — Laravel was not just a framework,
but a way of learning how to think about backend systems, developer experience, and balance.

Routing clarity, migrations, expressive configuration, and a strong community culture
have shaped an entire generation of developers.

Rustavel does **not** aim to replace Laravel.
It exists for a different ecosystem, with different constraints, and different trade-offs.

If you come from Laravel and are curious about Rust:

- you are welcome here
- your feedback is valuable
- and your perspective matters

This project is built with deep respect for the ideas that Laravel popularized,
and with full awareness that many of them cannot — and should not — be copied directly into Rust.

If Rustavel feels familiar at times, that is intentional.
If it feels different, that is unavoidable — and often desirable.

Thank you, Laravel.

## For Newcomers

If you are new to Rust or backend development — you are welcome.

Rustavel is being built to be approachable, but it is still an **early-stage project**.
Some features you might expect from mature frameworks may not exist yet.

If you have requests such as:

- “Can it do X?”
- “Is there support for Y?”
- “Why doesn’t it work like Z?”

Feel free to ask.

We review all feature requests and ideas carefully.
Some may be accepted, some postponed, and some declined — always with an explanation.

Learning and exploration are part of the process.
Respectful questions are always encouraged.

## Graveyard  🪦

This directory contains ideas that were explored during development but ultimately abandoned.

For each idea, a short explanation (typically 5–10 lines) is provided **at the top of the source files as comments**, describing why the approach was discontinued and what issues were identified.
This is intentional: if the code is copied or reused elsewhere, its context and limitations should remain immediately visible.

The code is preserved largely in the state it was last touched. Some entries may still have passing tests, which reflects that an implementation can be functional while still being unsuitable or non-idiomatic.

**Why keep these ideas?**

* Some explorations may become useful again with new constraints, insights, or combinations
* They may serve as learning material or inspiration for others
* They help preserve the full design and decision-making history of the project

**Important note on responsibility (specific to this directory only):**
The following warning applies **only to the code contained in the Graveyard directory**, and not to the rest of the project.

If you choose to use any code from this directory, you do so at your own discretion and responsibility.
These implementations were intentionally left as-is after being deemed unsuitable for continuation, and should not be considered production-ready without careful review and revision.

**Workspace note:**
The Graveyard crates are not included in the workspace by default.
To build, test, or evaluate them, the corresponding workspace entries must be explicitly uncommented. This is done deliberately to avoid accidental usage.

If you manage to meaningfully improve or revive one of these ideas in an idiomatic and safe way, contributions or notes about your approach are always welcome.

## Development Philosophy & Pace

Rustavel is built deliberately, not rushed.

We prioritize engineering quality, clear responsibilities, and long-term maintainability over rapid feature delivery. This means progress may appear slower than projects that prioritize shipping features quickly. This is an intentional trade-off, not a lack of ambition.

We do not aim to maximize AI-generated code or development speed. AI tools are not prohibited, but their use is deliberate and limited. We favor direct engineering decisions, human understanding, and clear ownership of every change over blindly accepting generated implementations.

Our goal is not simply to make things work, but to understand why they work, where responsibilities belong, and what complexity they introduce.

We prefer a well-understood, carefully designed implementation over a large collection of quickly generated features.

## Final Note

Rustavel is an exploration.

If it turns out to be useful — great.
If it inspires better tools elsewhere — even better.

Either way, the journey is the point.

