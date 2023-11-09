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

pub fn get_temperatures_vec_strings(sys: &System) -> Vec<Vec<String>> {
    sys.components()
        .par_iter()
        .map(create_temperature_vec_strings)
        .collect()
}

fn create_temperature_vec_strings(component: &Component) -> Vec<String> {
    vec![
        format!("{}", component.label()),
        format!("{}", component.temperature()),
    ]
}

pub fn get_temperatures_boxed(sys: &System) -> HashMap<String, Box<dyn ITemperature>> {
    sys.components()
        .par_iter()
        .map(component_temperature_box_to_tuple)
        .collect()
}

fn component_temperature_box_to_tuple(component: &Component) -> (String, Box<dyn ITemperature>) {
    let key = component.label().to_owned();
    let value = new_temperature_struct(component);

    (key, Box::new(value))
}

fn new_temperature_struct(component: &Component) -> Temperature {
    Temperature {
        value: component.temperature(),
        max: component.max(),
        critical: component.critical(),
    }
}
