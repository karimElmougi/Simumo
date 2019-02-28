//todo:: remove this when program will be complete
#![allow(dead_code)]
extern crate argparse;
#[macro_use]
extern crate erased_serde;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate specs_derive;

extern crate csv;
extern crate dimensioned as dim;
extern crate proc_macro2;
extern crate rand;
extern crate serde;
extern crate simumo_derive;
extern crate specs;
extern crate typeinfo;
extern crate typeinfo_derive;
extern crate uuid;
extern crate piston_window;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;



mod topology;
mod command_line;
mod types;

mod components;
mod configurations;
mod errors;
mod internal_prelude;
mod metrics;
mod osmgraph_api;
mod ressources;
mod rng;
mod simulation;
mod systems;
mod util;


fn main() {

    //let args = command_line::CommandLineArguments::parse();
    //let config = Configuration::from_path(&args.configuration_path).unwrap();
    //config.setup();
    //
    //if args.verbose {}

    let mut simulation = simulation::Simulation::new();
    simulation.run_simulation();



}
