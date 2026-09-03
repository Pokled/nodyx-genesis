"""
Experience 012, la predation.

Spec : BIBLE/experiments/012_predation.md

Question : une entite qui peut manger une autre entite fait-elle emerger une chaine
alimentaire, de facon mecanisee (jamais un `if` qui nomme un "predateur") ? Cree-t-elle
une pression selective nouvelle (taille, vitesse de fuite, perception) ? L'ecosysteme
tient-il ?

Montage : un ecosysteme jouet, proche des essentiels de Genesis.
  - Espace continu 1x1, un champ de ressources sur une grille qui se regenere.
  - Chaque entite : position, energie, age, et un genome heritable de 4 traits dans 0..1 :
      size        grande capacite d'energie, gros metabolisme de base, plus lente
      speed       pas de deplacement plus grand, mais plus cher
      perception  sent les ressources ET les predateurs plus loin
      aggression  propension a attaquer (trait pour voir s'il evolue, pas un tag "predateur")
  - Chimiotaxie : l'entite remonte le gradient de ressources a portee de perception.
  - Metabolisme, repas sur la case, reproduction par scission avec mutation, mort de faim
    ou de vieillesse. Population dynamique, plafonnee.

Predation (variantes) :
  V0  aucune predation : l'ecosysteme de reference.
  V1  predation opportuniste : une entite affamee, avec une entite nettement plus petite
      a portee courte, la mange (la proie meurt, le predateur gagne une fraction de son
      energie). Sequentiel dans l'ordre des indices, une prise par predateur et par tick,
      decisions collectees puis appliquees. Aucune regle ne nomme un predateur.
  V2  plus la fuite : une entite qui percoit une entite nettement plus grosse a portee
      change sa cible pour s'en eloigner (au lieu de remonter le gradient).
  V3  plus le gain : transfert trophique plus genereux et ratio de taille plus lache
      (la chasse paie mieux) : pour trouver la fenetre d'equilibre.

Prototype Python vectorise (numpy). Deterministe. A porter en Rust.
Sortie : results.html
"""

import numpy as np
import html
import pathlib

SEED = 1
CAP = 560           # plafond dur des tableaux ; l'equilibre naturel doit rester BIEN en dessous
START = 180
GRID = 22
T = 2800
WARMUP = 900

# -- ressources (calibrees pour que la population sature a ~55-70 % du plafond, avec de la
#    vraie faim : c'est la disette qui rend la predation rentable)
RES_MAX = 1.0
RES_REGEN = 0.0075        # vers RES_MAX, par tick
EAT_RATE = 0.26          # part du contenu de la case captee

# -- energie / cycle de vie
E_START = 0.60
E_REPRO = 1.00           # seuil de reproduction
E_DIV_COST = 0.06        # surcout de la division
E_CEIL = 1.60
LIFESPAN = 950
LIFESPAN_JITTER = 260
MATURITY = 0.34          # fraction de la vie avant de pouvoir se reproduire

# -- corps et deplacement (modules par les traits)
BASE_BURN = 0.0105
MOVE_STEP = 0.0060
MOVE_COST = 0.018

# -- perception
PERC_MIN = 0.030
PERC_SPAN = 0.105

# -- mutation
MUT_SIGMA = 0.045

# -- predation
PERIL = 0.42            # on chasse quand energie sous ce niveau
PREY_RATIO = 0.78      # proie.size < predateur.size * ce facteur
PREY_RATIO_V3 = 0.90
REACH = 0.013          # portee de l'attaque
TRANSFER = 0.55        # part de l'energie de la proie qui passe au predateur
TRANSFER_V3 = 0.72
AGGR_GATE = 0.15       # aggression minimale pour tenter une attaque

TRAITS = ["size", "speed", "perception", "aggression"]

VARIANTS = [
    ("V0", "aucune predation"),
    ("V1", "predation opportuniste"),
    ("V2", "plus la fuite du plus gros"),
    ("V3", "plus le gain (chasse qui paie mieux)"),
]


def cell_of(pos):
    c = np.clip((pos * GRID).astype(int), 0, GRID - 1)
    return c[:, 0], c[:, 1]


