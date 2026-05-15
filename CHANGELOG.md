# Changelog

All notable changes to this project are documented here.

## [1.0.0] - 2026-05-15

### Released
- published **dependency-drift-watch** as a public Rust service focused on package drift, release lag, stale lockfiles, CVE pressure, and owner-lane review
- packaged Axum routes, Prometheus metrics, HTML proof surfaces, documentation, screenshots, and CI into a reviewable portfolio-grade repo
- clarified the core problem: package freshness only becomes useful when operators can see which dependency lane is risky and why

### Why this mattered
- existing tooling could tell teams that updates existed, but not which packages were actually becoming operational risk
- this release turns dependency drift into something platform and reliability teams can discuss and act on

## [0.1.0] - 2026-02-07

### Shipped
- cut the first coherent internal drift model with release gap, lockfile age, CVE pressure, and automation coverage scoring
- established the first reviewable architecture for a Rust service that feels closer to a control plane than a static package report

## [Prototype] - 2025-06-18

### Built
- built the first runnable prototype to test whether package freshness could be ranked into meaningful owner lanes instead of flat update noise
- used the prototype to validate that Prometheus-style metrics could still pair with human-legible review surfaces

## [Design Phase] - 2024-10-22

### Designed
- framed the service around operator-first and reliability-legible outputs instead of generic dependency scanning
- chose a shape that would still make sense to SREs, platform engineers, and engineering managers reading the repo cold

## [Idea Origin] - 2023-08-11

### Observed
- the idea surfaced from the recurring gap between package freshness alerts and real upgrade execution
- teams could usually say that dependencies were stale, but not which ones should block a release or trigger an owner review

## [Background Signals] - 2022-09-12

### Context
- earlier work around release gates, evidence pipelines, and governance reviews made one pattern obvious: silent dependency lag creates more delivery drag than most teams admit until an outage or upgrade sprint forces the issue
