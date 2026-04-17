# PlausiDen AI Data Pipeline

How training data flows from generation through validation to model training.

## Pipeline Stages

```
[Data Sources] → [Generation] → [Validation] → [Storage] → [Training]
```

### Stage 1: Data Sources
- **External datasets:** Dolly 15K, OASST2, CVE v5, Wikidata
- **Ollama generation:** Magpie v2, conversational, domain-focused
- **Hand-crafted:** Security tools, hardware, tool-use examples
- **Brain.db derived:** Q&A pairs from 56M+ curated facts

### Stage 2: Generation
Scripts in `/root/LFI/scripts/`:
- `magpie_generate_v2.py` — Domain-focused instruction pairs via Ollama
- `generate_conversational_data.py` — Multi-turn dialogue generation
- `generate_security_training_data.py` — Linux/Kali/blue/red team data
- `generate_hardware_training_data.py` — Serial/HID/USB training
- `generate_tool_use_data.py` — OS navigation and tool-use examples
- `parse_cve_v5.py` — CVE security vulnerability fact extraction

### Stage 3: Validation
- Minimum length filtering (instruction >= 20, output >= 50)
- SHA-256 deduplication
- Generic greeting rejection
- Domain balance checking
- Contamination detection against benchmarks

### Stage 4: Storage
- JSONL files in `/home/user/LFI-data/`
- Brain.db for live RAG (56M+ facts)
- GitHub via git LFS for version control

### Stage 5: Training
- **LoRA/QLoRA:** Fine-tune qwen2.5-coder:7b on instruction pairs
- **ORPO:** Preference optimization (reference-free)
- **GRPO:** Reinforcement learning with verifiable rewards
- **RAG:** FTS5 search → quality-weighted ranking → prompt injection

## JSONL Format Standard

### Single-turn (Alpaca format)
```json
{
    "instruction": "User's question or request",
    "input": "Optional additional context",
    "output": "AI's response",
    "source": "dataset_name",
    "domain": "category"
}
```

### Multi-turn (Conversation format)
```json
{
    "conversations": [
        {"role": "user", "content": "First message"},
        {"role": "assistant", "content": "First response"},
        {"role": "user", "content": "Follow-up"},
        {"role": "assistant", "content": "Follow-up response"}
    ],
    "category": "task_completion",
    "turns": 2,
    "source": "conversational_training"
}
```

### Tool-use (Extended format)
```json
{
    "instruction": "Task description",
    "input": "",
    "output": "Response with command examples",
    "tool_calls": [{"tool": "bash", "command": "actual command"}],
    "source": "tool_use_training",
    "domain": "category"
}
```
