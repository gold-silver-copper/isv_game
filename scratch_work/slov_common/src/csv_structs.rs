use crate::*;





//id,isv,furn_tags,symbol

#[derive(Debug, Deserialize, Clone)]
pub struct VoxelCSV {
    pub voxel_id: String,
    pub latin: String,
    pub voxel_type: String,
    pub symbol: String,
    #[serde(deserialize_with = "deserialize_color")]
    pub fg_color: RatColor,
    #[serde(deserialize_with = "deserialize_color")]
    pub bg_color: RatColor,
}

impl Default for VoxelCSV {

    fn default() -> Self {

        Self{
            voxel_id: String::new(),
            latin: String::new(),
            voxel_type: String::new(),
            symbol: String::new(),
            fg_color: RatColor::White,
            bg_color: RatColor::Black,

        }
    }
}





#[derive(Debug, Deserialize, Clone)]
pub struct CSVTypeStore {
    pub voxels: HashMap<String,VoxelCSV>
}

impl CSVTypeStore {


    pub fn voxel_graphic_from_id(&self, voxel_id: &str) -> GraphicTriple {



        let voxel_info =  match self.voxels.get(voxel_id) {
            Some(x) => x.clone(),
            None => VoxelCSV::default()
        };

        (voxel_info.symbol,voxel_info.fg_color,voxel_info.bg_color)




    }


}

impl Default for CSVTypeStore {
    fn default() -> Self {
        let data_csv = include_bytes!("../../assets/data/latin_floors.csv");
   

        let mut csv_reader = csv::Reader::from_reader(data_csv.as_slice());
    
        let mut voxel_map = HashMap::new();
        for data_item in csv_reader.deserialize() {
            let data_item: VoxelCSV = data_item.unwrap();
    
            voxel_map.insert(data_item.voxel_id.clone(),data_item);
        }
    
    
      
    
        Self{
            voxels: voxel_map,
    
    
    
    
    
    
    
        }

    }

}



pub fn deserialize_color<'de, D>(deserializer: D) -> Result<RatColor, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let parts: Vec<u8> = s
        .split(',')
        .map(|part| part.trim().parse().unwrap_or(0))
        .collect();

    if parts.len() != 3 {
        return Err(serde::de::Error::custom("invalid color format"));
    }

    Ok(RatColor::Rgb(parts[0], parts[1], parts[2]))
}

