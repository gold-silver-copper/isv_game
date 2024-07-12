use crate::*;
pub fn draw_ascii_game(
   
    mut termres: ResMut<BevyTerminal<RataguiBackend>>,
    masterok: Res<Masterik>,
) {

   
    let client_world = &masterok.client_world;
    let client_pos = &masterok.client_pos;
    termres.terminal_game
        .draw(|frame| {
            let area = frame.size();
            let client_render =
                client_world.create_client_render_packet_for_entity(client_pos, &area);
            let client_graphics = client_render.spans_to_render;
            let mut render_lines = Vec::new();
            let needed_height = area.height as i16;

            if client_graphics.len() > 0 {
                for y in (0..needed_height) {
                    let myspanvec: Vec<_> = client_graphics[y as usize]
                        .iter()
                        .map(|x| Span::from(&x.0).fg(x.1).bg(x.2))
                        .collect();

                    let myline = ratatui::text::Line::from(myspanvec);

                    render_lines.push(myline);
                }
            }

            //neccesary beccause drawing is from the top
            render_lines.reverse();
            frame.render_widget(
                Paragraph::new(Text::from(render_lines))
                    .on_black()
                    .block(Block::new()),
                area,
            );
        })
        .expect("epic fail");
}

pub fn draw_ascii_info(terminal: &mut Terminal<RataguiBackend>, masterok: &Masterik) {}

// Render to the terminal and to egui , both are immediate mode
pub fn ui_example_system(
    mut contexts: EguiContexts,
    mut termres: ResMut<BevyTerminal<RataguiBackend>>,
    mut masterok: ResMut<Masterik>,
    ui_status: Res<UIState>,
    query_position: Query<(Entity, &GamePosition)>,
) {
  
    draw_ascii_info(&mut termres.terminal_info, &masterok);
    let mut gameframe = egui::Frame::default()
        .inner_margin(10.0)
        .outer_margin(0.0)
        .fill(egui::Color32::BLACK);

    let mut infoframe = egui::Frame::default()
        .inner_margin(0.0)
        .outer_margin(0.0)
        .fill(egui::Color32::BLACK);

    egui::CentralPanel::default()
        .frame(Frame::none())
        .show(contexts.ctx_mut(), |ui| {
            let av_height = ui.available_height().clamp(100., 1500.);

            egui::SidePanel::right("containeeee")
                .min_width(260.)
                .max_width(260.)
                .frame(infoframe)
                .show_inside(ui, |ui| {
                    ui.add(termres.terminal_info.backend_mut());
                });
            ui.separator();
            let av_width = ui.available_width().clamp(100., 1500.);
            egui::SidePanel::left("gameik")
                .min_width(av_width)
                .max_width(av_width)
                .frame(gameframe)
                .show_inside(ui, |ui| {
                    ui.add(termres.terminal_game.backend_mut());
                });
        });
}
