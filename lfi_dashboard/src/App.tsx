// ============================================================
// Sovereign Command Console (SCC) v4.0 — Production Dashboard
//
// PROTOCOL: Real-time WebSocket integration with LFI Cognitive Core
// SUBSTRATE: React, inline styles + CSS media queries (no framework)
// LAYOUT: Mobile-first, responsive to tablet and desktop
//
// BREAKPOINTS:
//   Mobile:  < 768px  — Single column, collapsible panels
//   Tablet:  768-1199 — Wider chat, collapsible telemetry
//   Desktop: >= 1200  — Persistent telemetry sidebar, wide chat
//
// ENDPOINTS:
//   ws://<host>:3000/ws/chat       — Bidirectional cognitive chat
//   ws://<host>:3000/ws/telemetry  — Real-time substrate telemetry
//   POST /api/auth                 — Sovereign key verification
//   POST /api/tier                 — Model tier switching
//   GET  /api/status               — Substrate status
//   GET  /api/facts                — Knowledge facts
//   GET  /api/qos                  — QoS compliance report
//
// DEBUG: console.debug() on every state change for Eruda inspector
// FIX: Eruda FAB positioned to avoid input bar overlap
// ============================================================

import React, { useState, useEffect, useRef, useCallback } from 'react';

// ---- Responsive hook ----
type Breakpoint = 'mobile' | 'tablet' | 'desktop';

function useBreakpoint(): Breakpoint {
  const [bp, setBp] = useState<Breakpoint>(() => {
    if (typeof window === 'undefined') return 'mobile';
    const w = window.innerWidth;
    if (w >= 1200) return 'desktop';
    if (w >= 768) return 'tablet';
    return 'mobile';
  });

  useEffect(() => {
    const onResize = () => {
      const w = window.innerWidth;
      const next: Breakpoint = w >= 1200 ? 'desktop' : w >= 768 ? 'tablet' : 'mobile';
      setBp(prev => {
        if (prev !== next) {
          console.debug("// SCC: Breakpoint changed:", prev, "->", next, "width:", w);
          return next;
        }
        return prev;
      });
    };
    window.addEventListener('resize', onResize);
    return () => window.removeEventListener('resize', onResize);
  }, []);

  return bp;
}

// ---- Types ----
interface ChatMessage {
  id: number;
  role: 'user' | 'assistant' | 'system' | 'web';
  content: string;
  mode?: string;
  confidence?: number;
  tier?: string;
  intent?: string;
  reasoning?: string[];
  plan?: { steps: number; complexity: number; goal: string };
  timestamp: number;
}

interface SubstrateStats {
  ram_available_mb: number;
  cpu_temp_c: number;
  vsa_orthogonality: number;
  axiom_pass_rate: number;
  is_throttled: boolean;
  logic_density: number;
}

interface QosReport {
  tier: string;
  overall_pass: boolean;
  checks: { name: string; pass: boolean; detail: string }[];
}

// ---- Color palette (high-contrast) ----
const C = {
  bg: '#08090f',
  bgCard: '#0f1019',
  bgInput: '#12131e',
  bgHover: '#1a1b2e',
  border: 'rgba(255,255,255,0.10)',
  borderFocus: 'rgba(99,140,255,0.45)',
  borderSubtle: 'rgba(255,255,255,0.06)',
  text: '#e8eaf0',
  textSecondary: '#a0a8c0',
  textMuted: '#6b7394',
  textDim: '#4a5072',
  accent: '#638cff',
  accentGlow: 'rgba(99,140,255,0.4)',
  accentBg: 'rgba(99,140,255,0.12)',
  accentBorder: 'rgba(99,140,255,0.25)',
  green: '#5bf0a0',
  greenBg: 'rgba(91,240,160,0.10)',
  greenBorder: 'rgba(91,240,160,0.20)',
  red: '#ff6b7a',
  redBg: 'rgba(255,107,122,0.10)',
  redBorder: 'rgba(255,107,122,0.20)',
  purple: '#c49cff',
  purpleBg: 'rgba(196,156,255,0.10)',
  purpleBorder: 'rgba(196,156,255,0.20)',
  yellow: '#ffd666',
  yellowBg: 'rgba(255,214,102,0.10)',
  font: "'SF Mono', 'Cascadia Code', 'JetBrains Mono', 'Fira Code', ui-monospace, SFMono-Regular, Menlo, Consolas, monospace",
};

