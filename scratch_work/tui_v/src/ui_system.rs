use crate::*;
pub fn draw_ascii_game(
    mut termres: ResMut<BevyTerminal<RataguiBackend>>,
    masterok: Res<Masterok>,
    mut ui_resources: ResMut<UIResources>,
    player_position: Query<(EntityID, &PointComponent), With<Player>>,
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
            let client_visible_ents = client_render.ent_vec;
            ui_resources.visible_ents = client_visible_ents;

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
