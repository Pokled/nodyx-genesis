# 03. Expérience 001, l'émergence d'une croyance partagée

Statut : brouillon de travail. Modèle pour toutes les expériences futures (tranchée 13).
Décision : on prototype ça maintenant, isolé, avant de s'engager sur 0.0.5 et 0.0.6.

Le pari du projet est "on ne déclare pas l'histoire". Toute l'émergence sociale (mèmes,
consensus, institutions, religion) doit naître d'une règle qui consomme des événements,
de la mémoire et du contact, jamais d'un `if age > X`. Si ce pari ne tient pas, il faut le
savoir avant d'avoir bâti trois versions dessus.

Cette expérience n'a pas besoin du moteur complet. C'est un modèle autonome.

---

## Question

Une population d'agents simples, qui vivent des faits sans les voir directement et qui se
parlent un peu, produit-elle :

1. une croyance qui **émerge** (personne ne l'a écrite),
2. qui **diverge** du fait,
3. qui **se détache** du fait (elle persiste après que le fait a changé),
4. et un noyau qui ressemble à une **institution** (un sous-groupe stable qui partage la
   croyance étroitement, résiste à l'expérience, et influence plus qu'il n'est influencé),

sans aucune règle qui dise "former un mythe" ou "créer une institution".

## Montage

- Population fixe de N agents sur une petite grille. On réutilise l'entité 0.0.1 plus un
  emplacement `belief`.
- Le monde a des régions. Chaque région a un fait objectif : `resource_is_safe: bool`.
  Ce fait bascule de temps en temps, de façon déterministe, selon un calendrier que les
  agents ne voient pas.
- Les agents perçoivent des résultats (mangé ici, gagné ou perdu de l'énergie), pas le fait.
- Chaque agent tient une croyance par région : `p_safe: f32` dans 0..1, avec une
  `confidence`.
- Mise à jour de la croyance : depuis sa propre expérience (poids fort), et depuis ce que
  disent les agents proches (poids faible). "Dire", c'est diffuser sa croyance aux voisins
  à chaque tick.
- Un `mème` est une paire (région, affirmation). On suit : combien d'agents le portent, à
  quelle force, et s'il correspond au fait.

## Mesures, tracées dans le temps

```
divergence(t)   = moyenne de |belief.p_safe - fait_reel| sur agents et regions
consensus(t)    = fraction d'agents dont la croyance est a moins d'epsilon du mode de la population
detachement     = est-ce que consensus reste haut apres que le fait a bascule (un mythe persiste)
institution     = existe-t-il un sous-groupe stable qui :
                    a) partage une croyance de facon serree
                    b) resiste a la mise a jour par l'experience
                    c) a une asymetrie d'influence positive (influence plus qu'il n'est influence)
```

## Variantes, en A/B à la même graine (tranchée 16)

| Variante | Ce qu'on ajoute | Attendu |
|---|---|---|
| V0 | rien, croyance depuis l'expérience seule | la divergence suit le fait, pas de mythe |
| V1 | transmission sociale | consensus plus rapide, un retard après le basculement du fait |
| V2 | poids de répétition (entendre souvent la même affirmation la renforce) | des mythes qui survivent au fait |
| V3 | effet fondateur (les premiers adoptants ont un petit bonus d'influence) | des noyaux qui ressemblent à des institutions |

## Critère de réussite

V2 ou V3 produit du détachement et un noyau institution, **sans** aucune règle qui nomme le
mythe ou l'institution. Si ni V2 ni V3 n'y arrivent, le pari de l'émergence doit être
repensé avant 0.0.5, et c'est exactement ce qu'on voulait savoir tôt.

## Livrable

Un run reproductible, identifié par sa graine. Les quatre mesures tracées dans le temps
pour chaque variante. Un paragraphe de verdict. Le tout dans
`experiments/001-emergence-croyance/`.

C'est le gabarit : une question, une graine, des mesures, un verdict, un rendu visuel.
Toutes les expériences suivantes suivent cette forme.


---

## Resultat, premier passage (prototype Python, 2026-08-31)

`run.py` dans `experiments/001-emergence/`, six iterations de reglage, `results.html` pour
les courbes.

**Etabli proprement, sans aucune regle qui le nomme :**
- La transmission locale entre agents produit du consensus.
- Une asymetrie d'influence apparait : environ 15 % des agents pesent bien plus que les
  autres et tiennent une position serree. Un embryon de "noyau" social, non declare.

**Non tranche par le prototype :**
- Le basculement d'un consensus neutre vers un mythe engage (symetrie non brisee dans ce
  jouet, retour a la moyenne trop fort).
- La derive de la memoire d'un evenement reel sur plusieurs generations (brassage entre
  groupes trop faible).
- La defense institutionnelle contre la correction.

**Decision.** Le pari de l'emergence n'est ni leve ni casse, le mecanisme est plausible.
Continuer a regler le jouet numpy reviendrait a le pousser jusqu'a ce qu'il dise ce qu'on
veut (tranchee 16). Le test complet se fait sur le vrai moteur a partir de 0.0.3, avec de
vrais agents, une vraie memoire, un vrai graphe social. Ce n'est pas un bloqueur pour
0.0.1 ni 0.0.2.

**A refaire sur le vrai moteur (experience 00X, 0.0.3+) :** memoire ancree qui se degrade,
transmission avec perte, renouvellement des generations, une affirmation sans verite a
comparer, et mesurer si un mythe engage et un groupe qui le defend emergent.