def run_variant(code):
    rng = np.random.default_rng(SEED)
    predation = code in ("V1", "V2", "V3")
    flee = code in ("V2", "V3")
    prey_ratio = PREY_RATIO_V3 if code == "V3" else PREY_RATIO
    transfer = TRANSFER_V3 if code == "V3" else TRANSFER

    alive = np.zeros(CAP, bool)
    alive[:START] = True
    pos = np.zeros((CAP, 2))
    pos[:START] = rng.random((START, 2))
    energy = np.zeros(CAP)
    energy[:START] = E_START
    age = np.zeros(CAP)
    age[:START] = rng.integers(0, LIFESPAN, START)
    death_age = np.zeros(CAP)
    death_age[:START] = LIFESPAN + rng.normal(0, LIFESPAN_JITTER, START)
    gen = np.zeros(CAP)                      # generation (profondeur de lignee)

    genome = np.zeros((CAP, 4))
    genome[:START] = rng.random((START, 4)) * 0.5 + 0.25   # traits centres, pas extremes

    res = np.full((GRID, GRID), RES_MAX * 0.6)

    hist = {k: np.zeros(T) for k in (
        "pop", "chain", "size", "speed", "perc", "aggr",
        "kills", "starv", "gen_mean", "hungry")}

    # accumulateurs post-warmup pour les correlations
    life_perc, life_len, life_by_pred = [], [], []   # perception, age a la mort, mort par predation ?

    for t in range(T):
        idx = np.where(alive)[0]
        n = idx.size
        if n == 0:
            break

        sz = genome[idx, 0]
        sp = genome[idx, 1]
        pc = genome[idx, 2]
        ag = genome[idx, 3]
        p = pos[idx]

        # -- regeneration des ressources
        res += RES_REGEN * (RES_MAX - res)

        # -- perception : rayon par entite
        perc_r = PERC_MIN + PERC_SPAN * pc

        # -- matrice de distances au carre, partagee entre fuite et predation
        need_d = predation or (flee and n > 1)
        if need_d and n > 1:
            d2all = ((p[:, None, :] - p[None]) ** 2).sum(2)
            np.fill_diagonal(d2all, 9.9)

        # -- cible de deplacement : gradient de ressources echantillonne en 4 points
        #    a distance perc_r ; ou fuite du plus gros voisin en V2/V3.
        offs = np.array([[1, 0], [-1, 0], [0, 1], [0, -1]], float)
        best_dir = np.zeros((n, 2))
        samp_val = np.full((n, 4), -1.0)
        for k in range(4):
            q = np.clip(p + offs[k] * perc_r[:, None], 0, 0.999)
            cx, cy = cell_of(q)
            samp_val[:, k] = res[cx, cy]
        kbest = samp_val.argmax(1)
        best_dir = offs[kbest]

        fleeing = np.zeros(n, bool)
        if flee and n > 1:
            bigger = sz[None, :] > sz[:, None] * 1.20
            near = d2all < (perc_r[:, None] ** 2)
            threat = bigger & near
            has = threat.any(1)
            if has.any():
                # direction moyenne opposee aux menaces
                rel = p[:, None, :] - p[None]           # de la menace vers moi
                w = threat[:, :, None].astype(float)
                away = (rel * w).sum(1)
                nrm = np.sqrt((away ** 2).sum(1)) + 1e-9
                best_dir[has] = away[has] / nrm[has, None]
                fleeing[has] = True

        step = MOVE_STEP * (0.40 + sp)
        jitter = rng.normal(0, 0.0015, (n, 2))
        newp = np.clip(p + best_dir * step[:, None] + jitter, 0, 0.999)
        moved = np.sqrt(((newp - p) ** 2).sum(1))
        pos[idx] = newp
        p = newp

        # -- metabolisme : burn de base (monte avec la taille) + cout du deplacement
        burn = BASE_BURN * (0.5 + sz) + MOVE_COST * moved * (0.3 + sp)
        energy[idx] -= burn

        # -- repas sur la case
        cx, cy = cell_of(p)
        avail = res[cx, cy]
        want = EAT_RATE * avail
        # une grosse entite mange plus par bouchee mais il faut plus pour la remplir
        gain = np.minimum(want, avail)
        # retirer du champ (cases partagees : on somme les prises via bincount)
        flat = cx * GRID + cy
        sub = np.bincount(flat, weights=gain, minlength=GRID * GRID).reshape(GRID, GRID)
        res -= sub
        np.clip(res, 0.0, RES_MAX, out=res)
        cap_e = E_CEIL * (0.7 + 0.6 * sz)
        energy[idx] = np.minimum(energy[idx] + gain, cap_e)
        field_e = float(gain.sum())
        prey_e = 0.0

        # -- predation (vectorisee). idx_killed = slots CAP tues ce tick (pour la mesure de survie).
        kills = 0
        idx_killed = np.array([], int)
        hungry_frac = float((energy[idx] < PERIL).mean())
        if predation and n > 1:
            hungry = energy[idx] < PERIL
            can_try = hungry & (ag > AGGR_GATE)
            smaller = sz[None, :] < sz[:, None] * prey_ratio
            targets = smaller & (d2all < REACH * REACH) & can_try[:, None]
            dd = np.where(targets, d2all, np.inf)
            prey_of = dd.argmin(1)
            has_prey = np.isfinite(dd[np.arange(n), prey_of])
            attackers = np.where(has_prey)[0]
            if attackers.size:
                order = np.argsort(dd[attackers, prey_of[attackers]])
                attackers = attackers[order]
                seen = set()
                A_list, B_list = [], []
                for a in attackers:
                    b = int(prey_of[a])
                    if b in seen or a in seen:
                        continue
                    seen.add(b); seen.add(a)
                    A_list.append(a); B_list.append(b)
                if A_list:
                    A_arr = np.array(A_list); B_arr = np.array(B_list)
                    gain_p = energy[idx[B_arr]] * transfer
                    energy[idx[A_arr]] = np.minimum(energy[idx[A_arr]] + gain_p, cap_e[A_arr])
                    alive[idx[B_arr]] = False
                    idx_killed = idx[B_arr]
                    prey_e = float(gain_p.sum())
                    kills = len(A_list)

        # -- vieillissement
        age[idx] += 1.0

        # -- mort (faim, age, predation) ; mesure perception vs age a la mort pour toutes.
        died_starv = (energy < 0.0) & alive
        died_age = (age >= death_age) & alive
        if t >= WARMUP:
            dd_all = np.where(died_starv | died_age)[0]
            for di in dd_all:
                life_perc.append(float(genome[di, 2])); life_len.append(float(age[di])); life_by_pred.append(0)
            for di in idx_killed:
                life_perc.append(float(genome[di, 2])); life_len.append(float(age[di])); life_by_pred.append(1)
        alive[died_starv | died_age] = False
        hist["starv"][t] = float(died_starv.sum())

        # -- reproduction (par scission)
        idx = np.where(alive)[0]
        if idx.size:
            szr = genome[idx, 0]
            cap_e_r = E_CEIL * (0.7 + 0.6 * szr)
            mature = age[idx] >= death_age[idx] * MATURITY
            ready = (energy[idx] >= E_REPRO) & mature
            ri = idx[ready]
            free = np.where(~alive)[0]
            nnew = min(ri.size, free.size, CAP - int(alive.sum()))
            if nnew > 0:
                ri = ri[:nnew]
                fr = free[:nnew]
                energy[ri] = (energy[ri] - E_DIV_COST) * 0.5
                energy[fr] = energy[ri]
                child = genome[ri] + rng.normal(0, MUT_SIGMA, (nnew, 4))
                np.clip(child, 0.0, 1.0, out=child)
                genome[fr] = child
                pos[fr] = np.clip(pos[ri] + rng.normal(0, 0.01, (nnew, 2)), 0, 0.999)
                age[fr] = 0.0
                death_age[fr] = LIFESPAN + rng.normal(0, LIFESPAN_JITTER, nnew)
                gen[fr] = gen[ri] + 1
                alive[fr] = True

        # -- mesures du tick
        idx = np.where(alive)[0]
        n = idx.size
        hist["pop"][t] = n
        hist["kills"][t] = kills
        hist["hungry"][t] = hungry_frac
        hist["chain"][t] = prey_e / (field_e + prey_e + 1e-9)
        if n:
            hist["size"][t] = float(genome[idx, 0].mean())
            hist["speed"][t] = float(genome[idx, 1].mean())
            hist["perc"][t] = float(genome[idx, 2].mean())
            hist["aggr"][t] = float(genome[idx, 3].mean())
            hist["gen_mean"][t] = float(gen[idx].mean())

    post = slice(WARMUP, T)
    pop_mean = float(hist["pop"][post].mean())
    pop_min = float(hist["pop"][post].min())
    collapsed = pop_min < START * 0.06
    chain = float(hist["chain"][post].mean())
    d_size = float(hist["size"][post].mean() - hist["size"][WARMUP])
    d_perc = float(hist["perc"][post].mean() - hist["perc"][WARMUP])
    d_aggr = float(hist["aggr"][post].mean() - hist["aggr"][WARMUP])
    d_speed = float(hist["speed"][post].mean() - hist["speed"][WARMUP])

    def corr(xs, ys):
        if len(xs) < 50:
            return 0.0
        x = np.array(xs)
        y = np.array(ys)
        if x.std() < 1e-9 or y.std() < 1e-9:
            return 0.0
        return float(np.corrcoef(x, y)[0, 1])

    corr_perc_life = corr(life_perc, life_len)
    # perception protege-t-elle de la predation ? correlation perception <-> "mort par predation"
    # (negatif = les hautes perceptions se font moins manger). Sur les morts post-warmup.
    corr_perc_pred = corr(life_perc, life_by_pred) if any(life_by_pred) else 0.0
    hungry_mean = float(hist["hungry"][post].mean())
    frac_by_pred = float(np.mean(life_by_pred)) if life_by_pred else 0.0

    return dict(
        code=code, hist=hist,
        pop_mean=pop_mean, pop_min=pop_min, collapsed=collapsed,
        chain=chain, d_size=d_size, d_perc=d_perc, d_aggr=d_aggr, d_speed=d_speed,
        corr_perc_life=corr_perc_life, corr_perc_pred=corr_perc_pred,
        hungry_mean=hungry_mean, frac_by_pred=frac_by_pred,
        kills_total=float(hist["kills"][post].sum()),
        flee=flee, predation=predation,
    )


