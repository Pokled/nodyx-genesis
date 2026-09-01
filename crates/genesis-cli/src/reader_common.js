// Commun aux lecteurs (lives.html, series.html). Injecte a la place de __COMMON_JS__.
"use strict";

function css(v) {
  return getComputedStyle(document.documentElement).getPropertyValue(v).trim();
}

// -- Tableau triable : clic sur un <th>, tri des lignes du <tbody>, bascule asc/desc. --
function sortableTable(table) {
  const heads = [...table.tHead.rows[0].cells];
  heads.forEach((th, ci) => {
    th.style.cursor = 'pointer';
    th.title = 'trier';
    th.addEventListener('click', () => {
      const body = table.tBodies[0];
      const rows = [...body.rows];
      const asc = !(th.dataset.dir === 'asc');
      heads.forEach(h => { h.classList.remove('sort-asc', 'sort-desc'); delete h.dataset.dir; });
      th.dataset.dir = asc ? 'asc' : 'desc';
      th.classList.add(asc ? 'sort-asc' : 'sort-desc');
      const val = tr => {
        const raw = tr.cells[ci].dataset.v ?? tr.cells[ci].textContent;
        const n = parseFloat(String(raw).replace(/[^0-9.eE+-]/g, ''));
        return { n, s: String(raw).trim().toLowerCase() };
      };
      rows.sort((a, b) => {
        const x = val(a), y = val(b);
        const bothNum = !isNaN(x.n) && !isNaN(y.n) && x.s.replace(/[\d.,\s%]/g, '') === '';
        const cmp = bothNum ? x.n - y.n : x.s.localeCompare(y.s, 'fr');
        return asc ? cmp : -cmp;
      });
      rows.forEach(r => body.appendChild(r));
    });
  });
}

// -- Petite pastille SVG pour une legende. `kind` : cross | diamond | dot | ring | line. --
function swatchSVG(kind, color) {
  const c = color;
  let inner;
  if (kind === 'cross') {
    inner = `<line x1="3" y1="3" x2="13" y2="13" stroke="${c}" stroke-width="2.2"/><line x1="3" y1="13" x2="13" y2="3" stroke="${c}" stroke-width="2.2"/>`;
  } else if (kind === 'diamond') {
    inner = `<path d="M8,2 L14,8 L8,14 L2,8 Z" fill="${c}" fill-opacity="0.5" stroke="${c}" stroke-width="1.6"/>`;
  } else if (kind === 'dot') {
    inner = `<circle cx="8" cy="8" r="5" fill="${c}" fill-opacity="0.8"/>`;
  } else if (kind === 'ring') {
    inner = `<circle cx="8" cy="8" r="5" fill="none" stroke="${c}" stroke-width="2"/>`;
  } else {
    inner = `<line x1="1" y1="8" x2="15" y2="8" stroke="${c}" stroke-width="2.4"/>`;
  }
  return `<svg viewBox="0 0 16 16" width="16" height="16" aria-hidden="true" style="flex:none;vertical-align:middle">${inner}</svg>`;
}

// -- Rend une legende horizontale a partir d'items {kind, color, label}. --
function renderLegend(el, items) {
  el.innerHTML = items.map(it =>
    `<span class="lg-item">${swatchSVG(it.kind, it.color)}<span>${it.label}</span></span>`
  ).join('');
}
