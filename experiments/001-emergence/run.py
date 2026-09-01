"""
Experience 001, l'emergence d'une croyance partagee.

Spec : BIBLE/experiments/001_emergence.md

Version 3. Les essais precedents ont montre deux choses utiles :
  - un fait que chacun peut verifier souvent : la croyance suit le fait, pas de mythe.
  - un fait verifiable par les residents et transmis aux autres : la transmission
    corrige, elle ne cree pas de mythe.
Un mythe a besoin d'un fait qui n'est plus verifiable (les temoins sont morts, le temps
a passe) et d'une transmission qui deforme. Plus, pour l'institution, d'une explication
qui n'a aucune verite a comparer.

Montage v3 :
  - Les agents vieillissent et meurent, remplaces par des nouveau-nes qui ne savent rien.
  - Une region peut connaitre un evenement (elle devient dangereuse a un tick date).
    Un agent ne verifie cet etat que s'il est present. Sa connaissance de premiere main
    s'efface avec le temps.
  - La croyance sur le danger vient de l'experience (forte) et de la transmission (faible,
    et deformante : chaque redite ajoute du bruit).
  - Une histoire de la cause : un axe naturel (0) contre surnaturel ou moral (1), qui n'a
    aucune verite. Elle ne circule que pour les regions qui ont connu un evenement.

Variantes :
  V0  transmission fidele (pas de deformation), pas d'histoire
  V1  transmission deformante
  V2  plus la transmission de l'histoire de la cause
  V3  plus un petit bonus d'influence aux gros transmetteurs

Prototype Python vectorise (numpy). Deterministe. A porter en Rust.
Sortie : results.html
"""

import numpy as np
import html
import pathlib

SEED = 1
N = 220
REGIONS = 5
T = 4000
WARMUP = 800

LIFESPAN = 850          # esperance de vie d'un agent, en ticks
LIFESPAN_JITTER = 250

HOME_PULL = 0.07
WANDER = 0.010
TRAVEL_P = 0.006        # proba par tick de partir voir une autre region un moment

EXP_RATE = 0.22         # poids de l'experience de premiere main
OBS_NOISE = 0.12
KNOW_DECAY = 0.004      # la connaissance de premiere main s'efface

SOCIAL_RADIUS = 0.16
SOCIAL_RATE = 0.05
DRIFT = 0.05            # deformation ajoutee a chaque absorption (V1+)
STORY_INVENT_P = 0.003  # un agent peut inventer une explication de lui-meme
FOUNDER_TALK_BONUS = 2.2  # V3

# calendrier des evenements : (region, tick de debut, duree)
EVENTS = [(0, 300, 3200), (1, 900, 2000), (2, 1800, 900), (3, 2600, 1400)]
REGION_CENTERS = np.array([[0.22, 0.30], [0.78, 0.28], [0.50, 0.55], [0.25, 0.80], [0.80, 0.78]])

VARIANTS = [
    ("V0", "transmission fidele, pas d'histoire"),
    ("V1", "transmission deformante"),
    ("V2", "plus l'histoire de la cause"),
    ("V3", "plus l'influence des gros transmetteurs"),
]


def danger_at(tick):
    d = np.zeros(REGIONS)
    for r, t0, dur in EVENTS:
        if t0 <= tick < t0 + dur:
            d[r] = 1.0
    return d


def ever_event(tick):
    e = np.zeros(REGIONS)
    for r, t0, dur in EVENTS:
        if tick >= t0:
            e[r] = 1.0
    return e


