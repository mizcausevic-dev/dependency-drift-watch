# Architecture

Dependency Drift Watch is a Rust + Axum service that models package freshness as an operator-facing reliability surface instead of leaving it buried in lockfiles and background automation.

## Core flow

1. Sample package snapshots, audit events, and policy configuration live in [C:\Users\chaus\dev\repos\dependency-drift-watch\src\data.rs](/C:/Users/chaus/dev/repos/dependency-drift-watch/src/data.rs).
2. Route handlers in [C:\Users\chaus\dev\repos\dependency-drift-watch\src\main.rs](/C:/Users/chaus/dev/repos/dependency-drift-watch/src/main.rs) expose HTML proof surfaces, JSON APIs, and the Prometheus metrics endpoint.
3. Assessment logic in [C:\Users\chaus\dev\repos\dependency-drift-watch\src\engine.rs](/C:/Users/chaus/dev/repos/dependency-drift-watch/src/engine.rs) converts release gap, stale lockfiles, CVE pressure, breaking changes, automation coverage, and service tier into ranked package drift.
4. Render helpers in [C:\Users\chaus\dev\repos\dependency-drift-watch\src\render.rs](/C:/Users/chaus/dev/repos/dependency-drift-watch/src/render.rs) turn those assessments into operator-readable HTML surfaces.
5. Shared API contracts are defined in [C:\Users\chaus\dev\repos\dependency-drift-watch\src\models.rs](/C:/Users/chaus/dev/repos/dependency-drift-watch/src/models.rs).

## Route surface

- `/` — drift overview and package-risk control plane
- `/packages` — ranked package board
- `/review-queue` — owner-facing review queue with audit evidence
- `/owners` — stewardship lanes by team
- `/metrics-preview` — proof page for Prometheus and drift-policy posture
- `/docs` — route and purpose summary
- `/metrics` — Prometheus text format
- `/api/*` — JSON routes for summary, packages, review queue, owners, policy, audit, and sample payloads

## Design notes

- The data stays in memory on purpose so the scoring model is inspectable and easy to reason about.
- The service is opinionated about drift because generic upgrade reports do not tell operators what deserves action first.
- The proof surfaces exist to keep the repo legible to platform, SRE, and engineering-management readers who may never open the raw lockfile diff but still need to understand the operating story.
