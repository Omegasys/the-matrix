pub enum ObfuscationLayer {
    Shadowsocks,
    V2Ray,
    Meek,
    OPC,
    SoftLayer,
}

pub struct ObfuscationChain {
    layers: Vec<ObfuscationLayer>,
}

impl ObfuscationChain {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn add_layer(mut self, layer: ObfuscationLayer) -> Self {
        self.layers.push(layer);
        self
    }

    pub fn apply(&self, mut data: Vec<u8>) -> Vec<u8> {
        for layer in &self.layers {
            data = match layer {
                ObfuscationLayer::Shadowsocks => crate::networking::obfuscation::shadowsocks::obfuscate(&data),
                ObfuscationLayer::V2Ray => crate::networking::obfuscation::v2ray::wrap(&data),
                ObfuscationLayer::Meek => crate::networking::obfuscation::meek::disguise_as_http(&data),
                ObfuscationLayer::OPC => crate::networking::obfuscation::opc::opc_transform(&data),
                ObfuscationLayer::SoftLayer => crate::networking::obfuscation::soft_leather::wrap_soft(&data),
            };
        }
        data
    }

    pub fn reverse(&self, mut data: Vec<u8>) -> Vec<u8> {
        for layer in self.layers.iter().rev() {
            data = match layer {
                ObfuscationLayer::Shadowsocks => crate::networking::obfuscation::shadowsocks::deobfuscate(&data),
                ObfuscationLayer::V2Ray => crate::networking::obfuscation::v2ray::unwrap(&data),
                ObfuscationLayer::Meek => crate::networking::obfuscation::meek::extract(&data),
                ObfuscationLayer::OPC => crate::networking::obfuscation::opc::opc_reverse(&data),
                ObfuscationLayer::SoftLayer => crate::networking::obfuscation::soft_leather::unwrap_soft(&data),
            };
        }
        data
    }
}
