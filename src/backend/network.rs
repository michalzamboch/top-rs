#![allow(dead_code)]

use std::cmp;
use sysinfo::{NetworkData, NetworkExt, NetworksExt, System, SystemExt};

#[derive(Debug, Clone, Hash, Default, PartialEq, Eq)]
pub struct Networking {
    max_received_net: Vec<u64>,
    max_transmited_net: Vec<u64>,
}

impl Networking {
    pub fn new(sys: &mut System) -> Networking {
        let network_count = Networking::get_network_count(sys);

        Networking {
            max_received_net: vec![0; network_count],
            max_transmited_net: vec![0; network_count],
        }
    }

    fn get_network_count(sys: &mut System) -> usize {
        sys.refresh_networks();
        sys.refresh_networks_list();

        let net = sys.networks();
        net.iter().count()
    }

    pub fn network(&mut self, sys: &System) {
        for (index, net_info) in sys.networks().into_iter().enumerate() {
            self.max_received_net[index] = self.get_max_received(index, net_info.1);

            self.max_transmited_net[index] = self.get_max_transmited(index, net_info.1);
        }
    }

    fn get_max_received(&self, index: usize, net_info: &NetworkData) -> u64 {
        let received = self.max_received_net[index];
        cmp::max(received, net_info.received())
    }

    fn get_max_transmited(&self, index: usize, net_info: &NetworkData) -> u64 {
        let transmitted = self.max_transmited_net[index];
        cmp::max(transmitted, net_info.transmitted())
    }

    fn print_net_info(&self, index: usize, net_info: (&String, &NetworkData)) {
        println!(
            "{} {}: {}/{} B",
            index,
            net_info.0,
            net_info.1.received(),
            net_info.1.transmitted()
        );
    }
}