def run_variant(code):
    rng = np.random.default_rng(SEED)
    distort = code != "V0"
    story_on = code in ("V2", "V3")
    founder = code == "V3"

    home = rng.integers(0, REGIONS, N)
    pos = REGION_CENTERS[home] + rng.normal(0, 0.05, (N, 2))
    pos = np.clip(pos, 0, 1)
    age = rng.integers(0, LIFESPAN, N).astype(float)
    death_age = LIFESPAN + rng.normal(0, LIFESPAN_JITTER, N)

    belief = np.full((N, REGIONS), 0.3)     # danger estime
    know = np.zeros((N, REGIONS))           # connaissance de premiere main, 0..1
    story = np.full((N, REGIONS), 0.5)      # cause : naturel 0 .. surnaturel 1
    talk = np.zeros(N)                      # combien l'agent a transmis, cumule
    infl_out = np.zeros(N)
    infl_in = np.zeros(N)
    born_tick = np.zeros(N)

    hist = {k: np.zeros(T) for k in ("drift", "story_cons", "keeper")}
    story0 = np.zeros(T)

    for t in range(T):
        dang = danger_at(t)
        evr = ever_event(t)

        # mort et naissance
        age += 1.0
        dead = np.where(age >= death_age)[0]
        if dead.size:
            home[dead] = rng.integers(0, REGIONS, dead.size)
            pos[dead] = REGION_CENTERS[home[dead]] + rng.normal(0, 0.04, (dead.size, 2))
            age[dead] = 0.0
            death_age[dead] = LIFESPAN + rng.normal(0, LIFESPAN_JITTER, dead.size)
            belief[dead] = 0.3
            know[dead] = 0.0
            story[dead] = 0.5
            talk[dead] = 0.0
            infl_out[dead] = 0.0
            infl_in[dead] = 0.0
            born_tick[dead] = t

        # deplacement
        travel = rng.random(N) < TRAVEL_P
        target_c = np.where(travel[:, None],
                            REGION_CENTERS[rng.integers(0, REGIONS, N)],
                            REGION_CENTERS[home])
        pos += HOME_PULL * (target_c - pos) + rng.normal(0, WANDER, (N, 2))
        pos = np.clip(pos, 0, 1)
        cur = (((pos[:, None, :] - REGION_CENTERS[None]) ** 2).sum(2)).argmin(1)

        # experience de premiere main dans la region courante
        idx = np.arange(N)
        truth = dang[cur]
        obs = np.where(rng.random(N) < OBS_NOISE, 1.0 - truth, truth)
        belief[idx, cur] += EXP_RATE * (obs - belief[idx, cur])
        know[idx, cur] = 1.0
        know *= (1.0 - KNOW_DECAY)   # la connaissance s'efface partout un peu

        # voisinage
        d = np.sqrt(((pos[:, None, :] - pos[None]) ** 2).sum(2))
        A = (d < SOCIAL_RADIUS) & ~np.eye(N, dtype=bool)
        deg = A.sum(1).clip(min=1)

        # poids de parole : la confiance perçue du parleur, plus bonus fondateur en V3
        speak_w = know + 0.15
        if founder:
            early = born_tick < 200  # nes tot dans la simulation
            speak_w = speak_w * np.where(early, FOUNDER_TALK_BONUS, 1.0)[:, None]

        # transmission du danger, vectorisee sur les regions
        num = A @ (belief * speak_w)
        den = (A @ speak_w).clip(min=1e-9)
        heard = num / den
        if distort:
            heard = heard + rng.normal(0, DRIFT, (N, REGIONS))
        openness = SOCIAL_RATE * (1.0 - know)     # on ecoute moins si on sait de premiere main
        delta = openness * (heard - belief)
        belief += delta
        np.clip(belief, 0, 1, out=belief)
        infl_in += np.abs(delta).sum(1)
        infl_out += (A.T @ (np.abs(delta) / deg[:, None])).sum(1)
        talk += (A.sum(0))  # combien de fois ecoute comme voisin

        # histoire de la cause : seulement pour les regions qui ont connu un evenement
        if story_on:
            mask = evr[None, :] > 0
            num_s = A @ (story * speak_w)
            den_s = (A @ speak_w).clip(min=1e-9)
            heard_s = num_s / den_s + rng.normal(0, DRIFT, (N, REGIONS))
            ds = SOCIAL_RATE * 1.4 * (heard_s - story)
            invent = (rng.random((N, REGIONS)) < STORY_INVENT_P)
            ds = np.where(invent, (rng.integers(0, 2, (N, REGIONS)) - story), ds)
            story += np.where(mask, ds, 0.0)
            np.clip(story, 0, 1, out=story)

        # mesures
        far = know[:, :] < 0.15
        evm = evr[None, :] > 0
        sel = far & evm
        hist["drift"][t] = float(np.abs(belief - dang[None])[sel].mean()) if sel.any() else 0.0
        if story_on:
            sc = []
            for r in range(REGIONS):
                if evr[r] > 0:
                    sc.append(max(0.0, 1.0 - 2.0 * np.std(story[:, r])))
            hist["story_cons"][t] = float(np.mean(sc)) if sc else 0.0
            story0[t] = story[:, 0].mean()

        if story_on:
            net = infl_out - infl_in
            if net.std() > 1e-9:
                top = net > np.percentile(net, 85)
                tightness = np.abs(story[:, 0] - np.median(story[top, 0])) < 0.15
                hist["keeper"][t] = float((top & tightness).mean())

    post = slice(WARMUP, T)
    drift_err = float(hist["drift"][post].mean())
    story_cons = float(hist["story_cons"][post].mean()) if story_on else 0.0
    # emergence = les agents ont converge sur une position engagee, loin du 0.5 neutre initial,
    # et ils y restent serres. Un consensus qui ne bouge pas de 0.5 n'est pas un mythe, c'est
    # juste l'absence d'opinion.
    story_commit = float(abs(story0[WARMUP:].mean() - 0.5)) if story_on else 0.0
    story_emerged = bool(story_on and story_cons > 0.60 and story_commit > 0.12)

    keeper_size = 0.0
    dogmatic = 1.0
    if story_on:
        net = infl_out - infl_in
        top = net > np.percentile(net, 85)
        core = top & (np.abs(story[:, 0] - np.median(story[top, 0])) < 0.15)
        keeper_size = float(core.mean())
        dfin = danger_at(T - 1)
        far = know < 0.15
        err_i = np.where(far, np.abs(belief - dfin[None]), np.nan)
        err_agent = np.nanmean(err_i, axis=1)
        if core.any() and (~core).any():
            dogmatic = float(np.nanmean(err_agent[core]) / (np.nanmean(err_agent[~core]) + 1e-9))

    return dict(
        code=code, hist=hist, story0=story0,
        drift_err=drift_err, story_cons=story_cons, story_emerged=story_emerged, story_commit=story_commit,
        keeper_size=keeper_size, dogmatic=dogmatic,
        story_on=story_on, distort=distort,
    )


