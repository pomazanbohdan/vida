"""Render the focused research into one offline HTML briefing (standard library only)."""

from __future__ import annotations

import html
import json
import re
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parent
REPORT = ROOT / "research.md"
KIT = Path(
    "C:/Users/pomaz/.codex/plugins/cache/bmad/bmad-toolbox/6.13.0-next/"
    "skills/bmad-deep-recon/scripts/recon_kit.py"
)


def inline(value: str) -> str:
    value = html.escape(value)
    value = re.sub(r"`([^`]+)`", r"<code>\1</code>", value)
    value = re.sub(r"\*\*([^*]+)\*\*", r"<strong>\1</strong>", value)
    value = re.sub(
        r"\[([^\]]+)\]\((https?://[^)]+)\)",
        lambda m: f'<a href="{html.escape(m.group(2), quote=True)}">{m.group(1)}</a>',
        value,
    )
    value = re.sub(r"\[(\d+)\]", r'<a class="cite" href="#src-\1">[\1]</a>', value)
    return value


def blocks(section: str) -> str:
    output: list[str] = []
    items: list[str] = []
    ordered = False

    def flush() -> None:
        nonlocal items
        if items:
            tag = "ol" if ordered else "ul"
            output.append(f"<{tag}>" + "".join(items) + f"</{tag}>")
            items = []

    for line in section.strip().splitlines():
        if not line.strip():
            flush()
        elif line.startswith("- "):
            if ordered:
                flush()
            ordered = False
            items.append(f"<li>{inline(line[2:])}</li>")
        elif re.match(r"^\d+\. ", line):
            if items and not ordered:
                flush()
            ordered = True
            items.append(f"<li>{inline(re.sub(r'^\d+\. ', '', line))}</li>")
        else:
            flush()
            output.append(f"<p>{inline(line)}</p>")
    flush()
    return "\n".join(output)


def render() -> None:
    raw = REPORT.read_text(encoding="utf-8")
    front, body = raw.split("---\n", 2)[1:]
    metadata = dict(re.findall(r"^([a-z_]+):\s*['\"]?(.+?)['\"]?$", front, re.M))
    body = body.split("\n## Sources\n", 1)[0]
    sections = re.split(r"(?m)^## (.+)\n", body)
    summary = ""
    cards: list[str] = []
    toc: list[str] = []
    for i in range(1, len(sections), 2):
        heading, content = sections[i], sections[i + 1]
        if heading == "Executive summary":
            summary = blocks(content)
            continue
        anchor = f"section-{i}"
        toc.append(f'<a href="#{anchor}">{html.escape(heading)}</a>')
        cards.append(f'<section id="{anchor}"><h2>{html.escape(heading)}</h2>{blocks(content)}</section>')

    result = subprocess.run(
        [sys.executable, str(KIT), "escape-sources", str(REPORT)],
        check=True,
        capture_output=True,
        text=True,
        encoding="utf-8",
    )
    sources = json.loads(result.stdout)
    if sources["invalid_urls"]:
        raise ValueError(f"Invalid source URLs: {sources['invalid_urls']}")
    title = html.escape(metadata.get("title", "VIDA research"))
    content = "\n".join(cards)
    menu = "\n".join(toc)
    output = f"""<!doctype html><html lang="uk"><head><meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1"><title>{title}</title>
<style>:root{{color-scheme:light dark;--bg:#f7f9fc;--fg:#172338;--card:#fff;--line:#d9e2ed;--accent:#2856a0}}
@media(prefers-color-scheme:dark){{:root{{--bg:#10151e;--fg:#eaf0f8;--card:#1b2432;--line:#364254;--accent:#a5c8ff}}}}
*{{box-sizing:border-box}}body{{margin:0;background:var(--bg);color:var(--fg);font:16px/1.55 system-ui,sans-serif}}
header,main{{max-width:1080px;margin:auto;padding:1.2rem}}h1{{font-size:clamp(1.6rem,4vw,2.5rem)}}
nav{{position:sticky;top:0;background:var(--card);padding:.65rem;display:flex;gap:.8rem;overflow:auto;white-space:nowrap;border-block:1px solid var(--line)}}
section,.summary{{background:var(--card);padding:1rem 1.4rem;margin:1.2rem 0;border:1px solid var(--line);border-radius:10px}}
.summary{{border-left:5px solid var(--accent)}}section{{scroll-margin-top:5rem}}a{{color:var(--accent)}}
table{{display:block;overflow:auto;border-collapse:collapse}}td,th{{border:1px solid var(--line);padding:.5rem}}
details{{margin:1.2rem 0;background:var(--card);padding:1rem;border:1px solid var(--line);border-radius:10px}}
summary{{cursor:pointer;font-weight:650}}.unverified{{background:#a61e32;color:#fff;padding:.1rem .4rem;border-radius:4px}}
code{{background:var(--bg);padding:.1rem .25rem;border-radius:4px}}.cite{{white-space:nowrap}}
</style></head><body><header><small>TECHNICAL RESEARCH · 2026-09-20 · NORMAL VALIDATION</small>
<h1>{title}</h1><p>Decision: {html.escape(metadata.get('decision', ''))}</p></header>
<main><div class="summary"><h2>Executive summary</h2>{summary}</div>
<nav aria-label="Зміст">{menu}<a href="#sources">Sources</a></nav>{content}
<div id="sources"><details><summary>Sources ({sources['rows']}) · <span class="unverified">4 ledger claims single-source</span></summary>{sources['html']}</details></div>
</main></body></html>"""
    target = ROOT / "research-briefing.html"
    target.write_text(output, encoding="utf-8")
    print(f"{target} ({sources['rows']} sources)")


if __name__ == "__main__":
    render()
