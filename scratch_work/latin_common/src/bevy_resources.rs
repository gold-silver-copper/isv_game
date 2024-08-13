use crate::*;
// Create resource to hold the ratatui terminal
#[derive(Resource)]
pub struct BevyTerminal<RataguiBackend: ratatui::backend::Backend> {
    pub terminal_game: Terminal<RataguiBackend>,
    pub terminal_info: Terminal<RataguiBackend>,
}

// Implement default on the resource to initialize it
impl Default for BevyTerminal<RataguiBackend> {
    fn default() -> Self {
        let mut backend1 = RataguiBackend::new(20, 20);
        backend1.set_font_size(20);
        let mut terminal1 = Terminal::new(backend1).unwrap();

        let mut backend2 = RataguiBackend::new(20, 20);
        backend2.set_font_size(12);
        let mut terminal2 = Terminal::new(backend2).unwrap();

        BevyTerminal {
            terminal_game: terminal1,
            terminal_info: terminal2,
        }
    }
}

#[derive(Clone, Debug, Resource)]
pub struct Masterok {
    pub small_rngik: SmallRng,

    pub game_map: GameMap,

    pub world_seed: u32,
}

impl Default for Masterok {
    fn default() -> Self {
        let rngik: u32 = 87243563;

        let gm = GameMap::generate_test(100);

        Self {
            small_rngik: SmallRng::seed_from_u64(rngik as u64),

            game_map: GameMap { voxeltile_grid: gm },

            world_seed: rngik.clone(),
        }
    }
}

#[derive(Clone, Debug, Resource)]
pub struct UIResources {
    pub visible_ents: Vec<(Entity, EntityType)>,
    pub latin_conjugator: Latin,
}

impl Default for UIResources {
    fn default() -> Self {
        Self {
            visible_ents: Vec::new(),
            latin_conjugator: Latin::new(
                "nouns.csv".into(),
                "adjectives.csv".into(),
                "verbs.csv".into(),
            ),
        }
    }
}

pub fn spawn_with_point_and_type<T: Bundle>(
    mut commands: &mut Commands,
    point: MyPoint,
    ent_type: EntityType,
    bundle: T,
    game_map: &mut GameMap,
) -> Entity {
    let mut e = commands.spawn((
        // Initialize all your components and bundles here
        PointComponent(point.clone()),
        ent_type.clone(),
    ));
    e.insert(bundle);
    let eid = e.id();

    let vox = game_map.get_mut_voxel_at(&point);

    match vox {
        Some(voxik) => voxik.entity_map.insert(eid.clone(), ent_type),
        None => panic!(
            "ATTEMPTING TO SPAWN ENTITY IN NON VOXEL AT LOC: {:#?}",
            point
        ),
    };

    eid
}

pub fn setup(mut commands: Commands, mut masterok: ResMut<Masterok>) {
    // create a new entity

    spawn_with_point_and_type(
        &mut commands,
        (5, 5),
        EntityType::Animalia,
        (Player),
        &mut masterok.game_map,
    );

    for boop in 1..20 {
        spawn_with_point_and_type(
            &mut commands,
            (boop * 2, boop * 3),
            EntityType::Animalia,
            (),
            &mut masterok.game_map,
        );
    }
}

pub fn set_custom_font(mut contexts: EguiContexts) {
    let mut fonts = FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters):
    fonts.font_data.insert(
        "my_font".to_owned(),
        FontData::from_static(include_bytes!("../../assets/fonts/hiero_mono.ttf")),
    ); // .ttf and .otf supported

    // Put my font first (highest priority):
    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .insert(0, "my_font".to_owned());

    contexts.ctx_mut().set_fonts(fonts);
}
