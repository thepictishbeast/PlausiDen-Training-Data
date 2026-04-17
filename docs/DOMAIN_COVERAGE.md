# Domain Coverage Analysis

## Current Coverage

| Domain | Examples | Source | Quality | Gap Assessment |
|--------|----------|--------|---------|----------------|
| Cybersecurity | 44 + CVE facts | Security training + CVE v5 | High | Need more offensive/defensive scenarios |
| Linux Tools | 10 | Security training | High | Need 50+ more (systemd, networking, storage) |
| Debian/APT | 4 | Security training | High | Need 20+ (packaging, upgrades, troubleshooting) |
| Kali Tools | 9 | Security training | High | Need 30+ (recon, exploit, post-exploit, reporting) |
| Blue Team | 7 | Security training | High | Need 20+ (SIEM, EDR, forensics, IR) |
| Red Team | 4 | Security training | High | Need 20+ (social engineering, pivoting, C2) |
| Anonymity | 3 | Security training | High | Need 10+ (Tor advanced, I2P, mix networks) |
| Privacy | 4 | Security training | High | Need 15+ (PGP workflow, encrypted comms, OPSEC) |
| Serial/Hardware | 14 | Hardware training | High | Need 20+ (CAN bus, JTAG, firmware extraction) |
| HID/USB | 6 | Hardware training | High | Need 15+ (USB protocols, HID descriptors) |
| Tool Use | 30 | Tool-use training | High | Need 200+ across all categories |
| General Knowledge | 14,080 | Dolly 15K | Medium | Good baseline, needs domain-specific augmentation |
| Rust Programming | ~50 | Magpie v2 | Medium | Need 200+ (ownership, async, unsafe, macros) |
| Machine Learning | ~50 | Magpie v2 | Medium | Need 200+ (transformers, training, evaluation) |
| Distributed Systems | ~50 | Magpie v2 | Medium | Need 100+ (consensus, CRDTs, mesh) |
| Databases | ~50 | Magpie v2 | Medium | Need 100+ (SQLite, FTS5, optimization) |
| Philosophy | ~30 | Magpie v2 | Medium | Need 50+ (consciousness, ethics, epistemology) |
| Mathematics | ~50 | Magpie v2 | Medium | Need 100+ (linear algebra, probability, information theory) |
| Conversational | 3 + target 300 | Ollama generated | Medium | CRITICAL GAP — need 10,000+ |

## Critical Gaps (Priority Order)

1. **Conversational/Multi-turn:** Only 3 examples. Need 10,000+ to teach natural interaction.
2. **Tool-use execution:** Only 30 examples. Need 500+ showing actual command execution patterns.
3. **Error recovery:** Almost none. Need 200+ examples of AI making mistakes and correcting.
4. **Task completion:** Very few. Need 500+ end-to-end task execution dialogues.
5. **OS navigation:** Limited to Linux. Need Windows, macOS, container, and embedded coverage.

## Planned Additions

- OASST2 dataset (~10K conversations)
- Brain.db fact-derived Q&A pairs (~5K)
- Expanded Ollama generation across all domains
- CAN bus / automotive security
- JTAG/firmware extraction
- SIEM/EDR integration
- Incident response procedures
- Forensic analysis workflows
