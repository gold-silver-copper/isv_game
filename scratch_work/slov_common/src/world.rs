use crate::*;

#[derive(Clone, Debug)]
pub struct MyWorld {
    pub voxeltile_grid: RTree<Voxel>,
    pub entity_tree: RTree<PositionComponent>,
    pub entity_map: HashMap<EntityID, EntityType>,
    pub ent_loc_index: HashMap<EntityID, MyPoint>,
    pub server_stuff: ServerStuff,
    pub turn_counter: u32,
    pub small_rngik: SmallRng,
  

    pub world_seed: u32,
    pub entity_counter: u64,
}

impl Default for MyWorld {
    fn default() -> Self {
        let rngik: u32 = 87243563;

        Self {
            entity_tree: RTree::new(),
            entity_map: HashMap::new(),
            server_stuff: ServerStuff::default(),
            turn_counter: 0,
            small_rngik: SmallRng::seed_from_u64(rngik as u64),
            

            world_seed: rngik.clone(),
            entity_counter: 1,
            ent_loc_index: HashMap::new(),
            voxeltile_grid: MyWorld::generate_test(rngik),
        }
    }
}

impl MyWorld {
    pub fn receive(&mut self, input_pair: (ActionType, AccountID)) {
        let entity_id_of_account = self
            .server_stuff
            .accid_entid_map
            .get(&input_pair.1)
            .unwrap_or(&0);

        if entity_id_of_account != &(0 as u64) {
            self.server_stuff
                .input_queue
                .insert(entity_id_of_account.clone(), input_pair.0);
        }

        //   println!("inserted");
    }

    pub fn new_test() -> MyWorld {
        let mut x = MyWorld::default();
    
        x
    }

    pub fn interpret_and_execute(&mut self) {
        let my_clone = self.server_stuff.input_queue.clone();
        self.server_stuff.input_queue.clear();
        self.server_stuff.output_queue = RTree::new();

        self.turn_counter += 1;

        for (eid, action) in &my_clone {
            if let Some(ent_loc) = self.ent_loc_index.get(&eid) {
                let caller_loc = ent_loc.clone();

                let success_type = match action {
                    ActionType::Go(loc) => Action::go(self, &eid, loc),
              
                    ActionType::Wait => SuccessType::Success,

                    _ => panic!("not implemented"),
                };
                self.server_stuff.output_queue.insert(ActionPacket {
                    action: action.clone(),
                    success: success_type,
                    action_location: caller_loc,
                    action_subject: eid.clone(),
                })
            }
        }

        // Extend with actual game logic
    }
    //z must be above 0 for movement
    pub fn make_account(&mut self) -> (AccountID, EntityID, MyPoint) {
        let pp = (80, 80);
        let eid = self.new_entity(&pp, &EntityType::Human);

        //increment then use value for creating account , send account and entity id to Human
        self.server_stuff.account_counter += 17;

        let aid = self.server_stuff.account_counter.clone();

        self.server_stuff
            .accid_entid_map
            .insert(self.server_stuff.account_counter.clone(), eid.clone());

        (aid, eid, pp)
    }

    pub fn new_entity(&mut self, point: &MyPoint, spawn_type: &EntityType) -> EntityID {
        self.entity_counter += 1;

        //GET ENTITY ID AND START ADDING COMPONENTS AFTER IT
        let eid = self.entity_counter.clone();

        let pc = PositionComponent {
            entity_id: eid.clone(),
            point: point.clone(),
        };

        self.ent_loc_index.insert(eid.clone(), point.clone());

        self.entity_tree.insert(pc);
        self.entity_map.insert(eid.clone(), spawn_type.clone());

        //END ADDING COMPONENTS HERE EXTRA INCREMENT CAUSE WHY NOT
        self.entity_counter += 1;

        return eid;
    }

    pub fn delete_entity(&mut self, eidik: &EntityID) -> SuccessType {
        let ahahaha = self.ent_loc_index.get(eidik).unwrap_or(&(0, 0));

        let pc = PositionComponent {
            entity_id: eidik.clone(),
            point: ahahaha.clone(),
        };

        self.ent_loc_index.remove(eidik);

        self.entity_tree.remove(&pc);
        self.entity_map.remove(eidik);

        return SuccessType::Success;
    }

    // World initialization function.
    pub fn init_world(&mut self) -> RTree<Voxel> {
        let rngik = self.world_seed.clone();

        let a = MyWorld::generate_test(rngik);

        a
    }

