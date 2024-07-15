use crate::*;
pub fn draw_ascii_game(
    mut termres: ResMut<BevyTerminal<RataguiBackend>>,
    masterok: Res<Masterok>,
    player_position: Query<(Entity, &PointComponent), With<Player>>,
) {
    let (pid, client_pos) = player_position.single();

    termres
        .terminal_game
        .draw(|frame| {
            let area = frame.size();
            let client_render = masterok
                .game_map
                .create_client_render_packet_for_entity(&client_pos.0, &area);

            let client_graphics = client_render.voxel_grid;

            let mut render_lines = Vec::new();
            let needed_height = area.height as i16;

            if client_graphics.len() > 0 {
                for y in (0..needed_height) {
                    let myspanvec: Vec<_> = client_graphics[y as usize]
                        .iter()
                        .map(|x| Span::from(x.0).fg(x.1).bg(x.2))
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

pub fn draw_ascii_info(mut termres: ResMut<BevyTerminal<RataguiBackend>>) {
    let name_string = format! {"{}","hello"};

    let mut messages_to_show = Vec::new();

    messages_to_show.push(Line::from("Věči...."));

    termres
        .terminal_info
        .draw(|frame| {
            let area = frame.size();

            //neccesary beccause drawing is from the top

            frame.render_widget(
                Paragraph::new(messages_to_show)
                    .on_black()
                    .block(Block::new().title(name_string).borders(Borders::ALL)),
                area,
            );
        })
        .expect("epic fail");
}

// Render to the terminal and to egui , both are immediate mode
pub fn ui_example_system(
    mut contexts: EguiContexts,
    mut termres: ResMut<BevyTerminal<RataguiBackend>>,
) {
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
            let av_height = ui.available_height().clamp(100., 4500.);

            egui::SidePanel::right("containeeee")
                .min_width(260.)
                .max_width(260.)
                .frame(infoframe)
                .show_inside(ui, |ui| {
                    ui.add(termres.terminal_info.backend_mut());
                });

            let av_width = ui.available_width().clamp(100., 4500.);
            egui::SidePanel::left("gameik")
                .min_width(av_width)
                .max_width(av_width)
                .frame(gameframe)
                .show_inside(ui, |ui| {
                    ui.add(termres.terminal_game.backend_mut());
                });
        });
}
