#![allow(dead_code)]

use rayon::prelude::*;
use std::collections::HashMap;
use sysinfo::*;

#[derive(Debug, Clone, Default)]
pub struct Temperature {
    value: f32,
    max: f32,
    critical: Option<f32>,
}

pub fn get_temperatures(sys: &System) -> HashMap<String, Temperature> {
    sys.components()
        .par_iter()
        .map(component_temperature_to_tuple)
        .collect()
}

fn component_temperature_to_tuple(component: &Component) -> (String, Temperature) {
    let key = component.label().to_owned();
    let value = new_temperature_struct(component);

    (key, value)
}

fn new_temperature_struct(component: &Component) -> Temperature {
    Temperature {
        value: component.temperature(),
        max: component.max(),
        critical: component.critical(),
    }
}
