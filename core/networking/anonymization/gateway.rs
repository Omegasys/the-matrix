pub enum AnonymizationLayer {
    Tor,
    Freenet,
    Lokinet,
    Zeronet,
}

pub struct AnonymizationGateway {
    layers: Vec<AnonymizationLayer>,
}

impl AnonymizationGateway {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn add_layer(mut self, layer: AnonymizationLayer) -> Self {
        self.layers.push(layer);
        self
    }

    pub fn route(&self, mut data: Vec<u8>) -> Vec<u8> {
        for layer in &self.layers {
            data = match layer {
                AnonymizationLayer::Tor => crate::networking::anonymization::tor::route_through_tor(&data),
                AnonymizationLayer::Freenet => crate::networking::anonymization::freenet::route_through_freenet(&data),
                AnonymizationLayer::Lokinet => crate::networking::anonymization::lokinet::route_through_lokinet(&data),
                AnonymizationLayer::Zeronet => crate::networking::anonymization::zeronet::route_through_zeronet(&data),
            };
        }
        data
    }

    pub fn unwrap(&self, mut data: Vec<u8>) -> Vec<u8> {
        for layer in self.layers.iter().rev() {
            data = match layer {
                AnonymizationLayer::Tor => crate::networking::anonymization::tor::unwrap_tor(&data),
                AnonymizationLayer::Freenet => crate::networking::anonymization::freenet::unwrap_freenet(&data),
                AnonymizationLayer::Lokinet => crate::networking::anonymization::lokinet::unwrap_lokinet(&data),
                AnonymizationLayer::Zeronet => crate::networking::anonymization::zeronet::unwrap_zeronet(&data),
            };
        }
        data
    }
}
