use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub struct Voxel<T: Material + ToColor>  {
    pub floor: Option<Floor>,
    pub roof: Option<Roof<T>>,
    pub furniture: Option<Furniture>,
    pub entity_set: HashSet<EntityID>,
    pub voxel_pos: MyPoint,
}

impl<T: Material + ToColor>  Voxel<T> {
    pub fn to_graphic(
        &self,
        visible: bool,
        ent_types: &HashMap<EntityID, EntityType>,
    ) -> GraphicTriple {
        let mut floor = match &self.floor {
            Some(fl) => fl.to_graphic_triple(),
            None => (" ", RatColor::Black, RatColor::Black),
        };

        let mut plus_furn: GraphicTriple = match &self.furniture {
            Some(furn) => (furn.symbol(), furn.fg_color(), floor.2.clone()),
            None => floor,
        };

        if visible {
            for ent in &self.entity_set {
                if let Some(etyp) = ent_types.get(ent) {
                    let pik = etyp.to_graphic_triple();

                    if plus_furn.0 != "@" {
                        plus_furn = (pik.0, pik.1, plus_furn.2);
                    }
                }
            }
            plus_furn
        } else {
            let mut plus_roof: GraphicTriple = match &self.roof {
                Some(roof) => (roof.symbol(), roof.fg_color(), roof.bg_color()),
                None => plus_furn,
            };

            plus_roof.1 = dim(plus_roof.1, 0.3);
            plus_roof.2 = dim(plus_roof.2, 0.3);

            plus_roof
        }
    }

    pub fn blocks_movement(&self, ent_types: &HashMap<EntityID, EntityType>) -> bool {
        let furn_blocks = match &self.furniture {
            Some(furn) => furn.blocks_movement(),
            None => false,
        };

        if furn_blocks {
            return true;
        } else {
            for ent in &self.entity_set {
                if let Some(etyp) = ent_types.get(ent) {
                    let pik = etyp.blocks_movement();

                    if pik {
                        return true;
                    }
                }
            }
            return false;
        }
    }

    pub fn blocks_vision(&self) -> bool {
        match &self.furniture {
            Some(furn) => furn.blocks_vision(),
            None => false,
        }
    }
}

impl<T: Material + ToColor>  RTreeObject for Voxel<T> {
    type Envelope = AABB<(i64, i64)>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point((self.voxel_pos.0, self.voxel_pos.1))
    }
}

impl<T: Material + ToColor>  PointDistance for Voxel<T> {
    fn distance_2(&self, point: &(i64, i64)) -> i64 {
        self.voxel_pos.distance_2(point)
    }

    fn contains_point(&self, point: &(i64, i64)) -> bool {
        self.voxel_pos.contains_point(point)
    }
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Floor {
    Liquid(LiquidType),
    Earth(EarthType),
}

#[derive(Clone, Debug, Display, PartialEq)]
pub enum LiquidType {
    Water,
  
}

pub fn dim(color: RatColor, factor: f32) -> RatColor {
    match color {
        RatColor::Rgb(r, g, b) => RatColor::Rgb(
            ((r as f32 * factor).clamp(0.0, 127.0)) as u8,
            ((g as f32 * factor).clamp(0.0, 127.0)) as u8,
            ((b as f32 * factor).clamp(0.0, 127.0)) as u8,
        ),
        _ => RatColor::Gray,
    }
}
