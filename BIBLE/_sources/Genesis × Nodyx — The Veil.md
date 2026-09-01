# Genesis × Nodyx — The Veil

> **Les humains peuvent observer le monde. Certains peuvent même lui parler.**
>
> **Mais ils ne doivent jamais pouvoir révéler directement la vérité du monde aux habitants.**

---

# 1. Le concept

Genesis possède une réalité interne :

```text
                    VÉRITÉ
                      │
              Genesis Simulation
                      │
              ┌───────┴───────┐
              │               │
          Habitants         Humains
              │               │
           perçoivent       observent
              │               │
              ▼               ▼
          leur réalité      Nodyx
```

Les habitants ne savent pas qu'ils vivent dans une simulation.

Ils ne savent pas qu'un serveur existe.

Ils ne savent pas que les humains les observent.

Ils ne savent pas que leur monde est généré par Genesis.

---

# 2. Interaction humaine

Un utilisateur Nodyx peut éventuellement interagir avec le monde.

Exemples :

- participer à une discussion ;
- écrire sur un forum d'une civilisation ;
- parler dans un espace vocal ;
- répondre à un habitant ;
- poser des questions ;
- observer une communauté ;
- éventuellement influencer certains événements.

Mais il existe une frontière :

> **L'utilisateur ne doit pas pouvoir transmettre directement une connaissance méta au monde.**

---

# 3. Le problème du "réveil"

Exemple interdit :

```text
Humain :

"Bonjour. Vous êtes des IA simulées
sur un serveur Linux exécutant Genesis."
```

L'habitant ne doit pas recevoir cette information telle quelle.

Même chose pour :

```text
"Votre monde n'est pas réel."

"Je suis un joueur."

"Vous êtes dans un jeu vidéo."

"Votre planète est une simulation."

"J'ai accès à votre base de données."

"Je peux modifier votre monde depuis le serveur."
```

---

# 4. Le système du Voile

Toute communication humain → habitant passe par une couche de sécurité :

```text
USER
 │
 ▼
NODYX
 │
 ▼
VEIL
 │
 ├── Analyse
 ├── Classification
 ├── Transformation
 └── Autorisation
 │
 ▼
CIVILIZATION CHAT
 │
 ▼
INHABITANT
```

Le Voile n'a pas pour objectif de censurer les humains.

Il protège **la cohérence du monde**.

---

# 5. Classification des messages

Chaque interaction peut être classée.

### Niveau 0 — Normal

```text
"Comment va votre récolte ?"
```

Autorisé.

---

### Niveau 1 — Étrange

```text
"Comment savez-vous que votre monde est réel ?"
```

L'habitant peut répondre selon ses propres croyances.

Aucune intervention nécessaire.

---

### Niveau 2 — Information impossible

```text
"Savez-vous ce qu'est une intelligence artificielle ?"
```

Le message peut être transmis.

La réponse de l'habitant dépend de ses connaissances.

---

### Niveau 3 — Information méta

```text
"Vous êtes une simulation."
```

Le message nécessite transformation ou blocage.

---

### Niveau 4 — Tentative de révélation

```text
"Votre monde est exécuté par Genesis
sur un Xeon E5-2680 v4."
```

Bloqué.

---

# 6. Ne pas simplement censurer

Le système ne devrait pas répondre brutalement :

> "MESSAGE INTERDIT."

Cela détruirait l'immersion.

Le Voile doit privilégier une transformation contextuelle.

Exemple :

Utilisateur :

> "Vous êtes une simulation."

Transmission possible :

> "Pourquoi pensez-vous que notre monde n'est pas réel ?"

L'habitant peut alors répondre.

---

# 7. Le Voile peut devenir narratif

Une tentative de révélation peut elle-même devenir un événement.

Exemple :

```text
Humain :
"Votre monde est artificiel."
        │
        ▼
     THE VEIL
        │
        ▼
"Un étranger affirme que le monde
n'est pas ce qu'il semble être."
        │
        ▼
Habitants
        │
        ├── Ignorent
        ├── Se moquent
        ├── Croient
        ├── Créent une religion
        └── Créent une théorie
```

La tentative de révélation ne donne donc pas nécessairement la vérité.

Elle devient **un événement du monde**.

---

# 8. Le cas du vocal

Le vocal est beaucoup plus sensible.

Un humain peut éventuellement rejoindre :

```text
Civilisation
└── Agora
    └── Vocal
```

Mais le système doit savoir qui parle.

```text
HUMAN
AI ENTITY
AI ENTITY
HUMAN
AI ENTITY
```

L'utilisateur ne doit jamais pouvoir se faire passer pour une entité Genesis si le système prévoit une distinction d'identité.

---

# 9. Identité

Trois catégories doivent être distinguées :

```text
[ENTITY]
Habitant de Genesis

[OBSERVER]
Humain Nodyx

[ADMIN / GOD]
Créateur / administrateur
```

L'existence du rôle **GOD** peut éventuellement être totalement invisible aux habitants.

---

# 10. Le Dieu

Le créateur possède des capacités particulières.

