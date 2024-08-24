use crate::*;
// ANCHOR: app
#[derive(Default)]
pub struct App {
    entity_counter: i64,
    components: ComponentHolder,
    input_state: InputState,
    inventory_list_state: ListState,
    inventory_eid_vec: Vec<EntityID>,
    inventory_name_vec: Vec<String>,


    exit: bool,
    game_map: GameMap,
    action_map: ActionMap,
    local_player_id: EntityID,
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        self.init();
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

    pub fn init(&mut self) {
        let pik = (5, 5);

        self.local_player_id = self.spawn_player_at(&pik);
        self.spawn_item_at(&(5, 8), ItemType::Weapon(Weapon::Sword));
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
        let lid = self.local_player_id.clone();

        match self.input_state {
            InputState::Basic => match key_event.code {
                KeyCode::Char('q') => self.exit(),

                KeyCode::Char('w') => {
                    self.action_map
                        .insert(lid, GameAction::Go(CardinalDirection::North));
                }
                KeyCode::Char('s') => {
                    self.action_map
                        .insert(lid, GameAction::Go(CardinalDirection::South));
                }
                KeyCode::Char('a') => {
                    self.action_map
                        .insert(lid, GameAction::Go(CardinalDirection::West));
                }
                KeyCode::Char('d') => {
                    self.action_map
                        .insert(lid, GameAction::Go(CardinalDirection::East));
                }
                KeyCode::Char('i') => {
                    self.generate_inventory_eid_vec();
                    self.inventory_list_state.select_first();
                    self.input_state = InputState::Inventory;
                   
                }
                _ => {}
            },
            InputState::Inventory => match key_event.code {
                KeyCode::Char('i') => {
                    self.input_state = InputState::Basic;
                }
                KeyCode::Char('w') => {

            
                    self.inventory_list_state
                        .select_previous();
                }
                KeyCode::Char('s') => {
                    self.inventory_list_state
                    .select_next();
                }
                _ => {}
            },
            _ => panic!("input state not implemented"),
        }

        Ok(())
    }

    fn handle_movement(&mut self, eid: &EntityID, cd: &CardinalDirection) -> Result<()> {
        let xyik = cd.to_xyz();
        if let Some(e_pos) = self.components.positions.get_mut(eid) {
            let destination = (e_pos.0 + xyik.0, e_pos.1 + xyik.1);
            // println!("epos got");

            if let Some(dest_vox) = self.game_map.get_mut_voxel_at(&destination) {
                if !dest_vox.blocks_movement(&self.components.ent_types) {
                    // println!("dest no block");
                    dest_vox.entity_set.insert(eid.clone());

                    if let Some(origin_vox) = self.game_map.get_mut_voxel_at(e_pos) {
                        origin_vox.entity_set.remove(eid);
                    }
                    *e_pos = destination;
                }
            }
        }

        Ok(())
    }

    fn handle_actions(&mut self) -> Result<()> {
        let a_map = self.action_map.clone();
        self.action_map.drain();

        for (eid, act) in a_map {
            //println!("moving");

            match act {
                GameAction::Go(cd) => self.handle_movement(&eid, &cd)?,
                _ => panic!("meow"),
            }
        }

        Ok(())
    }

    pub fn spawn_player_at(&mut self, point: &MyPoint) -> EntityID {
        let pid = self.spawn_animal_at(point);
        let iid = self.create_item(ItemType::Weapon(Weapon::Sword));
        let iid2 = self.create_item(ItemType::Clothing(Clothing::Toga));
        let iid3 = self.create_item(ItemType::Weapon(Weapon::Mace));

        let player_equip = self
            .components
            .equipments
            .get_mut(&pid)
            .expect("MUST HAVE QUEIP");
        player_equip.equipped.insert(iid);
        player_equip.inventory.insert(iid2);
        player_equip.inventory.insert(iid3);

        pid
    }

    pub fn spawn_animal_at(&mut self, point: &MyPoint) -> EntityID {
        let eid = self.get_unique_eid();
        self.components.positions.insert(eid.clone(), point.clone());
        self.components
            .ent_types
            .insert(eid.clone(), EntityType::Animalia);
        self.components
            .equipments
            .insert(eid.clone(), Equipment::default());

        let voxik = self
            .game_map
            .get_mut_voxel_at(point)
            .expect("cant spawn ent in empty voxel");

        voxik.entity_set.insert(eid.clone());

        eid.clone()
    }

