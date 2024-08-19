use crate::*;

#[derive(Clone, Debug)]
pub struct GameMap {
    pub voxeltile_grid: RTree<Voxel>,
}

impl Default for GameMap {
    fn default() -> Self {
        let mut batchvec = Vec::new();
        for x in 0..MAP_SIZE {
            for y in 0..MAP_SIZE {
                let val = x as f32 / 8.0;
                let floor = if val > 0.2 {
                    DIRT_FLOOR
                } else if val > 0.0 {
                    CLAY_FLOOR
                } else if val > -0.9 {
                    SAND_FLOOR
                } else {
                    WATER_FLOOR //water
                };

                if x == 15 && y > 8 {
                    //Some(WALL_FURNITURE),
                    batchvec.push(Voxel {
                        floor: Some(floor),
                        furniture: Some(WALL_FURNITURE),
                        roof: None,
                        entity_set: HashSet::new(),
                        voxel_pos: (x, y),
                    });
                } else if x < 15 && y > 8 {
                    //Some(WALL_FURNITURE),
                    batchvec.push(Voxel {
                        floor: Some(floor),
                        furniture: None,
                        roof: Some(TEGULA_ROOF),
                        entity_set: HashSet::new(),
                        voxel_pos: (x, y),
                    });
                } else {
                    //Some(WALL_FURNITURE),
                    batchvec.push(Voxel {
                        floor: Some(floor),
                        furniture: None,
                        roof: None,
                        entity_set: HashSet::new(),
                        voxel_pos: (x, y),
                    });
                }
            }
        }

        let newtree = RTree::bulk_load(batchvec);

        GameMap {
            voxeltile_grid: newtree,
        }
    }
}

impl GameMap {
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

    pub fn get_mut_voxel_at(&mut self, point: &MyPoint) -> Option<&mut Voxel> {
        if let Some(boop) = self.voxeltile_grid.locate_at_point_mut(point) {
            Some(boop)
        } else {
            None
        }
    }

    pub fn create_client_render_packet_for_entity(
        &self,
        ent_pos_comp: &MyPoint,
        render_rect: &Rect,
        ent_types: &HashMap<EntityID, EntityType>
    ) -> RenderPacket {
        {
            let render_width = render_rect.width;
            let render_height = render_rect.height;
            let w_radius = render_width / 2;
            let h_radius = render_height / 2;

            let e_pos = (ent_pos_comp.0.clone(), ent_pos_comp.1.clone());

            let same_z = locate_square(&e_pos, w_radius as i64, h_radius as i64);

            let local_voxels = self.voxeltile_grid.locate_in_envelope(&same_z);

            let bottom_left_of_game_screen = (e_pos.0 - w_radius as i64, e_pos.1 - h_radius as i64);

            // THIS GRID IS INDEXD Y FIRST
            let mut voxel_grid = create_2d_array(render_width.into(), render_height.into());

            let fov = field_of_view_set(
                BracketPoint {
                    x: e_pos.0 as i32,
                    y: e_pos.1 as i32,
                },
                60,
                self,
            );

            //    println!("FOV IS {:#?}",fov);

            let mut visible_ents = Vec::new();

            for lv in local_voxels {
                let relative_point_x = lv.voxel_pos.0 - bottom_left_of_game_screen.0;
                let relative_point_y = lv.voxel_pos.1 - bottom_left_of_game_screen.1;

                if (0 < relative_point_y)
                    && (relative_point_y < render_height as i64)
                    && (0 < relative_point_x)
                    && (relative_point_x < render_width as i64)
                {
                    let bp = BracketPoint {
                        x: lv.voxel_pos.0 as i32,
                        y: lv.voxel_pos.1 as i32,
                    };

                    let boop = if fov.contains(&bp) {
                        for ent in &lv.entity_set {
                            visible_ents.push(ent.clone());
                        }

                        lv.to_graphic(true,ent_types)
                    } else {
                        lv.to_graphic(false,ent_types)
                    };

                    voxel_grid[relative_point_y as usize][relative_point_x as usize] = boop;
                }
            }

            //merge grids

            RenderPacket {
                voxel_grid: voxel_grid,
                ent_vec: visible_ents,
            }
        }
    }
}

impl BaseMap for GameMap {
    fn is_opaque(&self, idx: usize) -> bool {
        let bp = self.index_to_point2d(idx);
        let mp = (bp.x as i64, bp.y as i64);
        let vox = self.get_voxel_at(&mp);
        match vox {
            Some(voxik) => voxik.blocks_vision(),
            None => true,
        }
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }
}

impl Algorithm2D for GameMap {
    fn dimensions(&self) -> BracketPoint {
        BracketPoint::new(MAP_SIZE, MAP_SIZE)
    }
}
