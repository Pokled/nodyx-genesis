# Fidélité à la genèse

Les 10 règles sacrées des documents d'origine, vérifiées contre les 17 tranchées.
Objectif : rendre visible toute dérive entre le projet voulu et le projet décidé.

Dernière révision : 2026-08-31.

---

| # | Règle sacrée | Tenue par | Risque, et où on le surveille |
|---|---|---|---|
| 1 | Le monde continue sans le joueur | T-3 (le monde tourne sans client), T-5 (observation passive), T-17 (durée) | Aucun. Renforcée : la roadmap fait du "headless" la base, pas un jalon. |
| 2 | L'histoire n'est jamais écrite à l'avance | T-7 (émergence mécanisée ou pas livrée), T-14 (on ne trafique pas un monde pour le garder), T-16 (on ne règle pas une règle parce qu'elle a produit un monde plus plaisant) | Le "moment public" par jalon (T-11) pourrait pousser à régler le monde pour le spectacle. Surveillance : chaque moment de la roadmap est une observation d'émergence, jamais un événement scripté. Si le monde ne l'a pas produit, on le dit publiquement. |
| 3 | Les conséquences sont persistantes | invariant 4 (event log immuable), T-15 (autopsie), persistance | Aucun. |
| 4 | Les individus possèdent une mémoire | modèle mémoire à partir de 0.0.3, T-8 (ancrage) | Aucun. En 0.0.1 la mémoire n'existe pas encore, c'est voulu. |
| 5 | Les sociétés peuvent émerger | T-7, expérience 001 (levée du pari avant 0.0.5) | Le pari peut ne pas tenir. C'est justement ce qu'on teste tôt. Si l'émergence ne se mécanise pas, on repense, on ne déclare pas. |
| 6 | Le LLM ne définit pas la réalité | invariant 1, T-6 (le LLM propose du sens, le moteur possède les nombres) | Faible au niveau social. Surveillance : la validation sémantique des sorties LLM doit être définie dans `05_COGNITION.md`. |
| 7 | Le rendu n'est qu'une fenêtre sur le monde | T-3 (ViewState, le moteur ne dessine rien) | Aucun. Renforcée. |
| 8 | Nodyx est la couche numérique, pas le moteur | invariant 2, invariant 10, T-9 | CT-11 non tranché : qui valide les permissions d'une action Nodyx. À fixer dans `09_NODYX_VOILE.md`. |
| 9 | Les expériences sont reproductibles | T-4 (temps), T-5 (déterminisme strict), T-13 (suite d'expériences), contrat de déterminisme du modèle de données | Le LLM casse le déterminisme strict à partir de 0.0.5. Traité : rejeu des sorties enregistrées, pas re-simulation. Honnête, borné. |
| 10 | Le monde doit pouvoir surprendre ses créateurs | T-14, T-16, invariant "le développeur ne connaît pas la fin" | Le vrai danger du projet. T-16 est la garde principale : notre jugement est une annotation, jamais une modification. |

---

## Ce qui a été volontairement resserré par rapport aux documents d'origine

- **Le focus du joueur ne change plus la profondeur de simulation** (T-5). Les documents
  d'origine voulaient que l'observation soit "interactive". On l'a retirée pour préserver la
  reproductibilité et la crédibilité. Plus fidèle à la règle 1 et à la règle 9 que ce que le
  corpus proposait.
- **Le "god game" a été réduit** à un rôle (Gardien) au lieu d'un menu de pouvoirs
  (astéroïdes, miracles, modification d'organismes). Si cette part compte, elle mérite sa
  propre tranchée plutôt qu'une note.
- **L'arc cosmique** (espace, premier contact, entités extérieures) est repoussé en vision
  longue non datée, au profit d'une démo publique en six mois.

## Ce qui a été ajouté et qui n'était pas dans les documents d'origine

- La notion de **beaucoup de mondes** et de **bibliothèque** (T-14). Le corpus parlait d'un
  monde unique.
- L'**autopsie** traçable comme livrable (T-15).
- Le principe **notre jugement est une annotation, jamais une modification** (T-16).
- La contrainte de **longévité extrême** (T-17), qui découle de l'étoile polaire.
