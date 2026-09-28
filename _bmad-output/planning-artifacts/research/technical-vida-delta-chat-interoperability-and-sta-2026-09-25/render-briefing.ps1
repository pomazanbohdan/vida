param()

$runDir = Split-Path -Parent $PSCommandPath
$reportPath = Join-Path $runDir 'research.md'
$outputPath = Join-Path $runDir 'research-briefing.html'
$reconKit = 'C:/Users/pomaz/.codex/plugins/cache/bmad/bmad-toolbox/6.13.0-next/skills/bmad-deep-recon/scripts/recon_kit.py'

$markdown = Get-Content -LiteralPath $reportPath -Raw -Encoding UTF8
$markdown = [regex]::Replace($markdown, '\A---\r?\n.*?\r?\n---\r?\n', '', [System.Text.RegularExpressions.RegexOptions]::Singleline)
$tableStart = $markdown.IndexOf('| № |')
$localStart = $markdown.IndexOf('Локальні нормативні джерела:', $tableStart)
if ($tableStart -lt 0 -or $localStart -lt 0) { throw 'Source appendix boundaries not found.' }
$markdown = $markdown.Substring(0, $tableStart) + "`n@@SOURCE_TABLE@@`n`n" + $markdown.Substring($localStart)
$markdown = [regex]::Replace($markdown, '(?<!\!)\[(\d+)\](?!\()', '[$1](#src-$1)')
$sourceJson = & uv run $reconKit escape-sources $reportPath | ConvertFrom-Json
if ($sourceJson.invalid_urls.Count -gt 0) { throw 'Invalid source URL in appendix.' }
$body = (ConvertFrom-Markdown -InputObject $markdown).Html
$body = $body.Replace('<p>@@SOURCE_TABLE@@</p>', $sourceJson.html).Replace('@@SOURCE_TABLE@@', $sourceJson.html)

$head = @'
<!doctype html>
<html lang="uk"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>VIDA — Iroh-only та межа сумісності з Delta Chat</title>
<style>
:root{color-scheme:light dark;--bg:#f6f4f0;--paper:#fffdf9;--fg:#242b32;--muted:#59636c;--line:#ddd8cf;--accent:#ad5f43;--warn:#8b4e2e}
@media(prefers-color-scheme:dark){:root{--bg:#181c20;--paper:#23292e;--fg:#f3f1ed;--muted:#bdc4c7;--line:#495057;--accent:#e0a184;--warn:#ecc09f}}
*{box-sizing:border-box}body{margin:0;background:var(--bg);color:var(--fg);font:16px/1.62 system-ui,-apple-system,Segoe UI,sans-serif}a{color:var(--accent)}
.mast{background:#262b30;color:#f9f6f0;padding:2rem max(1rem,calc((100vw - 1080px)/2)) 1.7rem}.mast h1{font-size:clamp(1.6rem,3vw,2.5rem);line-height:1.2;margin:.4rem 0}.mast p{margin:.25rem 0;color:#d6d3cd}.tag{display:inline-block;border:1px solid #96745f;border-radius:99px;padding:.2rem .65rem;margin-right:.4rem;font-size:.8rem}
.layout{display:grid;grid-template-columns:210px minmax(0,1fr);gap:1.5rem;max-width:1120px;margin:auto;padding:1.5rem 1rem}.toc{align-self:start;position:sticky;top:1rem;font-size:.86rem;max-height:calc(100vh - 2rem);overflow:auto}.toc a{display:block;padding:.3rem 0;text-decoration:none;color:var(--muted)}.toc a:hover{color:var(--accent)}main{min-width:0;background:var(--paper);border:1px solid var(--line);border-radius:14px;padding:1.5rem 2rem;box-shadow:0 8px 24px #0000000b}main>h1:first-child{display:none}h2{border-top:1px solid var(--line);padding-top:1.4rem;margin-top:2rem}h3{margin-top:1.5rem}h2:first-of-type{border-top:0;margin-top:.6rem;padding-top:0}p,li{max-width:80ch}table{width:100%;border-collapse:collapse;display:block;overflow:auto;font-size:.88rem}td,th{text-align:left;vertical-align:top;padding:.55rem .7rem;border-bottom:1px solid var(--line)}th{background:color-mix(in srgb,var(--accent) 12%,var(--paper))}blockquote{border-left:3px solid var(--accent);padding-left:1rem;color:var(--muted)}code{overflow-wrap:anywhere}strong{color:var(--fg)}.evidence{font-size:.85rem;color:var(--muted);margin:.5rem 0 1.3rem}.badge{font-weight:700;border-radius:99px;padding:.08rem .5rem;border:1px solid var(--line)}.badge.medium{color:var(--warn)}.badge.pending{color:var(--warn);border-color:var(--warn)}
@media(max-width:760px){.layout{display:block}.toc{position:static;max-height:none;background:var(--paper);border:1px solid var(--line);border-radius:10px;padding:.5rem 1rem;margin-bottom:1rem}main{padding:1.2rem}}
</style></head><body>
<header class="mast"><span class="tag">BMad Deep Recon · Technical</span><span class="tag">2026-09-25</span><span class="tag">16 перевірених опорних тверджень</span><h1>VIDA: незалежний Messenger на Iroh</h1><p>VIDA створює власні протоколи повідомлень і синхронізації; Delta Chat та інші проєкти — джерела патернів, не залежності чи сумісні клієнти.</p><p>Відкритий транспортний нюанс: Web-клієнту Iroh потрібен relay, але це не Chatmail і не поштовий шлюз.</p></header>
<div class="layout"><nav class="toc" aria-label="Зміст"><strong>Зміст</strong><div id="toc-list"></div></nav><main id="report">
'@
$tail = @'
</main></div><script>
const heads=[...document.querySelectorAll('main h2')];const toc=document.getElementById('toc-list');heads.forEach((h,i)=>{if(!h.id)h.id='section-'+i;const a=document.createElement('a');a.href='#'+h.id;a.textContent=h.textContent;toc.appendChild(a)});
for(const el of document.querySelectorAll('td:last-child')){if(el.textContent.trim()==='high')el.innerHTML='<span class="badge">high</span>';if(el.textContent.trim()==='medium')el.innerHTML='<span class="badge medium">medium</span>'}
</script></body></html>
'@
[System.IO.File]::WriteAllText($outputPath, $head + $body + $tail, [System.Text.UTF8Encoding]::new($false))
Write-Output $outputPath
