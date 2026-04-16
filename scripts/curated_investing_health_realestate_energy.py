import sqlite3, hashlib
DB = "/home/user/.local/share/plausiden/brain.db"
def get_conn():
    conn = sqlite3.connect(DB, timeout=300)
    conn.execute("PRAGMA journal_mode=WAL")
    conn.execute("PRAGMA busy_timeout=300000")
    return conn
def mk(p,t): return f"{p}_{hashlib.md5(t.encode()).hexdigest()[:8]}"
def ins(conn, facts, src, dom, q):
    c = conn.cursor()
    n = 0
    for t in facts:
        try:
            c.execute("INSERT OR IGNORE INTO facts (key,value,source,confidence,domain,quality_score) VALUES (?,?,?,?,?,?)", (mk(src,t),t,src,q,dom,q))
            n += c.rowcount
        except: pass
    conn.commit()
    return n

investing = [
    "Value investing (Buffett/Graham): Buy companies trading below intrinsic value. Metrics: P/E ratio (price/earnings — lower = cheaper), P/B (price/book value), FCF yield (free cash flow/market cap), ROIC (return on invested capital — measures capital allocation skill). Margin of safety: buy at significant discount to intrinsic value. 'Be fearful when others are greedy, greedy when others are fearful.'",
    "Portfolio theory (Markowitz): Diversification reduces risk without proportionally reducing returns. Efficient frontier: set of portfolios offering highest return for each risk level. Modern practice: 60/40 stocks/bonds (traditional), all-weather (Dalio: stocks, bonds, gold, commodities), factor investing (value, momentum, quality, size). Rebalance periodically to maintain target allocation.",
    "Options basics: Call (right to buy at strike price) vs Put (right to sell). Premium = intrinsic value + time value. Greeks: Delta (price sensitivity to underlying), Gamma (rate of delta change), Theta (time decay — options lose value daily), Vega (volatility sensitivity), Rho (interest rate sensitivity). Covered calls (income strategy), protective puts (insurance), straddles (bet on volatility).",
]

health_science = [
    "Sleep science: Adults need 7-9 hours. Sleep cycles: ~90 min (NREM stages 1-3 + REM). Deep sleep (NREM3) = physical restoration, memory consolidation. REM = emotional processing, creativity. Sleep debt is real and cumulative. Blue light suppresses melatonin — avoid screens 1hr before bed. Consistent wake time matters more than bedtime. Caffeine half-life = 5-6 hours — avoid after noon.",
    "Exercise physiology: Aerobic (running, cycling — improves cardiovascular fitness, VO2max) vs Resistance (weight training — builds muscle, increases metabolic rate, preserves bone density). Minimum effective dose: 150 min/week moderate aerobic OR 75 min vigorous + 2 resistance sessions. Progressive overload: gradually increase weight/reps/volume. Recovery is when adaptation happens — don't overtrain.",
    "Nutrition fundamentals: Macros — Protein (4 cal/g, 0.7-1g per lb body weight for muscle, essential amino acids), Carbs (4 cal/g, primary energy source, fiber important — 25-30g/day), Fat (9 cal/g, essential for hormones, brain function — focus on unsaturated). Micronutrients: Vitamin D (most people deficient), Omega-3 (EPA/DHA from fish/algae), Magnesium, Iron (especially women). Calorie balance determines weight change.",
    "Mental health basics: CBT (Cognitive Behavioral Therapy — identify and reframe distorted thinking patterns, evidence-based for anxiety + depression), exposure therapy (gradual confrontation of fears — highly effective for phobias/OCD), mindfulness (present-moment awareness, reduces rumination), exercise (as effective as medication for mild-moderate depression in meta-analyses). Seeking help is strength, not weakness.",
]

real_estate = [
    "Real estate investing: Cash flow = rental income - (mortgage + taxes + insurance + maintenance + vacancy). Cap rate = NOI / property value (higher = better yield, more risk). 1% rule: monthly rent should be ≥1% of purchase price. Leverage: 20% down = 5x leverage (amplifies gains AND losses). Appreciation + cash flow + tax benefits (depreciation) + equity buildup = total return.",
    "Real estate analysis: Comparable sales (comps) for valuation, rental comps for income estimation, inspect before buying (foundation, roof, HVAC, plumbing, electrical). Due diligence: title search, survey, environmental assessment, zoning verification. House hacking: live in one unit of multi-family, rent others — reduces or eliminates personal housing cost while building portfolio.",
]

energy_climate = [
    "Climate science: CO2 concentrations have risen from 280 ppm (pre-industrial) to 420+ ppm. Global average temperature up ~1.1°C since 1850. Effects: sea level rise (3.6mm/year), more intense weather events, ocean acidification, ecosystem disruption, ice sheet melting. Paris Agreement target: limit to 1.5°C. IPCC: need net-zero CO2 by 2050 for 1.5°C pathway.",
    "Energy transition: Solar costs dropped 89% since 2010 (now cheapest new electricity in most of world). Wind dropped 70%. Battery storage costs dropping ~15%/year. Nuclear provides reliable baseload but expensive to build. Green hydrogen for hard-to-electrify sectors (steel, shipping, aviation). Grid modernization: smart grids, demand response, interconnectors, long-duration storage (iron-air, compressed air, gravity).",
]

conn = get_conn()
t = 0
t += ins(conn, investing, "curated_investing", "finance", 0.95)
t += ins(conn, health_science, "curated_health", "science", 0.93)
t += ins(conn, real_estate, "curated_realestate", "finance", 0.93)
t += ins(conn, energy_climate, "curated_energy", "science", 0.93)
conn.close()
print(f"Inserted {t} curated facts (investing, health, real estate, energy/climate)")
