use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Frame},
    EguiContexts, EguiPlugin,
};
use std::collections::HashMap;
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

fn setup(mut commands: Commands, mut masterok: ResMut<Masterik>) {
    commands.spawn(
        (Camera2dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            ..default()
        }),
    );
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
        backend1.set_font_size(16);
        let mut terminal1 = Terminal::new(backend1).unwrap();

        BevyTerminal {
            terminal_info: terminal1,
        }
    }
}

//master state structure
#[derive(Resource)]
pub struct Masterik {}

//run when changing seeds to preserve other settings
impl Masterik {}

impl Default for Masterik {
    fn default() -> Self {
        Self {}
    }
}

fn ui_example_system(
    mut contexts: EguiContexts,
    mut termres: ResMut<BevyTerminal<RataguiBackend>>,
    masterok: Res<Masterik>,
) {
    //draws info to ratatui terminal
    draw_info_menu(&mut termres.terminal_info, &masterok);

    let mut frame = egui::Frame::default()
        .inner_margin(4.0)
        .outer_margin(0.0)
        .fill(egui::Color32::BLACK);

    //limit panel to certain size that is guaranteed to fit text
    egui::SidePanel::right("my_left_panel")
        .frame(frame)
        .min_width(322.0)
        .max_width(322.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.add(termres.terminal_info.backend_mut());
        });
}

fn draw_info_menu(terminal: &mut Terminal<RataguiBackend>, masterok: &Masterik) {
    terminal
        .draw(|frame| {
            let area = frame.size();

            let mut lines =
                (Text::from(vec![Line::from(format!("meoiw: {} ", 6)), Line::from(" ")]));

            frame.render_widget(
                Paragraph::new(lines)
                    .on_black()
                    .block(Block::new().title("Salve").gray().borders(Borders::ALL)),
                area,
            );
        })
        .expect("epic fail");
}


/// This will be used to identify the main player entity
#[derive(Component)]
struct Player;

type XPosition = i64;
type YPosition = i64;
type ZPosition = i64;

#[derive(Component)]
struct GamePosition (XPosition,YPosition,ZPosition);

#[derive(Component)]
enum VoxelType {
    Air,
    Dirt,
}


#[derive(Resource)]
pub struct GameMap {

    tile_map: HashMap<GamePosition,VoxelType>,


}