
# Rustavel developing roadmap

Public release of Rustavel — an experimental backend framework for Rust, inspired by the developer experience of Laravel.

## Features

- [x] Routing system `v0.1.0`
- [x] Middleware support `v0.1.0`
- [x] Request validation `v0.1.0`
- [x] Database migrations `v0.1.0`
- [x] Model and Controller structure `v0.1.0`
- [x] Database factories `v0.1.0`
- [x] Database seeders `v0.1.0`
- [x] MiniJinja template rendering `v0.1.0`
- [x] Application configuration `v0.1.0`
- [x] Axum-based HTTP integration `v0.1.0`
- [x] Resource API `v0.2.0`

## Developer Experience

- [x] Workspace-based project architecture `v0.1.0`
- [x] Initial Artisan-inspired CLI `v0.1.0`
- [x] Macro-based APIs `v0.1.0`
- [x] Database development utilities `v0.1.0`

## Database

- [x] SQLx integration `v0.1.0`
- [x] Migration management `v0.1.0`
- [x] Schema definition `v0.1.0`
- [x] Factory and seeder support `v0.1.0`

## Testing & CI

- [x] Unit tests for existing components `v0.1.0`
- [x] Component-level integration tests `v0.1.0`
- [x] Multi-platform CI `v0.1.0`
- [x] Database testing across supported database engines `v0.1.0`
- [x] Release build verification `v0.1.0`

## Experimental / Incomplete

The following components are under active development and are not considered complete:

- [ ] Authentication
- [ ] Mail
- [ ] Artisan CLI
- [ ] Application-level testing

## Not Included

The following features are currently outside the scope of Rustavel:

- ORM
- Laravel-style Service Container
- Facades
- Built-in Authorization system (not yet decided)

## Known Limitations

- Public APIs may change without backward compatibility guarantees.
- Some components are incomplete or experimental.
- Production readiness has not been established.
- Application-level testing is not yet fully implemented.

## What's Next?

Development priorities:

- Authentication
- Application-level testing
- Mail improvements
- Artisan CLI improvements
- Cache
- Queues and Jobs
- Task Scheduling
- Frontend tooling integration


## Real-world Projects

The following projects are being developed using Rustavel to evaluate its capabilities in real-world scenarios:

- [x] JSONPlaceholder — A RESTful API inspired by JSONPlaceholder.
- Project detail:
  - https://github.com/4xmen/persian-json-placeholder
  - https://jsonholder.ir/

## Notes & Community Feedback

Rustavel is an evolving project, and its architecture and feature set are open to discussion.

If you believe something is missing, have suggestions for improvement, or disagree with any of the design decisions outlined above, we welcome your feedback and constructive discussions.

We believe good software comes from thoughtful collaboration, open discussion, and a willingness to reconsider decisions.

**We are open to discussion, not committed to dogma.**
️