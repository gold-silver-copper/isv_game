use crate::*;

#[derive(Clone, Debug)]
pub struct MyWorld {
    pub voxeltile_grid: RTree<Voxel>,

    pub small_rngik: SmallRng,

    pub world_seed: u32,
}

impl Default for MyWorld {
    fn default() -> Self {
        let rngik: u32 = 87243563;

        Self {
            small_rngik: SmallRng::seed_from_u64(rngik as u64),

            world_seed: rngik.clone(),

            voxeltile_grid: MyWorld::generate_test(rngik),
        }
    }
}

impl MyWorld {
    pub fn new_test() -> MyWorld {
        let mut x = MyWorld::default();

        x
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

    pub fn create_client_render_packet_for_entity(
        &self,
        ent_pos_comp: &GamePosition,
        render_rect: &Rect,
        ent_vec: std::vec::Vec<(&components::GamePosition, &components::GameRenderable)>,
    ) -> RenderPacket {
        {
            let render_width = render_rect.width;
            let render_height = render_rect.height;
            let w_radius = render_width / 2;
            let h_radius = render_height / 2;

            let e_pos = (ent_pos_comp.x.clone(), ent_pos_comp.y.clone());

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
                    let boop = lv.to_graphic();
                    voxel_grid[relative_point_y as usize][relative_point_x as usize] = boop;
                }
            }

            //merge grids

            for (ent_pos, ent_renderable) in ent_vec {
                let ent_relative = (
                    ent_pos.x - bottom_left_of_game_screen.0,
                    ent_pos.y - bottom_left_of_game_screen.1,
                );
                let graphic = ent_renderable.to_graphictriple();

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
