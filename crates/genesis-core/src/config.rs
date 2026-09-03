//! Configuration de simulation.
//!
//! Tranchee 12 : des chiffres de depart, un seul fichier. Les valeurs par defaut ici
//! sont exactement celles de `BIBLE/genesis.starter.toml`. On peut les surcharger avec
//! un fichier `.toml`.

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SimConfig {
    pub world: WorldCfg,
    pub planet: PlanetCfg,
    pub season: SeasonCfg,
    pub time: TimeCfg,
    pub resources: ResourcesCfg,
    pub bricks: BricksCfg,
    pub environment: EnvironmentCfg,
    pub metabolism: MetabolismCfg,
    pub biology: BiologyCfg,
    pub lifecycle: LifecycleCfg,
    pub reproduction: ReproductionCfg,
    pub cohesion: CohesionCfg,
    pub cells: CellsCfg,
    pub cognition: CognitionCfg,
    pub voice: VoiceCfg,
    pub watch: WatchCfg,
    pub view: ViewCfg,
    pub persistence: PersistenceCfg,
    pub events: EventsCfg,
}

impl Default for SimConfig {
    fn default() -> Self {
        SimConfig {
            world: WorldCfg::default(),
            planet: PlanetCfg::default(),
            season: SeasonCfg::default(),
            time: TimeCfg::default(),
            resources: ResourcesCfg::default(),
            bricks: BricksCfg::default(),
            environment: EnvironmentCfg::default(),
            metabolism: MetabolismCfg::default(),
            biology: BiologyCfg::default(),
            lifecycle: LifecycleCfg::default(),
            reproduction: ReproductionCfg::default(),
            cohesion: CohesionCfg::default(),
            cells: CellsCfg::default(),
            cognition: CognitionCfg::default(),
            voice: VoiceCfg::default(),
            watch: WatchCfg::default(),
            view: ViewCfg::default(),
            persistence: PersistenceCfg::default(),
            events: EventsCfg::default(),
        }
    }
}

impl SimConfig {
    pub fn load(path: &Path) -> std::io::Result<Self> {
        let text = std::fs::read_to_string(path)?;
        toml::from_str(&text)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
    }

