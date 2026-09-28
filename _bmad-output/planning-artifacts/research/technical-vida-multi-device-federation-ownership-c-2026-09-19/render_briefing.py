"""Render the research markdown to an offline, self-contained briefing."""

from __future__ import annotations

import html
import json
import os
import re
import subprocess
import sys
from pathlib import Path

from markdown_it import MarkdownIt


ROOT = Path(__file__).resolve().parent
REPORT = ROOT / "research.md"
KIT = Path(
    "C:/Users/pomaz/.codex/plugins/cache/bmad/bmad-toolbox/6.13.0-next/"
    "skills/bmad-deep-recon/scripts/recon_kit.py"
)


def render() -> None:
    raw = REPORT.read_text(encoding="utf-8")
    front, body = raw.split("---\n", 2)[1:]
    metadata = dict(re.findall(r"^([a-z_]+):\s*['\"]?(.+?)['\"]?$", front, re.M))
    body = body.split("\n## Джерела\n", 1)[0]
    body = body.split("\n# VIDA: ", 1)[1]
    body = "# VIDA: " + body
    summary_match = re.search(
        r"(?ms)^## Висновок для рішення\n(.*?)(?=^## )", body
    )
    if summary_match is None:
        raise ValueError("Executive summary section missing")
    summary = summary_match.group(1)
    body = body[: summary_match.start()] + body[summary_match.end() :]
    md = MarkdownIt("commonmark", {"html": False}).enable("table")
    content = md.render(body)
    summary_html = md.render(summary)
    headings: list[tuple[str, str]] = []

    def heading_id(match: re.Match[str]) -> str:
        anchor = f"section-{len(headings) + 1}"
        label = re.sub(r"<[^>]*>", "", match.group(1))
        headings.append((anchor, label))
        return f'<h2 id="{anchor}">{match.group(1)}</h2>'

    content = re.sub(r"<h2>(.*?)</h2>", heading_id, content)
    for name in ("content", "summary_html"):
        fragment = locals()[name]
        fragment = re.sub(
            r"\[(\d+)\]",
            lambda match: f'<a class="cite" href="#src-{match.group(1)}">[{match.group(1)}]</a>',
            fragment,
        )
        if name == "content":
            content = fragment
        else:
            summary_html = fragment

    source_result = subprocess.run(
        [sys.executable, str(KIT), "escape-sources", str(REPORT)],
        check=True,
        capture_output=True,
        text=True,
        encoding="utf-8",
        env={**os.environ, "PYTHONIOENCODING": "utf-8"},
    )
    source_data = json.loads(source_result.stdout)
    if source_data["invalid_urls"]:
        raise ValueError(f"Invalid source URLs: {source_data['invalid_urls']}")
    toc = "".join(
        f'<a href="#{anchor}">{html.escape(label)}</a>' for anchor, label in headings
    )
    title = html.escape(metadata.get("title", "VIDA research"))
    date = html.escape(metadata.get("updated", "2026-09-19"))
    output = f"""<!doctype html>
<html lang="uk"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title}</title><style>
:root {{ color-scheme:light dark; --bg:#f7f9fc; --fg:#172338; --card:#fff; --line:#d9e2ed; --accent:#2856a0; --muted:#50627d; }}
@media (prefers-color-scheme:dark) {{ :root {{ --bg:#10151e; --fg:#eaf0f8; --card:#1b2432; --line:#364254; --accent:#9fc3ff; --muted:#bac8db; }} }}
* {{ box-sizing:border-box }} html {{ scroll-behavior:smooth }} body {{ margin:0; background:var(--bg); color:var(--fg); font:16px/1.55 system-ui,-apple-system,"Segoe UI",sans-serif }}
header,main,footer {{ max-width:1080px; margin:auto; padding:1.2rem }} header {{ padding-top:2.7rem }} h1 {{ font-size:clamp(1.6rem,4vw,2.5rem); line-height:1.17 }} h2 {{ border-bottom:1px solid var(--line); padding-bottom:.35rem; scroll-margin-top:5rem }} h3 {{ scroll-margin-top:5rem }}
.meta {{ color:var(--muted); font-size:.9rem }} .summary {{ background:var(--card); border:1px solid var(--line); border-left:5px solid var(--accent); border-radius:10px; padding:1rem 1.35rem; margin:1rem 0 }}
nav {{ position:sticky; top:0; z-index:5; background:var(--card); border-block:1px solid var(--line); padding:.65rem 1rem; display:flex; gap:.9rem; overflow:auto; white-space:nowrap }}
nav a, a {{ color:var(--accent) }} nav a {{ font-size:.9rem; text-decoration:none }} section {{ margin:2rem 0 }} table {{ display:block; overflow:auto; border-collapse:collapse; width:100%; font-size:.9rem }} th,td {{ border:1px solid var(--line); padding:.55rem; vertical-align:top }} th {{ background:var(--card) }}
code {{ background:var(--card); border:1px solid var(--line); border-radius:4px; padding:.1em .25em }} .cite {{ white-space:nowrap; font-size:.88em }} details {{ border:1px solid var(--line); border-radius:8px; padding:.8rem; background:var(--card) }} summary {{ cursor:pointer; font-weight:650 }}
.badge {{ display:inline-block; padding:.12rem .45rem; border-radius:999px; font-size:.76rem; font-weight:700; margin-inline:.2rem }} .verified {{ background:#d6f1e0; color:#14512c }} .medium {{ background:#fff0c7; color:#654b00 }} .unverified {{ background:#f9d4d7; color:#7f1020; border:1px solid #c53142 }}
@media (prefers-color-scheme:dark) {{ .verified {{ background:#184c32; color:#c8f7d5 }} .medium {{ background:#5c4610; color:#fff1bd }} .unverified {{ background:#651e29; color:#ffe5e8 }} }}
</style></head><body>
<header><div class="meta">TECHNICAL RESEARCH · {date} · STANDARD · NORMAL VALIDATION</div><h1>VIDA: багатопристроєві зміни, федерація та підтвердження операцій</h1><p class="meta">Рішення: синхронізація · вузли федерації · authority та квитанції бронювання</p></header>
<main><div class="summary"><h2>Висновок для рішення</h2>{summary_html}</div>
<nav aria-label="Зміст">{toc}<a href="#sources">Джерела</a></nav>
<section>{content}</section>
<section id="sources"><details><summary>Джерела ({source_data['rows']}) · <span class="badge verified">verified</span><span class="badge medium">medium</span><span class="badge unverified">unverified</span></summary>{source_data['html']}</details></section>
</main><footer class="meta">Єдине джерело змісту — research.md; дата наступної перевірки live API наведена у звіті.</footer></body></html>"""
    target = ROOT / "research-briefing.html"
    target.write_text(output, encoding="utf-8")
    print(f"{target} ({len(output)} chars, {source_data['rows']} sources)")


if __name__ == "__main__":
    render()
