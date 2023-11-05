#![allow(dead_code)]

use rayon::prelude::*;
use std::collections::HashMap;
use sysinfo::*;

use crate::types::temperature_traits::ITemperature;

#[derive(Debug, Clone, Default)]
pub struct Temperature {
    value: f32,
    max: f32,
    critical: Option<f32>,
}

impl ITemperature for Temperature {
    fn value(&self) -> String {
        format!("{} °C", self.value)
    }

    fn max(&self) -> String {
        format!("{} °C", self.max)
    }

    fn overheating(&self) -> bool {
        match self.critical {
            Some(critical) => critical <= self.value,
            None => false,
        }
    }
}

pub fn get_temperatures(sys: &System) -> HashMap<String, Temperature> {
    sys.components()
        .par_iter()
        .map(component_temperature_to_tuple)
        .collect()
}

pub fn get_temperatures_boxed(sys: &System) -> HashMap<String, Box<dyn ITemperature>> {
    sys.components()
        .par_iter()
        .map(component_temperature_box_to_tuple)
        .collect()
}

fn component_temperature_to_tuple(component: &Component) -> (String, Temperature) {
    let key = component.label().to_owned();
    let value = new_temperature_struct(component);

    (key, value)
}

fn component_temperature_box_to_tuple(component: &Component) -> (String, Box<dyn ITemperature>) {
    let key = component.label().to_owned();
    let value = new_temperature_struct(component);

    (key, Box::new(value))
}

fn new_temperature_box(component: &Component) -> Box<dyn ITemperature> {
    let temp = Temperature {
        value: component.temperature(),
        max: component.max(),
        critical: component.critical(),
    };

    Box::new(temp)
}

fn new_temperature_struct(component: &Component) -> Temperature {
    Temperature {
        value: component.temperature(),
        max: component.max(),
        critical: component.critical(),
    }
}
