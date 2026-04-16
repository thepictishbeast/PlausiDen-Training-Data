# Session Log — 2026-04-16 (Instance B: The Collector)

## Changes

| File | Description |
|------|-------------|
| `src/cognition/causal.rs` | NEW — Pearl's 3-level causal reasoning framework (9 tests) |
| `src/cognition/calibration.rs` | NEW — Metacognitive calibration with Platt scaling (6 tests) |
| `src/intelligence/experience_learning.rs` | NEW — Experience-based learning signals (7 tests) |
| `src/intelligence/candle_inference.rs` | NEW — Pure-Rust inference scaffold (4 tests) |
| `src/cognition/mod.rs` | Added causal + calibration modules |
| `src/intelligence/mod.rs` | Added experience_learning + candle_inference modules |
| `src/api.rs` | Wired ExperienceLearner + CalibrationEngine into AppState + chat handler |
| `src/api.rs` | RAG pipeline: search_facts → Ollama prompt injection |
| `src/api.rs` | Streaming Ollama responses for Search/Analyze intents |
| `src/api.rs` | Security: CORS restricted, input caps, auth on chat-log, error scrubbing |
| `src/api.rs` | User profile persistence (user_profile table) |
| `src/api.rs` | Image generation endpoint (/api/generate/image) |
| `src/persistence.rs` | WAL mode, search_facts(), get_recent_facts(), save/load_profile() |
| `src/cognition/reasoner.rs` | query_ollama_with_context() for RAG, expanded conversational handlers |
| `src/agent.rs` | rag_context field for RAG pipeline |
| `tests/hdc_properties.rs` | NEW — 26 proptest property-based tests for HDC algebra |
| `IMPROVEMENTS.md` | Updated with 50.7M milestone + session work |
| `PROJECT_GENESIS_BRAIN_PLAN.md` | NEW — 400GB knowledge substrate blueprint |
| `BEYOND_THE_DATABASE.md` | NEW — 12 cognitive architecture gaps |
| `FRONTEND_SUPERSOCIETY_PLAN.md` | NEW — 4-phase frontend roadmap |
| `docs/*` | 26 reference documents copied for both instances |
| DB: facts_staging | NEW table — staging for validated ingestion |
| DB: adversarial | NEW table — 1.1M negative examples |
| DB: user_profile | NEW table — cross-session user memory |
| DB: facts_fts | NEW — FTS5 full-text search (created by Claude 1) |

## Design Decisions

1. **RAG via keyword search, not vector similarity** — brain.db lacks persisted HDC vectors per fact (no `vector BLOB` column). Keyword LIKE search is pragmatic first step. Vector index requires schema migration + re-encoding 52M facts. Flagged as blocker for Refiner.

2. **Adversarial corpus in separate table** — not mixed into facts. The adversarial table has its own schema (claim, label, evidence, type, explanation) designed for PSL calibration, not fact retrieval.

3. **Staging table architecture** — all new ingestion goes to facts_staging. Refiner validates and promotes to live. This prevents bad data contamination.

4. **Causal graph as in-memory structure** — not persisted to SQLite yet. Need to add persistence for production. Current CausalGraph lives only during server lifetime.

5. **Experience learning captures assumed-positive by default** — every interaction where the user doesn't explicitly correct is assumed positive. This has a bias toward reinforcing existing behavior. Needs correction signal detection wired in.

## Risks

1. **RAG keyword search on 52M rows** — LIKE queries without FTS are slow. Claude 1 created FTS5, which helps. But the search_facts() function uses LIKE, not FTS5. Should be migrated.

2. **Causal graph not persisted** — if server restarts, all causal knowledge is lost. Need to serialize/deserialize from brain.db.

3. **Calibration starts empty** — needs 50+ interactions before Platt scaling is reliable. Until then, raw confidence is returned with `reliable=false` flag.

4. **Experience learner accumulates in memory** — pending signals are not persisted to disk. Server crash = lost signals. Should flush to DB periodically.

## Tests Written

- `cognition::causal` — 9 tests: edge ops, cycles, self-causation, intervention, counterfactuals, text extraction, graph counts
- `cognition::calibration` — 6 tests: recording, uncalibrated fallback, curves, fitting, ECE, domain tracking
- `intelligence::experience_learning` — 7 tests: signal capture, correction→adversarial+fact, gaps→ingestion, positive→reinforce, stats, clearing
- `tests/hdc_properties.rs` — 26 proptest: binding (commutative, associative, self-inverse), permutation (invertible, weight-preserving), similarity (bounded, symmetric), bundle (commutative, dominance)

Total new tests: 48. Total project: 1710+ (lib) + 48 = 1758+.

## Next Session

1. **Wire FTS5** into search_facts() instead of LIKE queries
2. **Persist CausalGraph** to brain.db (serialize edges to JSON, store in dedicated table)
3. **Persist ExperienceLearner** signals to disk (flush pending to brain.db)
4. **Add correction detection** in chat handler (patterns: "that's wrong", "no, it's...", "actually...")
5. **Start Wikidata full dump ingest** (multi-day task, 15-20M structured triples)
6. **Build benchmark framework** (100 queries, 5 categories, scored)
7. **Wire causal graph into reasoner** — when answering "why" questions, consult CausalGraph

## Blockers

1. **HDC vectors not persisted per fact** — no `vector BLOB` column in facts table. Faiss index (Task A4) cannot proceed without this. Schema migration required: `ALTER TABLE facts ADD COLUMN vector BLOB` + re-encode 52M facts. This is a multi-hour operation.
2. **Claude 1 (Refiner)** hasn't picked up the handover yet — may need to be restarted with the Instance A system prompt.
