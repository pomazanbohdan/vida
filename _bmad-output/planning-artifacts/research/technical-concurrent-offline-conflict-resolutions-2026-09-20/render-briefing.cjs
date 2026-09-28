// Render the cited Markdown report as one offline HTML page. Generated output is not the source of truth.
const fs = require('node:fs');
const path = require('node:path');
const { execFileSync } = require('node:child_process');
const { marked } = require('C:/Users/pomaz/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/node_modules/marked');

const runDir = __dirname;
const reportPath = path.join(runDir, 'research.md');
const kitPath = 'C:/Users/pomaz/.codex/plugins/cache/bmad/bmad-toolbox/6.13.0-next/skills/bmad-deep-recon/scripts/recon_kit.py';
const report = fs.readFileSync(reportPath, 'utf8');
const sourceStart = report.indexOf('\n## Джерела\n');
const freshnessStart = report.indexOf('\n## Актуальність\n');
if (sourceStart < 0 || freshnessStart < sourceStart) throw new Error('Report sections missing');

const sources = JSON.parse(execFileSync('uv', ['run', kitPath, 'escape-sources', reportPath], {
  cwd: 'C:/project/vida', encoding: 'utf8'
}));
if (sources.invalid_urls.length) throw new Error(`Invalid source URLs: ${sources.invalid_urls.join(', ')}`);

const preface = report.replace(/^---\r?\n[\s\S]*?\r?\n---\r?\n/, '');
const bodyMd = preface.slice(0, preface.indexOf('\n## Джерела\n'));
const freshnessMd = preface.slice(preface.indexOf('\n## Актуальність\n'));
const withSourceLinks = (s) => s.replace(/\[(\d+)\]/g, (_, n) => `[${n}](#src-${n})`);
let body = marked.parse(withSourceLinks(bodyMd), { gfm: true });
let nav = '';
let section = 0;
body = body.replace(/<h2>([\s\S]*?)<\/h2>/g, (_, heading) => {
  section += 1;
  const id = `section-${section}`;
  const label = heading.replace(/<[^>]+>/g, '');
  nav += `<a href="#${id}">${label}</a>`;
  return `<h2 id="${id}">${heading}</h2>`;
});
body = body.replace(/(<h2 id="section-1">[\s\S]*?<\/h2>)([\s\S]*?)(?=<h2 id="section-2">)/,
  '<section class="summary">$1$2</section>');
const freshness = marked.parse(withSourceLinks(freshnessMd), { gfm: true });
const html = `<!doctype html>
<html lang="uk"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>VIDA · два офлайн-рішення одного конфлікту</title>
<style>
:root{color-scheme:light dark;--bg:#f5f7fb;--panel:#fff;--fg:#192638;--muted:#526171;--line:#d6deea;--accent:#1769aa;--soft:#e9f4fc;--warn:#815619}
@media(prefers-color-scheme:dark){:root{--bg:#101720;--panel:#172231;--fg:#e6edf5;--muted:#a5b3c2;--line:#324052;--accent:#8dc4ff;--soft:#1e3447;--warn:#f4c979}}
*{box-sizing:border-box}html{scroll-behavior:smooth}body{margin:0;background:var(--bg);color:var(--fg);font:16px/1.6 system-ui,-apple-system,"Segoe UI",sans-serif}header{padding:2.2rem max(1rem,calc((100vw - 1120px)/2));background:var(--panel);border-bottom:1px solid var(--line)}header h1{margin:.3rem 0;font-size:clamp(1.55rem,3vw,2.4rem)}.meta{color:var(--muted);font-size:.9rem}.layout{max-width:1120px;margin:auto;display:grid;grid-template-columns:220px minmax(0,1fr);gap:1.6rem;padding:1.5rem 1rem 4rem}nav{position:sticky;top:1rem;align-self:start;max-height:calc(100vh - 2rem);overflow:auto;background:var(--panel);border:1px solid var(--line);border-radius:12px;padding:1rem}nav strong{display:block;margin-bottom:.6rem}nav a{display:block;color:var(--accent);text-decoration:none;padding:.18rem 0}main{min-width:0;background:var(--panel);border:1px solid var(--line);border-radius:12px;padding:1.5rem 2rem}main>h1{display:none}.summary{border-left:4px solid var(--accent);background:var(--soft);padding:.15rem 1rem;margin-bottom:1.5rem;border-radius:0 8px 8px 0}.summary h2{margin-top:1rem}h2{margin-top:2.2rem;border-bottom:1px solid var(--line);padding-bottom:.3rem}h3{margin-top:1.5rem}a{color:var(--accent)}p,li{max-width:83ch}table{border-collapse:collapse;display:block;overflow-x:auto;width:100%;font-size:.91rem}th,td{border:1px solid var(--line);padding:.45rem .6rem;text-align:left;vertical-align:top}th{background:var(--soft)}code,pre{font-family:ui-monospace,SFMono-Regular,Consolas,monospace}pre{overflow:auto;background:var(--soft);border:1px solid var(--line);border-radius:8px;padding:1rem}details{margin-top:2.2rem;border:1px solid var(--line);border-radius:8px;padding:.8rem}summary{cursor:pointer;font-weight:700}.sources td:last-child{color:var(--warn);font-weight:600}.freshness{margin-top:2rem}a[href^="#src-"]{font-size:.82em;font-weight:700;white-space:nowrap}
@media(max-width:760px){.layout{display:block}nav{position:static;max-height:none;margin-bottom:1rem}main{padding:1.2rem}header{padding:1.4rem 1rem}}
</style></head><body>
<header><div class="meta">VIDA · Technical research · 20.09.2026 · стандартна глибина · перевірка: normal</div><h1>Два офлайн-рішення одного конфлікту</h1><div class="meta">Питання: що робити, коли двоє уповноважених людей по-різному вирішили той самий конфлікт? Рекомендація ще не затверджена.</div></header>
<div class="layout"><nav aria-label="Зміст"><strong>Зміст</strong>${nav}<a href="#sources">Джерела</a><a href="#freshness">Актуальність</a></nav><main>${body}
<details id="sources"><summary>Джерела · ${sources.rows} перевірених посилань</summary>${sources.html}</details>
<section id="freshness" class="freshness">${freshness}</section></main></div></body></html>`;
fs.writeFileSync(path.join(runDir, 'research-briefing.html'), html, 'utf8');
console.log(`rendered ${path.join(runDir, 'research-briefing.html')}`);