// ---- Main Component ----
const SovereignCommandConsole: React.FC = () => {
  const bp = useBreakpoint();
  const isDesktop = bp === 'desktop';
  const isTablet = bp === 'tablet';
  const isMobile = bp === 'mobile';
  console.debug("// SCC v4.0: Component mounting, breakpoint:", bp);

  // ---- State ----
  const [isAuthenticated, setIsAuthenticated] = useState(() => {
    const stored = localStorage.getItem('lfi_auth') === 'true';
    console.debug("// SCC: Auth from localStorage:", stored);
    return stored;
  });
  const [password, setPassword] = useState('');
  const [authError, setAuthError] = useState('');
  const [authLoading, setAuthLoading] = useState(false);
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [input, setInput] = useState('');
  const [isConnected, setIsConnected] = useState(false);
  const [isThinking, setIsThinking] = useState(false);
  const [expandedReasoning, setExpandedReasoning] = useState<number | null>(null);
  const [showTelemetry, setShowTelemetry] = useState(false);
  const [showAdmin, setShowAdmin] = useState(false);
  const [currentTier, setCurrentTier] = useState<string>('Pulse');
  const [tierSwitching, setTierSwitching] = useState(false);
  const [facts, setFacts] = useState<{ key: string; value: string }[]>([]);
  const [qosReport, setQosReport] = useState<QosReport | null>(null);
  const [adminLoading, setAdminLoading] = useState('');
  const [stats, setStats] = useState<SubstrateStats>({
    ram_available_mb: 0, cpu_temp_c: 0, vsa_orthogonality: 0.02,
    axiom_pass_rate: 1.0, is_throttled: false, logic_density: 0
  });

  const chatWsRef = useRef<WebSocket | null>(null);
  const telemetryWsRef = useRef<WebSocket | null>(null);
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLTextAreaElement>(null);

  // ---- Helpers ----
  const getHost = () => {
    const h = window.location.hostname || '127.0.0.1';
    console.debug("// SCC: Resolved host:", h);
    return h;
  };

  const scrollToBottom = useCallback(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, []);

  useEffect(() => { scrollToBottom(); }, [messages, scrollToBottom]);

  useEffect(() => {
    console.debug("// SCC: Persisting auth:", isAuthenticated);
    localStorage.setItem('lfi_auth', isAuthenticated.toString());
  }, [isAuthenticated]);

  // ---- Eruda FAB repositioning ----
  // Moves the Eruda floating action button above the input bar on mobile
  useEffect(() => {
    const moveEruda = () => {
      const erudaEntry = document.getElementById('eruda-entry-btn') ||
        document.querySelector('.eruda-entry-btn') as HTMLElement;
      if (erudaEntry) {
        console.debug("// SCC: Repositioning Eruda FAB");
        erudaEntry.style.bottom = isMobile ? '80px' : '20px';
        erudaEntry.style.right = '10px';
        erudaEntry.style.zIndex = '9998';
      }
    };
    // Try immediately and after a delay (Eruda may load asynchronously)
    moveEruda();
    const timer = setTimeout(moveEruda, 2000);
    return () => clearTimeout(timer);
  }, [isMobile, isAuthenticated]);

  // ---- WebSocket: Chat ----
  useEffect(() => {
    if (!isAuthenticated) {
      console.debug("// SCC: Skipping chat WS — not authenticated");
      return;
    }
    const wsUrl = `ws://${getHost()}:3000/ws/chat`;
    console.debug("// SCC: Connecting chat WS:", wsUrl);
    let reconnectTimer: ReturnType<typeof setTimeout>;

    const connect = () => {
      console.debug("// SCC: chat WS connect()");
      const ws = new WebSocket(wsUrl);
      chatWsRef.current = ws;

      ws.onopen = () => {
        console.debug("// SCC: Chat WS OPEN");
        setIsConnected(true);
        setMessages(prev => [...prev, {
          id: Date.now(), role: 'system', content: 'Cognitive link established.',
          timestamp: Date.now()
        }]);
      };

      ws.onmessage = (event) => {
        try {
          const msg = JSON.parse(event.data);
          console.debug("// SCC: Chat msg:", msg.type);

          if (msg.type === 'chat_response') {
            setIsThinking(false);
            setMessages(prev => [...prev, {
              id: Date.now(), role: 'assistant',
              content: msg.content || '',
              mode: msg.mode, confidence: msg.confidence,
              tier: msg.tier, intent: msg.intent,
              reasoning: msg.reasoning, plan: msg.plan,
              timestamp: Date.now(),
            }]);
            // Sync tier from response
            if (msg.tier) setCurrentTier(msg.tier);
          } else if (msg.type === 'web_result') {
            console.debug("// SCC: Web result, sources:", msg.source_count);
            setMessages(prev => [...prev, {
              id: Date.now(), role: 'web',
              content: `${msg.source_count} sources | trust: ${(msg.trust * 100).toFixed(0)}%\n\n${msg.summary}`,
              timestamp: Date.now(),
            }]);
          } else if (msg.type === 'chat_error') {
            console.debug("// SCC: Chat error:", msg.error);
            setIsThinking(false);
            setMessages(prev => [...prev, {
              id: Date.now(), role: 'system',
              content: `Error: ${msg.error}`, timestamp: Date.now(),
            }]);
          }
        } catch (e) {
          console.error("// SCC: Chat parse error:", e);
        }
      };

      ws.onclose = (ev) => {
        console.debug("// SCC: Chat WS CLOSED:", ev.code);
        setIsConnected(false);
        reconnectTimer = setTimeout(connect, 3000);
      };

      ws.onerror = (ev) => {
        console.error("// SCC: Chat WS ERROR:", ev);
        setIsConnected(false);
      };
    };

    connect();
    return () => { clearTimeout(reconnectTimer); chatWsRef.current?.close(); };
  }, [isAuthenticated]);

  // ---- WebSocket: Telemetry ----
  useEffect(() => {
    if (!isAuthenticated) return;
    const wsUrl = `ws://${getHost()}:3000/ws/telemetry`;
    console.debug("// SCC: Connecting telemetry WS:", wsUrl);
    let reconnectTimer: ReturnType<typeof setTimeout>;

    const connect = () => {
      const ws = new WebSocket(wsUrl);
      telemetryWsRef.current = ws;
      ws.onmessage = (event) => {
        try {
          const msg = JSON.parse(event.data);
          if (msg.type === 'telemetry' && msg.data) {
            setStats(prev => ({ ...prev, ...msg.data }));
          }
        } catch (e) { console.error("// SCC: Telemetry parse error:", e); }
      };
      ws.onclose = () => { reconnectTimer = setTimeout(connect, 5000); };
      ws.onerror = (ev) => console.error("// SCC: Telemetry WS ERROR:", ev);
    };

    connect();
    return () => { clearTimeout(reconnectTimer); telemetryWsRef.current?.close(); };
  }, [isAuthenticated]);

  // ---- Auth ----
  const handleLogin = async () => {
    console.debug("// SCC: handleLogin");
    setAuthError('');
    setAuthLoading(true);
    try {
      const url = `http://${getHost()}:3000/api/auth`;
      console.debug("// SCC: POST", url);
      const res = await fetch(url, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ key: password }),
      });
      const data = await res.json();
      console.debug("// SCC: Auth response:", data);
      if (data.status === 'authenticated') setIsAuthenticated(true);
      else setAuthError('Sovereign key rejected.');
    } catch (e) {
      console.error("// SCC: Auth error:", e);
      setAuthError('Backend unreachable. Is the server running on port 3000?');
    } finally { setAuthLoading(false); }
  };

  const handleLogout = () => {
    console.debug("// SCC: Logout");
    localStorage.removeItem('lfi_auth');
    chatWsRef.current?.close();
    telemetryWsRef.current?.close();
    setIsAuthenticated(false);
    setMessages([]);
  };

  // ---- Tier Switch ----
  const handleTierSwitch = async (tier: string) => {
    console.debug("// SCC: Switching tier to:", tier);
    setTierSwitching(true);
    try {
      const url = `http://${getHost()}:3000/api/tier`;
      const res = await fetch(url, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ tier }),
      });
      const data = await res.json();
      console.debug("// SCC: Tier switch response:", data);
      if (data.status === 'ok') {
        setCurrentTier(data.tier);
        setMessages(prev => [...prev, {
          id: Date.now(), role: 'system',
          content: `Model tier switched to ${data.tier}.`,
          timestamp: Date.now(),
        }]);
      }
    } catch (e) {
      console.error("// SCC: Tier switch error:", e);
    } finally { setTierSwitching(false); }
  };

  // ---- Admin actions ----
  const fetchFacts = async () => {
    console.debug("// SCC: Fetching facts");
    setAdminLoading('facts');
    try {
      const res = await fetch(`http://${getHost()}:3000/api/facts`);
      const data = await res.json();
      setFacts(data.facts || []);
    } catch (e) { console.error("// SCC: Facts fetch error:", e); }
    finally { setAdminLoading(''); }
  };

  const fetchQos = async () => {
    console.debug("// SCC: Fetching QoS report");
    setAdminLoading('qos');
    try {
      const res = await fetch(`http://${getHost()}:3000/api/qos`);
      const data = await res.json();
      setQosReport(data);
    } catch (e) { console.error("// SCC: QoS fetch error:", e); }
    finally { setAdminLoading(''); }
  };

  const clearChat = () => {
    console.debug("// SCC: Clearing chat");
    setMessages([]);
  };

  // Sync tier from status endpoint on connect
  useEffect(() => {
    if (!isAuthenticated) return;
    const fetchStatus = async () => {
      try {
        const res = await fetch(`http://${getHost()}:3000/api/status`);
        const data = await res.json();
        if (data.tier) setCurrentTier(data.tier);
      } catch (_) { /* server might not be up yet */ }
    };
    fetchStatus();
  }, [isAuthenticated]);

  // ---- Send ----
  const handleSend = () => {
    const trimmed = input.trim();
    console.debug("// SCC: handleSend, len:", trimmed.length, "wsState:", chatWsRef.current?.readyState);
    if (!trimmed || !chatWsRef.current || chatWsRef.current.readyState !== WebSocket.OPEN) return;

    setMessages(prev => [...prev, {
      id: Date.now(), role: 'user', content: trimmed, timestamp: Date.now()
    }]);
    chatWsRef.current.send(JSON.stringify({ content: trimmed }));
    console.debug("// SCC: Sent to WS");
    setIsThinking(true);
    setInput('');
    inputRef.current?.focus();
  };

  const handleInputChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    setInput(e.target.value);
    const el = e.target;
    el.style.height = 'auto';
    el.style.height = Math.min(el.scrollHeight, 160) + 'px';
  };

  const formatTime = (ts: number) => new Date(ts).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });

  const tierColor = (t: string) => {
    if (t.includes('BigBrain')) return C.purple;
    if (t.includes('Bridge')) return C.yellow;
    return C.green;
  };

  // ============================================================
  // RENDER: Login
  // ============================================================
  if (!isAuthenticated) {
    console.debug("// SCC: Rendering login, breakpoint:", bp);
    return (
      <div style={{
        display: 'flex', alignItems: 'center', justifyContent: 'center',
        minHeight: '100vh', width: '100%',
        background: C.bg, padding: isMobile ? '24px' : '48px',
        fontFamily: C.font,
      }}>
        <div style={{
          width: '100%', maxWidth: isDesktop ? '440px' : '400px',
          padding: isDesktop ? '48px' : '32px',
          background: C.bgCard, border: `1px solid ${C.accentBorder}`,
          borderRadius: '16px',
          boxShadow: '0 12px 48px rgba(0,0,0,0.6)',
        }}>
          <div style={{ textAlign: 'center', marginBottom: '28px' }}>
            <div style={{
              display: 'inline-flex', alignItems: 'center', justifyContent: 'center',
              width: '72px', height: '72px', borderRadius: '50%',
              background: C.accentBg, border: `2px solid ${C.accentBorder}`,
              boxShadow: `0 0 24px ${C.accentGlow}`,
            }}>
              <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke={C.accent} strokeWidth="1.5">
                <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
                <path d="M12 8v4M12 16h.01"/>
              </svg>
            </div>
          </div>
          <h1 style={{
            fontSize: '16px', fontWeight: 800, textAlign: 'center',
            letterSpacing: '0.2em', textTransform: 'uppercase',
            color: C.text, marginBottom: '6px',
          }}>Sovereign Command Console</h1>
          <p style={{ fontSize: '13px', textAlign: 'center', color: C.textMuted, marginBottom: '32px' }}>
            Enter your sovereign key to authenticate
          </p>
          <input
            type="password" autoFocus
            style={{
              width: '100%', padding: '14px 16px',
              background: 'rgba(0,0,0,0.3)', border: `1px solid ${C.accentBorder}`,
              borderRadius: '10px', outline: 'none', color: C.text,
              fontSize: '16px', fontFamily: 'inherit', boxSizing: 'border-box', marginBottom: '12px',
            }}
            placeholder="AUTH_KEY"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            onKeyDown={(e) => e.key === 'Enter' && handleLogin()}
          />
          {authError && (
            <p style={{
              color: C.red, fontSize: '13px', textAlign: 'center', marginBottom: '12px',
              padding: '10px', background: C.redBg, borderRadius: '8px',
              border: `1px solid ${C.redBorder}`,
            }}>{authError}</p>
          )}
          <button onClick={handleLogin} disabled={authLoading || !password}
            style={{
              width: '100%', padding: '14px',
              background: C.accentBg, border: `1px solid ${C.accentBorder}`,
              borderRadius: '10px', color: C.accent, fontSize: '14px', fontWeight: 800,
              textTransform: 'uppercase', letterSpacing: '0.15em',
              cursor: authLoading ? 'wait' : 'pointer', fontFamily: 'inherit',
              opacity: !password ? 0.4 : 1,
              transition: 'all 0.2s',
            }}>
            {authLoading ? 'Authenticating...' : 'Initiate Link'}
          </button>
        </div>
        <style>{`
          * { box-sizing: border-box; }
          body { margin: 0; padding: 0; }
          input::placeholder { color: ${C.textDim}; }
        `}</style>
      </div>
    );
  }

  // ============================================================
  // RENDER: Main Console
  // ============================================================
  console.debug("// SCC: Rendering console, msgs:", messages.length, "bp:", bp);

  const chatMaxWidth = isDesktop ? '880px' : isTablet ? '700px' : '100%';
  const chatPadding = isDesktop ? '24px 32px' : isTablet ? '20px 24px' : '12px 14px';
  const sidebarWidth = 300;
  const userBubbleMaxWidth = isDesktop ? '70%' : '88%';

  // Telemetry stats data
  const telemetryCards = [
    { label: 'RAM', value: `${stats.ram_available_mb}`, unit: 'MB', color: C.accent, bg: C.accentBg, border: C.accentBorder },
    { label: 'CPU', value: `${stats.cpu_temp_c.toFixed(0)}`, unit: '\u00B0C', color: stats.cpu_temp_c > 65 ? C.red : C.green, bg: stats.cpu_temp_c > 65 ? C.redBg : C.greenBg, border: stats.cpu_temp_c > 65 ? C.redBorder : C.greenBorder },
    { label: 'VSA', value: `${(100 - stats.vsa_orthogonality * 100).toFixed(1)}`, unit: '%', color: C.purple, bg: C.purpleBg, border: C.purpleBorder },
    { label: 'PSL', value: `${(stats.axiom_pass_rate * 100).toFixed(0)}`, unit: '%', color: C.green, bg: C.greenBg, border: C.greenBorder },
  ];

  const renderTelemetryCard = (s: typeof telemetryCards[0], compact = false) => (
    <div key={s.label} style={{
      padding: compact ? '10px 12px' : '12px 14px', borderRadius: '10px',
      background: s.bg, border: `1px solid ${s.border}`,
    }}>
      <div style={{ fontSize: '10px', color: C.textMuted, fontWeight: 700, textTransform: 'uppercase', letterSpacing: '0.08em', marginBottom: compact ? '3px' : '5px' }}>{s.label}</div>
      <div style={{ fontSize: compact ? '18px' : '20px', fontWeight: 800, color: s.color }}>
        {s.value}<span style={{ fontSize: '11px', color: C.textDim, marginLeft: '2px' }}>{s.unit}</span>
      </div>
    </div>
  );

  // Desktop sidebar
  const renderSidebar = () => (
    <aside style={{
      width: `${sidebarWidth}px`, flexShrink: 0,
      background: C.bgCard, borderLeft: `1px solid ${C.border}`,
      display: 'flex', flexDirection: 'column', overflowY: 'auto',
    }}>
      {/* Telemetry */}
      <div style={{ padding: '20px', borderBottom: `1px solid ${C.borderSubtle}` }}>
        <div style={{ fontSize: '11px', fontWeight: 800, color: C.textMuted, textTransform: 'uppercase', letterSpacing: '0.12em', marginBottom: '14px' }}>
          Substrate Telemetry
        </div>
        <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '10px' }}>
          {telemetryCards.map(s => renderTelemetryCard(s, true))}
        </div>
        {stats.is_throttled && (
          <div style={{
            marginTop: '10px', padding: '10px', background: C.redBg,
            border: `1px solid ${C.redBorder}`, borderRadius: '8px',
            textAlign: 'center', fontSize: '11px', fontWeight: 800, color: C.red, textTransform: 'uppercase',
            letterSpacing: '0.08em',
          }}>Thermal Throttle</div>
        )}
      </div>
      {/* Status */}
      <div style={{ padding: '20px', borderBottom: `1px solid ${C.borderSubtle}` }}>
        <div style={{ fontSize: '11px', fontWeight: 800, color: C.textMuted, textTransform: 'uppercase', letterSpacing: '0.12em', marginBottom: '14px' }}>
          Status
        </div>
        <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
          {[
            { label: 'Connection', value: isConnected ? 'LIVE' : 'DOWN', color: isConnected ? C.green : C.red },
            { label: 'Tier', value: currentTier, color: tierColor(currentTier) },
            { label: 'Throttled', value: stats.is_throttled ? 'YES' : 'NO', color: stats.is_throttled ? C.red : C.green },
            { label: 'Logic Density', value: stats.logic_density.toFixed(3), color: C.purple },
          ].map(row => (
            <div key={row.label} style={{ display: 'flex', justifyContent: 'space-between', fontSize: '13px' }}>
              <span style={{ color: C.textMuted }}>{row.label}</span>
              <span style={{ color: row.color, fontWeight: 700 }}>{row.value}</span>
            </div>
          ))}
        </div>
      </div>
      {/* Admin actions */}
      <div style={{ padding: '20px' }}>
        <div style={{ fontSize: '11px', fontWeight: 800, color: C.textMuted, textTransform: 'uppercase', letterSpacing: '0.12em', marginBottom: '14px' }}>
          Administration
        </div>
        <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
          <button onClick={fetchFacts} disabled={adminLoading === 'facts'} style={{
            padding: '10px', fontSize: '12px', fontWeight: 700, color: C.accent,
            background: C.accentBg, border: `1px solid ${C.accentBorder}`, borderRadius: '8px',
            cursor: 'pointer', fontFamily: 'inherit', textTransform: 'uppercase', letterSpacing: '0.05em',
          }}>{adminLoading === 'facts' ? 'Loading...' : 'View Facts'}</button>
          <button onClick={fetchQos} disabled={adminLoading === 'qos'} style={{
            padding: '10px', fontSize: '12px', fontWeight: 700, color: C.purple,
            background: C.purpleBg, border: `1px solid ${C.purpleBorder}`, borderRadius: '8px',
            cursor: 'pointer', fontFamily: 'inherit', textTransform: 'uppercase', letterSpacing: '0.05em',
          }}>{adminLoading === 'qos' ? 'Loading...' : 'QoS Report'}</button>
          <button onClick={clearChat} style={{
            padding: '10px', fontSize: '12px', fontWeight: 700, color: C.textMuted,
            background: 'transparent', border: `1px solid ${C.border}`, borderRadius: '8px',
            cursor: 'pointer', fontFamily: 'inherit', textTransform: 'uppercase', letterSpacing: '0.05em',
          }}>Clear Chat</button>
        </div>
        {/* Facts display */}
        {facts.length > 0 && (
          <div style={{ marginTop: '14px' }}>
            <div style={{ fontSize: '10px', fontWeight: 700, color: C.textMuted, marginBottom: '8px', textTransform: 'uppercase' }}>
              Knowledge Facts ({facts.length})
            </div>
            <div style={{ maxHeight: '200px', overflowY: 'auto' }}>
              {facts.map((f, i) => (
                <div key={i} style={{ fontSize: '11px', padding: '6px 8px', borderBottom: `1px solid ${C.borderSubtle}` }}>
                  <span style={{ color: C.accent, fontWeight: 700 }}>{f.key}</span>
                  <span style={{ color: C.textDim }}> = </span>
                  <span style={{ color: C.textSecondary }}>{f.value}</span>
                </div>
              ))}
            </div>
          </div>
        )}
        {/* QoS display */}
        {qosReport && (
          <div style={{ marginTop: '14px' }}>
            <div style={{ fontSize: '10px', fontWeight: 700, color: C.textMuted, marginBottom: '8px', textTransform: 'uppercase' }}>
              QoS Report
            </div>
            <div style={{
              padding: '10px', borderRadius: '8px', fontSize: '11px',
              background: qosReport.overall_pass ? C.greenBg : C.redBg,
              border: `1px solid ${qosReport.overall_pass ? C.greenBorder : C.redBorder}`,
              color: qosReport.overall_pass ? C.green : C.red,
              fontWeight: 700,
            }}>
              {qosReport.overall_pass ? 'ALL CHECKS PASS' : 'COMPLIANCE ISSUES'}
            </div>
            <pre style={{ fontSize: '10px', color: C.textMuted, whiteSpace: 'pre-wrap', marginTop: '8px', lineHeight: '1.5' }}>
              {JSON.stringify(qosReport, null, 2).slice(0, 500)}
            </pre>
          </div>
        )}
      </div>
    </aside>
  );

  return (
    <div style={{
      display: 'flex', flexDirection: 'column', height: '100vh', width: '100%',
      background: C.bg, color: C.text,
      fontFamily: C.font,
      overflow: 'hidden',
    }}>
      {/* ========== HEADER ========== */}
      <header style={{
        display: 'flex', alignItems: 'center', justifyContent: 'space-between',
        padding: isDesktop ? '10px 24px' : '8px 14px',
        background: C.bgCard,
        borderBottom: `1px solid ${C.border}`,
        flexShrink: 0, zIndex: 50, minHeight: isMobile ? '50px' : '52px',
      }}>
        {/* Left: branding + status */}
        <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
          {/* SCC logo mark */}
          <div style={{
            width: '28px', height: '28px', borderRadius: '8px',
            background: C.accentBg, border: `1px solid ${C.accentBorder}`,
            display: 'flex', alignItems: 'center', justifyContent: 'center',
            flexShrink: 0,
          }}>
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke={C.accent} strokeWidth="2">
              <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
            </svg>
          </div>
          <div>
            <div style={{
              fontSize: '13px', fontWeight: 800, letterSpacing: '0.08em', textTransform: 'uppercase',
              color: C.text, lineHeight: 1,
            }}>SCC</div>
            <div style={{
              fontSize: '10px', color: isConnected ? C.green : C.red,
              fontWeight: 700, letterSpacing: '0.05em', marginTop: '2px',
            }}>
              {isConnected ? 'ONLINE' : 'OFFLINE'}
            </div>
          </div>
          {/* Desktop: inline stats */}
          {isDesktop && (
            <div style={{ display: 'flex', gap: '16px', marginLeft: '16px', fontSize: '12px', color: C.textDim }}>
              <span>{stats.ram_available_mb} MB</span>
              <span>{stats.cpu_temp_c.toFixed(0)}{'\u00B0'}C</span>
              <span style={{ color: tierColor(currentTier) }}>{currentTier}</span>
            </div>
          )}
        </div>

        {/* Right: controls */}
        <div style={{ display: 'flex', alignItems: 'center', gap: isMobile ? '6px' : '10px' }}>
          {/* Tier selector */}
          <select
            value={currentTier}
            disabled={tierSwitching}
            onChange={(e) => handleTierSwitch(e.target.value)}
            style={{
              padding: isMobile ? '5px 20px 5px 8px' : '6px 28px 6px 10px',
              fontSize: '11px', fontWeight: 700,
              background: C.bgInput,
              border: `1px solid ${C.purpleBorder}`,
              borderRadius: '8px',
              color: C.purple,
              cursor: tierSwitching ? 'wait' : 'pointer',
              fontFamily: 'inherit',
              textTransform: 'uppercase',
              appearance: 'none',
              WebkitAppearance: 'none',
              backgroundImage: `url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='8' height='8' viewBox='0 0 8 8'%3E%3Cpath fill='%23c49cff' d='M0 2l4 4 4-4z'/%3E%3C/svg%3E")`,
              backgroundRepeat: 'no-repeat',
              backgroundPosition: `right ${isMobile ? '6px' : '10px'} center`,
            }}
          >
            <option value="Pulse">Pulse</option>
            <option value="Bridge">Bridge</option>
            <option value="BigBrain">BigBrain</option>
          </select>

          {/* Stats toggle (mobile/tablet) */}
          {!isDesktop && (
            <button onClick={() => setShowTelemetry(!showTelemetry)} style={{
              padding: '5px 10px', fontSize: '11px', fontWeight: 700,
              background: showTelemetry ? C.accentBg : 'transparent',
              border: `1px solid ${showTelemetry ? C.accentBorder : C.border}`, borderRadius: '8px',
              color: showTelemetry ? C.accent : C.textMuted,
              cursor: 'pointer', fontFamily: 'inherit', textTransform: 'uppercase',
            }}>Stats</button>
          )}

          {/* Admin toggle (mobile/tablet) */}
          {!isDesktop && (
            <button onClick={() => setShowAdmin(!showAdmin)} style={{
              padding: '5px 10px', fontSize: '11px', fontWeight: 700,
              background: showAdmin ? C.purpleBg : 'transparent',
              border: `1px solid ${showAdmin ? C.purpleBorder : C.border}`, borderRadius: '8px',
              color: showAdmin ? C.purple : C.textMuted,
              cursor: 'pointer', fontFamily: 'inherit', textTransform: 'uppercase',
            }}>Admin</button>
          )}

          <button onClick={handleLogout} style={{
            padding: '5px 10px', fontSize: '11px', fontWeight: 700,
            background: 'transparent', border: `1px solid ${C.border}`,
            borderRadius: '8px', color: C.textDim, cursor: 'pointer', fontFamily: 'inherit',
            textTransform: 'uppercase',
          }}>Logout</button>
        </div>
      </header>

      {/* ========== TELEMETRY PANEL (mobile/tablet, collapsible) ========== */}
      {!isDesktop && showTelemetry && (
        <div style={{
          display: 'grid', gridTemplateColumns: isTablet ? 'repeat(4, 1fr)' : 'repeat(2, 1fr)',
          gap: '8px', padding: '12px 14px', background: C.bgCard,
          borderBottom: `1px solid ${C.border}`, flexShrink: 0,
        }}>
          {telemetryCards.map(s => renderTelemetryCard(s))}
          {stats.is_throttled && (
            <div style={{
              gridColumn: '1 / -1', padding: '10px', background: C.redBg,
              border: `1px solid ${C.redBorder}`, borderRadius: '8px',
              textAlign: 'center', fontSize: '12px', fontWeight: 800, color: C.red, textTransform: 'uppercase',
            }}>Thermal Throttle Active</div>
          )}
        </div>
      )}

      {/* ========== ADMIN PANEL (mobile/tablet, collapsible) ========== */}
      {!isDesktop && showAdmin && (
        <div style={{
          padding: '14px', background: C.bgCard,
          borderBottom: `1px solid ${C.border}`, flexShrink: 0,
        }}>
          <div style={{ display: 'flex', gap: '8px', flexWrap: 'wrap' }}>
            <button onClick={fetchFacts} disabled={adminLoading === 'facts'} style={{
              padding: '8px 14px', fontSize: '11px', fontWeight: 700, color: C.accent,
              background: C.accentBg, border: `1px solid ${C.accentBorder}`, borderRadius: '8px',
              cursor: 'pointer', fontFamily: 'inherit', textTransform: 'uppercase',
            }}>{adminLoading === 'facts' ? 'Loading...' : 'Facts'}</button>
            <button onClick={fetchQos} disabled={adminLoading === 'qos'} style={{
              padding: '8px 14px', fontSize: '11px', fontWeight: 700, color: C.purple,
              background: C.purpleBg, border: `1px solid ${C.purpleBorder}`, borderRadius: '8px',
              cursor: 'pointer', fontFamily: 'inherit', textTransform: 'uppercase',
            }}>{adminLoading === 'qos' ? 'Loading...' : 'QoS'}</button>
            <button onClick={clearChat} style={{
              padding: '8px 14px', fontSize: '11px', fontWeight: 700, color: C.textMuted,
              background: 'transparent', border: `1px solid ${C.border}`, borderRadius: '8px',
              cursor: 'pointer', fontFamily: 'inherit', textTransform: 'uppercase',
            }}>Clear Chat</button>
          </div>
          {/* Inline results */}
          {facts.length > 0 && (
            <div style={{ marginTop: '10px', maxHeight: '150px', overflowY: 'auto', fontSize: '11px' }}>
              {facts.map((f, i) => (
                <div key={i} style={{ padding: '4px 0', borderBottom: `1px solid ${C.borderSubtle}` }}>
                  <span style={{ color: C.accent, fontWeight: 700 }}>{f.key}</span>
                  <span style={{ color: C.textDim }}> = </span>
                  <span style={{ color: C.textSecondary }}>{f.value}</span>
                </div>
              ))}
            </div>
          )}
          {qosReport && (
            <pre style={{ marginTop: '10px', fontSize: '10px', color: C.textMuted, whiteSpace: 'pre-wrap', maxHeight: '150px', overflowY: 'auto' }}>
              {JSON.stringify(qosReport, null, 2).slice(0, 400)}
            </pre>
          )}
        </div>
      )}

      {/* ========== BODY: Chat + Sidebar ========== */}
      <div style={{ display: 'flex', flex: 1, overflow: 'hidden' }}>
        {/* CHAT AREA */}
        <main style={{ flex: 1, overflowY: 'auto', padding: chatPadding, WebkitOverflowScrolling: 'touch' as any }}>
          <div style={{ maxWidth: chatMaxWidth, margin: '0 auto' }}>
            {/* Empty state */}
            {messages.length === 0 && (
              <div style={{ textAlign: 'center', padding: isDesktop ? '100px 24px' : '60px 24px' }}>
                <div style={{
                  width: '64px', height: '64px', borderRadius: '16px',
                  background: C.accentBg, border: `2px solid ${C.accentBorder}`,
                  display: 'inline-flex', alignItems: 'center', justifyContent: 'center',
                  marginBottom: '20px', boxShadow: `0 0 32px ${C.accentGlow}`,
                }}>
                  <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke={C.accent} strokeWidth="1.5">
                    <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
                    <path d="M12 8v4M12 16h.01"/>
                  </svg>
                </div>
                <p style={{ fontSize: '18px', fontWeight: 700, color: C.text, margin: '0 0 8px' }}>Sovereign Command Console</p>
                <p style={{ fontSize: '13px', color: C.textMuted, margin: 0 }}>
                  Type a message to begin. I can reason, plan, search, and code.
                </p>
              </div>
            )}

            {/* Messages */}
            {messages.map((msg) => (
              <div key={msg.id} style={{ marginBottom: isDesktop ? '20px' : '14px' }}>
                {/* System messages */}
                {msg.role === 'system' && (
                  <div style={{
                    textAlign: 'center', padding: '8px 16px', fontSize: '12px',
                    color: C.textMuted, fontStyle: 'italic',
                  }}>
                    {msg.content}
                  </div>
                )}

                {/* Web results */}
                {msg.role === 'web' && (
                  <div style={{
                    padding: '14px 16px', borderRadius: '12px',
                    background: C.greenBg, border: `1px solid ${C.greenBorder}`,
                    maxWidth: isDesktop ? '75%' : '100%',
                  }}>
                    <div style={{ fontSize: '11px', fontWeight: 800, color: C.green, textTransform: 'uppercase', letterSpacing: '0.08em', marginBottom: '8px' }}>
                      Web Intelligence
                    </div>
                    <pre style={{
                      whiteSpace: 'pre-wrap', wordBreak: 'break-word',
                      fontSize: '13px', lineHeight: '1.6', color: '#b8f0d0', margin: 0,
                    }}>{msg.content}</pre>
                  </div>
                )}

                {/* User messages */}
                {msg.role === 'user' && (
                  <div style={{ display: 'flex', justifyContent: 'flex-end' }}>
                    <div style={{
                      maxWidth: userBubbleMaxWidth, padding: '12px 16px',
                      background: C.accentBg, border: `1px solid ${C.accentBorder}`,
                      borderRadius: '16px 16px 4px 16px', fontSize: '14px', lineHeight: '1.6',
                      color: '#d0deff', wordBreak: 'break-word',
                    }}>
                      {msg.content}
                      <div style={{ fontSize: '10px', color: C.textDim, marginTop: '6px', textAlign: 'right' }}>
                        {formatTime(msg.timestamp)}
                      </div>
                    </div>
                  </div>
                )}

                {/* Assistant messages */}
                {msg.role === 'assistant' && (
                  <div style={{ display: 'flex', justifyContent: 'flex-start' }}>
                    <div style={{ maxWidth: isDesktop ? '80%' : '96%', width: '100%' }}>
                      {/* Badges */}
                      <div style={{ display: 'flex', flexWrap: 'wrap', gap: '6px', marginBottom: '6px' }}>
                        {msg.tier && (
                          <span style={{
                            padding: '3px 10px', fontSize: '10px', fontWeight: 800,
                            background: C.accentBg, border: `1px solid ${C.accentBorder}`,
                            borderRadius: '6px', color: C.accent, textTransform: 'uppercase',
                            letterSpacing: '0.06em',
                          }}>{msg.tier}</span>
                        )}
                        {msg.mode && (
                          <span style={{
                            padding: '3px 10px', fontSize: '10px', fontWeight: 800,
                            background: C.purpleBg, border: `1px solid ${C.purpleBorder}`,
                            borderRadius: '6px', color: C.purple, textTransform: 'uppercase',
                            letterSpacing: '0.06em',
                          }}>{msg.mode}</span>
                        )}
                        {msg.confidence !== undefined && (
                          <span style={{
                            padding: '3px 10px', fontSize: '10px', fontWeight: 800,
                            background: msg.confidence > 0.7 ? C.greenBg : C.yellowBg,
                            border: `1px solid ${msg.confidence > 0.7 ? C.greenBorder : 'rgba(255,214,102,0.20)'}`,
                            borderRadius: '6px',
                            color: msg.confidence > 0.7 ? C.green : C.yellow,
                          }}>{(msg.confidence * 100).toFixed(0)}%</span>
                        )}
                      </div>

                      {/* Response body */}
                      <div style={{
                        padding: '14px 18px',
                        background: C.bgCard,
                        border: `1px solid ${C.border}`,
                        borderRadius: '4px 16px 16px 16px',
                        fontSize: '14px', lineHeight: '1.65',
                        color: C.text,
                        whiteSpace: 'pre-wrap', wordBreak: 'break-word',
                      }}>
                        {msg.content}
                        <div style={{ fontSize: '10px', color: C.textDim, marginTop: '8px' }}>
                          {formatTime(msg.timestamp)}
                          {msg.intent && (
                            <span style={{ marginLeft: '10px', color: C.textMuted }}>
                              {msg.intent.split('{')[0]}
                            </span>
                          )}
                        </div>
                      </div>

                      {/* Reasoning toggle */}
                      {msg.reasoning && msg.reasoning.length > 0 && (
                        <div style={{ marginTop: '8px' }}>
                          <button
                            onClick={() => setExpandedReasoning(expandedReasoning === msg.id ? null : msg.id)}
                            style={{
                              display: 'flex', alignItems: 'center', gap: '6px',
                              padding: '6px 12px', fontSize: '11px', fontWeight: 700,
                              color: C.textMuted, background: 'transparent',
                              border: `1px solid ${C.border}`, borderRadius: '8px',
                              cursor: 'pointer', fontFamily: 'inherit', textTransform: 'uppercase',
                              letterSpacing: '0.04em',
                            }}
                          >
                            Reasoning ({msg.reasoning.length}) {expandedReasoning === msg.id ? '\u25B2' : '\u25BC'}
                          </button>
                          {expandedReasoning === msg.id && (
                            <div style={{
                              marginTop: '8px', padding: '14px',
                              background: 'rgba(0,0,0,0.25)',
                              borderLeft: `3px solid ${C.accentBorder}`,
                              borderRadius: '0 10px 10px 0',
                            }}>
                              {msg.reasoning.map((step, j) => (
                                <p key={j} style={{ fontSize: '12px', color: C.textSecondary, lineHeight: '1.6', margin: '4px 0' }}>
                                  <span style={{ color: C.accent, fontWeight: 700 }}>[{j}]</span> {step}
                                </p>
                              ))}
                            </div>
                          )}
                        </div>
                      )}

                      {/* Plan */}
                      {msg.plan && (
                        <div style={{
                          marginTop: '8px', padding: '12px 14px',
                          background: C.accentBg, border: `1px solid ${C.accentBorder}`,
                          borderRadius: '10px', fontSize: '12px', color: C.textSecondary,
                        }}>
                          <span style={{ fontWeight: 800, color: C.accent }}>PLAN: </span>
                          {msg.plan.steps} steps | complexity: {msg.plan.complexity.toFixed(2)} | {msg.plan.goal.slice(0, 100)}
                        </div>
                      )}
                    </div>
                  </div>
                )}
              </div>
            ))}

            {/* Thinking indicator */}
            {isThinking && (
              <div style={{
                display: 'flex', alignItems: 'center', gap: '10px',
                padding: '14px 18px', fontSize: '13px', color: C.accent,
              }}>
                <div style={{ display: 'flex', gap: '5px' }}>
                  {[0,1,2].map(i => (
                    <div key={i} style={{
                      width: '7px', height: '7px', background: C.accent, borderRadius: '50%',
                      animation: 'scc-bounce 1.4s infinite ease-in-out',
                      animationDelay: `${i * 0.16}s`,
                    }} />
                  ))}
                </div>
                Processing...
              </div>
            )}
            <div ref={messagesEndRef} />
          </div>
        </main>

        {/* DESKTOP SIDEBAR */}
        {isDesktop && renderSidebar()}
      </div>

      {/* ========== INPUT BAR ========== */}
      <div style={{
        padding: isDesktop ? '14px 24px' : '10px 14px',
        paddingBottom: isMobile ? 'max(10px, env(safe-area-inset-bottom))' : '14px',
        background: C.bgCard, borderTop: `1px solid ${C.border}`, flexShrink: 0,
        zIndex: 40,
      }}>
        <div style={{
          maxWidth: isDesktop ? '880px' : isTablet ? '700px' : '100%',
          margin: '0 auto',
          display: 'flex', alignItems: 'flex-end', gap: '8px',
          background: C.bgInput,
          border: `1px solid ${input ? C.borderFocus : C.border}`,
          borderRadius: '14px', padding: '4px',
          transition: 'border-color 0.2s',
          boxShadow: input ? `0 0 12px ${C.accentGlow}` : 'none',
        }}>
          <textarea
            ref={inputRef}
            value={input}
            onChange={handleInputChange}
            onKeyDown={(e) => { if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); handleSend(); }}}
            placeholder="Enter directive..."
            style={{
              flex: 1, background: 'transparent', border: 'none', outline: 'none',
              resize: 'none', fontSize: '15px', lineHeight: '1.5', padding: '10px 12px',
              color: C.text, fontFamily: 'inherit', minHeight: '44px', maxHeight: '160px',
            }}
            rows={1}
          />
          <button
            onClick={handleSend}
            disabled={!input.trim() || !isConnected}
            className="scc-send-btn"
            style={{
              width: '44px', height: '44px',
              display: 'flex', alignItems: 'center', justifyContent: 'center',
              background: input.trim() && isConnected ? C.accentBg : 'transparent',
              border: `1px solid ${input.trim() && isConnected ? C.accentBorder : 'transparent'}`,
              borderRadius: '10px',
              color: input.trim() && isConnected ? C.accent : C.textDim,
              cursor: input.trim() && isConnected ? 'pointer' : 'default',
              flexShrink: 0, transition: 'all 0.15s',
            }}
          >
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
              <path d="m22 2-7 20-4-9-9-4z"/><path d="M22 2 11 13"/>
            </svg>
          </button>
        </div>
        <div style={{
          maxWidth: isDesktop ? '880px' : isTablet ? '700px' : '100%',
          margin: '6px auto 0', display: 'flex', justifyContent: 'space-between',
          fontSize: '10px', color: C.textDim, padding: '0 8px',
        }}>
          <span>{isConnected ? 'Link active' : 'Reconnecting...'}</span>
          <span>Shift+Enter for newline</span>
        </div>
      </div>

      {/* ========== GLOBAL STYLES ========== */}
      <style>{`
        @keyframes scc-bounce {
          0%,80%,100% { transform: scale(0); opacity: 0.5; }
          40% { transform: scale(1); opacity: 1; }
        }
        * { box-sizing: border-box; }
        body { margin: 0; padding: 0; overflow: hidden; background: ${C.bg}; }
        html { background: ${C.bg}; }
        input::placeholder, textarea::placeholder { color: ${C.textDim}; }
        ::-webkit-scrollbar { width: 6px; }
        ::-webkit-scrollbar-track { background: transparent; }
        ::-webkit-scrollbar-thumb { background: rgba(255,255,255,0.10); border-radius: 4px; }
        ::-webkit-scrollbar-thumb:hover { background: rgba(255,255,255,0.18); }
        .scc-send-btn:hover:not(:disabled) { background: rgba(99,140,255,0.25) !important; border-color: rgba(99,140,255,0.4) !important; }
        select option { background: ${C.bgInput}; color: ${C.purple}; }
        button:active { transform: scale(0.97); }
        @media (hover: hover) {
          button:hover { opacity: 0.9; }
        }
        @media (hover: none) {
          button:hover { opacity: 1; }
          .scc-send-btn:hover:not(:disabled) { background: ${C.accentBg} !important; }
        }
        /* Push Eruda FAB above our input bar */
        #eruda { z-index: 9999 !important; }
        .eruda-entry-btn { bottom: 80px !important; right: 10px !important; }
      `}</style>
    </div>
  );
};

export default SovereignCommandConsole;
