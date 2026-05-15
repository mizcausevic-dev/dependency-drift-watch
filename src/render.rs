use crate::engine;
use crate::models::{OwnerLane, PackageAssessment};

fn shell(title: &str, subtitle: &str, current: &str, body: &str) -> String {
    let summary = engine::dashboard_summary();
    let nav = [
        ("/", "Overview", "overview"),
        ("/packages", "Packages", "packages"),
        ("/review-queue", "Review Queue", "review-queue"),
        ("/owners", "Owners", "owners"),
        ("/metrics-preview", "Metrics", "metrics"),
        ("/docs", "Docs", "docs"),
    ];

    let side_links = nav
        .iter()
        .map(|(href, label, key)| {
            format!(
                r#"<a class="side-link {}" href="{}">{}</a>"#,
                if current == *key { "active" } else { "" },
                href,
                label
            )
        })
        .collect::<Vec<_>>()
        .join("");

    let tab_links = nav
        .iter()
        .map(|(href, label, key)| {
            format!(
                r#"<a class="tab-pill {}" href="{}">{}</a>"#,
                if current == *key { "active" } else { "" },
                href,
                label
            )
        })
        .collect::<Vec<_>>()
        .join("");

    format!(
        r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>{title}</title>
    <style>
      :root {{
        color-scheme: dark;
        --bg: #04101a;
        --panel: rgba(9, 17, 29, 0.94);
        --line: rgba(255,255,255,0.08);
        --text: #f3f7fd;
        --muted: #9cadc5;
        --soft: #6e819b;
        --blue: #79cbff;
        --indigo: #627dff;
        --green: #67dfa7;
        --amber: #f2c56d;
        --red: #ff8697;
      }}
      * {{ box-sizing: border-box; }}
      body {{
        margin: 0;
        font-family: Inter, "Segoe UI", system-ui, sans-serif;
        color: var(--text);
        background:
          radial-gradient(circle at top left, rgba(121,203,255,0.14), transparent 24%),
          radial-gradient(circle at top right, rgba(255,134,151,0.08), transparent 18%),
          linear-gradient(180deg, #02070d 0%, #05101a 100%);
      }}
      a {{ color: inherit; text-decoration: none; }}
      .shell {{ min-height: 100vh; display: grid; grid-template-columns: 250px minmax(0,1fr); }}
      .sidebar {{
        background: rgba(0,0,0,0.28);
        border-right: 1px solid rgba(255,255,255,0.06);
        backdrop-filter: blur(16px);
        padding: 24px 18px;
        display: flex;
        flex-direction: column;
      }}
      .brand {{
        display: flex; align-items: center; gap: 12px; padding: 8px 10px 18px;
        border-bottom: 1px solid rgba(255,255,255,0.06);
      }}
      .brand-mark {{
        width: 40px; height: 40px; border-radius: 12px; display:grid; place-items:center;
        background: linear-gradient(135deg, #1090bf, #627dff); color:white; font-weight:900;
        box-shadow: 0 0 18px rgba(98,125,255,0.28);
      }}
      .brand strong {{ display:block; font-size:14px; }}
      .brand span {{ display:block; margin-top:4px; color:var(--blue); font-size:10px; letter-spacing:.18em; text-transform:uppercase; }}
      nav {{ margin-top: 18px; }}
      .side-link {{
        display:block; padding:13px 14px; border-radius:14px; color:#8195b4; font-size:12px;
        font-weight:700; text-transform:uppercase; letter-spacing:.12em;
      }}
      .side-link.active {{ color:var(--blue); background:rgba(121,203,255,0.08); border:1px solid rgba(121,203,255,0.16); }}
      .side-link:hover {{ color:var(--text); background:rgba(255,255,255,0.04); }}
      .meta {{ margin-top:auto; padding:16px 12px 8px; border-top:1px solid rgba(255,255,255,0.06); }}
      .meta dt {{ color:#687c98; font-size:10px; text-transform:uppercase; letter-spacing:.14em; margin-bottom:4px; }}
      .meta dd {{ margin:0 0 14px; font-size:12px; font-weight:700; line-height:1.5; }}
      .topbar {{
        height:72px; position:sticky; top:0; z-index:2; display:flex; align-items:center; justify-content:space-between;
        padding:0 34px; background:rgba(0,0,0,0.34); border-bottom:1px solid rgba(255,255,255,0.06); backdrop-filter: blur(16px);
      }}
      .status-chip {{
        display:inline-flex; align-items:center; gap:10px; padding:9px 14px; border-radius:999px;
        border:1px solid rgba(121,203,255,0.14); background:rgba(121,203,255,0.05); color:#b9e1ff;
        font-size:10px; font-weight:800; text-transform:uppercase; letter-spacing:.18em;
      }}
      .status-dot {{ width:8px; height:8px; border-radius:50%; background:var(--blue); box-shadow:0 0 12px rgba(121,203,255,0.84); }}
      .topbar-right {{ display:flex; align-items:center; gap:22px; }}
      .meta-block {{ display:flex; flex-direction:column; align-items:flex-end; }}
      .meta-block span {{ color:#6d809b; font-size:9px; text-transform:uppercase; letter-spacing:.15em; }}
      .meta-block strong {{ margin-top:4px; font-size:11px; text-transform:uppercase; letter-spacing:.12em; }}
      .action-pill {{
        display:inline-flex; align-items:center; padding:12px 16px; border-radius:999px; color:white;
        background:linear-gradient(135deg, #1090bf, #627dff); box-shadow:0 0 20px rgba(98,125,255,0.24);
        font-size:10px; font-weight:900; letter-spacing:.18em; text-transform:uppercase;
      }}
      .wrap {{ max-width: 1280px; margin:0 auto; padding:34px; }}
      .hero {{
        border:1px solid var(--line); border-radius:28px; padding:28px;
        background: linear-gradient(180deg, rgba(9,16,28,0.96), rgba(6,11,20,0.94));
        box-shadow: 0 26px 60px rgba(0,0,0,0.34);
      }}
      .hero-eyebrow {{ margin-bottom:18px; color:var(--blue); font-size:11px; letter-spacing:.28em; text-transform:uppercase; font-weight:800; }}
      h1 {{ margin:0; font-size:clamp(38px,5vw,70px); line-height:.92; font-family:Georgia, "Times New Roman", serif; letter-spacing:-.04em; }}
      .hero-subtitle {{ margin-top:14px; max-width:860px; color:var(--muted); font-size:19px; line-height:1.55; }}
      .hero-strip {{ display:flex; flex-wrap:wrap; gap:14px; margin-top:24px; }}
      .hero-kpi {{ min-width:180px; padding:14px 16px; border-radius:18px; border:1px solid rgba(255,255,255,0.06); background:rgba(255,255,255,0.03); }}
      .hero-kpi .k {{ color:#6f83a0; font-size:10px; text-transform:uppercase; letter-spacing:.14em; font-weight:800; }}
      .hero-kpi .v {{ margin-top:6px; font-size:28px; font-weight:800; }}
      .hero-callout {{
        margin-top:18px; padding:18px 20px; border-radius:18px; border:1px solid rgba(255,255,255,0.06); background:rgba(2,8,17,0.62);
      }}
      .hero-callout strong {{ display:block; color:var(--amber); font-size:10px; text-transform:uppercase; letter-spacing:.18em; margin-bottom:8px; }}
      .hero-callout p {{ margin:0; color:#dce7fb; font-size:17px; line-height:1.5; }}
      .tab-row {{ display:flex; gap:10px; flex-wrap:wrap; margin-top:20px; }}
      .tab-pill {{
        display:inline-flex; align-items:center; padding:10px 14px; border-radius:999px; border:1px solid rgba(255,255,255,0.08);
        background:rgba(255,255,255,0.03); color:#afc0d8; font-size:11px; font-weight:800; text-transform:uppercase; letter-spacing:.12em;
      }}
      .tab-pill.active {{ color:var(--amber); border-color:rgba(242,197,109,0.18); background:rgba(242,197,109,0.08); }}
      .page-section {{ margin-top:24px; border-radius:26px; border:1px solid var(--line); background:var(--panel); overflow:hidden; box-shadow:0 24px 54px rgba(0,0,0,0.24); }}
      .section-head {{ padding:20px 24px 14px; border-bottom:1px solid rgba(255,255,255,0.05); }}
      .section-head strong {{ display:block; color:var(--blue); font-size:10px; text-transform:uppercase; letter-spacing:.2em; margin-bottom:10px; }}
      .section-head h2 {{ margin:0; font-family:Georgia, "Times New Roman", serif; font-size:24px; letter-spacing:-.03em; }}
      .section-head p {{ margin:10px 0 0; color:var(--muted); font-size:15px; line-height:1.55; }}
      .section-body {{ padding:24px; }}
      .stats-grid, .three-col {{ display:grid; gap:18px; grid-template-columns:repeat(4,minmax(0,1fr)); }}
      .three-col {{ grid-template-columns:repeat(3,minmax(0,1fr)); }}
      .stat-card, .metric-card {{
        border-radius:20px; padding:18px 18px 20px; border:1px solid rgba(255,255,255,0.06);
        background:linear-gradient(180deg, rgba(255,255,255,0.04), rgba(0,0,0,0.08));
      }}
      .stat-card .label, .micro {{
        color:#71839d; font-size:10px; text-transform:uppercase; letter-spacing:.16em; font-weight:800;
      }}
      .stat-card .value {{ margin-top:10px; font-size:36px; font-weight:900; }}
      .stat-card .sub, .metric-card .desc {{ margin-top:10px; color:var(--muted); font-size:14px; line-height:1.45; }}
      .metric-card .title {{ margin-top:8px; font-size:16px; font-weight:800; }}
      .panel-grid {{ display:grid; gap:14px; }}
      .insight-grid {{ display:grid; gap:18px; grid-template-columns:1.2fr 1fr; }}
      .panel {{
        border-radius:22px; border:1px solid rgba(255,255,255,0.06); background:rgba(4,9,18,0.55); padding:22px;
      }}
      .panel h3 {{ margin:0 0 14px; font-size:18px; }}
      .chart-shell {{
        border-radius:22px; border:1px solid rgba(255,255,255,0.06); background:rgba(4,9,18,0.55); padding:22px;
      }}
      .bars {{ display:flex; align-items:flex-end; justify-content:space-between; gap:14px; height:180px; margin-top:18px; }}
      .bar-col {{ flex:1; text-align:center; }}
      .bar-wrap {{ height:124px; display:flex; align-items:flex-end; justify-content:center; }}
      .bar {{
        width:100%; max-width:76px; border-radius:18px 18px 8px 8px; background:linear-gradient(180deg, var(--blue), var(--indigo));
        box-shadow:0 0 24px rgba(98,125,255,0.24);
      }}
      .bar-label {{ margin-top:8px; color:var(--muted); font-size:11px; text-transform:uppercase; letter-spacing:.12em; }}
      .bar-value {{ margin-top:6px; font-size:15px; font-weight:800; }}
      .package-grid {{ display:grid; gap:16px; }}
      .package-card {{ border-radius:22px; border:1px solid rgba(255,255,255,0.06); background:rgba(4,9,18,0.6); overflow:hidden; }}
      .package-top {{ display:grid; grid-template-columns:minmax(0,1fr) auto auto; gap:18px; align-items:center; padding:20px 22px; }}
      .package-card h3 {{ margin:0; font-size:22px; font-weight:800; letter-spacing:-.03em; }}
      .meta-text {{ margin-top:8px; color:var(--muted); font-size:13px; }}
      .tag {{
        display:inline-flex; align-items:center; justify-content:center; padding:8px 12px; border-radius:999px; font-size:10px; font-weight:900; letter-spacing:.16em; text-transform:uppercase;
      }}
      .healthy {{ color:var(--green); background:rgba(103,223,167,0.12); border:1px solid rgba(103,223,167,0.14); }}
      .watch {{ color:var(--amber); background:rgba(242,197,109,0.12); border:1px solid rgba(242,197,109,0.14); }}
      .critical {{ color:var(--red); background:rgba(255,134,151,0.12); border:1px solid rgba(255,134,151,0.14); }}
      .score-stack {{ text-align:right; }}
      .score-stack .label {{ color:#6f83a0; font-size:9px; text-transform:uppercase; letter-spacing:.16em; font-weight:800; }}
      .score-stack .value {{ margin-top:6px; font-size:28px; font-weight:900; }}
      .package-bottom {{ padding:18px 22px 22px; border-top:1px solid rgba(255,255,255,0.05); background:rgba(255,255,255,0.02); }}
      .two-col {{ display:grid; grid-template-columns:1fr 1fr; gap:18px; }}
      .signal-pill {{
        display:inline-flex; align-items:center; padding:8px 10px; border-radius:999px; background:rgba(121,203,255,0.09); color:var(--blue);
        font-size:10px; font-weight:800; letter-spacing:.12em; text-transform:uppercase;
      }}
      .pill-stack {{ display:flex; flex-wrap:wrap; gap:10px; }}
      .meter-row + .meter-row {{ margin-top:14px; }}
      .meter-head {{ display:flex; justify-content:space-between; gap:16px; margin-bottom:8px; color:#cfe0f7; font-size:12px; font-weight:700; }}
      .meter-track {{ height:10px; border-radius:999px; background:rgba(255,255,255,0.05); overflow:hidden; }}
      .meter-fill {{ height:100%; border-radius:999px; }}
      .meter-fill.good {{ background:linear-gradient(90deg, #1e7fc7, #67dfa7); }}
      .meter-fill.watch {{ background:linear-gradient(90deg, #2f82ff, #f2c56d); }}
      .meter-fill.hot {{ background:linear-gradient(90deg, #d14d6c, #ff8697); }}
      .table-shell {{ overflow:auto; border-radius:22px; border:1px solid rgba(255,255,255,0.06); }}
      table {{ width:100%; border-collapse:collapse; min-width:960px; }}
      th, td {{ text-align:left; padding:14px 14px; border-bottom:1px solid rgba(255,255,255,0.06); vertical-align:top; }}
      th {{ color:#7d90aa; font-size:10px; text-transform:uppercase; letter-spacing:.16em; }}
      td {{ font-size:13px; color:#edf3fb; }}
      .log-shell {{ border-radius:22px; border:1px solid rgba(255,255,255,0.08); background:rgba(2,6,12,0.88); overflow:hidden; }}
      .log-head {{ padding:16px 18px; display:flex; align-items:center; gap:12px; border-bottom:1px solid rgba(255,255,255,0.08); background:rgba(255,255,255,0.03); }}
      .lights {{ display:flex; gap:8px; }}
      .lights i {{ width:11px; height:11px; border-radius:50%; display:block; }}
      .lights i:nth-child(1) {{ background:rgba(255,134,151,0.55); }}
      .lights i:nth-child(2) {{ background:rgba(242,197,109,0.55); }}
      .lights i:nth-child(3) {{ background:rgba(103,223,167,0.55); }}
      .log-head strong {{ color:var(--blue); font-size:10px; letter-spacing:.18em; text-transform:uppercase; }}
      .log-body {{ padding:18px 18px 8px; }}
      .log-line {{ display:grid; grid-template-columns:170px 180px minmax(0,1fr) 90px; gap:14px; align-items:start; padding:10px 12px; border-radius:14px; }}
      .log-line + .log-line {{ margin-top:8px; }}
      .log-line:hover {{ background:rgba(255,255,255,0.03); }}
      .log-time {{ color:#6f83a0; font-size:11px; font-family:"Cascadia Code", Consolas, monospace; }}
      .log-action {{ color:var(--blue); font-size:11px; font-family:"Cascadia Code", Consolas, monospace; font-weight:800; letter-spacing:.08em; }}
      .log-resource strong {{ display:block; font-size:12px; }}
      .log-resource span {{ display:block; margin-top:4px; color:var(--muted); font-size:12px; line-height:1.45; }}
      .result-good {{ color:var(--green); }}
      .result-warning {{ color:var(--amber); }}
      .result-bad {{ color:var(--red); }}
      .code-panel {{ border-radius:22px; border:1px solid rgba(255,255,255,0.08); background:rgba(2,6,12,0.92); padding:18px 20px 20px; }}
      .code-head {{ display:flex; align-items:center; justify-content:space-between; padding-bottom:12px; margin-bottom:16px; border-bottom:1px solid rgba(255,255,255,0.08); }}
      .code-head span {{ color:var(--blue); font-size:10px; font-weight:800; text-transform:uppercase; letter-spacing:.18em; }}
      pre {{ margin:0; white-space:pre-wrap; overflow:auto; color:#dce8fb; font-size:13px; line-height:1.6; font-family:"Cascadia Code", Consolas, monospace; }}
      .footer-strip {{ display:flex; justify-content:space-between; gap:16px; margin-top:18px; padding:4px 2px 10px; color:#6d809b; font-size:10px; text-transform:uppercase; letter-spacing:.16em; }}
      .footer-strip strong {{ color:#b8c9de; }}
      @media (max-width:1080px) {{
        .shell {{ grid-template-columns:1fr; }}
        .sidebar {{ display:none; }}
        .stats-grid, .three-col, .insight-grid, .two-col {{ grid-template-columns:1fr; }}
        .package-top {{ grid-template-columns:1fr; align-items:start; }}
        .topbar {{ padding:0 18px; height:auto; min-height:72px; flex-wrap:wrap; gap:12px; }}
        .topbar-right, .footer-strip {{ flex-wrap:wrap; }}
        .log-line {{ grid-template-columns:1fr; }}
      }}
    </style>
  </head>
  <body>
    <div class="shell">
      <aside class="sidebar">
        <div class="brand">
          <div class="brand-mark">DD</div>
          <div>
            <strong>Dependency Drift Watch</strong>
            <span>Reliability lane: DEP-DRIFT</span>
          </div>
        </div>
        <nav>{side_links}</nav>
        <dl class="meta">
          <dt>Package fleet</dt>
          <dd>{package_count} tracked packages</dd>
          <dt>Critical drift</dt>
          <dd>{critical_packages} packages</dd>
          <dt>Lead lane</dt>
          <dd>{riskiest_owner_lane}</dd>
        </dl>
      </aside>
      <main>
        <header class="topbar">
          <div class="status-chip"><span class="status-dot"></span>Dependency scan loop live</div>
          <div class="topbar-right">
            <div class="meta-block"><span>Automation coverage</span><strong>{automation_coverage}% enabled</strong></div>
            <div class="meta-block"><span>Average release gap</span><strong>{avg_release_gap} releases</strong></div>
            <a class="action-pill" href="/metrics">Open drift metrics</a>
          </div>
        </header>
        <div class="wrap">
          <section class="hero">
            <div class="hero-eyebrow">Dependency Drift Watch</div>
            <h1>{title}</h1>
            <p class="hero-subtitle">{subtitle}</p>
            <div class="hero-strip">
              <div class="hero-kpi"><div class="k">Packages</div><div class="v">{package_count}</div></div>
              <div class="hero-kpi"><div class="k">Critical drift</div><div class="v">{critical_packages}</div></div>
              <div class="hero-kpi"><div class="k">Watch packages</div><div class="v">{watch_packages}</div></div>
              <div class="hero-kpi"><div class="k">Riskiest owner lane</div><div class="v" style="font-size:20px">{riskiest_owner_lane}</div></div>
            </div>
            <div class="hero-callout">
              <strong>Lead recommendation</strong>
              <p>{lead_recommendation}</p>
            </div>
            <div class="tab-row">{tab_links}</div>
          </section>
          {body}
          <div class="footer-strip">
            <span><strong>Discipline:</strong> platform reliability</span>
            <span><strong>Focus:</strong> release lag / lockfile age / CVE pressure / owner review</span>
            <span><strong>Surface:</strong> operator-first / reliability-legible</span>
          </div>
        </div>
      </main>
    </div>
  </body>
</html>"#,
        title = title,
        subtitle = subtitle,
        body = body,
        side_links = side_links,
        tab_links = tab_links,
        package_count = summary.package_count,
        critical_packages = summary.critical_packages,
        watch_packages = summary.watch_packages,
        automation_coverage = summary.automated_update_coverage_percent,
        avg_release_gap = summary.avg_release_gap,
        riskiest_owner_lane = summary.riskiest_owner_lane,
        lead_recommendation = summary.lead_recommendation,
    )
}

fn score_bars(pkg: &PackageAssessment) -> String {
    let metrics = vec![
        ("Release gap", ((pkg.snapshot.release_gap.min(10) as usize) * 10)),
        (
            "Lockfile age",
            ((pkg.snapshot.lockfile_age_days.min(120) as usize) * 100) / 120,
        ),
        ("CVE pressure", ((pkg.snapshot.cve_signals.min(4) as usize) * 25)),
        ("Drift score", pkg.drift_score as usize),
    ];

    metrics
        .into_iter()
        .map(|(label, value)| {
            let tone = if value < 40 {
                "good"
            } else if value < 70 {
                "watch"
            } else {
                "hot"
            };
            format!(
                r#"<div class="meter-row"><div class="meter-head"><span>{}</span><span>{}%</span></div><div class="meter-track"><div class="meter-fill {}" style="width:{}%"></div></div></div>"#,
                label, value, tone, value
            )
        })
        .collect::<Vec<_>>()
        .join("")
}

pub fn render_overview() -> String {
    let summary = engine::dashboard_summary();
    let packages = engine::packages();
    let chart_bars = packages
        .iter()
        .take(6)
        .map(|pkg| {
            let height = (pkg.drift_score as f32 * 1.05).max(18.0);
            format!(
                r#"<div class="bar-col"><div class="bar-wrap"><div class="bar" style="height:{:.0}px"></div></div><div class="bar-value">{}</div><div class="bar-label">{}</div></div>"#,
                height, pkg.drift_score, pkg.snapshot.package_name
            )
        })
        .collect::<Vec<_>>()
        .join("");

    let board = packages
        .iter()
        .take(3)
        .map(package_card)
        .collect::<Vec<_>>()
        .join("");

    let body = format!(
        r#"
      <section class="page-section">
        <div class="section-head">
          <strong>Drift overview</strong>
          <h2>Dependency freshness only matters when owners can see which gaps are becoming operational risk.</h2>
          <p>This service treats dependency drift as a platform-reliability problem: package lag, stale lockfiles, CVE pressure, and breaking changes all show up in the same control lane.</p>
        </div>
        <div class="section-body">
          <div class="stats-grid">
            <div class="stat-card"><div class="label">Critical packages</div><div class="value">{}</div><div class="sub">Packages that already deserve owner review before the next release train moves.</div></div>
            <div class="stat-card"><div class="label">Watch packages</div><div class="value">{}</div><div class="sub">Dependencies that are aging into risk but can still be recovered through normal planning.</div></div>
            <div class="stat-card"><div class="label">Automation coverage</div><div class="value">{}%</div><div class="sub">How much of the tracked fleet still benefits from an automated freshness path.</div></div>
            <div class="stat-card"><div class="label">Average release gap</div><div class="value">{}</div><div class="sub">Mean lag between current package versions and their latest known releases.</div></div>
          </div>
          <div class="insight-grid" style="margin-top:20px;">
            <div class="chart-shell">
              <div class="micro">Top package drift</div>
              <h3 style="margin:10px 0 0;">Where release lag is starting to compound into real delivery risk.</h3>
              <div class="bars">{}</div>
              <p style="margin-top:16px;color:var(--muted);font-size:13px;line-height:1.5;">A dependency can look harmless until release lag, stale lockfiles, and owner neglect combine into a blocked upgrade or an avoidable incident.</p>
            </div>
            <div class="panel">
              <h3>What the drift watch is really for</h3>
              <div class="panel-grid">
                <div class="metric-card">
                  <div class="micro">Owner review</div>
                  <div class="title">Show which lane should move first.</div>
                  <div class="desc">The repo is strongest when it turns abstract freshness into a concrete owner queue.</div>
                </div>
                <div class="metric-card">
                  <div class="micro">Release planning</div>
                  <div class="title">Separate watch drift from critical drift.</div>
                  <div class="desc">Not every gap deserves panic, but some absolutely deserve to block a release window.</div>
                </div>
                <div class="metric-card">
                  <div class="micro">Security context</div>
                  <div class="title">Keep CVE pressure tied to adoption scope.</div>
                  <div class="desc">A stale tier-0 package is not the same problem as a stale internal dev dependency.</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>
      <section class="page-section">
        <div class="section-head">
          <strong>Top package board</strong>
          <h2>The riskiest package lanes stay visible as a working board, not just a lockfile diff.</h2>
          <p>Every package card ties release gap, lockfile age, CVE signals, and owner lane together so the next reliability move is obvious.</p>
        </div>
        <div class="section-body">
          <div class="package-grid">{}</div>
        </div>
      </section>
        "#,
        summary.critical_packages,
        summary.watch_packages,
        summary.automated_update_coverage_percent,
        summary.avg_release_gap,
        chart_bars,
        board
    );

    shell(
        "Control-plane summary for dependency drift risk.",
        "Package count, release lag, stale lockfiles, CVE pressure, and owner recommendations at a glance.",
        "overview",
        &body,
    )
}

pub fn render_packages() -> String {
    let rows = engine::packages()
        .iter()
        .map(package_card)
        .collect::<Vec<_>>()
        .join("");

    let body = format!(
        r#"
      <section class="page-section">
        <div class="section-head">
          <strong>Package board</strong>
          <h2>The packages most likely to turn silent drift into delivery risk.</h2>
          <p>This is the main operator surface for reviewing release gaps, stale lockfiles, breaking changes, and CVE pressure before they pile up inside a reliability lane.</p>
        </div>
        <div class="section-body">
          <div class="package-grid">{}</div>
        </div>
      </section>
        "#,
        rows
    );

    shell(
        "Review queue for package drift and upgrade pressure.",
        "The dependencies most likely to need containment or owner review first.",
        "packages",
        &body,
    )
}

pub fn render_review_queue() -> String {
    let rows = engine::review_queue()
        .iter()
        .map(|pkg| {
            format!(
                r#"<tr><td><strong>{}</strong><div class="meta-text">{}</div></td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td><span class="tag {}">{}</span></td></tr>"#,
                pkg.snapshot.package_name,
                pkg.snapshot.owner,
                pkg.snapshot.release_gap,
                pkg.snapshot.lockfile_age_days,
                pkg.snapshot.cve_signals,
                pkg.snapshot.adoption_scope,
                pkg.verdict,
                pkg.verdict
            )
        })
        .collect::<Vec<_>>()
        .join("");

    let audit_rows = engine::audit_log()
        .iter()
        .map(|event| {
            let result_class = match event.result.as_str() {
                "success" => "result-good",
                "warning" => "result-warning",
                _ => "result-bad",
            };
            format!(
                r#"<div class="log-line"><div class="log-time">{}</div><div class="log-action">{}</div><div class="log-resource"><strong>{}</strong><span>{}</span></div><div class="{}">{}</div></div>"#,
                event.timestamp, event.action, event.resource, event.detail, result_class, event.result
            )
        })
        .collect::<Vec<_>>()
        .join("");

    let body = format!(
        r#"
      <section class="page-section">
        <div class="section-head">
          <strong>Review queue</strong>
          <h2>The dependencies that deserve human review before the next release cycle moves ahead.</h2>
          <p>Queueing by verdict makes drift visible as a workload, not just a static report. Owners can see which package crossed the line and why.</p>
        </div>
        <div class="section-body">
          <div class="table-shell">
            <table>
              <thead>
                <tr>
                  <th>Package</th>
                  <th>Release gap</th>
                  <th>Lockfile age</th>
                  <th>CVE signals</th>
                  <th>Adoption scope</th>
                  <th>Verdict</th>
                </tr>
              </thead>
              <tbody>{}</tbody>
            </table>
          </div>
        </div>
      </section>
      <section class="page-section">
        <div class="section-head">
          <strong>Audit evidence</strong>
          <h2>A replayable log of scans, flags, and owner-lane recommendations.</h2>
          <p>Dependency monitoring gets more trustworthy when the system explains which rule fired, which package moved, and what recommendation was emitted next.</p>
        </div>
        <div class="section-body">
          <div class="log-shell">
            <div class="log-head">
              <div class="lights"><i></i><i></i><i></i></div>
              <strong>dependency-drift-watch runtime log</strong>
            </div>
            <div class="log-body">{}</div>
          </div>
        </div>
      </section>
        "#,
        rows, audit_rows
    );

    shell(
        "Queue packages by drift severity and owner review pressure.",
        "A reliability-first surface for deciding which dependency lanes should move before release debt compounds.",
        "review-queue",
        &body,
    )
}

pub fn render_owners() -> String {
    let cards = engine::owner_lanes()
        .iter()
        .map(owner_card)
        .collect::<Vec<_>>()
        .join("");

    let body = format!(
        r#"
      <section class="page-section">
        <div class="section-head">
          <strong>Owner lanes</strong>
          <h2>Dependency freshness improves when responsibility is visible, not implied.</h2>
          <p>Owner lanes reveal where release lag is concentrated, which ecosystems are drifting together, and which package should become the lane’s first remediation target.</p>
        </div>
        <div class="section-body">
          <div class="three-col">{}</div>
        </div>
      </section>
        "#,
        cards
    );

    shell(
        "Owner lanes for dependency stewardship and release planning.",
        "Group package drift by the teams that need to review, approve, or execute upgrades.",
        "owners",
        &body,
    )
}

pub fn render_metrics_preview() -> String {
    let metrics = engine::prometheus_metrics();
    let config = engine::policy_config();
    let body = format!(
        r#"
      <section class="page-section">
        <div class="section-head">
          <strong>Metrics surface</strong>
          <h2>The drift scan should be measurable before anyone wires an alert or a deployment gate on top.</h2>
          <p>This page makes the policy thresholds, metric series, and alert targets visible so the repo feels like a real reliability component instead of a generic watch script.</p>
        </div>
        <div class="section-body">
          <div class="insight-grid">
            <div class="panel">
              <h3>Policy configuration</h3>
              <div class="panel-grid">
                <div class="metric-card">
                  <div class="micro">Scan cadence</div>
                  <div class="title">{} hours</div>
                  <div class="desc">Frequent enough to catch release lag before it compounds across the next planning window.</div>
                </div>
                <div class="metric-card">
                  <div class="micro">Freshness window</div>
                  <div class="title">{} days</div>
                  <div class="desc">Packages older than this should start to feel review pressure even without active CVE signals.</div>
                </div>
                <div class="metric-card">
                  <div class="micro">Critical gap threshold</div>
                  <div class="title">{} releases</div>
                  <div class="desc">Crossing this line moves a package into a much less forgiving review posture.</div>
                </div>
              </div>
            </div>
            <div class="code-panel">
              <div class="code-head"><span>/metrics</span><div class="lights"><i></i><i></i><i></i></div></div>
              <pre><code>{}</code></pre>
            </div>
          </div>
        </div>
      </section>
        "#,
        config.scan_interval_hours,
        config.freshness_window_days,
        config.critical_gap_threshold,
        metrics
    );

    shell(
        "Metrics preview for dependency drift scoring and owner-lane alerts.",
        "Prometheus text output plus explicit drift policy posture.",
        "metrics",
        &body,
    )
}

pub fn render_docs() -> String {
    let body = r#"
      <section class="page-section">
        <div class="section-head">
          <strong>Docs</strong>
          <h2>What the service exposes and why each route exists.</h2>
          <p>The app is intentionally small: it is a drift-scoring and review surface for package freshness, not a full dependency-management platform.</p>
        </div>
        <div class="section-body">
          <div class="three-col">
            <div class="metric-card">
              <div class="micro">GET /api/dashboard/summary</div>
              <div class="title">Fleet posture</div>
              <div class="desc">Returns package count, critical drift, automation coverage, average release gap, and the lead recommendation.</div>
            </div>
            <div class="metric-card">
              <div class="micro">GET /api/packages</div>
              <div class="title">Package board</div>
              <div class="desc">Returns the full package fleet ordered by drift score with flags and owner recommendations attached.</div>
            </div>
            <div class="metric-card">
              <div class="micro">GET /metrics</div>
              <div class="title">Prometheus payload</div>
              <div class="desc">Exposes drift score, release gap, lockfile age, CVE pressure, and automation coverage as text metrics.</div>
            </div>
          </div>
        </div>
      </section>
    "#;

    shell(
        "Rust service documentation for dependency drift review and scoring.",
        "Prometheus metrics, owner-lane review posture, and a reliability-first package-risk model.",
        "docs",
        body,
    )
}

fn package_card(pkg: &PackageAssessment) -> String {
    let flags = if pkg.exposure_flags.is_empty() {
        r#"<span class="signal-pill">stable drift posture</span>"#.to_string()
    } else {
        pkg.exposure_flags
            .iter()
            .map(|flag| format!(r#"<span class="signal-pill">{}</span>"#, flag))
            .collect::<Vec<_>>()
            .join("")
    };

    format!(
        r#"<div class="package-card">
          <div class="package-top">
            <div>
              <h3>{name}</h3>
              <div class="meta-text">{ecosystem} · {owner} · {tier} · {scope} · {current} → {latest}</div>
            </div>
            <span class="tag {verdict_class}">{verdict}</span>
            <div class="score-stack"><div class="label">Drift score</div><div class="value">{score}</div></div>
          </div>
          <div class="package-bottom">
            <div class="two-col">
              <div>{bars}</div>
              <div class="panel-grid">
                <div class="metric-card">
                  <div class="micro">Top risk</div>
                  <div class="title">{top_risk}</div>
                  <div class="desc">{recommendation}</div>
                </div>
                <div class="metric-card">
                  <div class="micro">Exposure flags</div>
                  <div class="pill-stack">{flags}</div>
                </div>
              </div>
            </div>
          </div>
        </div>"#,
        name = pkg.snapshot.package_name,
        ecosystem = pkg.snapshot.ecosystem,
        owner = pkg.snapshot.owner,
        tier = pkg.snapshot.service_tier,
        scope = pkg.snapshot.adoption_scope,
        current = pkg.snapshot.current_version,
        latest = pkg.snapshot.latest_version,
        verdict_class = pkg.verdict,
        verdict = pkg.verdict,
        score = pkg.drift_score,
        bars = score_bars(pkg),
        top_risk = pkg.top_risk,
        recommendation = pkg.recommendation,
        flags = flags
    )
}

fn owner_card(owner: &OwnerLane) -> String {
    let ecosystems = owner
        .ecosystems
        .iter()
        .map(|eco| format!(r#"<span class="signal-pill">{}</span>"#, eco))
        .collect::<Vec<_>>()
        .join("");

    format!(
        r#"<div class="metric-card">
          <div class="micro">Owner lane</div>
          <div class="title">{owner_name}</div>
          <div class="desc">{count} packages · {critical} critical · average drift {avg}</div>
          <div class="pill-stack" style="margin-top:14px;">{ecosystems}</div>
          <div class="desc" style="margin-top:14px;"><strong>Focus package:</strong> {focus}</div>
        </div>"#,
        owner_name = owner.owner,
        count = owner.package_count,
        critical = owner.critical_count,
        avg = owner.avg_drift_score,
        ecosystems = ecosystems,
        focus = owner.focus_package
    )
}
