# Training Datasets — PlausiDen AI

## Overview
- **Total facts:** 56,387,692
- **Distinct sources:** 169
- **FTS5 indexed:** 100%
- **Quality scored:** 99.99%

## Core Web Knowledge
| Dataset | Count | Quality | Domain |
|---------|------:|--------:|--------|
| C4 (all batches) | ~26.5M | 0.65-0.80 | web_knowledge |
| OpenWebText | 5M | 0.75 | web_knowledge |
| Wikipedia (1+2) | 5M | 0.90 | encyclopedic |
| Pile (uncopyrighted) | 1M | 0.80 | web_knowledge |

## NLI & Reasoning
| Dataset | Count | Quality | Domain |
|---------|------:|--------:|--------|
| ANLI (r1+r2+r3) | 163K | 0.90 | nli |
| SNLI | 549K | 0.90 | nli |
| XNLI (13 langs) | 5.5M | 0.85 | multilingual |
| AquaRAT | 81K | 0.90 | reasoning |
| WinoGrande | 40K | 0.90 | commonsense |

## Adversarial
| Dataset | Count | Quality | Domain |
|---------|------:|--------:|--------|
| FEVER gold | 228K | 0.90 | adversarial |
| TruthfulQA | 1.6K | 0.95 | adversarial |
| Curated | 1K | 0.95 | adversarial |

## Knowledge Graphs
| Dataset | Count | Quality | Domain |
|---------|------:|--------:|--------|
| ConceptNet 5.7 | 3.84M | 0.85 | knowledge |
| MITRE ATT&CK | 35K | 0.90 | cybersecurity |
| CWE | 969 | 0.95 | cybersecurity |

## Contamination: arc(67%), truthfulqa(52%), gsm8k(23%) flagged