def svg_chart(res, w=760, h=170):
    n = T
    hist = res["hist"]
    popmax = max(1.0, hist["pop"].max())

    def poly(series, color, scale=1.0, dash="", norm=None):
        pts = []
        for i in range(0, n, 5):
            x = 8 + (w - 16) * i / (n - 1)
            val = series[i] / norm if norm else series[i] * scale
            y = h - 18 - (h - 30) * float(np.clip(val, 0, 1))
            pts.append(f"{x:.1f},{y:.1f}")
        dd = f'stroke-dasharray="{dash}" ' if dash else ""
        return f'<polyline {dd}fill="none" stroke="{color}" stroke-width="1.7" points="{" ".join(pts)}"/>'

    warm = 8 + (w - 16) * WARMUP / (n - 1)
    return f'''<svg viewBox="0 0 {w} {h}" class="chart" role="img" aria-label="courbes {res['code']}">
  <rect x="8" y="4" width="{warm-8:.1f}" height="{h-22}" fill="var(--line)" opacity="0.30"/>
  <line x1="8" y1="{h-18}" x2="{w-8}" y2="{h-18}" stroke="var(--line)"/>
  {poly(hist["pop"], "var(--cool)", norm=popmax)}
  {poly(hist["chain"], "var(--div)")}
  {poly(hist["size"], "var(--warm)")}
  {poly(hist["perc"], "var(--cons)")}
  {poly(hist["aggr"], "var(--muted2)", dash="3 3")}
</svg>'''


