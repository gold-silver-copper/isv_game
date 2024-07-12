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

//MAKE IT POSSIBLE TO SET PLAYER AI MODE FOR AUTO PLAYING
#[derive(Clone, Debug, PartialEq, Component)]
pub enum ActionComponent {
    Wait,
    Take(),
    MeleeAttack(),
    Drop(),
    Give(),
    Hit(),
    Go(CardinalDirection),
    Quit,
}

pub fn action_processor(mut player_position: Query<(&mut GamePosition, &ActionComponent)>) {
    for (mut e_pos, e_action) in player_position.iter_mut() {
        match e_action {
            ActionComponent::Go(direction) => match direction {
                CardinalDirection::North => {
                    e_pos.y += 1;
                }
                CardinalDirection::South => {
                    e_pos.y -= 1;
                }
                CardinalDirection::West => {
                    e_pos.x -= 1;
                }
                CardinalDirection::East => {
                    e_pos.x += 1;
                }
            },
            _ => todo!("ACTION NOT IMPLEMENTED"),
        }
    }
}

pub fn action_remover(player_position: Query<(Entity, &ActionComponent)>, mut commands: Commands) {
    for (eid, _) in player_position.iter() {
        commands.entity(eid).remove::<ActionComponent>();
    }
}
