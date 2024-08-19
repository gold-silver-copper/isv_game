use crate::*;
// ANCHOR: app
#[derive(Debug, Default)]
pub struct App {
    entity_counter: i64,
   
    exit: bool,
    game_map: GameMap,
    action_map: ActionMap,
    local_player_id: EntityID,
    local_player_pos: MyPoint,
}

impl App {
  
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
            self.handle_actions()?;
        }
        Ok(())
    }
 
    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
    
 
    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
                .handle_key_event(key_event)
                .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}")),
            _ => Ok(()),
        }
    }
   
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),

            KeyCode::Char('w') => {self.action_map.insert(1,GameAction::Go(CardinalDirection::North));},
            KeyCode::Char('s') => {self.action_map.insert(1,GameAction::Go(CardinalDirection::South));},
            _ => {}
        }
        Ok(())
    }

    fn handle_movement(&mut self, eid: EntityID, cd:CardinalDirection)-> Result<()> {

        let xyik = cd.to_xyz();

        self.local_player_pos = (self.local_player_pos.0 + xyik.0,self.local_player_pos.1 + xyik.1);
        Ok(())
    }

    fn handle_actions(&mut self) -> Result<()> {

        let a_map = self.action_map.clone();


        for (eid, act) in a_map {
            //println!("moving");

            match act {
                GameAction::Go(cd) => self.handle_movement(eid,cd)?,
                _ => panic!("meow")
            }


        }

        self.action_map.drain();


        Ok(())

    }
  
    fn exit(&mut self) {
        self.exit = true;
    }


  
  
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
     
     
        let client_pos = self.local_player_pos.clone();



            let client_render = self
                .game_map
                .create_client_render_packet_for_entity(&client_pos, &area);

            let client_graphics = client_render.voxel_grid;
            let client_visible_ents = client_render.ent_vec;
        //    ui_resources.visible_ents = client_visible_ents;

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
            Paragraph::new(Text::from(render_lines))
                    .on_black()
                    .block(Block::new()) .render(area, buf);









    }
}
