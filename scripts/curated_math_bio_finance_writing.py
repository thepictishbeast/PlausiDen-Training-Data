import sqlite3, hashlib

DB = "/home/user/.local/share/plausiden/brain.db"
def get_conn():
    conn = sqlite3.connect(DB, timeout=300)
    conn.execute("PRAGMA journal_mode=WAL")
    conn.execute("PRAGMA busy_timeout=300000")
    return conn

def make_key(prefix, text):
    return f"{prefix}_{hashlib.md5(text.encode()).hexdigest()[:8]}"

def insert_facts(conn, facts, source, domain, quality):
    cur = conn.cursor()
    count = 0
    for text in facts:
        try:
            cur.execute("INSERT OR IGNORE INTO facts (key, value, source, confidence, domain, quality_score) VALUES (?,?,?,?,?,?)",
                (make_key(source, text), text, source, quality, domain, quality))
            count += cur.rowcount
        except: pass
    conn.commit()
    return count

mathematics = [
    "Calculus fundamentals: Derivative = instantaneous rate of change (slope of tangent line). Integral = accumulated area under a curve. Fundamental Theorem: differentiation and integration are inverse operations. Applications: physics (velocity/acceleration), economics (marginal cost/revenue), ML (gradient descent optimizes loss functions by following derivatives).",
    "Linear algebra for ML: Vectors (direction + magnitude), matrices (linear transformations), eigenvalues/eigenvectors (directions unchanged by transformation — PCA uses these), matrix multiplication (composition of transformations), SVD (Singular Value Decomposition — used in recommendation systems, dimensionality reduction). Every neural network is fundamentally matrix multiplications + nonlinear activations.",
    "Probability and statistics: Bayes' theorem P(A|B) = P(B|A)·P(A)/P(B) — update beliefs with new evidence. Central limit theorem: sample means approach normal distribution regardless of population distribution. Law of large numbers: sample average converges to expected value. Conditional probability is counterintuitive: Monty Hall problem, base rate neglect, prosecutor's fallacy.",
    "Information theory (Shannon): Entropy H(X) = -Σ p(x)·log₂p(x) — measures uncertainty/surprise. Maximum entropy = uniform distribution. Cross-entropy: measures how well a predicted distribution matches the true distribution — used as loss function in classification NNs. KL-divergence: asymmetric distance between distributions. Mutual information: shared information between variables.",
    "Graph theory basics: Nodes (vertices) and edges (connections). Directed vs undirected. Weighted vs unweighted. Key algorithms: BFS/DFS (traversal), Dijkstra (shortest path), Bellman-Ford (negative weights), Floyd-Warshall (all pairs), Kruskal/Prim (minimum spanning tree), topological sort (DAGs). Applications: social networks, routing, dependency resolution, PageRank.",
]

biology = [
    "DNA and genetics: DNA = double helix of nucleotides (A-T, C-G base pairs). Gene = DNA segment encoding a protein. Human genome: ~3 billion base pairs, ~20,000 protein-coding genes (only ~1.5% of DNA). Central dogma: DNA → (transcription) → mRNA → (translation) → Protein. Mutations: substitution, insertion, deletion. CRISPR-Cas9 enables precise gene editing since 2012.",
    "Evolution by natural selection (Darwin): Variation exists in populations, some variants are better suited to the environment, those individuals reproduce more, traits are inherited. Modern synthesis adds: random mutation as variation source, genetic drift (random changes in small populations), gene flow (migration), sexual selection. Evolution is not directed toward a goal — it's a blind optimization process.",
    "Cell biology: Prokaryotes (no nucleus — bacteria, archaea) vs Eukaryotes (nucleus + organelles — animals, plants, fungi). Key organelles: nucleus (DNA storage), mitochondria (energy/ATP — has its own DNA, formerly independent organism), ribosome (protein synthesis), ER (protein processing), Golgi (packaging/transport). Cell division: mitosis (identical copies) vs meiosis (sex cells, genetic recombination).",
    "Immune system: Innate immunity (first line — skin, inflammation, phagocytes, complement — fast but non-specific) and Adaptive immunity (B-cells produce antibodies, T-cells kill infected cells — slow initial response but creates memory). Vaccines work by training adaptive immunity without causing disease. Autoimmune diseases: immune system attacks own tissue. Immunotherapy: harnessing immune system against cancer.",
]

personal_finance = [
    "Compound interest: A = P(1 + r/n)^(nt). Albert Einstein allegedly called it the 8th wonder of the world. Rule of 72: years to double = 72/interest rate. Starting early matters enormously — $10K at age 25 vs 35 at 7% return: $150K vs $76K at age 65. Time in market beats timing the market. Dollar-cost averaging reduces impact of volatility.",
    "Investment vehicles: Index funds (track market index, low fees — S&P 500 averages ~10% annually), ETFs (exchange-traded funds, like index funds but traded like stocks), Bonds (fixed income, lower risk/return), REITs (real estate exposure without buying property), 401(k)/IRA (tax-advantaged retirement — always capture employer match). Diversification reduces risk without necessarily reducing returns.",
    "Personal finance fundamentals: Emergency fund (3-6 months expenses in high-yield savings), pay off high-interest debt first (avalanche method — highest rate first, or snowball — smallest balance first for psychological wins), budget (50/30/20: needs/wants/savings), increase income (skills, side projects), tax optimization (max retirement accounts, HSA), insurance (health, disability, term life if dependents).",
    "Credit and debt: Credit score factors (FICO): Payment history 35%, amounts owed 30%, length of history 15%, credit mix 10%, new credit 10%. Good credit saves thousands on mortgages and loans. High-interest debt (credit cards ~20% APR) is an emergency — pay off before investing. Student loans: income-driven repayment, PSLF, refinancing. Mortgage: 20% down avoids PMI, 15-year saves massively on interest vs 30-year.",
]

writing = [
    "Storytelling structure: Three-act structure (Setup → Confrontation → Resolution). Hero's Journey (Campbell): Ordinary World → Call to Adventure → Refusal → Mentor → Crossing Threshold → Tests/Allies/Enemies → Approach → Ordeal → Reward → Road Back → Resurrection → Return. In business: use stories to make data memorable (stories are 22x more memorable than facts alone).",
    "Clear writing principles (William Zinsser, 'On Writing Well'): Use simple words, short sentences, active voice. Cut every unnecessary word ('very', 'really', 'basically', 'in order to' → 'to'). One idea per sentence. Rewrite ruthlessly — first drafts are always too long. Writing is thinking on paper — if the writing is unclear, the thinking is unclear. Read your work aloud to catch awkward phrasing.",
    "Technical documentation: README should answer: What is this? Why should I care? How do I install/use it? Common patterns: quickstart guide, API reference, tutorials, how-to guides (Diátaxis framework). Write for the reader's context, not your knowledge level. Include working code examples. Keep docs next to code (they rot slower). Automate what you can (API docs from code, changelogs from commits).",
]

conn = get_conn()
total = 0
total += insert_facts(conn, mathematics, "curated_math", "mathematics", 0.95)
total += insert_facts(conn, biology, "curated_biology", "science", 0.95)
total += insert_facts(conn, personal_finance, "curated_pf", "finance", 0.95)
total += insert_facts(conn, writing, "curated_writing", "communication", 0.95)
conn.close()
print(f"Inserted {total} curated facts (math, biology, personal finance, writing)")
