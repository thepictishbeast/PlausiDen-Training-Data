Architectural Bible saved — see /home/user/Development/PlausiDen/ for the full spec. Key constraints this codebase must honor:

- PSA filter (Privacy, Security, Anonymity) on every feature
- No phone-home, no telemetry by default, no cloud dependency for core function
- Dispatch metadata (tier/mode/confidence) NEVER in AI response text
- SQLite + AES-256-GCM encryption at rest for all persistent data
- Tool trait interface per the spec
- DM Sans / Outfit for UI typography, JetBrains Mono for code
- 6-level surface hierarchy for themes
- DerivationTrace is first-class, not decorative

---

## Out of Scope

Per v1.2 §G.3. PlausiDen-Training-Data curates the datasets and
prompts that fine-tune the project's local AI models. It is a
data-engineering pipeline, not a shipped consumer deliverable. It
does NOT:

- **Ship training data to third parties.** All artifacts stay
  local or move through PlausiDen-owned infrastructure. No
  Hugging Face uploads, no cloud bucket mirrors, no
  telemetry-backed collection of user prompts.
- **Include PII or user-identifying content.** Every dataset
  entry passes the PSA (Privacy / Security / Anonymity) filter
  before landing. Synthetic or public-domain sources only.
- **Generate real user data.** The datasets here are for TRAINING
  models that produce plausibly-real-looking synthetic data; they
  are not themselves a source of real data. A model trained here
  injects its synthetic output via `plausiden-inject` downstream.
- **Provide model inference.** Inference + serving live in
  `PlausiDen-AI` (Claude 2's workspace) and consumer-facing
  repos like `PlausiDen-Desktop`. This repo only produces the
  training artifacts.
- **Enforce runtime privacy guarantees.** DerivationTrace
  metadata travels with each dataset entry, but enforcement
  (e.g. differential-privacy budget consumption at inference)
  is the consuming repo's responsibility.
- **Support arbitrary model architectures.** The pipeline assumes
  local-run LLMs at the scale we're willing to ship on a user's
  hardware (current target: 7-13B parameter models post-
  quantization). Scaling to foundation-model sizes is out of
  scope and would introduce cloud dependencies this project
  refuses to ship.
- **Act as the canonical model repository.** Model artifacts,
  weights, and inference configuration live in `PlausiDen-AI`.
  This repo produces training INPUTS, not model outputs.

Contributors should consult the AI-side OPSEC docs before adding
any new data source.
