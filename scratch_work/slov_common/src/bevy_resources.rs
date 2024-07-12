use crate::*;
// Create resource to hold the ratatui terminal
#[derive(Resource)]
pub struct BevyTerminal<RataguiBackend: ratatui::backend::Backend> {
   pub  terminal_game: Terminal<RataguiBackend>,
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

#[derive(Resource)]
pub struct Masterik {
    pub client_pos: GamePosition,

    pub messages: Vec<String>,
    pub client_world: MyWorld,
    pub is_logged_in: bool,
    pub button_entityid_map: HashMap<ItemKey, EntityID>,

    pub list_cursor_index: usize,
    pub targeted_ent_id: EntityID,
}

impl Masterik {
    pub fn refresh_menus(&mut self) {
        self.list_cursor_index = 0;
        self.targeted_ent_id = 0;
        self.button_entityid_map.drain();
    }
}

impl Default for Masterik {
  fn default() -> Self {
        Self {
            client_pos: GamePosition::new(),

            messages: Vec::new(),
            client_world: MyWorld::new_test(),
            is_logged_in: false,
            list_cursor_index: 0,
            targeted_ent_id: 0,
            button_entityid_map: HashMap::new(),
        }
    }
}

#[derive(PartialEq)]
pub enum MenuOpen {
    None,
    Take,
    Drop,
    Inventory,
    Stats,
    CursorInteract,
    PlayerLocationInteract,
    Attack,
}

#[derive(Resource)]
pub struct UIState {
   pub menu_open: MenuOpen,
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            menu_open: MenuOpen::None,
        }
    }
}

pub fn setup(mut commands: Commands) {
    // create a new entity
    commands.spawn((
        // Initialize all your components and bundles here
        Player,
        GamePosition { x: 5, y: 5 },
        GameRenderable::new_human(), // ...
    ));
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
