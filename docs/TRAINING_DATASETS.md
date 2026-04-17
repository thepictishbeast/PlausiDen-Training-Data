# PlausiDen AI Training Datasets

Master inventory of all training data for PlausiDen AI fine-tuning and RAG.

## Dataset Summary

| Dataset | Source | Pairs | Size | Domain | Status |
|---------|--------|-------|------|--------|--------|
| Dolly 15K | Databricks | 14,080 | ~13MB | General | Converted |
| Security Training | Hand-crafted | 44 | 36KB | Security/Linux/Kali | Complete |
| Tool-Use Training | Hand-crafted | 30 | 19KB | OS/CLI tools | Complete |
| Hardware Training | Hand-crafted | 14 | 16KB | Serial/HID/USB | Complete |
| Magpie v2 (Domain) | Ollama-generated | ~500 target | ~200KB | 12 domains | Generating |
| Conversational | Ollama-generated | ~300 target | ~50KB | Multi-turn | Generating |
| CVE v5 Facts | CVEProject GitHub | ~400K target | ~500MB | Cybersecurity | Parsing |
| Brain.db Facts | Curated | 56,387,692 | 74GB | All domains | Live |
| OASST2 | OpenAssistant | ~10K target | ~50MB | Conversational | Pending |
| Fact Q&A Pairs | Brain.db derived | ~5K target | ~10MB | All domains | Pending |

## Dataset Descriptions

### Dolly 15K (databricks-dolly-15k)
- **Source:** Databricks (CC-BY-SA 3.0)
- **Format:** JSONL with instruction/context/response/category
- **Categories:** brainstorming, classification, closed_qa, creative_writing, general_qa, information_extraction, open_qa, summarization
- **Quality:** High — human-written by Databricks employees

### Security Training
- **Source:** Hand-crafted by Claude 0
- **Domains:** linux_tools (10), debian_tools (4), kali_tools (9), blue_team (7), red_team (4), anonymity (3), privacy (4), security (3)
- **Quality:** High — expert-level command examples with explanations
- **Covers:** nmap, wireshark, burp, hashcat, sqlmap, metasploit, fail2ban, AIDE, auditd, nftables, AppArmor, Tor, GPG, LUKS, WireGuard

### Tool-Use Training
- **Source:** Hand-crafted by Claude 0
- **Domains:** file_operations, system_diagnostics, git_operations, database_operations, network_operations, security_operations, process_management, ai_operations, package_management, text_processing
- **Format:** Includes `tool_calls` field showing exact commands

### Hardware Training
- **Source:** Hand-crafted by Claude 0
- **Domains:** serial_devices (8), hid_devices (6)
- **Covers:** USB serial, I2C, SPI, GPIO, Bluetooth/BLE, USB HID, keystroke injection, BadUSB defense, USB traffic analysis, udev auto-configuration

### Magpie v2 (Domain-Focused)
- **Source:** Ollama-generated via qwen2.5-coder:7b
- **Method:** Topic-seeded question generation across 12 domains
- **Domains:** cybersecurity, rust_programming, machine_learning, distributed_systems, mathematics, neuroscience, systems_programming, databases, philosophy_of_mind, formal_methods, hyperdimensional_computing, privacy_sovereignty

### Conversational Training
- **Source:** Ollama-generated multi-turn dialogues
- **Categories:** task_completion, knowledge_qa, error_recovery, coding_help, sysadmin, philosophy, tool_use
- **Format:** Both multi-turn (conversations array) and single-turn (instruction/output)

### CVE v5 Facts
- **Source:** CVEProject/cvelistV5 (342,675 CVE JSON files)
- **Extracted fields:** CVE ID, description, CVSS score/severity, affected products, CWE IDs, publication date
- **Quality scoring:** Based on data richness (CVSS, products, CWE, description length)

### Brain.db (Live Knowledge Base)
- **Location:** `~/.local/share/plausiden/brain.db`
- **Facts:** 56,387,692
- **Features:** FTS5 full-text search, quality scoring, storage tiering, contamination flagging
- **Indexes:** domain, quality, temporal, contamination, tier

## Data Quality Standards

1. **Minimum lengths:** instruction >= 20 chars, output >= 50 chars
2. **Deduplication:** SHA-256 content hashing
3. **Filtering:** Reject generic greetings, filler responses
4. **Domain balance:** Target equal representation across domains
5. **Contamination checking:** Overlap detection with benchmark datasets

## File Locations

- Training data: `/home/user/LFI-data/`
- Generation scripts: `/root/LFI/scripts/`
- Brain database: `~/.local/share/plausiden/brain.db`