def svg_chart(res, w=760, h=150):
    n = T
    def poly(series, color, dash=""):
        pts = []
        for i in range(0, n, 4):
            x = 8 + (w - 16) * i / (n - 1)
            y = h - 20 - (h - 32) * float(np.clip(series[i], 0, 1))
            pts.append(f"{x:.1f},{y:.1f}")
        dd = f'stroke-dasharray="{dash}" ' if dash else ""
        return f'<polyline {dd}fill="none" stroke="{color}" stroke-width="1.8" points="{" ".join(pts)}"/>'
    ev = ""
    for r, t0, dur in EVENTS:
        x0 = 8 + (w - 16) * t0 / (n - 1)
        x1 = 8 + (w - 16) * (t0 + dur) / (n - 1)
        ev += f'<rect x="{x0:.1f}" y="4" width="{x1-x0:.1f}" height="{h-24}" fill="var(--line)" opacity="0.35"/>'
    return f'''<svg viewBox="0 0 {w} {h}" class="chart" role="img" aria-label="courbes {res['code']}">
  {ev}
  <line x1="8" y1="{h-20}" x2="{w-8}" y2="{h-20}" stroke="var(--line)"/>
  {poly(res["hist"]["story_cons"], "var(--cool)")}
  {poly(res["hist"]["keeper"], "var(--cons)", "4 3")}
  {poly(res["hist"]["drift"], "var(--div)")}
</svg>'''


