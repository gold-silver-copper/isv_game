use bevy_egui::{
    egui::{self, Frame},
    EguiContexts, EguiPlugin,
};
use bevy::{
   
    prelude::*,
};


use egui_ratatui::RataguiBackend;
use ratatui::{
    layout::Rect,
    prelude::{Line, Modifier, Span, Stylize, Terminal},
    text::Text,
    widgets::{Block, Borders, Paragraph, Wrap, *},
};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(EguiPlugin)
    // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
    // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
    .add_systems(Update, ui_example_system)
    .add_systems(Startup, setup)
    .init_resource::<Masterik>()

    .init_resource::<BevyTerminal<RataguiBackend>>()
    .run();
}


fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}

fn setup(
    mut commands: Commands,
 
    mut masterok: ResMut<Masterik>,
) {

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            ..default()
        }
      
    ));


}


//create resource to hold the ratatui terminal
#[derive(Resource)]
pub struct BevyTerminal<RataguiBackend: ratatui::backend::Backend> {
    pub terminal_info: Terminal<RataguiBackend>,
}

// Implement default on the resource to initialize it
impl Default for BevyTerminal<RataguiBackend> {
    fn default() -> Self {
        let mut backend1 = RataguiBackend::new(20, 20);
        backend1.set_font_size(14);
        let mut terminal1 = Terminal::new(backend1).unwrap();

        BevyTerminal {
            terminal_info: terminal1,
        }
    }
}

//master state structure
#[derive(Resource)]
pub struct Masterik {

}

//run when changing seeds to preserve other settings
impl Masterik {

}

impl Default for Masterik {
    fn default() -> Self {


        Self {
         
        }
    }
}