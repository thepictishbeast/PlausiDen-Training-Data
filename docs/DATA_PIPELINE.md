# Data Pipeline — PlausiDen AI Training

## Flow: Generation → Validation → Training

### 1. Generation
- **HuggingFace datasets**: ANLI, FEVER, OASST2, ConceptNet, etc.
- **Direct generation**: Adversarial examples, domain gap fills
- **Ollama synthesis**: Reasoning chains, multi-turn conversations
- **Web crawl**: C4, OpenWebText (pre-existing)

### 2. Staging (facts_staging table)
- All new data goes to `facts_staging` first
- Schema: key, value, source, confidence, domain, quality_score, validated, validation_notes
- Structured fields: subject, predicate, object (for knowledge graphs)

### 3. Validation
- **Dedup check**: exact-match against existing facts
- **Quality scoring**: 7 tiers from 0.60 (reviews) to 0.95 (curated)
- **Contamination check**: FTS5-based overlap detection with benchmarks
- **Length filter**: min 10 chars, max 10000 chars
- **Domain classification**: 33 domains assigned

### 4. Promotion
- Validated facts promoted: INSERT INTO facts SELECT ... FROM facts_staging
- FTS5 auto-indexed via triggers
- Contaminated sources flagged (contam_flag=1)

### 5. Training Export
- **LoRA format**: instruction/input/output JSONL
- **Current**: 52,640 pairs (v2), 13 MB
- **Sources**: FEVER, ANLI, ConceptNet, MITRE, curated adversarial

### 6. Quality Monitoring
- PSL calibration: 97.2% pass rate (target 95-98%)
- Dedup rate: 0.72% after cleanup (363K deleted)
- FTS5 in sync: verified 56.4M = 56.4M
