# Domain Coverage — PlausiDen AI

## Well-Covered (>1M facts)
- web_knowledge: 26.5M (47%)
- encyclopedic: 5.8M (10%)
- multilingual: 5.5M (10%)
- commerce: 3.6M (6%)
- knowledge: 3.3M (6%)
- news_topics: 2.6M (5%)
- instruction: 1.8M (3%)
- nli: 1.5M (3%)
- qa_general: 1.4M (2%)
- commonsense: 1.0M (2%)

## Adequate (100K-1M)
- sentiment: 816K | conversational: 561K
- code: 479K | biomedical: 394K
- reasoning: 347K | reading_comprehension: 245K
- qa_extractive: 219K | mathematics: 207K
- adversarial: 137K | academic: 128K

## Thin (10K-100K)
- cybersecurity: 36K | science: 33K
- finance: 10K

## CRITICAL GAPS (<10K)
- history: 6.8K | legal: 5K | business: 3.5K
- philosophy: 469 | technology: 75
- pentesting: 28 | economics: 21 | politics: 20
- social_science: 5 | communication: 3

## Action Items
1. Generate 10K+ pentesting/economics/politics/philosophy via Ollama
2. Reclassify web_knowledge with keyword filters
3. Ingest legal datasets (CUAD, CaseHOLD when HF fixes scripts)
4. Download history datasets (Wikipedia history articles)
