"""Render the offline research briefing from research.md (stdlib only)."""

from __future__ import annotations

import html
import json
import os
import re
import subprocess
import sys
from pathlib import Path
from urllib.parse import urlparse

ROOT = Path(__file__).resolve().parent
REPORT = ROOT / "research.md"
KIT = Path("C:/Users/pomaz/.codex/plugins/cache/bmad/bmad-toolbox/6.13.0-next/skills/bmad-deep-recon/scripts/recon_kit.py")


def kit(*args: str) -> dict:
    run = subprocess.run([sys.executable, str(KIT), *args], check=True, text=True, encoding="utf-8", capture_output=True, env={**os.environ, "PYTHONIOENCODING": "utf-8"})
    return json.loads(run.stdout)


def inline(value: str, refs: dict[str, str]) -> str:
    value = html.escape(value)
    value = re.sub(r"`([^`]+)`", r"<code>\1</code>", value)
    value = re.sub(r"\*\*([^*]+)\*\*", r"<strong>\1</strong>", value)

    def direct(match: re.Match[str]) -> str:
        url = html.unescape(match.group(2))
        return f'<a href="{html.escape(url, quote=True)}">{match.group(1)}</a>' if urlparse(url).scheme in {"http", "https"} else match.group(1)

    value = re.sub(r"\[([^\]]+)\]\((https?://[^)]+)\)", direct, value)
    value = re.sub(r"\[([^\]]+)\]\[(\d+)\]", lambda m: f'<a href="#src-{m.group(2)}">{m.group(1)}</a>', value)
    value = re.sub(r"(?<!\])\[(\d+)\]", r'<a class="cite" href="#src-\1">[\1]</a>', value)
    return value


def render_lines(lines: list[str], refs: dict[str, str]) -> str:
    out: list[str] = []
    items: list[str] = []
    kind = "ul"

    def flush() -> None:
        nonlocal items
        if items:
            out.append(f"<{kind}>" + "".join(items) + f"</{kind}>")
            items = []

    for line in lines:
        if not line.strip() or re.match(r"^\[\d+\]:\s", line):
            flush()
            continue
        if line.startswith("### "):
            flush()
            out.append(f"<h3>{inline(line[4:], refs)}</h3>")
        elif line.startswith("| "):
            flush()
            if re.match(r"^\|[\s:|-]+\|$", line):
                continue
            cells = [inline(cell.strip(), refs) for cell in line.strip().strip("|").split("|")]
            out.append("<div class='tablerow'>" + "".join(f"<span>{cell}</span>" for cell in cells) + "</div>")
        elif line.startswith("- ") or re.match(r"^\d+\. ", line):
            next_kind = "ol" if re.match(r"^\d+\. ", line) else "ul"
            if items and kind != next_kind:
                flush()
            kind = next_kind
            content = re.sub(r"^\d+\. ", "", line) if kind == "ol" else line[2:]
            items.append(f"<li>{inline(content, refs)}</li>")
        else:
            flush()
            out.append(f"<p>{inline(line, refs)}</p>")
    flush()
    return "\n".join(out)


def main() -> None:
    raw = REPORT.read_text(encoding="utf-8")
    _, front, body = raw.split("---\n", 2)
    metadata = dict(re.findall(r"^([a-z_]+):\s*['\"]?(.+?)['\"]?$", front, re.M))
    refs = dict(re.findall(r"(?m)^\[(\d+)\]:\s+(https?://\S+)", body))
    source_html = kit("escape-sources", str(REPORT))
    if source_html["invalid_urls"]:
        raise ValueError(source_html["invalid_urls"])
    sections = re.split(r"(?m)^## (.+)\n", body)
    lead = render_lines(sections[0].splitlines()[2:], refs)
    toc: list[str] = []
    cards: list[str] = []
    for index in range(1, len(sections), 2):
        heading, content = sections[index], sections[index + 1]
        if heading == "Джерела":
            continue
        anchor = f"s{index}"
        toc.append(f'<a href="#{anchor}">{html.escape(heading)}</a>')
        cards.append(f'<section id="{anchor}"><h2>{html.escape(heading)}</h2>{render_lines(content.splitlines(), refs)}</section>')
    title = html.escape(metadata.get("title", "VIDA research"))
    decision = html.escape(metadata.get("decision", ""))
    page = f"""<!doctype html><html lang="uk"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>{title}</title>
<style>:root{{color-scheme:light dark;--bg:#f5f3ef;--fg:#242424;--card:#fff;--line:#d7d1c9;--accent:#a44c37}}@media(prefers-color-scheme:dark){{:root{{--bg:#1d1e20;--fg:#eee8df;--card:#292b2e;--line:#4c4b49;--accent:#e99a7c}}}}*{{box-sizing:border-box}}body{{margin:0;background:var(--bg);color:var(--fg);font:16px/1.6 system-ui,sans-serif}}header,main{{max-width:1100px;margin:auto;padding:1.2rem}}h1{{font-size:clamp(1.8rem,4vw,2.7rem);line-height:1.15}}h2{{line-height:1.25}}h3{{margin-top:1.5rem}}nav{{position:sticky;top:0;display:flex;gap:1rem;overflow:auto;white-space:nowrap;padding:.7rem 1rem;background:var(--card);border-block:1px solid var(--line);z-index:2}}section,.lead,details{{background:var(--card);padding:1rem 1.35rem;margin:1rem 0;border:1px solid var(--line);border-radius:12px}}.lead{{border-left:5px solid var(--accent)}}section{{scroll-margin-top:5rem}}a{{color:var(--accent)}}code{{background:var(--bg);border-radius:4px;padding:0 .25rem}}.tablerow{{display:grid;grid-template-columns:repeat(auto-fit,minmax(180px,1fr));border:1px solid var(--line);margin:.15rem 0}}.tablerow span{{padding:.55rem;border-right:1px solid var(--line);overflow-wrap:anywhere}}li{{margin:.4rem 0}}details summary{{cursor:pointer;font-weight:700}}.warn{{display:inline-block;background:#812b2b;color:white;border-radius:5px;padding:.15rem .5rem}}</style></head><body>
<header><small>ТЕХНІЧНЕ ДОСЛІДЖЕННЯ · ОНОВЛЕНО 2026-09-28 · NORMAL VALIDATION</small><h1>{title}</h1><p>{decision}</p><span class="warn">Продуктовий напрям погоджено · WebRTC direct не доведено</span></header>
<main><div class="lead">{lead}</div><nav aria-label="Зміст">{''.join(toc)}<a href="#sources">Джерела</a></nav>{''.join(cards)}<details id="sources"><summary>Джерела ({source_html['rows']})</summary>{source_html['html']}</details></main></body></html>"""
    target = ROOT / "research-briefing.html"
    target.write_text(page, encoding="utf-8")
    windows = {"technical-library-version": 1, "platform-behavior": 3, "stable-technical-pattern": 24}
    staleness = kit("staleness", str(ROOT / "staleness-input.json"), "--windows", json.dumps(windows), "--today", "2026-09-28")
    print(f"{target} ({source_html['rows']} sources)")
    print(json.dumps(staleness, ensure_ascii=False))


if __name__ == "__main__":
    main()