def verdict(rs):
    d = {r["code"]: r for r in rs}
    v0, v1, v2, v3 = d["V0"], d["V1"], d["V2"], d["V3"]
    out = []

    live = [c for c in ("V1", "V2", "V3") if not d[c]["collapsed"]]
    if not live:
        out.append("Aucune variante avec predation ne tient : l'ecosysteme s'effondre "
                   "des que la chasse est rentable. Le prototype dit que la fenetre "
                   "d'equilibre est etroite ou absente aux reglages essayes ; il faut "
                   "soit un cout de chasse plus fort, soit un transfert trophique plus "
                   "faible, soit une proie qui se defend mieux.")
    else:
        best = max(live, key=lambda c: d[c]["chain"])
        b = d[best]
        out.append(f"Avec predation, une chaine alimentaire emerge sans qu'aucune regle "
                   f"ne nomme un predateur : en {best}, {b['chain']*100:.0f} % de l'energie "
                   f"ingeree en moyenne vient d'autres entites, et l'ecosysteme tient "
                   f"(population moyenne {b['pop_mean']:.0f}, plancher {b['pop_min']:.0f}).")
        press = []
        if abs(b["d_size"]) > 0.03:
            press.append(f"la taille {'monte' if b['d_size']>0 else 'baisse'} de "
                         f"{abs(b['d_size']):.2f}")
        if abs(b["d_perc"]) > 0.03:
            press.append(f"la perception {'monte' if b['d_perc']>0 else 'baisse'} de "
                         f"{abs(b['d_perc']):.2f}")
        if abs(b["d_aggr"]) > 0.03:
            press.append(f"l'aggression {'monte' if b['d_aggr']>0 else 'baisse'} de "
                         f"{abs(b['d_aggr']):.2f}")
        if press:
            out.append("Pression selective nouvelle : " + ", ".join(press) +
                       " (contre V0 ou les traits ne bougent quasi pas).")
        if d["V2"]["corr_perc_life"] > 0.08:
            out.append(f"La fuite rend la perception utile : correlation perception / "
                       f"duree de vie {d['V2']['corr_perc_life']:+.2f} en V2.")

    out.append("Decision a prendre par l'utilisateur : si une variante tient et fait "
               "emerger une chaine alimentaire avec une pression sur la taille ou la "
               "perception, la marche predation est plausible et se code sur le vrai "
               "moteur (piste A ou B du 012). Sinon, revoir les reglages avant tout "
               "engagement dans sim.rs.")
    return " ".join(out)


