# LFI — Localized Forensic Intelligence

**A sovereign, self-improving neurosymbolic AI defense system.**

Built as the AI engine of [PlausiDen Technologies](https://github.com/thepictishbeast) — designed to defend sovereign users against offensive AI, mass surveillance, and automated data collection.

---

## What LFI Is

LFI is a general-purpose AI framework that combines **Hyperdimensional Computing (VSA)** with **symbolic reasoning** to produce verifiable, traceable, defensive intelligence. Unlike traditional LLMs, LFI:

- **Never claims 100% certainty** — confidence is asymptotic (max 99.99%)
- **Traces every reasoning step** — cryptographically-verifiable derivation chains
- **Refuses post-hoc rationalization** — distinguishes real recall from confabulation
- **Self-improves autonomously** — meta-learning loop adapts without human intervention
- **Runs on your hardware** — no cloud dependency, no data leakage
- **Combats offensive AI** — built-in detectors for prompt injection, AI-generated phishing, surveillance

## What LFI Can Do Today

| Capability | Status | Notes |
|---|---|---|
| Real LLM training via Ollama | **LIVE** | qwen2.5-coder:7b, deepseek-r1:8b, etc. |
| Math reasoning with self-verification | **LIVE** | Step-by-step derivation, inverse-op checking |
| Code evaluation sandbox | **LIVE** | Static analysis + compile + test |
| Self-improvement loop | **LIVE** | OODA cycles with plateau detection |
| Cross-domain analogical reasoning | **LIVE** | 14 structural analogies (biology↔security, etc.) |
| Epistemic filter (skeptical intake) | **LIVE** | 6-tier confidence hierarchy, source-weighted |
| Defensive AI threat detection | **LIVE** | LLM text, prompt injection, phishing, bots |
| Continuous daemon mode | **LIVE** | Phase-rotating autonomous operation |
| Training data: 457 examples × 49 domains | **LIVE** | Security, math, code, defense, surveillance |

## Architecture

```
 ┌───────────────────────────────────────────────────────────────┐
 │                         LFI VSA Core                          │
 │                                                               │
 │  ┌──────────────┐  ┌──────────────┐  ┌────────────────────┐   │
 │  │  HDC Engine  │──│ PSL Auditor  │──│  Provenance        │   │
 │  │  (10k-dim    │  │  (10 axioms, │  │  (TracedDerivation │   │
 │  │   bipolar)   │  │   CARTA)     │  │   vs Reconstructed)│   │
 │  └──────────────┘  └──────────────┘  └────────────────────┘   │
 │         │                  │                  │               │
 │         └─────────┬────────┴──────────────────┘               │
 │                   │                                           │
 │  ┌────────────────┴────────────────────────────────────┐      │
 │  │              Intelligence Layer                     │      │
 │  │                                                     │      │
 │  │  • Self-Improvement Engine (OODA meta-learning)     │      │
 │  │  • Cross-Domain Reasoning (Gentner transfer)        │      │
 │  │  • Epistemic Filter (asymptotic confidence)         │      │
 │  │  • Defensive AI (threat detection)                  │      │
 │  │  • Generalization Tester (rote vs understanding)    │      │
 │  │  • Math Engine (verified step-by-step)              │      │
 │  │  • Code Evaluator (sandbox execution)               │      │
 │  │  • Local Inference (Ollama/Gemini/Claude/HTTP)      │      │
 │  │  • Daemon Mode (continuous operation)               │      │
 │  └─────────────────────────────────────────────────────┘      │
 └───────────────────────────────────────────────────────────────┘
```

## Test Coverage

**704 tests, 0 failures** across 70+ modules.

| Layer | Tests |
|---|---|
| HDC Core (vector, holographic, compute, liquid) | 80+ |
| PSL Governance (10 axioms, supervisor, coercion) | 45+ |
| Cognition (reasoner, MCTS, planner, knowledge) | 75+ |
| Intelligence (training, code eval, self-improve, defensive, generalization) | 120+ |
| HDLM (AST, codebook, intercept, renderers) | 35+ |
| Crypto Epistemology (commitments, provenance) | 15+ |
| Integration tests (adversarial, stress, pipeline) | 50+ |

## Quick Start

Prerequisites: Rust 1.75+, Ollama (optional for real LLM training).

```bash
git clone https://github.com/thepictishbeast/PlausiDen-AI.git
cd PlausiDen-AI/lfi_vsa_core
cargo test               # 704 tests should pass
```

**Run with real LLM training (requires Ollama):**

```bash
# Install Ollama and pull a lightweight model
curl -fsSL https://ollama.com/install.sh | sh
ollama pull qwen2.5-coder:7b

# Build and run training
cd lfi_vsa_core
cargo run --release --bin ollama_train -- --examples 50
```

See [OWNERS_GUIDE.md](OWNERS_GUIDE.md) for a plain-English setup and usage walkthrough.

## Core Principles

1. **Material reality > probabilistic prediction.** Every output is verifiable, not guessed.
2. **Epistemic honesty.** LFI distinguishes traced derivations from post-hoc rationalizations and labels them accordingly.
3. **Asymptotic confidence.** No claim reaches 100% certainty. Even formal proofs cap at 99.99%.
4. **Skeptical intake.** Unknown sources get low initial confidence. Corroboration from reputable sources required for promotion.
5. **Sovereign operation.** Runs entirely on your hardware. No cloud, no telemetry, no data collection.
6. **Defense in depth.** Multi-layer threat detection. Assume the attacker is AI-powered.
7. **Self-improvement over static training.** LFI is designed to compound its intelligence autonomously.

## Security Posture

- `#![forbid(unsafe_code)]` at crate root
- All public APIs return `Result<T, E>` or `Option<T>` — no implicit panics
- UTF-8 safe string handling throughout (no byte-slicing panics)
- Memory-leak-free (no `Box::leak()` in production paths)
- CARTA trust model: Untrusted → Suspicious → Provisional → Verified → Sovereign
- Every axiom evaluation produces a signed provenance trace

## Subsystem Documentation

- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) — System architecture deep-dive
- [docs/HDC_OPERATIONS.md](docs/HDC_OPERATIONS.md) — VSA mathematical foundations
- [docs/PSL_SUPERVISOR.md](docs/PSL_SUPERVISOR.md) — Axiom governance framework
- [docs/SECURITY.md](docs/SECURITY.md) — Threat model and mitigations
- [IMPROVEMENTS.md](IMPROVEMENTS.md) — Active development roadmap
- [OWNERS_GUIDE.md](OWNERS_GUIDE.md) — Plain-English setup and usage

## Hardware Targets

| Device | Status |
|---|---|
| Kali Linux / Debian workstation (i7/64GB/GPU) | Primary dev |
| Pixel 10 Pro XL (Tensor G5 "Laguna") | Planned (NDK build) |
| Cloud VPS (for always-on training) | Supported |

## Mission

LFI is the core defensive component of [PlausiDen](https://github.com/thepictishbeast), a sovereign technology stack that gives individual users the same defensive capabilities that state actors and corporations already have. Every citizen deserves a sovereign AI defender that answers only to them.

## License

Proprietary — PlausiDen Technologies. All rights reserved.
Contact the maintainer for licensing discussions.

---

**Current version: Active Development (training pipeline LIVE)**
**Last updated: 2026-04-14**
