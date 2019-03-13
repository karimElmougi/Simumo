extern crate specs;
use crate::metrics::second_deserialize;
use crate::metrics::Fdim;
use crate::systems::renderer::Color;
use dim::si::{Second, S};
use specs::prelude::*;
use typeinfo::TypeInfo;
use typeinfo_derive::*;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TrafficLightColor {
    RED,
    YELLOW,
    GREEN,
}

impl TrafficLightColor {
    pub fn get_rendering_color(&self) -> Color {
        match self {
            TrafficLightColor::RED => Color::RED,
            TrafficLightColor::YELLOW => Color::YELLOW,
            TrafficLightColor::GREEN => Color::GREEN
        }
    }
}

#[derive(Copy, Clone, Component, TypeInfo, Debug, Deserialize)]
#[storage(VecStorage)]
pub struct Light {
    #[serde(rename = "initial_color")]
    pub color: TrafficLightColor,
    #[serde(deserialize_with = "second_deserialize")]
    pub max_green_time: Second<Fdim>,
    #[serde(deserialize_with = "second_deserialize")]
    pub max_yellow_time: Second<Fdim>,
    #[serde(deserialize_with = "second_deserialize")]
    pub time: Second<Fdim>,
}

impl Light {
    pub fn new(
        color: TrafficLightColor,
        max_green_time: Second<Fdim>,
        max_yellow_time: Second<Fdim>,
        time: Second<Fdim>,
    ) -> Self {
        Self {
            color,
            max_green_time,
            max_yellow_time,
            time,
        }
    }
    pub fn reset_to_green(&mut self) {
        self.color = TrafficLightColor::GREEN;
        self.time = self.max_green_time;
    }
    pub fn reset_to_yellow(&mut self) {
        self.color = TrafficLightColor::YELLOW;
        self.time = self.max_yellow_time;
    }
    pub fn reset_to_red(&mut self) {
        self.color = TrafficLightColor::RED;
        self.time = 0.0 * S;
    }
}
