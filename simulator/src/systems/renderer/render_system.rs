use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use specs::{Read, ReadExpect, ReadStorage, System, Join, WriteExpect};
use graphics::*;
use crate::components::constant::Drawer;
use crate::components::dynamic::Position;
use crate::components::dynamic::Speed;
use crate::components::statics::trafficlight::Light;
use crate::ressources::lane_graph::IntersectionData;
use crate::ressources::lane_graph::LaneData;
use crate::ressources::lane_graph::LaneGraph;
use crate::systems::renderer::Color;
use crate::systems::renderer::drawableshape::Drawable;
use petgraph::graphmap::Neighbors;

const zoom_factor: f64 = 2.0;

pub struct DrawClear;
impl<'a> System<'a> for DrawClear {
    type SystemData = (WriteExpect<'a, GlGraphics>, ReadExpect<'a, RenderArgs>);

    fn run(&mut self, (mut g_handle, args): Self::SystemData) {
        g_handle.draw(args.viewport(), |_, gl| {
            clear(Color::GREENGRASS.get(), gl);
        });
    }
}

pub struct DrawMap;
impl<'a> System<'a> for DrawMap {
    type SystemData = (
        ReadExpect<'a, LaneGraph>,
        WriteExpect<'a, GlGraphics>,
        ReadExpect<'a, RenderArgs>,
    );

    fn run(&mut self, (lane_graph, mut g_handle, args): Self::SystemData) {
        for (nodeid, node) in lane_graph.get_nodes() {
            let position_node: (f64, f64) = node.position;
            let mut voisins: Neighbors<'_, u64, petgraph::Directed> = lane_graph.graph.neighbors(*nodeid);
            while let Some(voisin) = voisins.next() {
                let lane: &LaneData = lane_graph.lane_between((*nodeid, voisin));
                let lane_width: f64 = lane.width.unwrap().value_unsafe;
                let position_voisin: (f64, f64) = lane_graph.intersection(voisin).position;
                
                g_handle.draw(args.viewport(), |c, gl| {
                    draw_lane_between_two_points(
                        (position_node.0 * zoom_factor, position_node.1 * zoom_factor),
                        (position_voisin.0 * zoom_factor, position_voisin.1 * zoom_factor),
                        lane_width,
                        Color::GRAY,
                        c, gl
                    );
                });
            }
        }
    }
}

pub struct DrawTrafficLights;
impl<'a> System<'a> for DrawTrafficLights {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Light>,
        ReadStorage<'a, Drawer>,
        WriteExpect<'a, GlGraphics>,
        ReadExpect<'a, RenderArgs>,
    );

    fn run(&mut self, (positions, lights, drawers, mut g_handle, args): Self::SystemData) {
        for (position, light, drawer) in (&positions, &lights, &drawers).join() {
            g_handle.draw(args.viewport(), |c, gl| {
                drawer.figure.draw(
                    position.x.value_unsafe * zoom_factor,
                    position.y.value_unsafe * zoom_factor,
                    light.color.get_rendering_color(),
                    c, gl
                );
            });
        }
    }
}

pub struct DrawVehicles;
impl<'a> System<'a> for DrawVehicles {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Speed>,
        ReadStorage<'a, Drawer>,
        WriteExpect<'a, GlGraphics>,
        ReadExpect<'a, RenderArgs>,
    );

    fn run(&mut self, (positions, speeds, drawers, mut g_handle, args): Self::SystemData) {
        for (position, speed, drawer) in (&positions, &speeds, &drawers).join() {
            g_handle.draw(args.viewport(), |c, gl| {
                drawer.figure.draw(
                    position.x.value_unsafe * zoom_factor,
                    position.y.value_unsafe * zoom_factor,
                    Color::BLACK,
                    c, gl
                );
            });
        }
    }
}

fn draw_lane_between_two_points(
    p1: (f64, f64),
    p2: (f64, f64),
    width: f64,
    color: Color,
    c: Context, gl: &mut GlGraphics
) {
    let rectangle_length: f64 = (p2.0 - p1.0).hypot(p2.1 - p1.1);
    let rectangle_width: f64 = width;
    let rectangle_angle: f64 = (p2.1 - p1.1).atan2(p2.0 - p1.0);

    let transform = c
        .transform
        .trans(p1.0, p1.1)
        .rot_rad(rectangle_angle)
        .scale(rectangle_length, rectangle_width * zoom_factor);
    rectangle(color.get(), rectangle::square(0.0, 0.0, 1.0), transform, gl);
}