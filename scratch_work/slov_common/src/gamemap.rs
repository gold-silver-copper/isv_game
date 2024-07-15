use crate::*;

#[derive(Clone, Debug)]
pub struct GameMap {
    pub voxeltile_grid: RTree<Voxel>,
}

impl GameMap {
    pub fn generate_test(seed: u32) -> RTree<Voxel> {
        let hasher = noise::permutationtable::PermutationTable::new(seed);
        let boop = generate_complex_planet();

        let mut batchvec = Vec::new();
        for x in 0..MAP_SIZE {
            for y in 0..MAP_SIZE {
                let val = boop.get_value(x as usize, y as usize);
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
                        entity_map: HashMap::new(),
                        voxel_pos: (x, y),
                    });
                } else {
                    //Some(WALL_FURNITURE),
                    batchvec.push(Voxel {
                        floor: Some(floor),
                        furniture: None,
                        roof: None,
                        entity_map: HashMap::new(),
                        voxel_pos: (x, y),
                    });
                }
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
        ent_vec: std::vec::Vec<(&MyPoint, GraphicTriple)>,
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

            for lv in local_voxels {
                let relative_point_x = lv.voxel_pos.0 - bottom_left_of_game_screen.0;
                let relative_point_y = lv.voxel_pos.1 - bottom_left_of_game_screen.1;

                if (0 < relative_point_y)
                    && (relative_point_y < render_height as i64)
                    && (0 < relative_point_x)
                    && (relative_point_x < render_width as i64)
                {
                    let boop = lv.to_graphic(true);
                    voxel_grid[relative_point_y as usize][relative_point_x as usize] = boop;
                }
            }

            //merge grids

            for (ent_pos, ent_renderable) in ent_vec {
                let ent_relative = (
                    ent_pos.0 - bottom_left_of_game_screen.0,
                    ent_pos.1 - bottom_left_of_game_screen.1,
                );
                let graphic = ent_renderable.clone();

                if (0 < ent_relative.1)
                    && (ent_relative.1 < render_height as i64)
                    && (0 < ent_relative.0)
                    && (ent_relative.0 < render_width as i64)
                {
                    if voxel_grid[ent_relative.1 as usize][ent_relative.0 as usize].0
                        != String::from("@")
                    {
                        voxel_grid[ent_relative.1 as usize][ent_relative.0 as usize].0 =
                            graphic.0.clone();
                        voxel_grid[ent_relative.1 as usize][ent_relative.0 as usize].1 =
                            graphic.1.clone();
                    }
                }
            }

            RenderPacket {
                voxel_grid: voxel_grid,
            }
        }
    }
}
