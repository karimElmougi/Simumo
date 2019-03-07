use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::entities::entity_type::EntityType;
use super::generals::GeneralConfigurations;
use super::map::Map;
use super::systems::SystemsConfiguration;

/// Represent the root level configuration.
///
/// Todo: Can't handle empty field in serialization.
#[derive(Deserialize)]
pub struct Configuration {
    pub generals: GeneralConfigurations,
    pub map: Map,
    pub systems: SystemsConfiguration,
    pub entities: Vec<EntityType>
}

impl Configuration {
    pub fn from_path(args_path: &str) -> Result<Self, Box<Error>> {
        let config_path = Path::new(&args_path);
        let file = File::open(config_path)?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }
}