    pub fn generate_test(seed: u32) -> RTree<Voxel> {
        let hasher = noise::permutationtable::PermutationTable::new(seed);
        let boop = noise::utils::PlaneMapBuilder::new_fn(|point| {
            noise::core::open_simplex::open_simplex_2d(point.into(), &hasher)
        })
        .set_size(300, 300)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build();

        let mut batchvec = Vec::new();
        for x in 0..300 {
            for y in 0..300 {
                let val = boop.get_value(x as usize, y as usize);
                let floor = if val > 0.4 {
                    Floor::Burjan
                } else if val > -0.1 {
                    Floor::Trava
                } else if val > -0.2 {
                    Floor::Zemja
                } else if val > -0.3 {
                    Floor::Pěsȯk
                } else {
                    Floor::Voda
                };

                batchvec.push(Voxel {
                    floor: floor,

                    roof: Roof::Sky,
                    voxel_pos: (x, y),
                });
            }
        }
        let newtree = RTree::bulk_load(batchvec);

        newtree
    }

    pub fn set_voxel_at(&mut self, vox: &Voxel) {
        if let Some(boop) = self.voxeltile_grid.locate_at_point_mut(&vox.voxel_pos) {
            *boop = vox.clone();
        } else {
            self.voxeltile_grid.insert(vox.clone())
        }
    }

    pub fn get_voxel_at(&self, point: &MyPoint) -> Option<Voxel> {
        if let Some(boop) = self.voxeltile_grid.locate_at_point(point) {
            Some(boop.clone())
        } else {
            None
        }
    }


    pub fn entity_blocks_movement_at(&self, point: &MyPoint) -> bool {
        let entsatpoint = self.entity_tree.locate_all_at_point(point);

        for entt in entsatpoint {
            let enttype = self
                .entity_map
                .get(&entt.entity_id)
                .unwrap();

            match enttype {
                EntityType::Human => return true,
        
              
                _ => (),
            }
        }
        if point.0 > 0 && point.1 > 0 {
            return false;
        } else {
            return true;
        }
    }

    pub fn generate_isv_message(
        &self,
        act_packet: &ActionPacket,
        local_player_id: &EntityID,
    ) -> String {
        /*
            let msg_person = if local_player_id == &act_packet.action_subject {
                Person::Second
            } else {
                Person::Third
            };
        */
        let mut subject_pronoun = String::from("Ty");
        if local_player_id != &act_packet.action_subject {
            let nomik = self
                .entity_map
                .get(&act_packet.action_subject)
                .unwrap();

            subject_pronoun = nomik.minimal_string();
        }
        let meowik = match &act_packet.action {
          
            ActionType::Go(x) => format!("idti"),
           
            _ => format!(" jhxcvhas {}", ""),
        };

        let succik = match &act_packet.success {
            SuccessType::Success => {
                format!("")
            }
            SuccessType::Failure => {
                format!("ne mozzesz")
            }
        };

        if succik != "" {
            subject_pronoun.push_str(&succik);
        } else {
            subject_pronoun.push_str(&meowik);
        }

        let abc = format!(" {:#?}", &act_packet.action_location);
        subject_pronoun.push_str(&abc);
        subject_pronoun
    }

    pub fn get_visible_ents_from_ent(&self, ent: &EntityID) -> Vec<EntityID> {
        if let Some(e_pos) = self.ent_loc_index.get(&ent) {
            let render_width = 100;
            let render_height = 100;
            let w_radius = render_width / 2;
            let h_radius = render_height / 2;
            let same_z = locate_square(e_pos, w_radius as i64, h_radius as i64);

            let mut bop = Vec::new();

            let local_ents = self.entity_tree.locate_in_envelope(&same_z);

            for entt in local_ents {
                if &entt.entity_id != ent {
                    bop.push(entt.clone());
                }
            }

            bop.sort_by(|a, b| a.distance_2(e_pos).cmp(&b.distance_2(e_pos)));

            let meo = bop.iter().map(|x| x.entity_id).collect();

            return meo;
        } else {
            return Vec::new();
        }
    }

