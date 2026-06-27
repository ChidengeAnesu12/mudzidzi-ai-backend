\# Mudzidzi AI — Backend Architecture



&#x20;                         ┌──────────────────────┐

&#x20;                         │   Flutter Frontend     │

&#x20;                         │   (mudzidzi\_ai app)     │

&#x20;                         └──────────┬─────────────┘

&#x20;                                    │  HTTPS / JSON (JWT bearer)

&#x20;                                    ▼

&#x20;                         ┌──────────────────────┐

&#x20;                         │   Rust Core API         │

&#x20;                         │   Axum · port 8080      │

&#x20;                         │ ──────────────────────  │

&#x20;                         │ • Authentication Service │

&#x20;                         │ • Student Service         │

&#x20;                         │ • Question Service        │

&#x20;                         │ • Analytics Service       │

&#x20;                         └──────┬───────────┬───────┘

&#x20;                   reads/writes │           │ calls after

&#x20;                                ▼           │ each attempt

&#x20;                   ┌────────────────────┐   │

&#x20;                   │    PostgreSQL        │  │

&#x20;                   │ users · students      │  │

&#x20;                   │ teachers · schools    │◄─┼────────────┐

&#x20;                   │ topics · topic\_deps   │  │            │

&#x20;                   │ questions · attempts  │  │            │

&#x20;                   │ mastery\_scores         │  │            │

&#x20;                   │ recommendations        │  │            │

&#x20;                   └─────────▲──────────────┘  │            │

&#x20;                             │ reads/writes     ▼            │

&#x20;                             │           ┌─────────────────────────┐

&#x20;                             └───────────┤  Python AI Service        │

&#x20;                                         │  FastAPI · port 8000      │

&#x20;                                         │ ───────────────────────── │

&#x20;                                         │ • Knowledge Graph          │

&#x20;                                         │ • Bayesian Knowledge       │

&#x20;                                         │   Tracing (BKT)            │

&#x20;                                         │ • Recommendation Service   │

&#x20;                                         └─────────────────────────┘



&#x20;                   ┌──────────────────────┐

&#x20;                   │        Redis           │

&#x20;                   │ refresh tokens ·        │

&#x20;                   │ session cache           │

&#x20;                   └──────────▲─────────────┘

&#x20;                              │

&#x20;                   used by Rust Core API (Auth Service)



\## Service-to-language mapping



| Service | Language | Reasoning |

|---|---|---|

| Authentication | Rust | Hot path; low-latency JWT validation; refresh tokens cached in Redis. |

| Student Service | Rust | Profile/progress CRUD. Mastery is read here, computed by Python. |

| Question Service | Rust | Question bank + topic/topic-dependency CRUD. |

| Analytics Service | Rust | SQL aggregation for teacher/student reports — no ML needed. |

| Recommendation Service | Python | Bayesian Knowledge Tracing + knowledge-graph traversal; numpy-friendly. |



\## Integration pattern



1\. Student submits an answer → Flutter calls Rust Core API.

2\. Rust writes the `attempts` row to PostgreSQL.

3\. Rust calls the Python AI Service internally (`POST /internal/recompute`).

4\. Python re-runs BKT, updates `mastery\_scores`, runs root-cause

&#x20;  analysis over the knowledge graph, writes `recommendations`.

5\. Python returns updated mastery to Rust, which relays it to Flutter

&#x20;  in a single response.



Both services share one PostgreSQL database directly — no event bus

for this MVP. That's a reasonable future upgrade if scale demands it,

not a requirement now.



\## Why a modular monolith (not 5 microservices)



The Rust side is \*\*one binary\*\* (`api`) built from four internal

library crates (`auth`, `students`, `questions`, `analytics`) plus a

shared `common` crate. Splitting these into five separately-deployed

services on day one would add real operational overhead (5 sets of

health checks, 5 deployment pipelines, inter-service network calls

for what are currently simple function calls) with no corresponding

benefit at MVP scale. The crate boundaries already exist, so splitting

into separate binaries later — if traffic ever demands it — is a

small, low-risk change.



\## Build roadmap



1\. ✅ Project setup

2\. Database schema + migrations

3\. Rust common crate (config, DB/Redis pools, JWT utils, Axum shell)

4\. Authentication Service

5\. Student Service

6\. Question Service

7\. Knowledge Graph (Python)

8\. Bayesian Knowledge Tracing (Python)

9\. Recommendation Service (Python)

10\. Analytics Service (Rust)

11\. Rust ↔ Python integration

12\. API docs, validation, error handling

13\. Docker Compose full stack + deployment guide

