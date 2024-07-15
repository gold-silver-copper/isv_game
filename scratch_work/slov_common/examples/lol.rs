use slov_common::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<BevyTerminal<RataguiBackend>>()
        //Initialize the ratatui terminal
        .init_resource::<Masterok>()
        .init_resource::<UIResources>()
        .add_plugins(EguiPlugin)
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Update, ui_example_system)
        .add_systems(PreUpdate, draw_ascii_game)
        .add_systems(PreUpdate, draw_ascii_info)
        .add_systems(PreUpdate, keyboard_input_system)
        .add_systems(Update, action_processor)
        .add_systems(PostUpdate, action_remover)
        .add_systems(Startup, setup)
        .add_systems(PostStartup, set_custom_font)
        .run();
}