    pub fn create_client_render_packet_for_entity(
        &self,
        ent: &EntityID,
        render_rect: &Rect,
    ) -> RenderPacket {
        if let Some(e_pos) = self.ent_loc_index.get(ent) {
            let render_width = render_rect.width;
            let render_height = render_rect.height;
            let w_radius = render_width / 2;
            let h_radius = render_height / 2;
            let same_z = locate_square(e_pos, w_radius as i64, h_radius as i64);

            let local_ents = self.entity_tree.locate_in_envelope(&same_z);
            let local_voxels = self.voxeltile_grid.locate_in_envelope(&same_z);

            let local_actions = self
                .server_stuff
                .output_queue
                .locate_within_distance(e_pos.clone(), LOCAL_RANGE);

            let bottom_left_of_game_screen = (e_pos.0 - w_radius as i64, e_pos.1 - h_radius as i64);

            // THIS GRID IS INDEXD Y FIRST
            let mut voxel_grid = create_2d_array(render_width.into(), render_height.into());

            let mut ent_vec = Vec::new();

            for pc in local_ents {
                let relative_point_x = pc.point.0 - bottom_left_of_game_screen.0;
                let relative_point_y = pc.point.1 - bottom_left_of_game_screen.1;
                let et = self.entity_map.get(&pc.entity_id).unwrap().clone();

                ent_vec.push(((relative_point_x, relative_point_y), et.to_graphictriple()))
            }

            for lv in local_voxels {
                let relative_point_x = lv.voxel_pos.0 - bottom_left_of_game_screen.0;
                let relative_point_y = lv.voxel_pos.1 - bottom_left_of_game_screen.1;

                if (0 < relative_point_y)
                    && (relative_point_y < render_height as i64)
                    && (0 < relative_point_x)
                    && (relative_point_x < render_width as i64)
                {
                    let boop = lv.to_graphic();
                    voxel_grid[relative_point_y as usize][relative_point_x as usize] = boop;
                }
            }

            //merge grids

            for (ent_relative, ent_graphic) in ent_vec {
                if (0 < ent_relative.1)
                    && (ent_relative.1 < render_height as i64)
                    && (0 < ent_relative.0)
                    && (ent_relative.0 < render_width as i64)
                {
                    if voxel_grid[ent_relative.1 as usize][ent_relative.0 as usize].0
                        != String::from("@")
                    {
                        voxel_grid[ent_relative.1 as usize][ent_relative.0 as usize].0 =
                            ent_graphic.0;
                        voxel_grid[ent_relative.1 as usize][ent_relative.0 as usize].1 =
                            ent_graphic.1;
                    }
                }
            }

            for la in local_actions {
                //   actions.push(la.clone());
            }

            RenderPacket {
                spans_to_render: voxel_grid,

                messages_to_render: Vec::new(),
            }
        } else {
            // println!("DESSSSSS");
            RenderPacket::new()
        }
    }

    pub fn create_game_data_packet_for_entity(&self, ent: &EntityID) -> Option<GameDataPacket> {
        if let Some(e_pos) = self.ent_loc_index.get(ent) {
            let local_ents = self
                .entity_tree
                .locate_within_distance(e_pos.clone(), LOCAL_RANGE * 2);
            let local_voxels = self
                .voxeltile_grid
                .locate_within_distance(e_pos.clone(), LOCAL_RANGE / 2);
            let local_actions = self
                .server_stuff
                .output_queue
                .locate_within_distance(e_pos.clone(), LOCAL_RANGE / 8);

            let mut e_info = Vec::new();
            let mut v_diffs = Vec::new();
            let mut actions = Vec::new();

            for pc in local_ents {
                let et = self.entity_map.get(&pc.entity_id).unwrap().clone();
                e_info.push(EntityPacket {
                    entity_pos: pc.point.clone(),
                    entity_id: pc.entity_id.clone(),
                    entity_type: et.clone(),
                })
            }

            for vd in local_voxels {
                v_diffs.push(vd.clone());
            }

            for la in local_actions {
                actions.push(la.clone());
            }

            Some(GameDataPacket {
                entity_info: e_info,
                voxel_diffs: v_diffs,
                action_info: actions,
            })
        } else {
            None
        }
    }

    pub fn set_ent_loc(&mut self, ent: &EntityID, destination: &MyPoint) {
        if let Some(xyz) = self.ent_loc_index.get(ent) {
            self.entity_tree.remove(&PositionComponent {
                entity_id: ent.clone(),
                point: xyz.clone(),
            });
            self.entity_tree.insert(PositionComponent {
                entity_id: ent.clone(),
                point: destination.clone(),
            });
            self.ent_loc_index.insert(ent.clone(), destination.clone());
        }
    }

    pub fn move_entity_in_direction(
        &mut self,
        ent: &EntityID,
        cd: &CardinalDirection,
    ) -> SuccessType {
        if let Some(xyz) = self.ent_loc_index.get(ent) {
            let dir_point = cd.to_xyz();
            let goal = add_two_points(&xyz, &dir_point);

            if !self.entity_blocks_movement_at(&goal) {
                self.set_ent_loc(ent, &goal);
                return SuccessType::Success;
            } else {
                SuccessType::Failure
            }
        } else {
            SuccessType::Failure
        }
    }
}
