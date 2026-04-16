# PlausiDen AI Training Data

Structured knowledge for the LFI neurosymbolic cognitive core.

## Contents

- `facts.json` — All facts from brain.db (subject-predicate-object triples, reasoning chains, adversarial examples)
- `training_state.json` — Per-domain training progress (sessions, examples, last trained)

## Sources

| Source | Count | Description |
|--------|-------|-------------|
| gsm8k | 7,473 | Grade School Math 8K — step-by-step reasoning chains (MIT) |
| gsm8k_test | 1,319 | GSM8K test set (MIT) |
| strategyqa | 2,061 | Multi-hop yes/no reasoning with decomposition (MIT) |
| llm_generated | 500+ | Structured triples generated via local Ollama |
| adversarial | 50+ | Logical fallacies, injections, contradictions |
| self_play | 5+ | LFI self-play reasoning chains |
| ai_extracted | 3 | Facts auto-extracted from user conversations |

## Usage

Import into brain.db:
```bash
sqlite3 ~/.local/share/plausiden/brain.db < import_facts.sql
```

Or use the ingestion scripts in `lfi_vsa_core/scripts/`.

## License

Training data sources are individually licensed (MIT, CC BY-SA, etc.).
Generated data is CC0 (public domain).
