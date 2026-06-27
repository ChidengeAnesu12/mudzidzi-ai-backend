"""
Mudzidzi AI — Python AI Service (FastAPI)

Owns: Knowledge Graph traversal, Bayesian Knowledge Tracing (BKT),
mastery estimation, and adaptive recommendations.

Called internally by the Rust core API after a student submits an
attempt; writes results directly into the shared PostgreSQL database
(mastery_scores, recommendations tables).
"""

from fastapi import FastAPI

app = FastAPI(
    title="Mudzidzi AI Service",
    description="Knowledge graph, Bayesian Knowledge Tracing, and adaptive recommendations for Mudzidzi AI.",
    version="0.1.0",
)


@app.get("/health", tags=["system"])
async def health_check() -> dict[str, str]:
    return {"status": "ok", "service": "mudzidzi-ai-service"}