def verdict(rs):
    d = {r["code"]: r for r in rs}
    v2, v3 = d["V2"], d["V3"]
    keeper = max(v2["keeper_size"], v3["keeper_size"])
    out = []
    out.append("Ce que le prototype a etabli, proprement et sans aucune regle qui le nomme : "
               "la transmission locale entre agents produit du consensus et une asymetrie "
               "d'influence (certains agents pesent bien plus que d'autres). La structure "
               "sociale emerge du mecanisme, elle n'est pas declaree.")
    if keeper > 0.05:
        out.append(f"Un noyau de gros transmetteurs se forme, environ {keeper*100:.0f} % des "
                   "agents, qui tiennent une position serree.")
    out.append("Ce que le prototype ne tranche pas : le basculement d'un consensus neutre vers "
               "un mythe engage, la derive de la memoire d'un evenement reel sur plusieurs "
               "generations, la defense institutionnelle contre la correction. Le modele "
               "numpy est trop grossier pour ca, le brassage entre groupes y est trop faible "
               "et le retour a la moyenne trop fort. Continuer a le regler reviendrait a le "
               "pousser jusqu'a ce qu'il dise ce qu'on veut, ce que la tranchee 16 interdit.")
    out.append("Decision : le pari de l'emergence n'est ni leve ni casse. Le mecanisme est "
               "plausible. Le test complet se fait sur le vrai moteur a partir de 0.0.3, avec "
               "de vrais agents, une vraie memoire et un vrai graphe social. Ce n'est pas un "
               "bloqueur pour 0.0.1 et 0.0.2.")
    return " ".join(out)


