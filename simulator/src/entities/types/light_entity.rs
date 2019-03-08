use crate::components::types::constant::Identifier;
use crate::components::types::statics::trafficlight::Light;
use crate::entities::entity_type::Instantiable;
use crate::metrics::identifier_deserialize;
use specs::prelude::{Entities, LazyUpdate, Read};
use specs::Builder;
use specs::World;

#[derive(Deserialize, Debug)]
pub struct LightEntity {
    #[serde(deserialize_with = "identifier_deserialize")]
    pub id: Identifier,
    pub light: Light,
}

impl<'a> Instantiable<'a> for LightEntity {
    fn create(&self, world: &mut World) {
        world
            .create_entity()
            .with(self.id.clone())
            .with(self.light)
            .build();
    }
    fn spawn(&self, entities: &Entities<'a>, updater: Read<'a, LazyUpdate>) {
        let entity = entities.create();
        updater.insert(entity, self.id.clone());
        updater.insert(entity, self.light);
    }
}