    pub fn to_toml(&self) -> String {
        toml::to_string(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct WorldCfg {
    pub grid_width: u32,
    pub grid_height: u32,
    pub bounded: bool,
}
impl Default for WorldCfg {
    fn default() -> Self {
        WorldCfg { grid_width: 192, grid_height: 192, bounded: true }
    }
}

/// Constantes du monde, fixees a la creation. En 0.0.1 elles sont seulement affichees :
/// on sait dans quel environnement on se trouve. Les jalons suivants les feront moduler
/// des coefficients (metabolisme selon la temperature, cout de deplacement selon la
/// gravite, efficacite selon la pression et le milieu).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PlanetCfg {
    /// Temperature moyenne de l'environnement, en degres Celsius. Constante sur la vie du
    /// monde. Un monde loin de `temp_optimal_c` coute plus cher a habiter (voir
    /// `temp_metab_slope`) : plus de morts de faim, selection plus dure sur l'efficacite.
    pub temperature_c: f32,
    /// Temperature a laquelle le metabolisme est le moins cher.
    pub temp_optimal_c: f32,
    /// Surcout metabolique par degre d'ecart a l'optimum, en fraction. `0` = la temperature
    /// est inerte (bouton d'A/B). A 0,012 et 12 degres d'ecart : +14 % de depense de base.
    pub temp_metab_slope: f32,
    /// Amplitude thermique du genome (schema v18) : l'optimum metabolique d'une entite va de
    /// `temp_optimal_c - span/2` (trait `heat_tol = 0`, adaptee au froid) a `+ span/2`
    /// (`heat_tol = 1`, adaptee au chaud), en degres Celsius. `0` = `heat_tol` inerte, tout le
    /// monde partage l'optimum du monde (bouton d'A/B).
    pub heat_tol_span_c: f32,
    /// Milieu dans lequel baigne la vie : "eau", "acide", "air", ...
    pub medium: String,
    /// Gravite, en multiples de celle de la Terre. Multiplie le cout du deplacement : un
    /// monde lourd favorise les corps lents et economes, un monde leger l'inverse.
    pub gravity: f32,
    /// Pression, en atmospheres. Affichee, sans effet mecanise pour l'instant.
    pub pressure_atm: f32,
}
impl Default for PlanetCfg {
    fn default() -> Self {
        PlanetCfg {
            temperature_c: 15.0,
            temp_optimal_c: 15.0,
            temp_metab_slope: 0.012,
            heat_tol_span_c: 16.0,
            medium: "eau".to_string(),
            gravity: 1.0,
            pressure_atm: 1.0,
        }
    }
}

/// Les saisons (0.0.4, experiments/011). Le milieu n'est plus fige : la capacite nourriciere
/// de chaque case (son plafond et sa vitesse de regeneration) oscille lentement autour de sa
/// base, une sinusoide pure du tick (donc deterministe, et qui reprend juste apres un
/// rechargement). Une saison d'abondance laisse la population deborder, une saison maigre la
/// rabote d'un bon tiers : le monde respire au lieu d'etre epingle au plafond, et les goulots
/// de disette re-brassent le centre genetique de la population. `amplitude = 0` : milieu fige,
/// aucun effet, byte-identique a avant (bouton d'A/B). Pour que la saison morde il faut que la
/// nourriture, pas la matiere structurelle, soit le frein qui compte : voir `BricksCfg`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SeasonCfg {
    /// Amplitude de l'oscillation nourriciere, en fraction de la base. A 0,5 : la capacite des
    /// cases va de 0,5x a 1,5x au fil de l'annee. `0` = pas de saison nourriciere.
    pub amplitude: f32,
    /// Amplitude thermique de la saison, en degres Celsius : la temperature effective du monde
    /// va de `temperature_c - temp_amplitude_c` (plein hiver) a `+ temp_amplitude_c` (plein
    /// ete). Couplee au trait `heat_tol`, elle fait alterner la selection. `0` = pas de saison
    /// thermique.
    pub temp_amplitude_c: f32,
    /// Duree d'un cycle complet (une abondance + une disette), en annees-monde. Vaut pour les
    /// deux composantes, nourriciere et thermique.
    pub period_years: f32,
    /// Plancher : meme au creux, `season_factor` ne descend pas sous cette fraction de la
    /// base. Evite l'effondrement total deterministe d'un monde a forte amplitude.
    pub regen_floor: f32,
}
impl Default for SeasonCfg {
    fn default() -> Self {
        SeasonCfg { amplitude: 0.5, temp_amplitude_c: 5.0, period_years: 1.6, regen_floor: 0.15 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TimeCfg {
    pub tick_duration_seconds: u64,
    pub target_ticks_per_real_second: f32,
}
impl Default for TimeCfg {
    fn default() -> Self {
        TimeCfg { tick_duration_seconds: 3600, target_ticks_per_real_second: 60.0 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ResourcesCfg {
    pub regen_rate: f32,
    pub max_per_cell: f32,
    pub initial_fill: f32,
    /// La regeneration des cases ne tourne qu'un tick sur N (avec un taux multiplie par N).
    /// Le milieu change lentement, ca economise le balayage de la grille sans effet visible.
    pub regen_every: u32,
}
impl Default for ResourcesCfg {
    fn default() -> Self {
        ResourcesCfg { regen_rate: 0.015, max_per_cell: 10.0, initial_fill: 0.5, regen_every: 4 }
    }
}

/// Briques elementaires (0.0.2) : la matiere structurelle dont sont faits les corps. Un
/// monde en contient une quantite finie. Un corps vivant en immobilise `body_matter` ; le
/// reste est le stock libre (`WorldState.free_matter`). Une division prend `body_matter` du
/// stock libre pour batir l'enfant ; si le stock est a sec, la division echoue et le parent
/// patiente. La mort rend `body_matter` au stock. La somme est conservee exactement :
/// `free_matter + population * body_matter = matter_per_cell * nombre de cases`.
///
/// C'est le vrai frein de capacite : la population plafonne autour de
/// `matter_per_cell * cases / body_matter`, avec une oscillation (au plafond les divisions
/// calent, des morts liberent de la matiere, ca repart). Distinct de l'energie (le
/// carburant) et de la surexploitation (la fatigue du sol). Non spatial en 0.0.2 : une
/// geographie de la matiere viendra avec les biomes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BricksCfg {
    /// Matiere totale du monde = ce nombre * nombre de cases de la grille. Fixe la capacite
    /// de charge : environ `matter_per_cell * cases / body_matter` individus.
    pub matter_per_cell: f32,
    /// Matiere immobilisee par un corps vivant, liberee a la mort.
    pub body_matter: f32,
    /// Zone tampon, en fraction de la matiere totale du monde, sous laquelle la division
    /// devient probabiliste (V2). Tant que la matiere libre depasse ce coussin, une division
    /// reussit toujours ; en dessous, sa chance decroit lineairement jusqu'a 0 quand il ne
    /// reste la matiere que d'un seul corps. Adoucit le plateau : au lieu d'un plafond dur,
    /// la population respire dans cette bande. Invariant d'echelle (fraction, pas un nombre).
    pub comfort_frac: f32,
    /// Ticks de patience apres un echec de division faute de matiere, pour ne pas re-tenter
    /// a chaque tick (fraction de la gestation). Un echec probabiliste (matiere juste
    /// tendue) patiente deux fois moins : la matiere se libere vite au plateau.
    pub retry_frac: f32,
}
impl Default for BricksCfg {
    fn default() -> Self {
        BricksCfg {
            matter_per_cell: 0.26,
            body_matter: 1.0,
            comfort_frac: 0.06,
            retry_frac: 0.4,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct EnvironmentCfg {
    /// Matiere rendue a la case a la mort d'une entite : une part fixe (le corps) plus
    /// une part de l'energie restante. Ferme la boucle de decomposition.
    pub corpse_nutrients: f32,
    pub corpse_energy_return: f32,
    /// Surexploitation : chaque unite recoltee ajoute cette fraction de tension a la case.
    pub strain_per_harvest: f32,
    /// La tension decroit de ce facteur par tick. Une case surexploitee regenere lentement
    /// tant qu'elle n'a pas recupere. C'est par la qu'un boom degrade son propre milieu.
    pub strain_decay: f32,
}
impl Default for EnvironmentCfg {
    fn default() -> Self {
        EnvironmentCfg {
            corpse_nutrients: 1.5,
            corpse_energy_return: 0.5,
            strain_per_harvest: 0.06,
            strain_decay: 0.997,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct MetabolismCfg {
    pub base_burn: f32,
    pub move_cost: f32,
    pub eat_rate: f32,
}
impl Default for MetabolismCfg {
    fn default() -> Self {
        MetabolismCfg { base_burn: 0.05, move_cost: 0.02, eat_rate: 2.0 }
    }
}

/// Biologie de fond (0.0.3, tranche 8). Un scalaire `Entity.health` dans [0, 1] consolide la
/// condition biologique : il integre lentement les famines repetees et la vieillesse, au lieu
/// que le pipeline rejuge l'energie brute et l'age a chaque tick. La sante remonte quand
/// l'entite est rassasiee et pas trop vieille. Deux effets sur le monde : un corps use se
/// deplace moins vite et meurt plus tot de vieillesse. C'est la marche « Individu » de
/// l'escalier des echelles : la biologie recule au rang d'etat de fond, la cognition passe
/// devant. Deferre a l'etape 2 (0.0.6) : le bilan molecule qui remplace la simulation
/// membre a membre d'une cellule.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BiologyCfg {
    /// Fraction de l'esperance de vie sous laquelle un corps reste a pleine sante.
    pub wear_start: f32,
    /// Sante cible d'un corps a 1.5x son esperance de vie (usure de la vieillesse).
    pub wear_floor: f32,
    /// Fraction de l'ecart a la cible corrigee par tick quand l'entite est rassasiee.
    pub heal_rate: f32,
    /// Idem quand l'energie est sous le seuil de peril (cible 0) : plus rapide, la famine
    /// laisse une marque qui met du temps a s'effacer.
    pub damage_rate: f32,
    /// `max_step *= frail_slow + (1 - frail_slow) * health`. `1.0` = pas d'effet sur le
    /// mouvement (pour l'A/B).
    pub frail_slow: f32,
    /// `p_death_age *= 1 + wear_death_boost * (1 - health)`. `0` = pas d'effet sur la mort
    /// par age (pour l'A/B).
    pub wear_death_boost: f32,
}
impl Default for BiologyCfg {
    fn default() -> Self {
        BiologyCfg {
            wear_start: 0.6,
            wear_floor: 0.15,
            heal_rate: 0.015,
            damage_rate: 0.02,
            frail_slow: 0.5,
            wear_death_boost: 1.5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LifecycleCfg {
    pub starve_at: f32,
    pub lifespan_ticks_mean: u64,
    pub age_death_curve: f32,
}
impl Default for LifecycleCfg {
    fn default() -> Self {
        LifecycleCfg { starve_at: 0.0, lifespan_ticks_mean: 20000, age_death_curve: 4.0 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ReproductionCfg {
    /// "asexual" en 0.0.1 (scission). "sexual" et l'emergence du sexue viennent a 0.0.2+.
    pub mode: String,
    /// Energie minimale pour se scinder.
    pub energy_threshold: f32,
    /// Surcout metabolique de la replication, retire avant le partage en deux.
    pub energy_cost: f32,
    /// Rayon de recherche d'un partenaire. Inutilise en 0.0.1 (asexue), garde pour 0.0.2.
    pub partner_radius: f32,
    pub mutation_rate: f32,
    pub mutation_scale: f32,

    /// Probabilite qu'une mutation soit letale a la division : pas d'enfant viable.
    /// C'est le prix de la duplication imparfaite, cote genome.
    pub lethal_mutation_rate: f32,

    /// Temps de gestation de reference, en ticks. Apres une division, une entite ne peut
    /// pas se rediviser pendant `gestation_ticks_base * (1.5 - fertilite)` ticks.
    /// La fertilite est un trait du genome : le rythme de replication varie selon l'espece.
    pub gestation_ticks_base: u32,

    /// Perte a la naissance d'origine environnementale, dans un etat sans infrastructure.
    /// Probabilite qu'une division par ailleurs viable ne donne pas d'enfant. Les jalons
    /// suivants la font baisser quand une civilisation developpe des infrastructures.
    pub birth_loss_base: f32,

    /// Nombre d'entites en surplus sur une case pour que l'echec de division devienne
    /// quasi certain. Petit = frein de capacite serre. C'est ce qui empeche le bloom.
    pub crowding_half: f32,

    /// Maturite : une entite ne peut pas se diviser avant d'avoir vecu cette fraction de
    /// son esperance de vie. Un juvenile ne se reproduit pas. Ralentit fortement la
    /// croissance exponentielle sans plafond artificiel. En 0.0.1 on garde des mondes de
    /// l'ordre de la centaine (jalon "Deux", stade molecule) ; les jalons suivants
    /// relachent quand de vraies couches de ressources arrivent.
    pub maturity_frac: f32,
}
impl Default for ReproductionCfg {
    fn default() -> Self {
        ReproductionCfg {
            mode: "asexual".to_string(),
            energy_threshold: 8.0,
            energy_cost: 1.5,
            partner_radius: 2.0,
            mutation_rate: 0.05,
            mutation_scale: 0.1,
            lethal_mutation_rate: 0.06,
            gestation_ticks_base: 700,
            birth_loss_base: 0.30,
            crowding_half: 1.8,
            maturity_frac: 0.05,
        }
    }
}

/// Agregation : les entites proches et genetiquement voisines s'attirent et forment des
/// colonies. Une colonie protege la reproduction (l'agregat sert d'infrastructure), la
/// surpopulation d'une case la penalise toujours : l'optimum est une densite moyenne.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CohesionCfg {
    /// Portee de l'attraction, en cases.
    pub radius: f32,
    /// Poids maximal du melange vers la cible de cohesion, pour un trait `cohesion` de 1.
    pub pull_max: f32,
    /// Distance L1 de genome au dela de laquelle deux entites ne s'attirent plus.
    pub similarity_scale: f32,
    /// Plafond du support de colonie ressenti par une entite.
    pub support_cap: f32,
    /// Baisse du plancher `birth_loss` par unite de support de colonie.
    pub support_birth_relief: f32,
    /// Plancher de cohesion quand l'entite a faim (0 = une entite affamee ignore la
    /// cohesion et ne pense qu'a manger).
    pub hunger_damp: f32,

    // -- Retenue sur les communs (experience 003, V1). L'agregation reste passive
    //    (`pull_max` a 0). Ici `cohesion` decrit comment une entite deja dans un groupe de
    //    parents traite la case partagee : elle mange un peu moins (cout prive) et la
    //    fatigue beaucoup moins (bien commun). Actif des que l'un des deux est > 0.
    /// Part d'intake en moins pour un cooperateur pleinement entoure de parents.
    pub eat_restraint: f32,
    /// Part de surexploitation en moins dans la meme situation.
    pub strain_restraint: f32,
}
impl Default for CohesionCfg {
    fn default() -> Self {
        CohesionCfg {
            radius: 4.0,
            // Force de mouvement desactivee : elle destabilise l'ecosysteme (voir
            // experience 003). L'agregation reste passive (les entites convergent sur la
            // nourriture). Le trait `cohesion` agit via la retenue ci-dessous.
            pull_max: 0.0,
            similarity_scale: 2.0,
            support_cap: 4.0,
            support_birth_relief: 0.0,
            hunger_damp: 0.0,
            eat_restraint: 0.25,
            strain_restraint: 0.7,
        }
    }
}

/// Cellules (0.0.2, tranche 2, etape 1 « membrane »). Un amas d'entites proches,
/// genetiquement parentes, coherentes et persistant devient une `Cell` : une unite
/// reconnue. Les membres restent simules (l'etape 2 les de-simulera). Etre en cellule
/// apporte : le partage d'energie (chaque membre tend vers la moyenne du groupe, un tampon
/// contre la famine locale) et une reproduction protegee (`cell_birth_relief`). C'est ce
/// qui rend le trait `cohesion` vraiment adaptatif. Bascule reversible : une cellule qui se
/// disperse ou perd ses membres se dissout (T-7).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CellsCfg {
    /// Detection tous les N ticks (comme les veilleurs). L'entretien par tick est separe.
    pub check_every: u64,
    /// Deux entites se lient si a moins de N cases l'une de l'autre.
    pub link_dist: f32,
    /// ... et si leur distance L1 de genome est sous ce seuil (parentes).
    pub kin_dist: f32,
    /// Cohesion moyenne minimale du groupe pour qu'il forme une cellule.
    pub min_cohesion: f32,
    /// Taille minimale d'un amas pour former une cellule.
    pub min_members: u32,
    /// Rayon moyen maximal des membres autour du centre pour former une cellule.
    pub max_spread: f32,
    /// Hysteresis (V2) : une cellule deja formee ne se dissout qu'en dessous de cet effectif,
    /// pas des `min_members`. Plus bas = cellules plus tenaces.
    pub dissolve_members: u32,
    /// Hysteresis (V2) : ... et seulement au dela de ce rayon, pas des `max_spread`.
    pub dissolve_spread: f32,
    /// Delai de grace (V2) : une cellule fraiche ne peut pas se dissoudre avant cet age,
    /// sauf si elle tombe a zero membre. Laisse le temps a un amas de se stabiliser.
    pub grace_ticks: u64,
    /// Nombre de controles consecutifs ou le candidat doit tenir avant de former une cellule.
    pub persist_checks: u16,
    /// Un membre a plus de `rayon * ce facteur` du centre quitte la cellule.
    pub leave_factor: f32,
    /// Fraction de l'ecart a la moyenne d'energie de la cellule corrigee par tick (partage).
    pub energy_share: f32,
    /// Part d'echec de division en moins pour un membre de cellule (reproduction protegee).
    pub cell_birth_relief: f32,
    /// Fraction de la depense energetique de base en moins pour un membre de cellule EN DANGER
    /// de famine (0.0.2, tranche 2b) : membrane partagee, metabolisme mutualise. Le repli est
    /// proportionnel a l'enfoncement dans la zone de peril (`energy < peril_frac * seuil`) :
    /// nul pour un membre bien nourri, plein pour un membre qui frole la mort. Il amortit donc
    /// la disette sans distordre l'equilibre des temps gras ou vit la diversite genetique.
    /// C'est l'avantage de survie du pluricellulaire (fait tenir les lignees de cellules quand
    /// un monde sature). Bornee. `0` desactive (bouton d'A/B).
    pub cell_burn_relief: f32,

    /// Fusion : deux cellules stables dont les membranes se chevauchent et dont les genomes
    /// moyens se ressemblent n'en font plus qu'une. La plus grosse garde son identite et son
    /// histoire ; la petite est absorbee. `false` desactive (bouton d'A/B).
    pub fuse: bool,
    /// Chevauchement exige : `distance(centres) < (r1 + r2) * fuse_overlap`. Plus bas = il
    /// faut vraiment que les membranes s'interpenetrent.
    pub fuse_overlap: f32,
    /// ... et distance L1 des genomes moyens des deux cellules sous ce seuil. Plus lache que
    /// `kin_dist` : ce sont deja des groupes organises, la parente se juge au niveau cellule.
    pub fuse_kin: f32,

    /// Division (0.0.4, schema v19) : une cellule grande, mure et etiree se pince en deux.
    /// `false` desactive (bouton d'A/B). C'est la reproduction cellulaire : les cellules
    /// deviennent des unites qui naissent, grandissent et se divisent, la selection agit a
    /// leur niveau.
    pub divide: bool,
    /// Effectif minimal pour se diviser. Doit etre bien au-dessus de `min_members` : une
    /// cellule ne se divise que quand elle a de quoi faire deux cellules viables.
    pub divide_members: u32,
    /// Allongement minimal (`Cell.elongation`, etalement axe long / axe court) pour se
    /// diviser. Une cellule ronde ne se pince pas ; une cellule etiree par la chimiotaxie
    /// vers deux zones riches, oui.
    pub divide_elongation: f32,
    /// Age minimal avant la premiere division, en ticks. Laisse la cellule s'etablir.
    pub divide_age_ticks: u64,

    /// Repulsion (schema v19) : deux cellules qui se chevauchent mais dont les genomes sont
    /// trop distants pour fusionner (`> fuse_kin`) se repoussent doucement, leurs membres
    /// glissent a l'oppose de l'autre centre. La membrane devient une frontiere : des cellules
    /// non parentes ne se traversent plus, elles se cotoient. `false` desactive (A/B).
    pub repel: bool,
    /// Force de la repulsion : fraction du chevauchement corrigee par tick, par membre.
    /// Bornee, tres douce pour ne pas casser la chimiotaxie ni la cohesion.
    pub repel_strength: f32,
}
impl Default for CellsCfg {
    fn default() -> Self {
        CellsCfg {
            check_every: 200,
            link_dist: 2.0,
            kin_dist: 0.7,
            min_cohesion: 0.45,
            min_members: 12,
            max_spread: 6.0,
            dissolve_members: 6,
            dissolve_spread: 11.0,
            grace_ticks: 600,
            persist_checks: 4,
            leave_factor: 1.9,
            energy_share: 0.15,
            cell_birth_relief: 0.4,
            cell_burn_relief: 0.5,
            fuse: true,
            fuse_overlap: 0.5,
            fuse_kin: 0.9,
            divide: true,
            divide_members: 42,
            divide_elongation: 1.9,
            divide_age_ticks: 4000,
            repel: true,
            repel_strength: 0.06,
        }
    }
}

/// Cognition (0.0.3, tranche 1 « le premier souvenir »). Une entite qui percoit assez bien
/// (`perception_min`), a vecu assez longtemps (`age_min_frac`) et vient de subir un choc
/// s'eveille en agent : elle gagne une memoire episodique spatiale qui biaise son
/// deplacement. Reversible : un agent sans souvenir depuis `lapse_ticks` retombe. Substrat
/// seme, pas de personnalite heritee ni de besoins pour l'instant (voir `05_COGNITION.md`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CognitionCfg {
    /// Perception minimale pour s'eveiller. Au-dessus du milieu de la plage de depart
    /// (0.35..0.65) : seuls certains individus, et la selection peut pousser le trait si
    /// etre agent aide a survivre.
    pub perception_min: f32,
    /// Fraction de l'esperance de vie a avoir vecu avant de pouvoir s'eveiller : un juvenile
    /// n'a pas d'histoire.
    pub age_min_frac: f32,
    /// Energie sous `peril_frac * energy_threshold` : choc de peril (lieu a eviter).
    pub peril_frac: f32,
    /// Gain d'energie en un tick au-dessus de ce seuil : choc d'aubaine (lieu a retrouver).
    pub bounty_abs: f32,
    /// Intervalle minimal, en ticks, entre deux chocs enregistres pour une meme entite.
    pub shock_interval: u64,
    /// Un agent fraichement eveille ne peut pas retomber avant cet age (comme les cellules).
    pub grace_ticks: u64,
    /// Un agent sans aucun souvenir depuis ce delai retombe entite de fond.
    pub lapse_ticks: u64,
    /// Facteur de decroissance de la force d'un souvenir, par tick.
    pub memory_decay: f32,
    /// Un souvenir sous cette force est oublie.
    pub memory_eps: f32,
    /// Nombre maximal de souvenirs episodiques (le plus faible cede la place).
    pub max_memories: u32,
    /// Deux souvenirs de meme nature a moins de `memory_merge_dist` cases fusionnent.
    pub memory_merge_dist: f32,
    /// Poids maximal du biais memoire dans la cible de deplacement d'un agent.
    pub mem_weight: f32,
    /// Portee spatiale du noyau de souvenir, en cases.
    pub mem_radius: f32,
    /// Un agent a moins de N cases d'une mort la voit et en garde un souvenir ancre
    /// (0.0.3, tranche 3). `0` desactive.
    pub witness_radius: f32,
    /// Ne retenir que la mort d'un membre de sa propre lignee fondatrice (« un des siens »).
    pub witness_kin_only: bool,

    // -- Besoins (0.0.3, tranche 4). Jauges internes qui ponderent le comportement.
    /// Facteur de relachement de la faim par tick (elle suit vite la baisse d'energie,
    /// redescend lentement).
    pub hunger_relief: f32,
    /// Facteur de relachement de la peur par tick.
    pub fear_relief: f32,
    /// Portee, en cases, du noyau de menace autour d'un souvenir aversif (pour la peur).
    pub fear_radius: f32,
    /// Un choc de peril de moins de N ticks maintient la peur au maximum.
    pub fear_shock_window: u64,
    /// De combien la peur amplifie l'evitement des souvenirs aversifs.
    pub fear_gain: f32,
    /// Poids maximal du glissement vers le centre des siens quand l'agent est isole.
    pub social_pull: f32,
    /// Bouton maitre des besoins : met a l'echelle leurs effets sur le comportement.
    /// `0` = comportement de la tranche 3 (pour comparer).
    pub needs_weight: f32,
    /// Personnalite (0.0.3, tranche 5). `true` : `caution` et `curiosity` sont des traits
    /// du genome, herites et selectionnes. `false` : ils sont derives de `lifespan` et
    /// `perception` (comportement des tranches 1 a 4). Le genome porte les deux traits dans
    /// les deux cas : le flux RNG est identique, l'A/B est propre.
    pub heritable_personality: bool,

    // -- Souvenirs sociaux (0.0.3, tranche 7). Un agent reconnait les autres agents proches.
    /// Deux agents a moins de N cases se « voient » (renforcent leur relation).
    pub social_radius: f32,
    /// Controle de proximite (couteux) tous les N ticks seulement.
    pub social_check_every: u64,
    /// Gain de familiarite par controle ou l'autre agent est proche.
    pub social_fam_gain: f32,
    /// Relachement de la familiarite par controle.
    pub social_decay: f32,
    /// Une relation sous cette familiarite est oubliee.
    pub social_eps: f32,
    /// Quand un agent isole glisse vers les siens, part du glissement qui vise son ami le
    /// plus proche plutot que le centre de masse. `0` = comportement de la tranche 6.
    pub friend_pull: f32,
}
impl Default for CognitionCfg {
    fn default() -> Self {
        CognitionCfg {
            perception_min: 0.62,
            age_min_frac: 0.15,
            peril_frac: 0.18,
            bounty_abs: 3.0,
            shock_interval: 150,
            grace_ticks: 800,
            lapse_ticks: 2500,
            memory_decay: 0.9985,
            memory_eps: 0.05,
            max_memories: 12,
            memory_merge_dist: 3.0,
            mem_weight: 0.5,
            mem_radius: 7.0,
            witness_radius: 4.0,
            witness_kin_only: true,
            hunger_relief: 0.996,
            fear_relief: 0.99,
            fear_radius: 6.0,
            fear_shock_window: 200,
            fear_gain: 1.2,
            social_pull: 0.3,
            needs_weight: 1.0,
            heritable_personality: true,
            social_radius: 4.0,
            social_check_every: 8,
            social_fam_gain: 0.08,
            social_decay: 0.985,
            social_eps: 0.03,
            friend_pull: 0.5,
        }
    }
}

/// La Voix (jalon 0.0.4, tranche 1). Un agent qui subit le choc d'une famine emet une
/// alarme a sa position. Les agents a moins de `signal_radius` la percoivent : leur peur
/// monte au moins a `alarm_fear`, sans qu'aucun souvenir ne se forme (contagion breve, la
/// peur redescend ensuite via `fear_relief`). Un signal vit `signal_ttl` ticks. Aucun
/// lexique : c'est le premier etage vers le langage emergent (`06_EMERGENCE.md`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct VoiceCfg {
    /// Portee de perception d'un signal, en cases.
    pub signal_radius: f32,
    /// Duree de vie d'un signal, en ticks.
    pub signal_ttl: u64,
    /// Plancher de peur impose a un agent qui percoit une alarme. `0` desactive l'effet
    /// (pour l'A/B) : les alarmes sont alors emises et visibles mais sans consequence.
    pub alarm_fear: f32,
    /// Nombre maximal de signaux vivants a la fois. En famine generale, on n'en garde que
    /// les plus recents : de quoi peindre la panique sans balayer des millions de paires.
    pub max_signals: usize,
    /// Tranche 2 : un agent qui trouve un repas exceptionnel lance un appel. `false`
    /// desactive (aucun appel emis, pour l'A/B).
    pub bounty_call: bool,
    /// A quel point un appel entendu inflechit la cible de deplacement de l'agent qui decide,
    /// vers la position de l'appel. `0` = signal visible mais sans effet.
    pub bounty_pull: f32,
    /// Fraction du plafond d'une case au-dessus de laquelle elle est « franchement riche » :
    /// un agent qui y mange bien lance un appel.
    pub bounty_cell_frac: f32,
}
impl Default for VoiceCfg {
    fn default() -> Self {
        VoiceCfg {
            signal_radius: 5.0,
            signal_ttl: 4,
            alarm_fear: 0.5,
            max_signals: 128,
            bounty_call: true,
            bounty_pull: 0.35,
            bounty_cell_frac: 0.55,
        }
    }
}

/// Les veilleurs (sim.rs phase 8b) : detecteurs mecanises qui produisent des evenements
/// saillants, le materiau des chapitres. Ils ne mutent que `WorldState.watch`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct WatchCfg {
    /// Un controle tous les N ticks.
    pub interval_ticks: u64,
    /// Effectif minimal d'un groupe de genome pour etre un candidat espece.
    pub species_min_size: u32,
    /// Distance L1 minimale, sur les 6 traits, entre un groupe et le stock dominant.
    pub species_min_distance: f32,
    /// Nombre de controles consecutifs ou le candidat doit tenir avant d'etre reconnu.
    pub species_persist_checks: u16,
    /// Rayon moyen maximal des membres autour de leur centroide pour qu'un groupe de genome
    /// compte comme espece : un groupe distinct mais disperse partout, c'est de la derive.
    pub species_max_spread: f32,
    /// Fraction de population perdue sur la fenetre pour parler d'effondrement.
    pub crash_drop_frac: f32,
    /// Longueur de la fenetre de detection d'effondrement, en controles.
    pub crash_window_checks: u16,
    /// Nombre de controles consecutifs ou une autre cle de genome doit dominer avant qu'on
    /// parle de basculement du genome dominant (`GenomeShift`). `0` desactive.
    pub genome_shift_persist_checks: u16,
}
impl Default for WatchCfg {
    fn default() -> Self {
        WatchCfg {
            interval_ticks: 200,
            species_min_size: 25,
            species_min_distance: 0.9,
            species_persist_checks: 3,
            species_max_spread: 18.0,
            crash_drop_frac: 0.5,
            crash_window_checks: 6,
            genome_shift_persist_checks: 5,
        }
    }
}

/// Le lecteur : niveau de detail de la projection (`genesis-view::project`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ViewCfg {
    /// Au dela de ce nombre d'entites, la frame envoie des amas au lieu d'individus. Le
    /// fichier reste petit quelle que soit la population.
    pub detail_max_entities: u32,
    /// Cote de la grille d'agregation en mode amas. `cluster_grid^2` amas au plus.
    pub cluster_grid: u32,
}
impl Default for ViewCfg {
    fn default() -> Self {
        ViewCfg { detail_max_entities: 500, cluster_grid: 30 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PersistenceCfg {
    pub snapshot_interval_ticks: u64,
    pub event_log_partition_ticks: u64,
    /// Une ligne de serie temporelle de stats (`series.jsonl`) tous les N ticks. Le
    /// materiau du graphe d'evolution genetique (`series.html`).
    pub series_every: u64,
    /// Journal en pyramide (`serve` seulement) : `events.jsonl` ne garde en detail complet
    /// que les `journal_keep_ticks` derniers ticks. Au-dela, seuls les evenements de chapitre
    /// (`EventKind::is_chapter` : genese, espece, extinction de lignee, effondrement, palier,
    /// fusion, basculement) roulent dans `events.chronicle.jsonl` (append-only, grossit tres
    /// lentement), le reste est laisse tomber. Le moteur ne relit jamais le journal, le monde
    /// reste identique ; c'est ce qui permet a un direct de tourner des mois.
    /// `journal_keep_ticks = 0` desactive la compaction (journal append-only sans fin).
    pub journal_keep_ticks: u64,
}
impl Default for PersistenceCfg {
    fn default() -> Self {
        PersistenceCfg {
            snapshot_interval_ticks: 5000,
            event_log_partition_ticks: 100000,
            series_every: 500,
            journal_keep_ticks: 120_000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct EventsCfg {
    pub max_events_per_tick: u32,
    pub max_cascade_depth: u16,
}
impl Default for EventsCfg {
    fn default() -> Self {
        EventsCfg { max_events_per_tick: 4096, max_cascade_depth: 12 }
    }
}
