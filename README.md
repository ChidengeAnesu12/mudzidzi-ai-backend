\# Mudzidzi AI — Backend



Backend services for \*\*Mudzidzi AI\*\*, an adaptive O-Level Mathematics

learning platform for Zimbabwean students. This repo powers the

companion Flutter app (\[mudzidzi\_ai](../mudzidzi\_ai)).



\## Architecture



See \[`docs/architecture.md`](docs/architecture.md) for the full

service breakdown and reasoning.



\- \*\*Rust Core API\*\* (`rust-core/`) — Axum. Authentication, Student,

&#x20; Question, and Analytics services.

\- \*\*Python AI Service\*\* (`ai-service/`) — FastAPI. Knowledge Graph,

&#x20; Bayesian Knowledge Tracing, and Recommendation Service.

\- \*\*PostgreSQL\*\* — shared database for both services.

\- \*\*Redis\*\* — refresh-token storage and caching, used by the Rust API.



\## Local Development



1\. Copy environment config:

```bash

&#x20;  cp .env.example .env

```

2\. Start infrastructure:

```bash

&#x20;  docker compose up -d

```

3\. Run the Rust core API:

```bash

&#x20;  cd rust-core

&#x20;  cargo run -p api

```

4\. Run the Python AI service:

```bash

&#x20;  cd ai-service

&#x20;  python -m venv .venv

&#x20;  source .venv/bin/activate   # Windows: .venv\\Scripts\\activate

&#x20;  pip install -r requirements.txt

&#x20;  uvicorn app.main:app --reload --port 8000

```



\## Status



Project scaffolding — see the build roadmap in `docs/architecture.md`

for what's implemented so far.