def main():
    import sys
    import time as _t
    rs = []
    for c, _ in VARIANTS:
        t0 = _t.time()
        r = run_variant(c)
        rs.append(r)
        print(f"  [{c}] {(_t.time()-t0):.1f}s  pop={r['pop_mean']:.0f}", file=sys.stderr, flush=True)
    rows = ""
    for r in rs:
        desc = next(x for c, x in VARIANTS if c == r["code"])
        rows += (f"<tr><td><b>{r['code']}</b><br><span class='d'>{html.escape(desc)}</span></td>"
                 f"<td>{r['pop_mean']:.0f}<br><span class='d'>min {r['pop_min']:.0f}</span></td>"
                 f"<td>{r['chain']*100:.0f} %</td>"
                 f"<td>{r['d_size']:+.2f}</td>"
                 f"<td>{r['d_perc']:+.2f}</td>"
                 f"<td>{r['d_aggr']:+.2f}</td>"
                 f"<td>{r['corr_perc_life']:+.2f}</td>"
                 f"<td>{'effondre' if r['collapsed'] else 'tient'}</td></tr>")
    charts = ""
    for r in rs:
        desc = next(x for c, x in VARIANTS if c == r["code"])
        charts += (f"<figure><figcaption><b>{r['code']}</b> {html.escape(desc)}</figcaption>"
                   f"{svg_chart(r)}</figure>")
    v = verdict(rs)

    doc = f"""<!doctype html>
<html lang="fr"><head><meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Experience 012, predation</title>
<style>
  :root {{ color-scheme:light;
    --ground:#f3f2ee; --raised:#fff; --ink:#1b1a17; --ink-soft:#4a463e; --muted:#7d7869;
    --line:#ddd8cc;
    --div:#9a3b3b; --cons:#3b6f4e; --cool:#3b4f8c; --warm:#a8712b; --muted2:#8a7fa0; }}
  @media (prefers-color-scheme:dark) {{ :root {{ color-scheme:dark;
    --ground:#14150f; --raised:#1c1d16; --ink:#e9e6da; --ink-soft:#b7b2a2; --muted:#867f6d;
    --line:#2c2d22;
    --div:#d98a8a; --cons:#8fbfa0; --cool:#8fa4dd; --warm:#d6a866; --muted2:#b3a6cc; }} }}
  * {{ box-sizing:border-box; }}
  body {{ margin:0; background:var(--ground); color:var(--ink); font:16px/1.6 Georgia, serif; }}
  .wrap {{ max-width:900px; margin:0 auto; padding:48px 24px 96px; }}
  h1 {{ font-size:1.7rem; margin:0 0 .3rem; }}
  .sub {{ color:var(--muted); font-family:ui-monospace,monospace; font-size:.78rem;
    letter-spacing:.08em; text-transform:uppercase; margin-bottom:1.8rem; }}
  p {{ max-width:66ch; color:var(--ink-soft); }}
  .verdict {{ background:var(--raised); border:1px solid var(--line); border-left:3px solid var(--cons);
    border-radius:0 8px 8px 0; padding:1.1rem 1.3rem; margin:1.6rem 0 2.4rem; }}
  .verdict p {{ margin:0; color:var(--ink); }}
  table {{ border-collapse:collapse; width:100%; font-family:ui-monospace,monospace; font-size:.78rem;
    background:var(--raised); border:1px solid var(--line); border-radius:8px; overflow:hidden; margin:1.4rem 0 2rem; }}
  th,td {{ text-align:left; padding:.5rem .6rem; border-bottom:1px solid var(--line); vertical-align:top; }}
  th {{ background:var(--ground); text-transform:uppercase; font-size:.6rem; letter-spacing:.05em; color:var(--muted); }}
  tr:last-child td {{ border-bottom:none; }}
  td .d {{ color:var(--muted); font-family:Georgia,serif; font-size:.9em; }}
  figure {{ margin:0 0 1.4rem; background:var(--raised); border:1px solid var(--line); border-radius:8px; padding:.9rem 1rem; }}
  figcaption {{ font-family:ui-monospace,monospace; font-size:.76rem; color:var(--ink-soft); margin-bottom:.4rem; }}
  .chart {{ width:100%; height:auto; display:block; }}
  .legend {{ font-family:ui-monospace,monospace; font-size:.7rem; color:var(--muted); margin:.4rem 0 1.8rem; line-height:1.9; }}
</style></head><body><div class="wrap">
<h1>Experience 012, la predation</h1>
<div class="sub">graine {SEED} &nbsp; {START} entites au depart (plafond {CAP}) &nbsp; grille {GRID} &nbsp; {T} ticks</div>

<p>Un ecosysteme jouet : des entites qui remontent le gradient d'un champ de ressources,
mangent, se divisent avec mutation, meurent de faim ou de vieillesse. Quatre traits
heritables : taille, vitesse, perception, aggression. En V1 et au-dela, une entite affamee
avec une entite nettement plus petite a portee courte la mange (la proie meurt, le
predateur gagne une part de son energie). Aucune regle ne nomme un predateur : c'est une
condition d'energie, de taille et de distance. En V2, une entite qui percoit plus gros
qu'elle fuit. En V3, la chasse paie mieux.</p>

<div class="verdict"><p>{html.escape(v)}</p></div>

<table>
<tr><th>Variante</th><th>Population</th><th>Chaine alimentaire</th><th>&Delta; taille</th><th>&Delta; perception</th><th>&Delta; aggression</th><th>perc / vie</th><th>Ecosysteme</th></tr>
{rows}
</table>

<div class="legend">
<span style="color:var(--cool)">bleu</span> population (normalisee au pic) &nbsp;
<span style="color:var(--div)">rouge</span> part de l'energie ingeree qui vient d'une proie &nbsp;
<span style="color:var(--warm)">ocre</span> taille moyenne &nbsp;
<span style="color:var(--cons)">vert</span> perception moyenne &nbsp;
<span style="color:var(--muted2)">violet pointille</span> aggression moyenne<br>
zone grise : rodage (warmup), non compte dans les moyennes
</div>

{charts}

<p style="margin-top:2.2rem; color:var(--muted); font-family:ui-monospace,monospace; font-size:.72rem">
Prototype Python. Spec : BIBLE/experiments/012_predation.md. A porter en Rust si la marche est retenue.
</p>
</div></body></html>"""

    out = pathlib.Path(__file__).with_name("results.html")
    out.write_text(doc, encoding="utf-8")
    print("Ecrit :", out)
    for r in rs:
        print(f"  {r['code']}  pop={r['pop_mean']:.0f}(min {r['pop_min']:.0f})  faim={r['hungry_mean']*100:.0f}%  "
              f"chaine={r['chain']*100:.0f}%  morts_pred={r['frac_by_pred']*100:.0f}%  "
              f"dSize={r['d_size']:+.2f} dPerc={r['d_perc']:+.2f} dAggr={r['d_aggr']:+.2f}  "
              f"perc/pred={r['corr_perc_pred']:+.2f}  {'EFFONDRE' if r['collapsed'] else 'tient'}")
    print()
    print(verdict(rs))


if __name__ == "__main__":
    main()