    pub fn create_item(&mut self, item: ItemType) -> EntityID {
        let eid = self.get_unique_eid();

        self.components
            .ent_types
            .insert(eid.clone(), EntityType::Item(item));

        eid
    }

    pub fn set_ent_position(&mut self, eid: &EntityID, point: &MyPoint) {
        self.components.positions.insert(eid.clone(), point.clone());

        let voxik = self
            .game_map
            .get_mut_voxel_at(point)
            .expect("cant spawn ent in empty voxel");

        voxik.entity_set.insert(eid.clone());
    }

    pub fn spawn_item_at(&mut self, point: &MyPoint, item: ItemType) -> EntityID {
        let eid = self.create_item(item);
        self.set_ent_position(&eid, point);

        eid
    }

    pub fn generate_info_paragraph(&self) -> Paragraph {
        let player_equip = self
            .components
            .equipments
            .get(&self.local_player_id)
            .expect("PLAYER MUST HAVE EQUIPMENT COMPONENT");

        let mut wield_string = String::new();

        if player_equip.equipped.is_empty() {
            wield_string = String::from("nothing")
        } else {
            for (item) in player_equip.equipped.iter() {
                let item_type = self
                    .components
                    .ent_types
                    .get(item)
                    .expect("EVERY ITEM MUST HAVE AN ENTITY TYPE");
                let item_name = match item_type {
                    EntityType::Animalia => panic!("CANNOT WIELD ANIMALS... yet"),
                    EntityType::Item(itemik) => itemik.item_name(),
                };
                wield_string.push_str(&format!(" {item_name}"));
            }
        }

        let mut lines = (Text::from(vec![
            Line::from("HAI"),
            Line::from("Wielding..."),
            Line::from(wield_string),
        ]));

        Paragraph::new(Text::from(lines))
            .on_black()
            .block(Block::bordered())
    }

    pub fn render_inventory_list(&self) -> List {
        let wut = self.inventory_name_vec.clone();
       

      



        let list = List::new(wut)
            .block(Block::bordered().title("Inventory"))
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);

        list
    }
    pub fn generate_inventory_eid_vec(&mut self)  {


        let mut evec = Vec::new();
        let player_inv = self.components.equipments.get(&self.local_player_id).expect("must have equi");


        for itemik in player_inv.inventory.iter() {
            evec.push(itemik.clone());


        }


        let mut itemnamevec = Vec::new();

        for itik in evec.iter() {
 
         let typik = self.components.ent_types.get(itik).expect("ent type must have");
         let itname = match typik {
 
             EntityType::Animalia => {panic!("why do u have an animal in ur inventory")}
             EntityType::Item(itemik) => {itemik.item_name()}
         
             
         
         };
         itemnamevec.push(itname);
 
        }
        self.inventory_name_vec = itemnamevec;

        self.inventory_eid_vec = evec;


    }

    pub fn get_unique_eid(&mut self) -> EntityID {
        self.entity_counter += 1;
        self.entity_counter.clone()
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(80), Constraint::Min(20)],
        )
        .split(area);

        let client_pos = self
            .components
            .positions
            .get(&self.local_player_id)
            .unwrap_or(&(0, 0));

        let client_render = self.game_map.create_client_render_packet_for_entity(
            &client_pos,
            &layout[0],
            &self.components.ent_types,
        );

        let client_graphics = client_render.voxel_grid;
        let client_visible_ents = client_render.ent_vec;
        //    ui_resources.visible_ents = client_visible_ents;

        let mut render_lines = Vec::new();
        let needed_height = layout[0].height as i16;

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

        let mut list_state = self.inventory_list_state.clone();

        match self.input_state {
            InputState::Basic => {
                self.generate_info_paragraph().render(layout[1], buf);
            }
            InputState::Inventory => {
                ratatui::prelude::StatefulWidget::render(
                    self.render_inventory_list(),
                    layout[1],
                    buf,
                    &mut list_state,
                );
            }
            _ => panic!("INPUT STATE RENDER NOT IMPELEMNTED"),
        }

        //neccesary beccause drawing is from the top
        render_lines.reverse();
        Paragraph::new(Text::from(render_lines))
            .on_black()
            .block(Block::new())
            .render(layout[0], buf);
    }
}