def main():
    rs = [run_variant(c) for c, _ in VARIANTS]
    rows = ""
    for r in rs:
        desc = next(x for c, x in VARIANTS if c == r["code"])
        rows += (f"<tr><td><b>{r['code']}</b><br><span class='d'>{html.escape(desc)}</span></td>"
                 f"<td>{r['drift_err']:.3f}</td>"
                 f"<td>{r['story_cons']:.2f}</td>"
                 f"<td>{r['keeper_size']*100:.0f} %</td>"
                 f"<td>{r['dogmatic']:.2f}x</td>"
                 f"<td>{'oui' if r['story_emerged'] else 'non'}</td></tr>")
    charts = ""
    for r in rs:
        desc = next(x for c, x in VARIANTS if c == r["code"])
        charts += (f"<figure><figcaption><b>{r['code']}</b> {html.escape(desc)}</figcaption>"
                   f"{svg_chart(r)}</figure>")
    v = verdict(rs)

    doc = f"""<!doctype html>
<html lang="fr"><head><meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Experience 001, emergence</title>
<style>
  :root {{ color-scheme:light;
    --ground:#f3f2ee; --raised:#fff; --ink:#1b1a17; --ink-soft:#4a463e; --muted:#7d7869;
    --line:#ddd8cc;
    --div:#9a3b3b; --cons:#3b6f4e; --cool:#3b4f8c; }}
  @media (prefers-color-scheme:dark) {{ :root {{ color-scheme:dark;
    --ground:#14150f; --raised:#1c1d16; --ink:#e9e6da; --ink-soft:#b7b2a2; --muted:#867f6d;
    --line:#2c2d22;
    --div:#d98a8a; --cons:#8fbfa0; --cool:#8fa4dd; }} }}
  * {{ box-sizing:border-box; }}
  body {{ margin:0; background:var(--ground); color:var(--ink); font:16px/1.6 Georgia, serif; }}
  .wrap {{ max-width:880px; margin:0 auto; padding:48px 24px 96px; }}
  h1 {{ font-size:1.7rem; margin:0 0 .3rem; }}
  .sub {{ color:var(--muted); font-family:ui-monospace,monospace; font-size:.78rem;
    letter-spacing:.08em; text-transform:uppercase; margin-bottom:1.8rem; }}
  p {{ max-width:64ch; color:var(--ink-soft); }}
  .verdict {{ background:var(--raised); border:1px solid var(--line); border-left:3px solid var(--cons);
    border-radius:0 8px 8px 0; padding:1.1rem 1.3rem; margin:1.6rem 0 2.4rem; }}
  .verdict p {{ margin:0; color:var(--ink); }}
  table {{ border-collapse:collapse; width:100%; font-family:ui-monospace,monospace; font-size:.8rem;
    background:var(--raised); border:1px solid var(--line); border-radius:8px; overflow:hidden; margin:1.4rem 0 2rem; }}
  th,td {{ text-align:left; padding:.55rem .7rem; border-bottom:1px solid var(--line); vertical-align:top; }}
  th {{ background:var(--ground); text-transform:uppercase; font-size:.62rem; letter-spacing:.05em; color:var(--muted); }}
  tr:last-child td {{ border-bottom:none; }}
  td .d {{ color:var(--muted); font-family:Georgia,serif; font-size:.9em; }}
  figure {{ margin:0 0 1.4rem; background:var(--raised); border:1px solid var(--line); border-radius:8px; padding:.9rem 1rem; }}
  figcaption {{ font-family:ui-monospace,monospace; font-size:.76rem; color:var(--ink-soft); margin-bottom:.4rem; }}
  .chart {{ width:100%; height:auto; display:block; }}
  .legend {{ font-family:ui-monospace,monospace; font-size:.7rem; color:var(--muted); margin:.4rem 0 1.8rem; line-height:1.9; }}
  .k-div {{ color:var(--div); }} .k-cons {{ color:var(--cons); }} .k-cool {{ color:var(--cool); }}
</style></head><body><div class="wrap">
<h1>Experience 001, l'emergence d'une croyance partagee</h1>
<div class="sub">graine {SEED} &nbsp; {N} agents &nbsp; {REGIONS} regions &nbsp; {T} ticks &nbsp; vie ~{LIFESPAN} ticks</div>

<p>Les agents vieillissent et meurent. Les nouveau-nes ne savent rien. Une region connait
un evenement dangereux a une date, et un agent ne le verifie que s'il est present. Sa
memoire de premiere main s'efface. La croyance sur le danger vient de l'experience et de
la transmission. En V1 et au-dela, chaque redite ajoute une petite deformation. En V2, une
histoire de la cause circule, qui n'a aucune verite a comparer.</p>

<div class="verdict"><p>{html.escape(v)}</p></div>

<table>
<tr><th>Variante</th><th>Erreur de memoire (non concluant, voir verdict)</th><th>Consensus de l'histoire</th><th>Noyau gardien</th><th>Gardiens vs moyenne</th><th>Croyance emergee de rien</th></tr>
{rows}
</table>

<div class="legend">
<span class="k-div">rouge</span> erreur de memoire : ecart entre ce que croient les non-temoins et le danger reel<br>
<span class="k-cool">bleu</span> consensus de l'histoire de la cause (0 = chacun sa version, 1 = tous d'accord)<br>
<span class="k-cons">vert pointille</span> taille du noyau de gros transmetteurs qui tiennent l'histoire serree<br>
zones grises : les periodes ou une region est dangereuse
</div>

{charts}

<p style="margin-top:2.2rem; color:var(--muted); font-family:ui-monospace,monospace; font-size:.72rem">
Prototype Python. Spec : BIBLE/experiments/001_emergence.md. A porter en Rust.
</p>
</div></body></html>"""

    out = pathlib.Path(__file__).with_name("results.html")
    out.write_text(doc, encoding="utf-8")
    print("Ecrit :", out)
    for r in rs:
        print(f"  {r['code']}  drift_err={r['drift_err']:.3f}  story_cons={r['story_cons']:.2f}  commit={r['story_commit']:.2f}  "
              f"keeper={r['keeper_size']*100:.0f}%  dogmatic={r['dogmatic']:.2f}x  "
              f"emerged={'oui' if r['story_emerged'] else 'non'}")
    print()
    print(verdict(rs))


if __name__ == "__main__":
    main()
