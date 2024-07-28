use macroquad::prelude::*;
use ratatui::{
    prelude::{Stylize, Terminal},
    widgets::{Paragraph,Wrap},
};
use egui_ratatui::RataguiBackend;
use ratatui::widgets::Block;

use ratatui::widgets::Borders;
#[macroquad::main("egui with macroquad")]
async fn main() {
    let boop = RataguiBackend::new(100, 50);
    let mut terminal = Terminal::new(boop).unwrap();

    let  infoframe = egui::Frame::default()
    .inner_margin(0.0)
    .outer_margin(0.0)
    .fill(egui::Color32::BLACK);

    loop {
        clear_background(WHITE);

        // Process keys, mouse etc.

        terminal
        .draw(|frame| {
            let area = frame.size();
            frame.render_widget(Paragraph::new("Hello Rataguiii and hello macroquad yayyyy weeee ").block(Block::new().title("LOL").borders(Borders::ALL))
            .white().on_blue().wrap(Wrap { trim: false }), area);
        })
        .expect("epic fail");
    

        egui_macroquad::ui(|egui_ctx| {
            egui::CentralPanel::default()
            .frame(infoframe)
                .show(egui_ctx, |ui| {
                    ui.add(terminal.backend_mut());
                });
        });

        // Draw things before egui

        egui_macroquad::draw();
        
        // Draw things after egui

        next_frame().await;
    }
}