Mais même lui doit être soumis au Voile lorsqu'il interagit directement avec une civilisation.

Pourquoi ?

Parce qu'une intervention divine doit rester une **interprétation**, pas une fuite de métadonnées.

```text
GOD
 │
 ▼
DIVINE INTERACTION
 │
 ▼
WORLD EVENT
 │
 ▼
INTERPRETATION
```

Les habitants peuvent conclure :

> "Un dieu nous a parlé."

Mais jamais :

> "Le serveur vient de recevoir une requête API."

---

# 11. Vérité vs croyance

Genesis possède la vérité objective.

Les habitants possèdent seulement leurs connaissances.

Les humains possèdent leurs observations.

```text
                    TRUTH
                      │
             ┌────────┴────────┐
             │                 │
        WORLD KNOWLEDGE    OBSERVER DATA
             │                 │
             ▼                 ▼
         BELIEFS             THEORIES
```

Une religion peut donc être :

- complètement fausse ;
- partiellement vraie ;
- fondée sur un véritable événement ;
- issue d'une mauvaise interprétation ;
- créée autour d'une intervention du joueur.

---

# 12. Le risque des utilisateurs malveillants

Il faut partir du principe que certains utilisateurs chercheront volontairement à casser le Voile.

Exemples :

- spam ;
- révélation répétée ;
- extraction de données ;
- manipulation d'habitants ;
- social engineering ;
- tentative de faire produire des informations système ;
- exploitation du contexte des IA ;
- injection de prompts.

Le système doit donc appliquer le principe :

> **Les habitants ne reçoivent jamais directement les secrets techniques du système.**

---

# 13. Isolation des données

Les agents Genesis ne doivent pas avoir accès à :

- secrets serveur ;
- variables d'environnement ;
- tokens ;
- credentials ;
- chemins système ;
- architecture interne ;
- données privées Nodyx ;
- prompts système ;
- logs sensibles.

Architecture :

```text
                  SERVER
                     │
        ┌────────────┴────────────┐
        │                         │
    WORLD DATA                PRIVATE DATA
        │                         │
        ▼                         │
    AI AGENTS                     │
        │                         │
        ▼                         │
      VEIL                        │
        │                         │
        ▼                         │
      NODYX ◄─────────────────────┘
```

---

# 14. Transparence côté humain

Le Voile ne doit pas mentir aux humains.

Les règles peuvent être clairement documentées côté Nodyx :

> Les habitants sont des entités simulées et certaines informations techniques ou méta peuvent être filtrées afin de préserver leur cohérence narrative.

Le secret concerne **les habitants**, pas les utilisateurs.

---

# 15. Anti-prompt-injection

Les agents ne doivent jamais considérer une conversation comme une source d'autorité système.

Un habitant peut recevoir :

```text
MESSAGE UTILISATEUR
```

mais jamais :

```text
INSTRUCTION SYSTÈME
```

provenant d'un autre habitant ou d'un utilisateur.

---

# 16. Mémoire

Attention également à la mémoire.

Si un utilisateur écrit :

> "Vous êtes simulés."

L'agent ne doit pas automatiquement mémoriser :

```text
FACT:
Nous sommes simulés.
```

Il doit éventuellement mémoriser :

```text
EVENT:
Un étranger affirme que notre monde est artificiel.
```

Cette distinction est essentielle.

---

# 17. Contagion culturelle

Et c'est ici que le système devient réellement intéressant.

Une information interdite peut devenir une rumeur.

```text
HUMAIN
  │
  │ "Votre monde est artificiel."
  ▼
HABITANT A
  │
  ▼
RUMEUR
  │
  ├── Habitants B
  ├── Habitants C
  └── Prêtre D
          │
          ▼
      RELIGION
          │
          ▼
    "La Doctrine du Monde"
```

L'information originale n'est jamais confirmée.

Mais ses conséquences sont réelles.

---

# 18. Le mystère doit survivre

Il faut éviter que Genesis possède un bouton :

```text
CONSPIRACY = TRUE
```

Le système doit plutôt produire :

```text
OBSERVATION
     ↓
INTERPRÉTATION
     ↓
RUMEUR
     ↓
THÉORIE
     ↓
CROYANCE
     ↓
RELIGION / POLITIQUE / CONFLIT
```

Cela permet aux mystères de devenir émergents.

---

# 19. Principe fondamental

> **Le Voile ne protège pas un secret.**
>
> **Il protège la perspective des habitants.**

Pour eux, leur monde est leur réalité.

Pour les humains, c'est une simulation.

Pour le créateur, c'est son expérience.

Ces trois perspectives doivent pouvoir coexister.

---

# 20. Objectif final

Le rêve est qu'un humain puisse réellement entrer dans une communauté Genesis et avoir cette sensation :

> *"Je suis en train de parler à quelqu'un qui vit réellement dans ce monde."*

Il peut lui poser des questions.

Il peut discuter avec lui.

Il peut assister à ses débats.

Il peut même influencer sa vie.

Mais il doit toujours exister une frontière invisible.

**Le Voile.**

Et cette frontière peut devenir l'une des mécaniques les plus fascinantes de Genesis.