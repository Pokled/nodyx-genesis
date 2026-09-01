Bonjour à tous, aujourd’hui on va parler d’une simulation informatique incroyable,
qui semble capable de générer des créatures virtuelles à partir de presque rien. Elle
s’appelle Lenia, et on peut la voir comme une sorte d’extension du jeu de la vie.
Le jeu de la vie, vous vous en souvenez peut-être, c’est une simulation imaginée par John
Conway dans les années 1970, et à laquelle j’avais consacré toute une vidéo. Eh bien
Lenia, c’est encore plus fort et encore plus beau que le jeu de la vie.
[jingle]
Lenia, tout comme le jeu de la vie, appartient à ce domaine de recherche qu’on appelle
« La vie artificielle ». L’objectif de cette discipline, c’est de créer des
systèmes artificiels dont les comportements se rapprochent de ceux qu’on retrouve dans
les systèmes vivants. Et cela peut se faire différentes façons.
[ALIFE Soit en étudiant des réactions chimiques simples dont les interactions miment les processus
du vivant. Soit en essayant des créer des machines capables par exemple d’évoluer
et de se reproduire de façon autonome. Ou bien de façon plus accessible, en créant
des simulations informatiques qui imitent le vivant.]
Mais attention, quand je parle d’une simulation qui imite le vivant, je ne parle pas d’animer
un personnage dans un jeu vidéo. Ce qu’on cherche à faire, c’est à créer de l’émergence.
C’est-à-dire qu’on veut partir d’un système simple, avec des règles élémentaires,
mais qui, quand on le simule, produise des comportements macroscopiques inattendus.
C’est comme ça que fonctionne la vie telle qu’on la connait : c’est un phénomène
émergent. A la base du vivant, il y a « simplement » un grand nombre de réactions chimiques. Mais
dont la combinaison va faire émerger des processus complexes, allant jusqu’à l’auto-organisation
des organismes vivants.
Aujourd’hui, on va donc essayer d’explorer cette idée d’émergence, mais à partir
de règles informatiques simples, capables, quand on les simule, d’engendrer des comportements
complexes. Et pour ça, revoyons déjà comment fonctionne le jeu de la vie.
J’y avais consacré toute une vidéo que je vous invite à regarder, mais je vais quand
même rappeler les bases. Tout d’abord, le jeu de la vie, ça n’est pas vraiment
un jeu puisqu’il n’y a pas de joueur à proprement parler. C’est juste une simulation,
qui obéit à ses propres règles, et qu’on peut faire tourner sur un ordinateur.
[GRILLE Pour ça on part d’une grille, aussi grande qu’on le souhaite, et dont chaque
case représente une sorte de cellule virtuelle, qui peut être soit vivante, soit morte. Ici
je vais représenter les cellules mortes en noir, et les vivantes en jaune. Et ensuite
on va faire évoluer les cellules au tour par tour avec deux règles très simples.]
[REGLES A chaque tour, pour chaque cellule, on regarde ses 8 voisines, et on compte combien
il y en a de vivantes. La première règle, c’est qu’une cellule vivante a besoin
de 2 ou 3 voisines pour se maintenir en vie. On peut dire qu’en dessous de 2, elle meurt
d’isolation, mais au-delà de 4, elle meurt de surpopulation.
La deuxième règle, c’est que si une case vide, est entourée par exactement 3 voisines
vivantes, elle devient vivante. Et voilà c’est tout. A chaque tour, pour chaque cellule,
on compte les voisines, et on applique ces deux règles pour faire évoluer la grille.]
[FILM LIFE Pour voir ce que ça donne, on va partir d’une configuration initiale tirée
au hasard, par exemple celle-ci, et on va simuler, tour après tour. Et voici le genre
de choses qu’on obtient. Au début ça semble assez aléatoire, mais peu à peu on voit
des choses intéressantes émerger.
Déjà il y a des structures isolées qui semblent stables, comme celle qu’on voit
ici, ou là. Ensuite il y a des motifs qui oscillent entre deux états. Et puis, encore
plus intrigant, on a des structures qui semblent se déplacer en diagonale. On les surnomme
des planeurs.
Et ce qui est absolument fou avec ces planeurs, c’est qu’ils émergent des règles microscopiques.
A aucun moment dans le code on ne dit de prendre telle structure et de la déplacer sur la
grille. Il n’y a aucune notion de déplacement dans les règles du jeu de la vie. Le mouvement
apparent des planeurs « émerge » des règles d’évolution microscopiques.]
Depuis les années 70, on a trouvé avec le jeu de la vie toute une zoologie de motifs
localisés de plus en plus complexes, et ayant des propriétés intéressantes; comme le
fait d’être stable, ou de se déplacer.
Et même des motifs encore plus riches, comme ces structures capables d’engendrer des
planeurs. Il y a plein d’exemples très jolis, je ne vais pas tous vous les montrer,
je vous invite vraiment à aller voir la vidéo dédiée si vous ne l’avez pas déjà vue.
Car aujourd’hui ce qu’on veut, c’est aller plus loin !
Une chose qu’on peut déjà se demander, c’est : est-ce qu’on peut modifier les
règles du jeu de la vie, pour avoir des choses encore plus intéressantes ? Pourquoi ces
deux règles spécifiquement, qu’est-ce qui se passe si on en essaye d’autres ?
La règle usuelle, on la surnomme parfois B3/S23. B3 pour « born » avec 3 voisines,
et S pour « survives » avec 2 ou 3 voisines. Mais quid des autres règles possibles ? Par
exemples B34 et S125, ça donnerait quoi ? Eh bien ça ne produit rien d’aussi intéressant.
Il existe en principe 2^18 règles possibles, soit environ 260 000 possibilités. Mais la
plupart conduisent à des situations sans intérêt, comme l’envahissement de la grille
par des cellules vivantes, ou au contraire la destruction totale. Et très peu de règles
donnent des phénomènes de structures localisées, et rien d’aussi simple et beau que les règles
originelles du jeu de la vie.
Mais dans ce cas, est-ce qu’on ne pourrait pas essayer d’aller plus loin, de généraliser
les principes du jeu de la vie ? Eh bien c’est ce que fait Lenia !
[jingle]
Pour comprendre comment on va construire Lenia, il faut réaliser que le jeu de la vie est
ce qu’on appelle une simulation « discrète ». « Discrète », ici, c’est dans le sens
mathématique du terme, et ça signifie « qui n’est pas continu ». Et on va essayer
de s’affranchir de ça.
Déjà dans le jeu de la vie, les cellules ont un état discret. Elles sont soit vivantes,
soit mortes. Mais pas entre les deux. Eh bien pour aller plus loin, on va maintenant autoriser
n’importe quelle valeur entre 0 et 1. Ca veut dire qu’une cellule pourra valoir par
exemple 0,67 ou 0,08
[INITIALISATION Si on tire chaque valeur au hasard pour la situation de départ, on aura
quelque chose de ce genre : ici pour la représentation j’ai utilisée une échelle de couleur qui
va du noir au jaune pour représenter les valeurs entre 0 et 1. Et la valeur dans chaque
case vous pouvez la voir comme un niveau de vie de la cellule, ou une concentration d’une
certaine substance vitale.]
Bon, mais si maintenant chaque cellule peut avoir une valeur continue entre 0 et 1, comment
on peut adapter les règles d’évolution ?
[COMPTAGE Pour le comptage des voisines, on peut garder la même idée, mais faire cette
fois la somme des valeurs des 8 voisines. La seule différence c’est qu’au lieu
de récupérer un nombre entier, on va avoir un nombre réel compris entre 0 et 8, et qui
va représenter son voisinage.
Il va donc falloir adapter les règles, on ne peut plus se contenter de dire que la cellule
survit avec 2 voisins et survit ou apparait avec 3 voisins. Il faut imaginer un truc continu.
Par exemple avec une courbe de ce genre, qui donne le nouvel état en fonction de la somme
des voisines. Sur cette courbe ça veut dire par exemple qu’une cellule qui a une somme
des voisines de 2,5 recevra la valeur 0,75. Mais on peut imaginer tout un tas de formes
différentes pour cette courbe.]
Donc avoir un état continu pour les cellules, c’est jouable. En revanche, pour l’instant,
l’écoulement du temps, lui, est toujours discret : on avance au tour par tour, comme
dans le jeu de la vie. Mais on pourrait chercher aussi à le rendre continu : pour pouvoir
le faire avancer petit à petit.
[CROISSANCE Pour cela, il faut modifier un peu le sens de cette courbe qui indique quoi
faire en fonction de la somme des voisins. On va l’interpréter non pas comme le nouvel
état de la cellule au tour suivant, mais comme son taux de croissance.
Par exemple ici, avec une valeur de 2.5 en somme des voisins, on a la valeur de 0.5,
ça veut dire que la croissance sera de 0.5 par tour. Mais si on veut faire avancer le
temps de seulement 1/2 tour ou 1/4 de tour ou même 1/1000e de tour, on peut. On applique
le taux de croissance pendant la durée indiquée, et on peut traiter le temps comme s’il était
continu.
Et bien sûr, sur cette courbe, quand on a des valeurs négatives, ça indique une décroissance
de la cellule, par exemple quand il y a trop ou trop peu de voisinage.
Evidemment une petite subtilité c’est qu’avec cette idée de taux de croissance, on peut
se retrouver avec des cellules qui dépassent la valeur de 1, ou au contraire qui vont finir
en-dessous de 0. On ajoute donc juste une règle supplémentaire qui dit qu’on limite
les valeurs entre 0 et 1.]
A ce stade, les états des cellules sont continues, et le temps est continu. Eh bien la dernière
étape, c’est de rendre l’espace continu. Comment éviter cette idée de division un
peu arbitraire en cellules ?
[FILTRE Eh bien on va remplacer la règle de « compter les voisines immédiates » par
une règle du genre « compter toutes les cellules dans une certaine zone ». Par exemple
pour chaque point, on pourrait calculer la moyenne des valeurs des cellules situées
dans un certain rayon R autour. La notion de voisines immédiate disparait, et est remplacée
par une idée de proximité.
Encore mieux, on pourrait faire la moyenne dans une certaine zone en anneau comme ça.
Ca se rapprocherait de ce que fait le jeu de la vie, ou en comptant les voisines immédiates,
on regarde une sorte d’anneau autour de la cellule.
Techniquement, calculer la moyenne des voisines en pondérant de cette façon, c’est ce
qu’on appelle « appliquer un filtre ».]
Et je vous passe les détails mathématiques, mais cette idée de « compter avec un filtre »,
ça s’appelle une opération de convolution, et ça s’écrit très bien de façon parfaitement
continue, sans qu’il y ait même besoin de diviser explicitement le monde en cellules.
Evidemment pour faire le calcul, on va quand même devoir découper le monde en petites
cases, mais on pourra choisir ces cases aussi petites qu’on le souhaite, et on s’est
donc affranchis de l’idée d’un espace forcément fait de cellules discrètes de
taille fixe.
[RESUMÉ Résumons où nous en somme. Je vous ai proposé trois changements, trois manières
de généraliser le jeu de la vie : prendre des valeurs continues entre 0 et 1 dans les
cellules, sonder le voisinage en utilisant un filtre, par exemple un filtre en anneau,
et passer le résultat une fonction de croissance qui va déterminer un taux d’évolution
de la cellule.
Et ces trois idées constituent les principes de base de Lenia, et forment ce qu’on appelle
un automate cellulaire continu.]
Si ça vous intéresse, j’ai écrit un code Python pour montrer comment on fait toutes
ces adaptations, je le partagerai en description. Et cette formulation ouvre plein de nouvelles
possibilités pour essayer de trouver des règles qui engendrent des comportements émergents
intéressants. Tout ce qu’il faut faire, c’est choisir un filtre, choisir une fonction
de croissance, et simuler. Eh bien c’est parti !
[jingle]
Les principes de Lenia ont été décrits pour la première fois par le chercheur Bert
Chan en 2019. Il a notamment montré qu’en choisissant bien les règles, on pouvait retrouver
toute une zoologie de motifs, comme dans le jeu de la vie. Des motifs qui ressemblent
vraiment à des créatures virtuelles.
[LENIA Par exemple prenons un simple filtre en anneau et une fonction de croissance qui
ait une forme gaussienne. On retrouve bien l’idée qu’il y a une zone de croissance,
mais qu’avec trop ou trop peu de voisinage, il y a une décroissance due à l’isolation,
ou à la surpopulation.]
[FILM TACHE Et avec ça, regardons la configuration initiale suivante : une simple tâche. Ici
la valeur de chaque cellule est donc représentée par une couleur qui va du noir au jaune en
passant par le violet. On a des cellules qui ont des valeurs proches de 1 au centre, et
ça décroit quand on s’éloigne.
Si on lance la simulation à partir de cette situation initiale, voici ce qu’on obtient.
Il se passe quelque chose d’intéressant ! On voit que ça évolue, que des motifs
se forment et qu’ils envahissent en quelque sorte l’espace disponible, jusqu’à se
stabiliser. Ca n’est pas tout à fait comme dans le jeu de la vie, mais c’est prometteur.]
[ORBIUM Maintenant, considérez la configuration initiale suivante. Le motif que vous voyez
ici a été découvert par Bert Chan, et baptisé Orbium. C’est toujours le même code couleur
: jaune pour les valeurs élevées, violet orange pour les intermédiaires, et noir quand
c’est faible ou nul.
Si je lance la simulation avec les mêmes paramètres de filtre et de croissance qu’avant,
voici ce que j’observe. Le motif se déplace ! Et à nouveau, j’insiste pour ceux qui
trouveraient ça banal : en fait, à aucun moment dans les règles, on n’a codé une
notion de déplacement. Le déplacement émerge des règles microscopiques, comme c’était
le cas avec le planeur dans le jeu de la vie.
Ah oui et petit détail, notez au passage que depuis le début, j’utilise sans le
dire ce qu’on appelle des conditions périodiques : les cellules d’un bord sont considérées
comme en contact avec celles du bord opposé. Donc quand l’Orbium sort d’un côté,
il re-rentre de l’autre.
L’Orbium est amusant car on dirait vraiment une créature, comme une sorte de mollusque,
avec une structure interne, et notamment ce qu’on appelle en biologie une symétrie
bilatérale. Pour comparaison, je vous redessine à la même échelle aussi le filtre en anneau
qu’on utilise pour « compter les voisines ».
Et c’est intéressant d’en faire une interprétation comme une faculté sensorielle. D’une certaine
façon le filtre permet à chaque point de la créature de sentir son voisinage à une
certaine distance, et d’évoluer en fonction.]
Malgré tout, l’Orbium est un motif qui reste assez exceptionnel et fragile.
[RANDOM Si vous partez d’une configuration initiale aléatoire, vous allez toujours retrouver
les mêmes motifs, même si la façon dont ces motifs progresse est intéressante.]
Mais pour avoir des créatures encore plus riches et impressionnantes, il va falloir
un peu améliorer les règles de base.
[jingle]
Une première amélioration simple, c’est d’utiliser un filtre plus complexe, composé
par exemple de plusieurs anneaux d’intensités différentes. Cela revient d’une certaine
manière à pouvoir sentir le voisinage à différentes distances.
[HYDROGENIUM Avec cette règle, on peut voir émerger cette créature baptisée Hydrogenium.
On retrouve une symétrie bilatérale, et cette fois le déplacement n’est plus totalement
linéaire comme avec l’Orbium, ou le planeur du jeu de la vie. On voit la créature tourner
lègèrement d’un côté, puis de l’autre.]
Dans cette exemple, on a donc utilisé un filtre plus complexe, mais toujours la même
fonction de croissance. Mais on peut également choisir d’utiliser plusieurs filtres, ayant
chacun sa propre fonction, et de sommer les contributions. Cela permet des comportements
différenciés en fonction de ce qui est « senti » par les filtres aux différentes distances.
[FISH Voici un exemple d’une créature obtenue avec un système de ce genre. Elle ressemble
à un Orbium, mais on retrouve ce déplacement non-linéaire qu’on avait avec Hydrogenium.]
Mais l’amélioration la plus intéressante, ça consiste à utiliser plusieurs canaux.
Depuis le début, je vous ai dit que les valeurs qu’on représente, qui sont comprises entre
0 et 1, on peut les interpréter soit comme des intermédiaires entre vivant et mort,
soit comme par exemple la concentration d’une certaine espèce chimique.
Mais s’il pouvait y avoir plusieurs espèces chimiques présentes ? C’est-à-dire que
dans chaque case on ne stocke pas une seule valeur entre 0 et 1, mais plusieurs, disons
trois valeurs indépendantes. Ca veut dire qu’on pourrait avoir des endroits qui contiennent
beaucoup d’une substance, mais peu des deux autres, ou l’inverse.
Pour visualiser ce qu’on a quand on a 3 valeurs, c’est plus compliqué qu’avant,
mais on peut utiliser une représentation en rouge vert et bleu.
[RVB Voici un exemple : ce que vous voyez ici c’est un motif de départ où dans chaque
pixel on a stocké 3 valeurs. Une qu’on représente en rouge, l’autre en vert et
la dernière en bleu. Et la couleur de chaque pixel résulte du mélange des 3. Les zones
magenta contiennent du rouge et du bleu, en jaune c’est vert et rouge, et en blanc,
on a les 3 substances présentes en quantités proche de 1. On dit donc qu’on a maintenant
3 canaux de simulation.]
Ensuite, il faut adapter les règles pour tenir compte des ces multiples canaux. En
pratique on va permettre à chaque canal de sentir les autres avec un filtre dédié et
d’y réagir par une fonction de croissance. On imagine par exemple que le canal bleu soit
capable de sentir le canal vert dans son voisinage, et le vert sente à la fois disons le bleu
et le rouge.
En créant ainsi plusieurs filtres qui connectent les différents canaux, on peut voir émerger
des comportements très intéressants.
[AQUARIUM Par exemple ici, voici ce qu’il se passe avec ma configuration initiale. C’est
vraiment très chouette on voit notre motif se déplacer, et se séparer en deux pour
former une structure mobile plus grande et symétrique. C’est assez dingue, c’est
presque comme une sorte de division cellulaire !]
Il existe encore toute une zoologie de motifs qui ont été mis en évidence, je vous renvoie
aux vidéo de Bert Chan sur sa chaîne Youtube. Mais vous voyez déjà avec mes quelques simulations
que Lenia est un système extrêmement riche et intéressant, qui promet d’étendre beaucoup
ce qu’on arrivait à faire avec le jeu de la vie de Conway.
La principale difficulté, c’est que Lenia est tellement riche que l’espace des possibilités
est énorme. Entre les filtres, les fonctions de croissance, le nombre de canaux, et les
configurations de départ, le nombre de choix de paramètres à effectuer est considérable
avant d’espérer peut-être trouver une créature intéressante.
Au départ, la solution la plus commode consistait à essayer des trucs au hasard, et à chercher
à la main si on trouvait des motifs intéressants, prometteurs, ce qui est évidemment très
fastidieux. Mais ces dernières années, une équipe de l’INRIA à Bordeaux a eu l’idée
d’adapter des méthodes de machine learning pour cette tâche.
[jingle]
Le machine learning, c’est cette discipline qui s’intéresse à la façon dont des algorithmes
peuvent apprendre, et c’est notamment ce qui est à la base de la révolution actuelle
en intelligence artificielle, j’en ai parlé de nombreuse fois sur la chaîne.
Si vous connaissez un peu le fonctionnement des réseaux de neurones, il y a peut-être
des choses dans Lenia qui vous ont interpelé : on parle de filtres, d’une fonction de
croissance qui vient en quelque sorte activer le signal après filtration.
Ca ressemble pas mal au vocabulaire des réseaux de neurones, et notamment de ce qu’on appelle
les réseaux récurrents, dans lesquels ce qui sort du réseau lui est réinjecté en
entrée.
Lenia dans sa forme originelle n’a pas exactement la bonne structure, mais il a été possible
d’adapter sa paramétrisation pour la rendre plus similaire à certaines architectures
de réseaux de neurones. Et cela a ouvert la porte à l’utilisation de tout un arsenal
de techniques permettant d’explorer efficacement l’espace des possibilités.
Ainsi l’équipe Flowers de l’INRIA a utilisé des méthodes qu’ils avaient initialement
mises au point pour l’entrainement de réseaux de neurones. Des méthodes s’inspirant de
notions de psychologie du développement et de neurosciences, comme la curiosité et la
motivation intrinsèque.
Il faudrait vraiment que je consacre une vidéo à ces méthodes, mais il faut savoir qu’elles
se prêtent très bien à la découverte de nouvelles structures intéressantes dans l’énorme
espace des paramètres de Lenia. Et c’est ainsi qu’ont été découverts d’autres
motifs encore plus intrigants.
[FLOWERS Par exemple celui-ci, qui arrive à naviguer autour d’obstacles, à les contourner.
Et même à parcourir un labyrinthe.
Un point faible des premiers motifs Lenia, c’était leur stabilité. Comme dans le
jeu de la vie, la moindre modification pouvait les détruire. Mais on connait maintenant
des motifs qui résistent très bien aux perturbations. Par exemple qui sont capables de se reconstituer
si on les endommage, si on injecte du bruit, ou bien s’ils sont attaqués par des balles
virtuelles.
Certains de ces motifs sont aussi capable de se déplacer en fonction de la présence
d’un certain signal sur un canal. C’est un phénomène qui se rapproche de ce qu’on
appelle la chimiotaxie chez les bactéries, et qui leur permet de diriger leur mouvement
en fonction de la concentration de certaines espèces chimiques.
Et aussi, bien sûr, on a trouvé plus de motifs capable de se reproduire, une des caractéristiques
les plus importantes de la vie. Avec même ici un cas de reproduction de deux individus
qui en engendrent un troisième !
En ajustant encore les règles de base de Lenia, les scientifiques ont pu plus facilement
découvrir des formes stables, et même les mettre en compétition dans une sorte d’écosystème
virtuel, et observer des premiers éléments de sélection naturelle.]
Ces dernières années, Lenia s’est imposé comme l’un des systèmes de vie artificielle
les plus prometteurs, permettant d’étudier avec un nombre restreint de règles certains
phénomènes émergents qui caractérisent la vie au sens large.
Ces découvertes ont été grandement accélérées par l’adaptation de ces méthodes de machine
learning qui facilitent l’exploration de l’espace des paramètres. Et il est vraisemblable
que le panel de créatures virtuelles continue de s’enrichir dans les années à venir.
Merci d’avoir suivi la vidéo, un merci tout particulier aux chercheuses et chercheurs
de l’équipe Flowers de l’INRIA, avec qui j’ai eu le plaisir d’échanger, je
vous mets plein de références en description. Abonnez vous à la chaine, je continue de
le dire car Youtube diffuse de moins en moins mes vidéos, c’est dommage. Et en attendant
je vous dit à très vite pour une nouvelle vidéo, à bientôt. MINIATURE
