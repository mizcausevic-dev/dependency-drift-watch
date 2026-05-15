from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "screenshots"
OUT.mkdir(exist_ok=True)
LEGACY = [
    "01-overview.svg",
    "02-package-board.svg",
    "03-review-queue.svg",
    "04-metrics-proof.svg",
    "02-connector-board.svg",
    "03-audit-log.svg",
    "01-hero.png",
    "02-queue-lanes.png",
    "03-escalation-detail.png",
    "04-proof.png",
]


def write(name: str, content: str) -> None:
    (OUT / name).write_text(content, encoding="utf-8")


OVERVIEW = """<svg xmlns='http://www.w3.org/2000/svg' width='1600' height='980' viewBox='0 0 1600 980'>
  <rect width='1600' height='980' fill='#07101b'/>
  <rect x='0' y='0' width='260' height='980' fill='rgba(0,0,0,0.32)'/>
  <rect x='294' y='104' width='1240' height='248' rx='28' fill='#09101c' stroke='rgba(121,203,255,0.18)'/>
  <text x='332' y='146' fill='#79cbff' font-size='11' font-family='Segoe UI' letter-spacing='5'>DEPENDENCY DRIFT WATCH</text>
  <text x='332' y='212' fill='#f6f8fe' font-size='44' font-family='Georgia' font-weight='700'>Control-plane summary for dependency drift risk.</text>
  <text x='332' y='248' fill='#96a9c6' font-size='21' font-family='Segoe UI'>Package count, release lag, stale lockfiles, CVE pressure, and owner recommendations at a glance.</text>
  <rect x='332' y='274' width='280' height='132' rx='20' fill='rgba(255,255,255,0.04)' stroke='rgba(255,255,255,0.06)'/>
  <text x='354' y='302' fill='#71839d' font-size='10' font-family='Segoe UI' letter-spacing='3'>PACKAGES</text>
  <text x='354' y='346' fill='#f6f8fe' font-size='38' font-family='Segoe UI' font-weight='700'>6</text>
  <rect x='628' y='274' width='280' height='132' rx='20' fill='rgba(255,255,255,0.04)' stroke='rgba(255,255,255,0.06)'/>
  <text x='650' y='302' fill='#71839d' font-size='10' font-family='Segoe UI' letter-spacing='3'>CRITICAL DRIFT</text>
  <text x='650' y='346' fill='#f6f8fe' font-size='38' font-family='Segoe UI' font-weight='700'>3</text>
  <rect x='924' y='274' width='280' height='132' rx='20' fill='rgba(255,255,255,0.04)' stroke='rgba(255,255,255,0.06)'/>
  <text x='946' y='302' fill='#71839d' font-size='10' font-family='Segoe UI' letter-spacing='3'>AUTOMATION COVERAGE</text>
  <text x='946' y='346' fill='#f6f8fe' font-size='38' font-family='Segoe UI' font-weight='700'>33%</text>
  <rect x='1220' y='274' width='280' height='132' rx='20' fill='rgba(255,255,255,0.04)' stroke='rgba(255,255,255,0.06)'/>
  <text x='1242' y='302' fill='#71839d' font-size='10' font-family='Segoe UI' letter-spacing='3'>AVG RELEASE GAP</text>
  <text x='1242' y='346' fill='#f6f8fe' font-size='38' font-family='Segoe UI' font-weight='700'>5</text>
  <rect x='332' y='500' width='1240' height='356' rx='22' fill='rgba(4,9,18,0.55)' stroke='rgba(255,255,255,0.06)'/>
  <text x='356' y='534' fill='#79cbff' font-size='10' font-family='Segoe UI' letter-spacing='3'>TOP PACKAGE BOARD</text>
  <text x='356' y='566' fill='#f6f8fe' font-size='20' font-family='Georgia' font-weight='700'>The riskiest package lanes stay visible.</text>
</svg>"""

PACKAGES = """<svg xmlns='http://www.w3.org/2000/svg' width='1600' height='980' viewBox='0 0 1600 980'>
  <rect width='1600' height='980' fill='#07101b'/>
  <rect x='332' y='392' width='1240' height='496' rx='24' fill='rgba(10,18,33,0.88)' stroke='rgba(121,203,255,0.16)'/>
  <text x='356' y='426' fill='#79cbff' font-size='10' font-family='Segoe UI' letter-spacing='3'>PACKAGE BOARD</text>
  <text x='356' y='462' fill='#f6f8fe' font-size='24' font-family='Georgia' font-weight='700'>The dependencies most likely to need owner review first.</text>
  <text x='356' y='492' fill='#96a9c6' font-size='15' font-family='Segoe UI'>Release gap, stale lockfiles, breaking changes, and CVE pressure all stay visible on the same board.</text>
</svg>"""

QUEUE = """<svg xmlns='http://www.w3.org/2000/svg' width='1600' height='980' viewBox='0 0 1600 980'>
  <rect width='1600' height='980' fill='#07101b'/>
  <rect x='332' y='392' width='1240' height='496' rx='24' fill='rgba(10,18,33,0.88)' stroke='rgba(121,203,255,0.16)'/>
  <text x='356' y='426' fill='#79cbff' font-size='10' font-family='Segoe UI' letter-spacing='3'>REVIEW QUEUE</text>
  <text x='356' y='462' fill='#f6f8fe' font-size='24' font-family='Georgia' font-weight='700'>Queue packages by drift severity and owner pressure.</text>
</svg>"""

METRICS = """<svg xmlns='http://www.w3.org/2000/svg' width='1600' height='980' viewBox='0 0 1600 980'>
  <rect width='1600' height='980' fill='#07101b'/>
  <rect x='332' y='392' width='1240' height='496' rx='24' fill='rgba(10,18,33,0.88)' stroke='rgba(121,203,255,0.16)'/>
  <text x='356' y='426' fill='#79cbff' font-size='10' font-family='Segoe UI' letter-spacing='3'>METRICS SURFACE</text>
  <text x='356' y='462' fill='#f6f8fe' font-size='24' font-family='Georgia' font-weight='700'>Prometheus text output plus explicit drift policy posture.</text>
</svg>"""


if __name__ == "__main__":
    for legacy in LEGACY:
        (OUT / legacy).unlink(missing_ok=True)
    write("01-overview.svg", OVERVIEW)
    write("02-package-board.svg", PACKAGES)
    write("03-review-queue.svg", QUEUE)
    write("04-metrics-proof.svg", METRICS)
    print("rendered screenshots")
