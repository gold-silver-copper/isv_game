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
            
            game_map: GameMap{voxeltile_grid:gm},

            world_seed: rngik.clone(),

        
        }
    }
}

pub fn setup(mut commands: Commands) {
    // create a new entity
    commands.spawn((
        // Initialize all your components and bundles here
        Player,
        PointComponent((5,5)),
        GraphicComponent((String::from("@"),RatColor::White,RatColor::Black)), // ...
    ));



    for boop in 1..2000000 {
        commands.spawn((
            // Initialize all your components and bundles here
           
            PointComponent((boop*2,boop*3)),
            GraphicComponent((String::from("@"),RatColor::White,RatColor::Black)), // ...
        ));


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
