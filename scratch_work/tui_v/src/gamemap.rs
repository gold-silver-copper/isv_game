use crate::*;
use pathfinding::prelude::bfs;
pub struct GameMap {
    pub voxeltile_grid: RTree<Voxel>,
    pub ent_types_copy: HashMap<EntityID, EntityType>,
}

impl Default for GameMap {
    fn default() -> Self {
        let meow = gen_world();

        let mut batchvec = Vec::new();
        for x in 0..MAP_SIZE {
            for y in 0..MAP_SIZE {
                let val = meow.get_value(x as usize, y as usize);
                let floor = if val > -0.3 {
                    Floor::Grass
                } else if val > -0.6 {
                    Floor::Dirt
                } else if val > -0.8 {
                    Floor::Gravel //water
                } else {
                    Floor::Sand
                };
                let wall = if (val > 0.50) && (val < 0.60) {
                    Some(Furniture::Tree)
                } else {
                    None //water
                };
                batchvec.push(Voxel {
                    floor: Some(floor),
                    furniture: wall,

                    entity_set: Vec::new(),
                    voxel_pos: (x, y),
                });
            }
        }

        let newtree = RTree::bulk_load(batchvec);

        GameMap {
            voxeltile_grid: newtree,
            ent_types_copy: HashMap::new(),
        }
    }
}

impl GameMap {
    pub fn set_voxel_at(&mut self, vox: Voxel) {
        if let Some(boop) = self.voxeltile_grid.locate_at_point_mut(&vox.voxel_pos) {
            *boop = vox;
        } else {
            self.voxeltile_grid.insert(vox)
        }
    }

    pub fn get_voxel_at(&self, point: &MyPoint) -> Option<&Voxel> {
        if let Some(boop) = self.voxeltile_grid.locate_at_point(point) {
            Some(boop)
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

    pub fn generate_visible_ents_from_point(&self, ent_pos_comp: &MyPoint) -> Vec<EntityID> {
        let e_pos = (ent_pos_comp.0.clone(), ent_pos_comp.1.clone());

        let same_z = locate_square(&e_pos, FOV_RANGE, FOV_RANGE);

        let local_voxels = self.voxeltile_grid.locate_in_envelope(&same_z);

        let fov = field_of_view_set(
            BracketPoint {
                x: e_pos.0,
                y: e_pos.1,
            },
            FOV_RANGE,
            self,
        );

        let mut visible_ents = Vec::new();

        for lv in local_voxels {
            let bp = BracketPoint {
                x: lv.voxel_pos.0,
                y: lv.voxel_pos.1,
            };

            if fov.contains(&bp) {
                for ent in &lv.entity_set {
                    visible_ents.push(ent.clone());
                }
            }
        }
        visible_ents
    }

    pub fn create_client_render_packet_for_entity(
        &self,
        ent_pos_comp: &MyPoint,
        render_rect: &Rect,

        highlighted_line: Option<BresenhamInclusive>,
        seen_tiles: &HashSet<BracketPoint>,
    ) -> RenderPacket {
        {
            let render_width = render_rect.width;
            let render_height = render_rect.height;
            let w_radius = render_width / 2;
            let h_radius = render_height / 2;

            let e_pos = (ent_pos_comp.0.clone(), ent_pos_comp.1.clone());

            let same_z = locate_square(
                &e_pos,
                w_radius as CoordinateUnit,
                h_radius as CoordinateUnit,
            );

            let local_voxels = self.voxeltile_grid.locate_in_envelope(&same_z);

            let bottom_left_of_game_screen = (
                e_pos.0 - w_radius as CoordinateUnit,
                e_pos.1 - h_radius as CoordinateUnit,
            );

            // THIS GRID IS INDEXD Y FIRST
            let mut voxel_grid = create_2d_array(render_width.into(), render_height.into());

            let fov = field_of_view_set(
                BracketPoint {
                    x: e_pos.0,
                    y: e_pos.1,
                },
                FOV_RANGE,
                self,
            );

            for lv in local_voxels {
                let relative_point_x = lv.voxel_pos.0 - bottom_left_of_game_screen.0;
                let relative_point_y = lv.voxel_pos.1 - bottom_left_of_game_screen.1;

                if (0 < relative_point_y)
                    && (relative_point_y < render_height as CoordinateUnit)
                    && (0 < relative_point_x)
                    && (relative_point_x < render_width as CoordinateUnit)
                {
                    let bp = BracketPoint {
                        x: lv.voxel_pos.0,
                        y: lv.voxel_pos.1,
                    };

                    let boop = if fov.contains(&bp) {
                        lv.to_graphic(true, &self.ent_types_copy)
                    } else if seen_tiles.contains(&bp) {
                        lv.to_graphic(false, &self.ent_types_copy)
                    } else {
                        (
                            " ".into(),
                            ratatui::style::Color::Black,
                            ratatui::style::Color::Black,
                        )
                    };

                    voxel_grid[relative_point_y as usize][relative_point_x as usize] = boop;
                }
            }

            if let Some(highlight) = highlighted_line {
                for pointik in highlight {
                    let relative_point_x = pointik.x - bottom_left_of_game_screen.0;
                    let relative_point_y = pointik.y - bottom_left_of_game_screen.1;

                    if (0 < relative_point_y)
                        && (relative_point_y < render_height as CoordinateUnit)
                        && (0 < relative_point_x)
                        && (relative_point_x < render_width as CoordinateUnit)
                    {
                        let color_to_high =
                            &mut voxel_grid[relative_point_y as usize][relative_point_x as usize];

                        color_to_high.1 = dim(color_to_high.1, 0.4);
                        color_to_high.2 = dim(color_to_high.2, 1.4);
                    }
                }
            }

            //merge grids

            voxel_grid
        }
    }

    pub fn line_from_point_to_point_is_unblocked(
        &self,
        start: &MyPoint,
        end: &MyPoint,
        ent_types: &HashMap<EntityID, EntityType>,
    ) -> bool {
        let bresik = Bresenham::new(
            Point::from_tuple(start.clone()),
            Point::from_tuple(end.clone()),
        );

        for pointik in bresik {
            if let Some(voxik) = self.get_voxel_at(&(pointik.x, pointik.y)) {
                if voxik.blocks_movement(ent_types) {
                    return false;
                }
            }
        }
        true
    }
}

impl BaseMap for GameMap {
    fn is_opaque(&self, idx: usize) -> bool {
        let bp = self.index_to_point2d(idx);
        let mp = (bp.x, bp.y);
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
