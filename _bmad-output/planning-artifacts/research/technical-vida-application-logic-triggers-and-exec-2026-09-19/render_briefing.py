"""Render the cited Markdown report into a standalone offline briefing."""

from __future__ import annotations

import html
import json
import os
from pathlib import Path
import re
import subprocess
import sys

import markdown


ROOT = Path(__file__).resolve().parent
REPORT = ROOT / "research.md"
OUTPUT = ROOT / "research-briefing.html"
RECON = Path("C:/Users/pomaz/.codex/plugins/cache/bmad/bmad-toolbox/6.13.0-next/skills/bmad-deep-recon/scripts/recon_kit.py")


def main() -> None:
    text = REPORT.read_text(encoding="utf-8")
    if text.startswith("---\n"):
        text = text.split("---\n", 2)[2]
    before, after = text.split("## Джерела\n", 1)
    _, freshness = after.split("## Актуальність\n", 1)
    document = before + "## Джерела\n\nSRC-TABLE-PLACEHOLDER\n\n## Актуальність\n" + freshness
    md = markdown.Markdown(extensions=["tables", "toc", "fenced_code"])
    body = md.convert(document)
    for number in range(15, 0, -1):
        body = body.replace(f"[{number}]", f'<a class="cite" href="#src-{number}">[{number}]</a>')
    escaped = subprocess.run(
        [sys.executable, str(RECON), "escape-sources", str(REPORT)],
        capture_output=True,
        text=True,
        check=True,
        encoding="utf-8",
        env={**os.environ, "PYTHONIOENCODING": "utf-8"},
    )
    source_data = json.loads(escaped.stdout)
    if source_data["invalid_urls"]:
        raise ValueError(f"Invalid source URLs: {source_data['invalid_urls']}")
    body = body.replace(
        "<p>SRC-TABLE-PLACEHOLDER</p>",
        "<details open><summary>15 першоджерел · переглянути таблицю</summary>"
        + source_data["html"]
        + "</details>",
    )
    headings = re.findall(r'<h2 id="([^"]+)">([^<]+)</h2>', body)
    toc = "".join(f'<a href="#{html.escape(anchor)}">{html.escape(label)}</a>' for anchor, label in headings)
    page = """<!doctype html><html lang="uk"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><title>VIDA · Logic trigger research</title><style>
:root{color-scheme:light dark;--bg:#f6f7fb;--fg:#172033;--card:#fff;--muted:#586478;--line:#dce3ee;--accent:#335bd4;--warn:#8d4106}
@media(prefers-color-scheme:dark){:root{--bg:#101825;--fg:#eef2fb;--card:#1b2636;--muted:#b4c0d3;--line:#37475d;--accent:#94b1ff;--warn:#ffba75}}
*{box-sizing:border-box}body{margin:0;background:var(--bg);color:var(--fg);font:16px/1.6 system-ui,-apple-system,Segoe UI,sans-serif}a{color:var(--accent)}header{padding:2rem max(1rem,calc((100vw - 1120px)/2));background:var(--card);border-bottom:1px solid var(--line)}header h1{margin:.2rem 0;font-size:clamp(1.8rem,3vw,2.7rem)}.meta{color:var(--muted)}.shell{max-width:1120px;margin:auto;padding:1rem;display:grid;grid-template-columns:220px minmax(0,1fr);gap:1.5rem}.toc{position:sticky;top:1rem;align-self:start;display:grid;gap:.35rem;padding:1rem;background:var(--card);border:1px solid var(--line);border-radius:12px;max-height:90vh;overflow:auto}.toc a{text-decoration:none;font-size:.9rem}main{min-width:0}section{background:var(--card);border:1px solid var(--line);border-radius:14px;padding:1rem 1.4rem;margin:0 0 1rem}h2{margin:1.5rem 0 .6rem;scroll-margin-top:1rem}h3{scroll-margin-top:1rem}table{border-collapse:collapse;width:100%;font-size:.88rem;display:block;overflow-x:auto}th,td{border:1px solid var(--line);padding:.55rem;text-align:left;vertical-align:top}th{background:var(--bg)}code{background:var(--bg);padding:.15rem .3rem;border-radius:4px}blockquote{border-left:3px solid var(--accent);padding-left:1rem}details{margin:.5rem 0}summary{cursor:pointer;color:var(--accent);font-weight:600}.cite{font-size:.8em;margin-left:.08em}.badge{display:inline-block;padding:.16rem .55rem;border:1px solid var(--line);border-radius:999px;font-size:.82rem}.caution{color:var(--warn)}.sources td:last-child{font-weight:600}p,li{max-width:90ch}@media(max-width:760px){.shell{display:block}.toc{position:static;max-height:none;margin-bottom:1rem}.sources{font-size:.78rem}}
</style></head><body><header><div class="meta">Технічне дослідження · 19 вересня 2026 · focused / normal verification</div><h1>VIDA: що запускає прикладну логіку</h1><div class="meta">Для рішення щодо OQ-0045 · архітектурні пропозиції не затверджено</div><p><span class="badge">3 незалежно перевірені узагальнення</span> <span class="badge caution">12 фактів з одним першоджерелом</span></p></header><div class="shell"><nav class="toc" aria-label="Зміст"><strong>Зміст</strong>""" + toc + """</nav><main><section>""" + body + """</section></main></div></body></html>"""
    OUTPUT.write_text(page, encoding="utf-8")
    print(f"Rendered {OUTPUT} ({len(page)} chars, {source_data['rows']} sources)")


if __name__ == "__main__":
    main()
