https://fr.wikipedia.org/wiki/Jeu_de_la_vie

Aujourd'hui, on part en voyage. Et on ne part pas que vous et moi, on part sur le dos de cette petite créature
qui se propose d'être notre guide dans une simulation mathématique aux règles très simples sur une grille en 2D théoriquement infinie,
dans un univers dont la simplicité des règles ne laisserait pas une seule seconde le néophyte que vous êtes probablement, imaginer ce qui s'y déroule.
Voilà ce qu'on appelle de la soupe. Certains l'appellent la soupe primordiale.
C'est une notion initialement moins quadrillée que ce que vous êtes en train de regarder, puisqu'à la base, c'est un concept qui hypothèse la manière dont la vie a pu commencer sur Terre
il y a 3 à 4 milliards d'années. C'est une théorie qui présente une sorte de soupe, un environnement riche en composés organiques dont la vie pourrait avoir émergé.
C'est une théorie solide, même si on reste ouvert à d'autres hypothèses. L'avantage pour nous, c'est que la soupe primordiale dont on va parler aujourd'hui,
on peut la créer chez nous, la voir évoluer sous nos yeux, dans un jeu vieux de plus de 50 ans aux règles très simples, que n'importe qui peut faire tourner sur son PC
pour s'amuser à jouer avec les paramètres initiaux. Ceci dit vous, vous n'avez rien besoin de faire, c'est moi qui m'en charge.
Et même plutôt c'est notre petite créature qu'on va suivre tout le long de cette vidéo pour découvrir à ses côtés le monde dans lequel elle évolue.
Et si j'imagine bien que vous trouvez pour le moment que le monde en question ne paye pas de mine, je vous assure que ça va loin.
Pour vous aider à suivre si ça devient trop compliqué, vous avez dans la description le lien d'un site qui vous permettra de vous y retrouver plus facilement.
J'y retrace notre petit voyage dans les grandes lignes, avec les règles, les dates, les catégories de configuration.
De quoi retomber sur vos pattes en cas de besoin, et d'ailleurs ne me remerciez pas, le site est fait maison avec mes petites mains
grâce au partenaire de cette vidéo : Odoo. Odoo, pour ceux qui ne connaissent pas, c'est une plateforme qui propose une large gamme d'applications pour gérer son business,
dont, entre autres, un outil de création de site Web simple et rapide. C'est-à-dire que c'est fait pour que n'importe qui puisse le prendre en main rapidement
et c'est littéralement ce que j'ai fait pour celui-ci. J'arrive, on me demande mes objectifs, ma palette de couleurs, j'ajoute mes pages, je choisis mon thème et ensuite c'est tout droit, le site est déjà créé.
Pour le personnaliser, c'est une sorte de grand jeu de Lego qui propose simplement de prendre des blocs, de les glisser et de les déposer.
Je veux un titre et une image, je prends le bloc, je le glisse et je le dépose. Des comparaisons, pareil. Une chronologie juste ici,
des colonnes, des chiffres, une carte, tout ça, je prends et je dépose. Et une fois que c'est posé, je peux modifier le texte, les images, les couleurs, les animations.
Et même si j'ai du mal à écrire, pas d'inspiration, Odoo propose une IA qui peut écrire en fonction de ce que je lui demande.
Bref, le but premier de cet outil, c'est vraiment la simplicité, pouvoir construire intuitivement. C'est ce qui m'a permis de fabriquer ce site alors que je n'y connaissais rien.
C'est un site d'ailleurs qui ne m'a rien coûté parce que la première application Odoo que vous choisissez est gratuite à vie, avec un hébergement et un support illimités.
En plus de ça, Odoo nous offre notre nom de domaine personnalisé pendant un an. Et si la chose vous tente, vous trouverez dans la description et les commentaires
un lien sur lequel il vous suffit de cliquer pour vous lancer avec Odoo. Et un autre lien, évidemment, pour vous aider à suivre notre petit voyage plus facilement.
Petit voyage auquel on va retourner d'ailleurs. En fait, nous voilà de retour dans notre soupe primordiale et vous n'y comprenez rien et c'est normal,
donc on va commencer par la base. Ça, c'est ce qu'on appelle le Jeu de la Vie de Conway. Parce que cette version a été mise au point par un mathématicien du nom de John Conway en 1970.
Ce Jeu de la Vie fait partie d'une catégorie de jeux assez particulière qu'on appelle le jeu à zéro joueur, c'est-à-dire qu'il se joue tout seul.
Ici, plus spécifiquement, on a affaire à ce qu'on appelle un automate cellulaire. Un automate cellulaire, c'est un modèle mathématique composé de cellules
qui interagissent entre elles selon des règles précises. On va faire simple et fonctionner avec ce qu'on voit ici pour comprendre.
Là, on a une grille théoriquement infinie en 2D. Tous les petits carrés que vous voyez formés par la grille,
ce sont des cellules qui peuvent être dans deux états : mortes, en noir ou vivantes, en blanc.
Chaque cellule est entourée par huit autres cellules qu'on appelle des voisines. Ok, ça, c'est la base. Maintenant, comment est-ce qu'on donne vie à tout ça ?
Bah déjà, le jeu fonctionne par générations, une par une. L’état de la génération actuelle, donc ce qu'on voit là,
va déterminer l'état de la génération suivante. Et on avance génération par génération.
La grande question, c'est de savoir selon quelles règles on génère chaque génération. L'avantage pour nous, c'est que tout le principe du Jeu de la Vie de Conway,
c'est d'avoir des règles très simples et de voir à quel point les choses peuvent devenir complexes. Et pour le coup, c'est difficile de faire plus simple
puisque le Jeu de la Vie n'a que deux règles qu'on comprend très facilement si on réfléchit aux cellules comme à de vraies populations.
La première règle, c'est qu'une cellule morte qui a exactement trois cellules voisines vivantes devient vivante.
En gros, la vie dans les bonnes quantités crée la vie. La deuxième règle, c'est qu’une cellule vivante
qui possède deux ou trois voisines vivantes, reste vivante. Plus de trois voisines et la cellule meurt comme s'il y avait surpopulation.
Moins de deux voisines, la cellule meurt comme s'il y avait sous-population. On va prendre un exemple pour que vous assimiliez le concept plus facilement.
Voilà trois cellules vivantes côte à côte. Pour chaque cellule du groupe, on va calculer ce qui va se passer lors de la génération suivante.
Ok, pour cette cellule, on regarde ce qu'il y a autour, dans son entourage direct elle à deux voisines vivantes,
donc selon cette règle, elle reste vivante. Pour cette cellule pareil, deux voisines, donc elle reste vivante.
Et pour cette cellule aussi. Donc ces trois cellules vivantes restent vivantes. Mais qu'est-ce qu'il advient de cette cellule qui est morte ?
Elle, elle n'a pas seulement deux voisines vivantes, mais trois. Ce qui veut dire, selon la première règle, qu'elle devient vivante.
La vie en bonne quantité crée la vie. Bon, on vient de créer quelque chose. C'est pas mal déjà.
Mais qu'est-ce qui va se passer lors de la prochaine génération ? Eh bah, comme chaque cellule vivante possède trois voisines vivantes,
toutes les cellules restent vivantes, ce qui nous donne ce qu'on appelle un bloc, l'une des configurations les plus courantes du Jeu de la Vie de Conway.
Voilà comment le Jeu de la Vie fonctionne. À chaque génération, le logiciel calcule l'état de chaque cellule présente sur la grille en même temps,
et on avance, la vitesse, c'est moi qui la gère, ça dépendra de nos rencontres. Bref, ce bloc, il appartient à une grande catégorie de configurations
qu'on appelle des natures mortes. En gros, ce sont des configurations qui ne bougent pas. On en trouve un paquet. Et elle, on l'appelle la “ruche”,
elle la “table miroir”. Et c'est bien, ça nous fait de jolis dessins, mais c'est pas palpitant non plus.
Nous, on veut du mouvement. Voilà trois cellules vivantes côte à côte et c'est votre premier exercice.
Mettez sur pause, je vous laisse réfléchir à ce que ça va donner. Ok, on reprend pour chaque cellule.
Celle-ci n'a qu'une voisine vivante, donc elle mourra à la prochaine génération. Pareil pour celle-ci, morte.
Celle du milieu a deux voisines vivantes, donc elle reste en vie. Maintenant, on peut regarder les cellules mortes autour.
Celle-ci n'a que deux voisines vivantes, donc pas assez pour devenir vivante. Pareil pour ces trois-là. Mais celle-ci a trois voisines vivantes,
celle-ci aussi, donc elles prennent vie, ce qui nous donne, à la génération suivante, ceci. Et quand on laisse tourner,
voilà notre première forme de vie un peu intéressante : le blinker. Ça bouge, c'est joli et on peut faire mieux.
Le blinker, il appartient à la grande catégorie des oscillateurs. Ce sont des configurations qui se répètent après un certain nombre de générations.
Ça, par exemple, c'est le “cercle de feu”. Comme le blinker, c'est un oscillateur qui fonctionne en deux générations, c'est-à-dire qu'il lui faut deux générations pour revenir à son état initial.
Et on peut monter en générations. En trois, on a le "pulsar", en quatre, on a la "roue de Catherine",
en huit, le "figure eight", en vingt, le "145P20", en trente-sept, le "Beluchenko’s p37".
Bref, ça peut monter très haut. Mais parmi les oscillateurs qui nécessitent plus de générations, on trouve des choses dont je voudrais vous parler plus tard,
donc on va se garder ça sous le coude. Mais il y a peut-être quelque chose que vous commencez à vous dire en regardant ces configurations.
Comment on passe de la soupe à ça ? Parce que moi, quand je lance une soupe chez moi, j'arrive à pas grand chose.
Et c'est là que le titre de jeu à zéro joueur vous a peut-être induit en erreur. Le Jeu de la Vie, c’est pas juste un jeu qu'on lance et qu'on regarde.
C'est un jeu qu'on explore et avec lequel on est invité à expérimenter. Et ce qui se passe dès 1970 à la création du jeu,
c'est qu'on trouve une foule de gens qui cherchent, qui essayent, qui trouvent, qui nomment, qui mettent en commun. Et j'ai été très surpris de me retrouver complètement écrasé par la quantité d'informations
qu'on trouve sur le wiki du Jeu de la Vie, on trouve les explications des concepts, des catégories. On trouve des listes de configurations avec leurs composants,
leurs fonctionnements, leurs créateurs qui ont eux-mêmes des pages sur lesquelles sont recensées toutes leurs trouvailles, avec toujours des noms très inspirés.
Là, c'est le french kiss. Là, la mini cocotte-minute. Là, la machine à laver. On sent vraiment que c'est fait maison quoi.
Bref, ces configurations, ce sont des gens qui les ont découvertes et ils ont commencé dès la sortie du jeu. Donc John Conway met le principe du Jeu de la Vie au point,
il fait publier ses recherches la même année dans un journal appelé le Scientific American. Les gens lisent l'article et ils se disent :
Donc les gens essayent chez eux. Le truc, c'est qu'en 1970 faut se débrouiller parce que l'ordinateur, à ce moment-là, c'est pas forcément un objet très accessible.
Donc les gens se débrouillent. Ils commencent à dessiner leurs grilles, à placer des jetons pour représenter les cellules vivantes et ils font leurs essais, génération par génération, à la main.
C'est lent, c'est facile de faire des erreurs, mais rien qui n'empêche les passionnés de continuer à se passionner. Et Conway fait partie de ces gens-là.
À Cambridge, l'université où il enseigne, lui, il le faisait déjà même avant la publication de son papier,
évidemment, avec un jeu de go. C'était compliqué de tout faire tout seul et surtout de garder le fil des plus petites configurations.
Donc, il a invité un de ses amis, Richard, pour lui servir de blinker watcher. En gros, il avait la charge des blinkers et de toutes les petites configurations périodiques.
Donc il cherche, il tente des configurations. Et puis un jour, Richard dit à Conway :
Sauf que c'est pas un blinker. Richard vient de faire la découverte de la petite créature avec laquelle on va dorénavant voyager :
le glider. Quand on veut créer un glider, on pose nos cellules vivantes comme ça.
En fait, en fonction de la direction qu'on veut lui donner, on peut le tourner verticalement et horizontalement. Moi, je le place comme ça pour qu'il parte…
Bah ça, ça peut être notre deuxième exercice si vous voulez, vous êtes pas obligés mais si vous vous en sentez l'envie, mettez sur pause et devinez où on part.
Si vous avez la flemme, restez avec moi.
Bon allez, je vais pas faire durer le suspense, on va démarrer notre glider direction en haut à droite.
Eh bah voilà, on l'a notre véhicule. On peut explorer le Jeu de la Vie et on a de la route.
Notre première rencontre, c'est un autre vaisseau qu'on appelle le lightweight spaceship. Moi, je l'appelle le poisson, parce qu’on dirait un poisson.
Vous remarquerez que sa structure est très similaire à notre glider en un peu plus grand et avec deux blocs de plus à l'arrière.
Mais la vraie différence avec notre véhicule à nous, c'est que ce vaisseau emprunte une direction orthogonale, là où le nôtre se déplace en diagonale.
Un peu plus loin, on trouve des versions un peu plus grandes du lightweight spaceship : le middleweight spaceship et le heavyweight spaceship,
qui requièrent simplement d'allonger la coque du vaisseau. Ces trois configurations, notre glider aussi d'ailleurs, sont ce qu'on appelle des “vaisseaux naturels”,
à savoir qu'ils peuvent être trouvés naturellement dans la soupe sans intervention humaine. Des vaisseaux naturels, pour le moment, on en a trouvé 54
qui ont été triés par ordre de fréquence relative, ici la fréquence d'apparition de tous les vaisseaux qui apparaissent naturellement.
Typiquement le glider il représente 99,8 % des apparitions de vaisseaux naturels. On tombe immédiatement à 15 / 10 000 pour le lightweight spaceship.
Cette configuration, à peine plus grosse, tombe à 2 / 10 millions. On descend vite dans les milliards, puis les billions.
Bref, disons peu de chances de tomber dessus. Et ça, ça reste pour des configurations modestes et assez ennuyeuses d'ailleurs,
nous, on veut plus. Et la bonne nouvelle, c'est qu'on approche de quelque chose de nettement plus stimulant.
Je sais qu'on ne dirait pas un vaisseau, pourtant c’en est un qu'on appelle le Cordership à deux moteurs. Le Cordership à deux moteurs, il ne ressemble pas à grand chose,
pourtant, c'est un vaisseau qui a été voté Pattern of the Year en 2017 sur les forums de conwaylife.com.
Déjà, quand on accélère un peu la simulation, on se rend compte que ce n'est effectivement pas juste un fouillis de cellules, mais bien une entité qui avance.
Ensuite, il faut bien comprendre pourquoi on parle de Cordership à deux moteurs. Dans ce vaisseau, le moteur, c'est ce qu'on appelle un switch engine
qui produit une copie de lui-même après 48 générations et qui peut être utilisé pour propulser des vaisseaux.
Il faut savoir que le tout premier Cordership à avoir été découvert l'a été en 1991 et il possède treize moteurs.
La complexité croissante des découvertes de Corderships, ça n'a pas été de rajouter des moteurs, mais d'en enlever, de réussir à en fabriquer avec moins.
Un Cordership à dix moteurs a été trouvé la même année, à sept moteurs en 1993, à six en 1998,
jusqu'à ce qu'on arrive en 2017 avec cette trouvaille donc, du Cordership à deux moteurs qui remporte son petit prix.
Donc c'est une belle machine qu'on voit là. Mais en la regardant, on a un peu l'impression de voir des nuages de cellules avancer côte à côte,
sans trop d'intérêt les uns pour les autres. Mais je peux le laisser tourner en fond et je vous invite à fixer un morceau tout du long,
vous verrez qu'en fait, tout est connecté. Les pièces du vaisseau interagissent entre elles et influent les unes sur les autres pour faire avancer l'ensemble
et le maintenir dans un état cohérent, quoique très fragile. Je me permets un peu de sabotage. On met sur pause, on retire cette cellule au hasard,
ça vole tout de suite moins bien. La raison pour laquelle on n'a pas encore trouvé de Cordership à un moteur, c'est que ces moteurs sont très chaotiques.
Je vous remets un switch engine tout seul, ça part vraiment dans tous les sens. Donc si on veut en utiliser pour propulser un vaisseau,
pour éviter que les débris produits détruisent l'ensemble, on cumule les moteurs pour qu'ils nullifient leurs effets destructeurs.
C'est pour ça qu'il est compliqué de trouver de nouvelles configurations de Corderships. Trouver l’assemblement parfait des switch engines pour faire avancer le vaisseau
et garder sa cohérence malgré la nature chaotique des moteurs, bah c'est pas évident. D'ailleurs, pendant que je filmais ce plan d’assemblement de switch engines,
ça a fait ça. Voilà. Quoi qu'il en soit, les switch engines : chaotiques.
Trouver la bonne configuration : compliqué. Mais les gens ont réussi, et le Cordership à deux moteurs avance très lentement.
C'est… Très très lentement. La vitesse du Cordership à deux moteurs s'écrit comme ça : C / 12.
Donc 1 / 12ème de C. C représentant la vitesse de la lumière sur le Jeu de la Vie. 1 / 12ème de C c’est vraiment lent.
Pour comparer, notre glider il avance à une vitesse de C / 4 donc on trace et on s'éloigne.
Sauf que pas vraiment puisqu'on tombe sur un autre Cordership à deux moteurs
puis un autre… Ok, il se passe quoi là ? Bon, on va avancer, on a du chemin.
L'avantage, c'est que je peux accélérer… Encore…
Ok, voilà, on est arrivés. Voilà une usine à Corderships à deux moteurs. Pour comprendre ce qu'on regarde, il faut qu'on revienne à beaucoup plus simple.
John Conway, les tests de configurations sur le jeu de go, ça lui plaisait, mais en modération. Donc après la découverte du glider, il a mis en jeu une somme d'argent
que pourraient gagner ceux ou celles qui trouveraient une manière de fabriquer une usine à gliders. Pour s'assurer d'avoir quelque chose,
il a décidé d'offrir le prix à la première personne qui pourrait créer une configuration capable de générer une population à l'infini.
Et donc un petit groupe d'étudiants du MIT se mettent sur le coup et découvrent finalement ceci.
Voilà ce qu'on appelle un “gun”, plus spécifiquement un “glider gun”, encore plus spécifiquement un “gosper glider gun”.
Bon, moi j’aime pas les appeler "guns", je trouve ça trop violent pour rien. Donc je les appelle papa maman. Mais on va plutôt partir sur géniteurs par souci de crédibilité.
Je vous laisse regarder ces géniteurs quelques secondes pour que vous compreniez ce qui se passe. Bon, en fait c'est tout simple, il n'y a pas grand chose à comprendre.
En gros, la rencontre de deux groupes de cellules différents provoque deux choses à la fois. La première, c'est qu’un glider naît de la rencontre.
La deuxième, c'est que chaque groupe repart dans son coin, tape sur ces blocs et revient pour une nouvelle rencontre.
Des géniteurs, on en trouve de toutes sortes. On peut produire beaucoup de choses différentes à des timings différents et c'est encore une fois l'un des domaines dans lesquels les joueurs m'impressionnent le plus.
Parce qu'ils n'hésitent pas à assembler pour créer plus. Typiquement, avec une abeille et deux blocs, on peut créer la navette basique
qu'on peut assembler de manière à fabriquer le gosper glider gun, qu'on peut à son tour utiliser pour créer le “period 60 glider gun”,
ou le “5 and dime”, un générateur qui prend 150 générations à produire un glider,
c'est long. Et ça peut paraître inutile, mais moi, je vais m'en servir. Pour tout vous dire, le jeu m'a même forcé, moi, à combiner
alors que je me pensais juste spectateur du Jeu de la Vie. Pour ça, on revient à notre usine à Corderships à deux moteurs.
En fait, à la base, c’est pas une usine, c'est un convertisseur. J'ai pas trouvé d'usine complète quand j'ai cherché sur le wiki,
j'ai trouvé une usine à Corderships à trois moteurs ou à six mais pas à deux. Tout ce que j'ai trouvé,
c'est cette machine qui transforme un glider en Cordership à deux moteurs. Mais une fois que le glider est parti, bah c'est fini.
Donc on n'a pas d'usine. Ceci dit, évidemment, il m'a pas resté grand chose à faire. Il m'a suffi de trouver un géniteur à gliders, de le placer au bon endroit,
et tac. Sauf que ça a cassé l'usine parce que l'assemblage des gliders à ce point prend du temps et que la production de gliders du géniteur était trop élevée.
Donc j'ai cherché des géniteurs qui prenaient plus de temps à accoucher. Typiquement, je pensais que le “five and dime” que je vous ai présenté avant,
qui produit un glider toutes les 150 générations, je pensais qu'il prendrait assez son temps et que ça fonctionnerait comme ça.
Mais non, c'est quand même trop rapide, usine cassée. Donc je suis juste allé récupérer un géniteur de l'usine à Corderships à trois moteurs.
Assez lent celui-ci, ce qui nous a donné cette petite usine. Bon, rien de sorcier donc, mais j'ai quand même dû faire mon bricolage
et ça m'a laissé le temps de bien comprendre le fonctionnement de l'usine, très simple. Elle fonctionne avec un géniteur qui laisse partir sa progéniture dans un tuyau
qui amène le glider ici qui provoque une réaction générant plus de gliders, à leur tour amenés dans des tuyaux.
Tous les gliders se réunissent ici et forment lentement un Cordership à deux moteurs. C'est tout le temps comme ça que les usines fonctionnent.
La rencontre de différents groupes de cellules amène à la création d'un ou plusieurs autres groupes de cellules. Et puis, on peut toujours partir sur plus gros.
Voilà le "V gun". Ouais, lui je veux bien l'appeler gun il est… Intimidant. Ça, c'est le "maximum volatility gun".
C'est grand. Et pourtant, c'est ce qu'on appelle un single barrelled gun, à savoir qu'il ne produit qu'un seul courant de vaisseaux,
juste ici et simplement des gliders. Et c'est toujours super intéressant de se pencher sur ces constructions
et de comprendre comment ça fonctionne. Moi, c'est comme ça que je fais avant de vous en parler, je regarde et je cherche à comprendre.
D'ailleurs, je rappelle que je suis loin d'être un expert. Moi, je suis comme un enfant à la récré qui veut montrer ce qu'il a trouvé à ses copains.
Et mes copains, c'est vous. Donc toute cette vidéo, c'est moi qui viens vers vous pour vous montrer les jolis cailloux que j'ai trouvés dans la cour.
Le maximum volatility gun, il est assez simple à comprendre, mais il m'a quand même surpris. À le regarder de loin, moi je pensais qu'il y avait une source de gliders
qui ensuite rebondissait sur ces sortes de bumpers pour ensuite être envoyés dans l'espace. Et je me disais : “Bah c'est fait pour être joli, pour tournicoter dans tous les sens et c'est très bien comme ça.”
Mais en fait, c'est pas vraiment ça. Déjà on a des oscillateurs un peu partout
qui sont toutes ces structures qui oscillent en permanence. Et effectivement, à certains endroits ces oscillateurs servent de bumpers,
elles redirigent les gliders qui arrivent sur elles. Ce courant par exemple, il tape là, puis là,
puis là et il arrive enfin ici. À cet endroit, par contre, il se passe quelque chose de différent. La file de gliders qu'on suivait rencontre deux autres files de gliders
qui viennent elles aussi de rebondir sur plusieurs bumpers. Donc, on connaît le procédé, des groupes de cellules qui entrent en collision
et qui produisent un autre groupe de cellules. Sauf que là, c’est pas tout. On pourrait croire que c'est juste leur collision qui provoque la naissance d'un vaisseau.
Mais pas du tout. Regardez si je mets sur pause, que je retire cet oscillateur là et que je relance.
Bah la machine elle est cassée et quand j'accélère, on n'a plus de flux de production. Donc retour en arrière, ça c'est l'un des oscillateurs qui ne fonctionne pas comme bumper
mais qu'on utilise comme un objet dont la présence à cet endroit influe sur les objets qui passent à côté. Et à cet endroit, ça provoque,
en l'occurrence avec l'arrivée des trois flux de gliders, la naissance d'un vaisseau qui part vers la gauche. Donc on continue par là avec notre nouveau vaisseau
et on assiste à la même chose un peu plus loin. Collision de trois gliders, sans oscillateur cette fois-ci, ce qui donne naissance au même vaisseau en un peu plus petit.
Juste après, les vaisseaux passent à côté d'autres oscillateurs et leur passage provoque la naissance d'autres gliders qui partent à leur tour rebondir sur des bumpers pour créer d'autres vaisseaux.
On arrive finalement à l'avant dernier oscillateur de la file qui a un impact sur le petit vaisseau qui à son tour a un impact sur le grand vaisseau
qui donne naissance à un glider qui finalement part entre deux autres gliders dans le vide.
Et le petit vaisseau du haut rencontre un dernier oscillateur qui le transforme en glider et le renvoie dans la chaîne.
C'est bien foutu non ? Rien ne se perd, tout tourne. On a une jolie usine. Moi ça me plaît bien et en plus j'aime bien les flux de vaisseaux
qui s'entrecroisent sans jamais se frapper. C'est une vraie machine bien huilée. Bon, il y a plein d'autres usines,
plus grandes encore, qui produisent encore plus gros. Celle-ci est assez intimidante. Elle produit des Corderships à six moteurs
et il faut reconnaître que c'est quelque chose. On a l'impression d'assister aux manœuvres d'une usine spatiale gigantesque.
C'est assez génial à regarder, je trouve. Nous on continue à avancer.
Ça fait peut-être beaucoup d'infos, mais on peut respirer. Notre prochaine rencontre nous le permet. Ça, c'est un Mathusalem,
c'est une configuration qui nécessite un grand nombre de générations pour se stabiliser et qui devient beaucoup plus grande que sa configuration initiale
à un moment donné de son évolution. Sauf que nous, on est arrivés au tout début et que ce Mathusalem-ci prend 52 513 générations à se stabiliser.
Donc il reste un bout de temps avant d'en voir la fin et ça permet de souffler.
Qui sait à quoi ça ressemblera vraiment quand ce sera fini ? On s'éloigne déjà.
On saura pas, c'est la vie. Ah ça c’est pas mal. Je vous présente le "Pony express".
C'est ce qu'on appelle un puffer, c'est une configuration qui se déplace comme un vaisseau, mais qui laisse des débris derrière elle.
Typiquement, la configuration que j'ai découverte tout à l'heure, bah je l'ai pas découverte puisque je la retrouve en pièces du Pony express
qui peut de son côté nous empêcher de passer, donc il faut juste qu'on se dépêche. Voilà.
Donc je répète, ça c'est un puffer qui laisse des débris derrière lui et beaucoup de vaisseaux pourraient laisser des débris
mais s'arrangent pour nettoyer derrière eux. Lui par exemple, il laisse une longue ligne de débris derrière lui,
mais il a prévu le coup avec un vaisseau-balai pour la faire s'effondrer. Lui, il fait pareil avec un jeu de dominos.
Celui-ci avec de petits missiles.
Celui-là, il est vraiment particulier, les débris sortent à l'arrière, ils sont transformés en gliders, remontent par le centre et sont éliminés à l'avant
grâce à des blinkers qui se replacent au bon endroit à chaque fois. Je l'aime vraiment beaucoup lui.
Ah, lui c’est le "Backrake 3". C'est un autre puffer, mais dans une sous-catégorie bien particulière de puffers :
le rake. En gros, il laisse aussi des débris derrière lui. Sauf que ses débris sont des vaisseaux.
Et derrière, ça commence à devenir imposant.
Mais ça, c'est rien par rapport à ça. Je vous présente le "Breeder 1", le premier breeder jamais découvert.
C'est déjà un gros morceau, mais il reste minuscule par rapport à ce qu'il peut devenir parce que sa croissance est infinie.
En fait, c'est même plus que ça. Le Breeder 1 est la première configuration trouvée à présenter ce qu'on appelle une croissance quadratique,
qu'on résume de la sorte :
En gros, ça veut dire que la croissance n'est pas linéaire, elle est pas constante, mais qu'elle accélère avec le temps. Et vous vous demandez peut-être comment c'est possible avec le Breeder 1,
je vous explique. Cette configuration est un puffer, donc une configuration qui avance comme un vaisseau mais qui laisse derrière elle des débris.
Ceux qui ont bien suivi pourraient peut-être croire que c'est même un rake. Vous vous souvenez, c'est la sous-configuration de puffers
qui ne laisse derrière elle pas que de simples débris mais des vaisseaux. C'est bien ce qu'on voit ici, non ? Des vaisseaux produits par le Breeder 1.
Eh bah pas tout à fait. En fait, le Breeder 1, c'est pas un rake, c'est bel et bien un puffer, la subtilité, c'est que le Breeder 1 produit en débris des usines.
Ses débris, ce sont des usines à gliders qui restent au même endroit pour toujours en produisant des gliders en boucle.
Et ça donne ça.
Ok, on a compris qu'on pouvait faire grand, mais c'est pas le plus intéressant, le plus intéressant, il commence plus petit.
On revient à la base. L'un des objectifs de John Conway, quand il a mis sur pied ce Jeu de la Vie, c'était d'avoir un jeu aux règles simples
et de voir à quel point les choses pouvaient y devenir complexes. Il y a un truc qui pourrait vraiment satisfaire ce principe de complexité
et que Conway espérait dès le début, c'était de rendre son jeu Turing complet. Donc je vous donne la définition de Turing complet :
C'est un principe fondamental en informatique, qui fait référence à la capacité d'un système de calcul à simuler n'importe quelle machine de Turing,
ce qui voudrait dire, sans trop rentrer dans les détails, qu'il pourrait exécuter tout algorithme ou résoudre n'importe quel problème de calcul,
à condition de le décrire formellement et d'avoir assez d'espace et de temps. Est-ce qu'on pourrait faire ça avec le Jeu de la Vie ?
Est-ce qu'on passerait vraiment des vaisseaux à un ordinateur ? Déjà, il faudrait qu'on puisse envoyer des signaux,
qu'on trouve des substituts à l'électricité. Ça, on peut le faire avec des géniteurs. On peut envoyer des gliders à intervalle régulier ou non.
Ensuite, il faudrait pouvoir utiliser ces signaux avec des portes logiques. La porte “and” par exemple, qui veut dire “et”,
et qui est là pour vérifier qu'elle reçoit bien deux courants. Est-ce qu'on pourrait faire ça dans le Jeu de la Vie ? Bah plaçons déjà nos deux courants,
A ici, B ici, pas encore activés. Le but, ce serait d'avoir une porte qui ne laisserait passer un courant ici
que si elle reçoit bien deux signaux, donc C s’active uniquement si A et B sont activés. Pour faire ça, on utilise un géniteur activé qui envoie un courant X dans ce sens,
c'est ce X qui va servir de porte. Maintenant activons seulement A, le flux part, mais le courant est coupé par X
et il n'arrive pas en C. Donc ça fonctionne pas. Activons seulement B et pareil, le flux part coupé par X.
Donc on n'a rien au bout. Maintenant, si on active A et B en même temps, B va venir couper X et permettre à A de passer.
Ce qui nous donne effectivement un courant qui arrive en C. On a une porte “and” qui ne s'active que quand elle reçoit deux signaux.
Ok, c'est pas mal. C'est comme ça que les joueurs se débrouillent pour fabriquer des portes logiques. Et sur le même principe, on construit les deux autres portes logiques principales,
“or” et “not” utilisées pour construire toutes les autres portes logiques. Ensuite, on utilise des réflecteurs pour former des adaptateurs
qui permettent la conversion et l'ajustement des signaux entre les différentes parties du circuit pour assurer une communication fluide entre elles,
ce qui permet de connecter ces portes logiques pour réaliser des opérations plus complexes. On peut s'en servir comme d'additionneur
qui permet la réalisation d'additions de nombres binaires. Ça, c'est un composant essentiel pour construire des entités arithmétiques
capables d'effectuer des calculs plus complexes au sein de cet environnement. Et on continue. Les joueurs fabriquent ce qu'on appelle un “ALU”, une “Arithmetic & Logic Unit”,
responsable des opérations arithmétiques et logiques, qui permet de manipuler les données selon des instructions programmées,
en l'occurrence des instructions en 8 bits. Maintenant essentiel : il faut stocker ces informations. Est-ce qu'on peut faire ça ?
Est-ce qu'on peut fabriquer une mémoire dans le Jeu de la Vie ? Évidemment. Voilà une unité de mémoire qui utilise des bascules “RS”
et des portes logiques pour stocker des bits d'information binaire de manière à maintenir l'état de chaque information jusqu'à ce qu'une instruction opposée soit reçue,
mais qui sert aussi à lire et à écrire des instructions avec trois modules tout à gauche pour lire et décoder des adresses.
Et qu'est-ce qu'on fait avec tout ça ? Bah il nous faut des instructions. Voilà un programme avec le décodeur, les instructions, les adresses,
les données qu'on peut modifier avec un script Python qui permet de programmer l'ordinateur qu'on est en train de monter et comment on voit ce qu'on fait ?
Eh bah voilà l'écran… Qui commence à se remplir en fonction des instructions.
Sauf que c'est long. Wow c'est super long et mon PC a un peu de mal quand j'accélère. Mais on l’a, notre ordinateur,
on l'a notre Turing complet avec une grille et deux règles. Et dites vous que ça, Conway, il y avait pensé dès la création de son jeu.
Sauf que ça ne paraissait pas tout à fait évident au début. Ce qui lui avait fait comprendre que la chose devrait normalement être possible,
c'était la découverte du glider, dont il imaginait déjà le potentiel de transmission de données.
Notre glider à nous approche d'ailleurs de la fin de son petit voyage. Il en aura vu des choses :
les natures mortes, les oscillateurs, les vaisseaux, les Corderships, les usines, les puffers, les rakes, les breeders. Mais je vous assure qu'il n'a jamais vu ce dont il approche.
Parce que le Jeu de la Vie de Conway, ce qui rend le fait qu'il soit Turing complet vraiment intéressant et ce pourquoi je vous en ai parlé,
c’est pas juste pour vous montrer qu'on peut faire des additions à l'intérieur, c’est pour finir de la plus jolie des manières en vous rappelant qu'être Turing complet,
ça veut dire pouvoir faire tourner n'importe quel algorithme. Là, on ne pourrait pas rêver mieux pour finir notre voyage.
Et moi, je vois ça et donc je me dis : “C'est fou ce qu'on peut faire avec si peu.”
Et cette phrase, ça a été le départ d'une nouvelle aventure pour moi, ça a été ma porte d'entrée dans un monde loin de s'arrêter aux frontières du Jeu de la Vie de Conway.
Déjà parce que des Jeux de la Vie, il en existe plein créés par des gens qui ne se sont pas arrêtés à la découverte de configurations
dans le Jeu de la Vie original, mais qui se sont dit :
Highlife, par exemple, reprend la même base en décidant plutôt qu'une cellule peut devenir vivante si elle est entourée d'exactement six cellules vivantes.
Ça ne paraît rien, mais ça change complètement la dynamique du jeu avec l'apparition de réplicateurs, des structures qui peuvent se reproduire exactement
à une certaine distance d'elles-mêmes, avec un nombre défini de générations. Conway lui même dira en parlant de ce jeu :
Et on trouve tellement de variantes : Wireworld qui simule le comportement de fils électriques,
Langton’s ant qui fait se déplacer une fourmi sur une grille similaire au Jeu de la Vie, en changeant l'état des cellules sur lesquelles elle passe
et en suivant la direction indiquée par la cellule sur laquelle elle tombe. Turmites qui reprend le même principe avec plus d'états de couleur
pour les cellules et plus de deux directions de rotation. Et moi, je continue à me laisser porter, à tomber sur toujours plus.
Sur Lenia par exemple. C'est une sorte de nouvelle version du Jeu de la Vie de Conway. C'est un modèle d'automate cellulaire continu et multi-états.
En gros, il veut faire plus organique. Il laisse tomber cette histoire d'avancer génération par génération, en avançant plutôt dans le temps de manière continue,
et qui laisse aussi tomber cette histoire de grille en la remplaçant par un espace continu qui permet aux configurations de cellules de n'avoir pas qu'un état,
mort ou vivant, 0 ou 1, mais en permettant l'usage des décimales, une cellule peut maintenant être 0.23 ou 0.77,
ce qui donne naissance à des créatures qui paraissent immédiatement plus organiques, on dirait des vrais petits machins là, et à laquelle son créateur a plus tard permis l'utilisation de l'IA
pour qu'elles apprennent à se déplacer de manière plus optimale. Et je continue mon voyage et je tombe sur tellement d'algorithmes
et tellement de modèles qu'il faudra très probablement que je vous en parle dans une deuxième vidéo. Ce qui est certain, c'est que je deviens obsédé par la création de la vie artificielle.
Je tombe la tête la première dans ce qu'on appelle "l'artificial life". C'est un domaine de recherche qui étudie les systèmes réalisés par l'homme
et qui imite des comportements, des processus et des phénomènes de la vie biologique. C'est-à-dire qu'on se penche sur l'exploration des principes fondamentaux de la vie,
sur la création des formes de vie artificielles dans des environnements physiques ou virtuels. On explore les applications potentielles de la vie artificielle
dans plein de secteurs comme la robotique, la médecine. Ce qui ouvre aussi des questions éthiques et philosophiques sur notre compréhension de la vie et notre rôle en tant que créateurs de systèmes vivants.
Ce qui est certain, c'est que je deviens accro et que je recherche dans ces simulations toujours plus vrai.
Et donc je suis là sur mon PC, je regarde les petites bestioles de Lenia faire leur vie devant moi et je me dis :
“C'est fou quand même, on dirait de vrais petits machins qu'on pourrait observer avec un microscope.” Et puis je m'éloigne un peu de mon écran et je me rends compte, à ce moment-là,
que ce qui me fascine le plus dans la vie artificielle, en fait, c'est la Vie.
Et je cherche pas à faire le malin. Mais je vous le dis honnêtement comme ça m'est arrivé, l'étude de la vie artificielle m'a fait prendre,
ou plutôt reprendre conscience de la beauté et de la complexité des choses qui nous composent et nous entourent. Ça peut paraître étrange d'avoir besoin de passer par l'étude d'ersatz de vie
pour se rendre compte de la beauté de la vie. Mais, c'est comme ça que le cerveau humain fonctionne. On s'habitue, voilà. Il nous faut quelques mois, quelques jours, quelques heures
pour prendre les merveilles qui nous entourent pour acquises, pour les considérer comme des banalités. Alors si on peut changer de filtre, il faut pas hésiter à le faire.
Et c'est ce que je vous propose, de m'accompagner dans le superamas du Laniakea, puis dans le groupe local,
puis dans la Voie lactée et enfin dans le système solaire dans lequel on trouve, réchauffée par une étoile jaune composée à 75 % d'hydrogène,
une petite planète bleue qui, il y a 3 à 4 milliards d'années, a vu naître dans ses océans de petites créatures qu'on aurait pu,
si on avait été présent à ce moment-là, observer de la même manière qu'on a observé le Jeu de la Vie en se demandant ce que de si petites choses pouvaient bien faire,
en quoi elles allaient bien pouvoir se transformer avec assez d'espace et de temps, on aurait pu les approcher comme on approcherait une petite simulation,
avec la même curiosité qu'on a développée pour les petites créatures de Conway, on aurait pu regarder ces petits bouts de vie en nous laissant emporter par les possibilités
et en nous demandant ce que ça allait bien pouvoir donner. L'avantage pour nous, c'est qu'on n'a pas besoin de se demander.
