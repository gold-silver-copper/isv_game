use crate::*;
// ANCHOR: app
#[derive(Default)]
pub struct App {
    pub entity_counter: i64,
    pub components: ComponentHolder,
    pub input_state: InputState,
    pub action_results: Vec<ActionResult>,
    pub selected_menu: ItemVecType,
    pub item_list_state: ListState,
    pub exit: bool,
    pub game_map: GameMap,
    pub action_vec: ActionVec,
    pub local_player_id: EntityID,
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        self.init();
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
            if !self.action_vec.is_empty() {
                self.handle_ai();
                self.handle_actions()?;
                self.handle_deaths();
            }

            self.reload_ui();
        }
        Ok(())
    }

    pub fn handle_deaths(&mut self) {
        let mut healths_to_remove = Vec::new();
        for (eid, health) in &self.components.healths {
            if health.current_health <= 0 {
                if let Some(e_pos) = self.components.positions.remove(eid) {
                    if let Some(voxik) = self.game_map.get_mut_voxel_at(&e_pos) {
                        remove_ent_from_vec(&mut voxik.entity_set, eid);

                        if let Some(equi) = self.components.equipments.remove(eid) {
                            for ano in equi.equipped {
                                voxik.entity_set.push(ano);
                            }
                            for ano in equi.inventory {
                                voxik.entity_set.push(ano);
                            }
                        }
                    }
                }

                healths_to_remove.push(eid.clone());
            }
        }
        for edik in healths_to_remove {
            self.components.healths.remove(&edik);
        }
    }

    pub fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    pub fn init(&mut self) {
        let pik = (5, 5);

        self.local_player_id = self.spawn_player_at(&pik);

        let ai_guy = self.spawn_human_at(&(7, 9));
        let ai_guy = self.spawn_human_at(&(3, 4));
        let ai_guy = self.spawn_human_at(&(7, 7));
        let iid3 = self.create_item(ItemType::Weapon(Weapon::Mace));

        let ai_equip = self
            .components
            .equipments
            .get_mut(&ai_guy)
            .expect("MUST HAVE QUEIP");
        ai_equip.equipped.insert(iid3);
        self.spawn_item_at(&(5, 8), ItemType::Weapon(Weapon::Sword));
        self.spawn_item_at(&(5, 8), ItemType::Weapon(Weapon::Sword));
        self.spawn_item_at(&(5, 8), ItemType::Weapon(Weapon::Sword));
        self.spawn_item_at(&(5, 8), ItemType::Weapon(Weapon::Sword));
        self.spawn_item_at(&(5, 9), ItemType::Clothing(Clothing::Helma));
        self.spawn_item_at(&(5, 9), ItemType::Clothing(Clothing::Helma));
        self.spawn_item_at(&(5, 9), ItemType::Clothing(Clothing::Helma));
        self.spawn_item_at(&(5, 9), ItemType::Clothing(Clothing::Toga));
        self.spawn_item_at(&(5, 9), ItemType::Clothing(Clothing::Toga));
        self.spawn_item_at(&(5, 10), ItemType::RangedWeapon(RangedWeapon::Šuk));
        self.spawn_item_at(&(5, 10), ItemType::RangedWeapon(RangedWeapon::Šuk));
        self.spawn_item_at(&(5, 10), ItemType::RangedWeapon(RangedWeapon::Šuk));
        self.reload_ui();
    }

    pub fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
                .handle_key_event(key_event)
                .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}")),
            _ => Ok(()),
        }
    }

    pub fn manage_item_vec_input(&self) -> (bool, EntityID) {
        let ground_vec = self.ground_item_vec_at_ent(&self.local_player_id);
        let equi_vec = self.equipped_item_vec_of_ent(&self.local_player_id);
        let inv_vec = self.inventory_item_vec_of_ent(&self.local_player_id);

        if let Some(sid) = self.item_list_state.selected() {
            if self.input_state == InputState::Inventory {
                let moop = match self.selected_menu {
                    ItemVecType::Equipped => equi_vec.get(sid),
                    ItemVecType::Inventory => inv_vec.get(sid),
                    ItemVecType::Ground => ground_vec.get(sid),
                };

                if let Some(id_to_select) = moop {
                    let id_to_select = id_to_select.clone();

                    return (true, id_to_select);
                }
            } else if self.input_state == InputState::RangedAttack {
                let vecik = self.ranged_attackable_ents(&self.local_player_id);
                if let Some(moop) = vecik.get(sid) {
                    return (true, moop.clone());
                }
            }
        }

        (false, 0)
    }

    pub fn handle_ai(&mut self) {
        let mut conscious_ents = Vec::new();

        for boop in &self.components.ent_types {
            if (boop.1 == &EntityType::Human) && (boop.0 != &self.local_player_id) {
                conscious_ents.push(boop.0.clone());
            }
        }

        for meow in conscious_ents {
            self.action_vec
                .push(GameAction::Go(meow, CardinalDirection::West));
        }
    }

    pub fn reload_ui(&mut self) {
        match self.input_state {
            InputState::Inventory => {
                let boopik = match self.selected_menu {
                    ItemVecType::Equipped => {
                        self.equipped_item_vec_of_ent(&self.local_player_id).len()
                    }
                    ItemVecType::Inventory => {
                        self.inventory_item_vec_of_ent(&self.local_player_id).len()
                    }
                    ItemVecType::Ground => self.ground_item_vec_at_ent(&self.local_player_id).len(),
                };

                if let Some(sel_len) = self.item_list_state.selected_mut() {
                    if ((*sel_len >= boopik) && (boopik > 0)) {
                        *sel_len = boopik - 1;
                    }
                }
            }
            InputState::RangedAttack => {
                let boopik = self.ranged_attackable_ents(&self.local_player_id).len();
                if let Some(sel_len) = self.item_list_state.selected_mut() {
                    if ((*sel_len >= boopik) && (boopik > 0)) {
                        *sel_len = boopik - 1;
                        println!("boopik {boopik}");
                    }
                }
            }

            _ => (),
        }
    }

    pub fn handle_actions(&mut self) -> Result<()> {
        let a_map = self.action_vec.clone();
        self.action_vec = Vec::new();

        for act in a_map {
            //println!("moving");

            let act_result = match act {
                GameAction::Go(subj_id, cd) => {
                    (subj_id.clone(), self.handle_movement(&subj_id, &cd))
                }
                GameAction::Drop(subj_id, obj_id) => {
                    (subj_id.clone(), self.drop_item_from_inv(&subj_id, &obj_id))
                }
                GameAction::PickUp(subj_id, obj_id) => (
                    subj_id.clone(),
                    self.pickup_item_from_ground(&subj_id, &obj_id),
                ),
                GameAction::Equip(subj_id, obj_id) => {
                    (subj_id.clone(), self.equip_item_from_inv(&subj_id, &obj_id))
                }
                GameAction::UnEquip(subj_id, obj_id) => (
                    subj_id.clone(),
                    self.unequip_item_from_equipped(&subj_id, &obj_id),
                ),
                GameAction::RangedAttack(subj_id, obj_id) => {
                    (subj_id.clone(), self.ranged_attack(&subj_id, &obj_id))
                }
                GameAction::Wait(subj_id) => (subj_id.clone(), self.handle_wait(&subj_id)),
                _ => panic!("meow"),
            };

            if (act_result.0 == self.local_player_id)
                || (self
                    .generate_visible_ents_from_ent(&self.local_player_id)
                    .contains(&act_result.0))
            {
                self.action_results.push(act_result.1);
            }
        }

        Ok(())
    }
    pub fn title_block(title: &str) -> Block {
        let title = Title::from(title).alignment(Alignment::Center);
        Block::new()
            .borders(Borders::NONE)
            .padding(Padding::vertical(0))
            .title(title)
            .fg(Color::Blue)
    }

    pub fn ranged_attackable_ents(&self, subj: &EntityID) -> Vec<EntityID> {
        let mut new_vec = Vec::new();

        if let Some(subj_pos) = self.components.positions.get(subj) {
            let visible_ents = self.generate_visible_ents_from_ent(subj);
            for entik in visible_ents {
                let typik = self.get_ent_type(&entik);
                if typik.is_attackable() {
                    if let Some(obj_pos) = self.components.positions.get(subj) {
                        if self.game_map.line_from_point_to_point_is_unblocked(
                            subj_pos,
                            obj_pos,
                            &self.components.ent_types,
                        ) {
                            new_vec.push(entik);
                        }
                    }
                }
            }
        }
        new_vec
    }

    pub fn render_const_info(&self, area: Rect, buf: &mut Buffer) {
        let mut cur_hel = 0;
        let mut max_hel = 1;

        let ent_name = self.get_entity_name(&self.local_player_id);
        if let Some(health) = self.components.healths.get(&self.local_player_id) {
            cur_hel = health.current_health.clone();
            max_hel = health.max_health.clone();
        }
        let title = App::title_block(&ent_name);

        let label = Span::styled(
            format!("{}/{}", cur_hel, max_hel),
            Style::new().bold().fg(Color::Black),
        );
        Gauge::default()
            .block(title)
            .gauge_style(Style::new().fg(Color::Green).bg(Color::LightRed))
            .ratio(cur_hel as f64 / max_hel as f64)
            .label(label)
            .render(area, buf);
    }

    pub fn gen_symbol_name_line_vec(&self, id_vec: &Vec<EntityID>) -> Vec<Line> {
        let mut visible_lines = Vec::new();
        let visible_symbols = self.gen_item_symbol_vec(id_vec);
        let visible_names = self.gen_item_name_vec(id_vec);
        for boopik in 0..id_vec.len() {
            let stringik = format! {"{}  {}",visible_symbols[boopik],visible_names[boopik]};
            visible_lines.push(Line::from(stringik));
        }
        visible_lines
    }

    pub fn generate_info_paragraph(&self) -> Paragraph {
        let mut wield_string = String::new();
        if let Some(player_equip) = self.components.equipments.get(&self.local_player_id) {
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
                        EntityType::Human => panic!("CANNOT WIELD ANIMALS... yet"),
                        EntityType::Item(itemik) => itemik.item_name(),
                    };
                    wield_string.push_str(&format!(" {item_name}"));
                }
            }
        }

        let visible_lines = self
            .gen_symbol_name_line_vec(&self.generate_visible_ents_from_ent(&self.local_player_id));

        let mut standart = vec![
            Line::from("Wielding..."),
            Line::from(wield_string),
            Line::from("You see..."),
        ];
        standart.extend(visible_lines);

        let lines = (Text::from(standart));

        Paragraph::new(Text::from(lines))
            .on_black()
            .block(Block::bordered())
    }

    pub fn generate_event_paragraph(&self) -> Paragraph {
        let mut line_vec = Vec::new();

        //should add visible events too later
        let mut events_copy = self.action_results.clone();

        for x in 0..20 {
            let boop = events_copy.pop();
            if let Some(actik) = boop {
                line_vec.push(Line::from(self.generate_action_result_string(actik)));
            }
        }

        //  line_vec.reverse();
        let lines = (Text::from(line_vec));

        Paragraph::new(Text::from(lines))
            .on_black()
            .block(Block::bordered())
    }

    pub fn render_item_list(&self, title: &str, itemvectype: ItemVecType) -> List {
        let wut = match itemvectype {
            ItemVecType::Equipped => {
                self.gen_item_name_vec(&self.equipped_item_vec_of_ent(&self.local_player_id))
            }
            ItemVecType::Inventory => {
                self.gen_item_name_vec(&self.inventory_item_vec_of_ent(&self.local_player_id))
            }
            ItemVecType::Ground => {
                self.gen_item_name_vec(&self.ground_item_vec_at_ent(&self.local_player_id))
            }
        };

        let list = List::new(wut)
            .block(Block::bordered().title(title.to_string()))
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">")
            .repeat_highlight_symbol(true);

        list
    }
    pub fn render_ranged_attackable_list(&self, title: &str) -> List {
        let wut = self.ranged_attackable_ents(&self.local_player_id);
        let listik = self.gen_symbol_name_line_vec(&wut);

        let list = List::new(listik)
            .block(Block::bordered().title(title.to_string()))
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">")
            .repeat_highlight_symbol(true);

        list
    }

    pub fn gen_item_name_vec(&self, id_vec: &Vec<EntityID>) -> Vec<String> {
        let mut itemnamevec = Vec::new();

        for itik in id_vec.iter() {
            let typik = self
                .components
                .ent_types
                .get(itik)
                .expect("ent type must have");
            let itname = match typik {
                EntityType::Human => self.get_entity_name(itik),
                EntityType::Item(itemik) => itemik.item_name(),
            };
            itemnamevec.push(itname);
        }
        itemnamevec
    }
    pub fn gen_item_symbol_vec(&self, id_vec: &Vec<EntityID>) -> Vec<String> {
        let mut itemnamevec = Vec::new();

        for itik in id_vec.iter() {
            let typik = self
                .components
                .ent_types
                .get(itik)
                .expect("ent type must have");

            itemnamevec.push(typik.symbol());
        }
        itemnamevec
    }
    pub fn inventory_item_vec_of_ent(&self, eid: &EntityID) -> Vec<EntityID> {
        let mut evec = Vec::new();
        if let Some(ent_equi) = self.components.equipments.get(eid) {
            for itemik in ent_equi.inventory.iter() {
                evec.push(itemik.clone());
            }
        }

        evec
    }

    pub fn equipped_item_vec_of_ent(&self, eid: &EntityID) -> Vec<EntityID> {
        let mut evec = Vec::new();
        if let Some(ent_equi) = self.components.equipments.get(eid) {
            for itemik in ent_equi.equipped.iter() {
                evec.push(itemik.clone());
            }
        }

        evec
    }

    pub fn ground_item_vec_at_ent(&self, eid: &EntityID) -> Vec<EntityID> {
        let mut evec = Vec::new();

        if let Some(ent_pos) = self.components.positions.get(eid) {
            let ent_vox = self.game_map.get_voxel_at(ent_pos).unwrap();

            for boop in ent_vox.entity_set.iter() {
                let booptype = self.get_ent_type(boop);
                match booptype {
                    EntityType::Human => {}
                    EntityType::Item(x) => {
                        evec.push(boop.clone());
                    }
                }
            }
        }

        evec
    }

    pub fn generate_visible_ents_from_ent(&self, eid: &EntityID) -> Vec<EntityID> {
        let ent_pos = self.components.positions.get(eid).unwrap_or(&(0, 0));

        let mut visible_ents_with_self = self.game_map.generate_visible_ents_from_point(ent_pos);
        remove_ent_from_vec(&mut visible_ents_with_self, eid);

        visible_ents_with_self
    }

    pub fn line_from_ent_to_ent(
        &self,
        subj: &EntityID,
        obj: &EntityID,
    ) -> Option<BresenhamInclusive> {
        if let Some(startik) = self.components.positions.get(subj) {
            if let Some(endik) = self.components.positions.get(obj) {
                let start = Point::from_tuple(startik.clone());
                let end = Point::from_tuple(endik.clone());
                return Some(BresenhamInclusive::new(start, end));
            }
        }
        return None;
    }

    pub fn exit(&mut self) {
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

        let left_layout = layout[0];
        let right_layout = layout[1];

        let second_layout = Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(80), Constraint::Min(10)],
        )
        .split(left_layout);

        let third_layout = Layout::new(
            Direction::Vertical,
            [Constraint::Length(3), Constraint::Fill(1)],
        )
        .split(right_layout);

        let constant_info_layout = third_layout[0];

        let side_info_layout = third_layout[1];

        let game_screen_layout = second_layout[0];
        let event_layout = second_layout[1];

        let client_pos = self
            .components
            .positions
            .get(&self.local_player_id)
            .unwrap_or(&(0, 0));

        let (_, selected_ent) = self.manage_item_vec_input();
        let highlighted_ranged_line =
            self.line_from_ent_to_ent(&self.local_player_id, &selected_ent);

        let client_graphics = self.game_map.create_client_render_packet_for_entity(
            &client_pos,
            &game_screen_layout,
            &self.components.ent_types,
            highlighted_ranged_line,
        );

        let mut render_lines = Vec::new();
        let needed_height = game_screen_layout.height as i16;

        if client_graphics.len() > 0 {
            for y in (0..needed_height) {
                let myspanvec: Vec<_> = client_graphics[y as usize]
                    .iter()
                    .map(|x| Span::from(x.0.clone()).fg(x.1).bg(x.2))
                    .collect();

                let myline = ratatui::text::Line::from(myspanvec);

                render_lines.push(myline);
            }
        }

        let mut inv_state = match self.selected_menu {
            ItemVecType::Inventory => self.item_list_state.clone(),
            _ => ListState::default(),
        };
        let mut equip_state = match self.selected_menu {
            ItemVecType::Equipped => self.item_list_state.clone(),
            _ => ListState::default(),
        };
        let mut ground_state = match self.selected_menu {
            ItemVecType::Ground => self.item_list_state.clone(),
            _ => ListState::default(),
        };
        let mut ranged_state = match self.input_state {
            InputState::RangedAttack => self.item_list_state.clone(),
            _ => ListState::default(),
        };

        //neccesary beccause drawing is from the top
        render_lines.reverse();
        Paragraph::new(Text::from(render_lines))
            .on_black()
            .block(Block::new())
            .render(game_screen_layout, buf);

        self.render_const_info(constant_info_layout, buf);
        self.generate_info_paragraph().render(side_info_layout, buf);
        self.generate_event_paragraph().render(event_layout, buf);

        match self.input_state {
            InputState::Basic => (),

            InputState::Inventory => {
                let block = Block::bordered().title("Popup");
                let pop_area = popup_area(game_screen_layout, 80, 70);
                let pop_layout = Layout::new(
                    Direction::Horizontal,
                    [
                        Constraint::Min(20),
                        Constraint::Min(20),
                        Constraint::Min(20),
                    ],
                )
                .split(pop_area);
                Clear.render(pop_area, buf); //this clears out the background
                block.render(pop_area, buf); //this clears out the background
                ratatui::prelude::StatefulWidget::render(
                    self.render_item_list("Inventory", ItemVecType::Inventory),
                    pop_layout[1],
                    buf,
                    &mut inv_state,
                );
                ratatui::prelude::StatefulWidget::render(
                    self.render_item_list("Equipped", ItemVecType::Equipped),
                    pop_layout[2],
                    buf,
                    &mut equip_state,
                );
                ratatui::prelude::StatefulWidget::render(
                    self.render_item_list("Ground", ItemVecType::Ground),
                    pop_layout[0],
                    buf,
                    &mut ground_state,
                );
            }

            InputState::RangedAttack => {
                Clear.render(side_info_layout, buf); //this clears out the background
                let block = Block::bordered().title("Ranged Attack");
                block.render(side_info_layout, buf); //this clears out the background
                ratatui::prelude::StatefulWidget::render(
                    self.render_ranged_attackable_list("Ranged Attack"),
                    side_info_layout,
                    buf,
                    &mut ranged_state,
                );
            }

            _ => panic!("INPUT STATE RENDER NOT IMPELEMNTED"),
        }
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

pub fn remove_ent_from_vec(ent_vec: &mut Vec<EntityID>, ent_to_remove: &EntityID) {
    ent_vec.retain(|x| x != ent_to_remove);
}
