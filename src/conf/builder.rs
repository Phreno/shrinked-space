use super::Conf;

pub trait ConfBuilder {
    fn with_size(&mut self, size: i32) -> &mut Self;
    fn with_octaves(&mut self, octaves: i32) -> &mut Self;
    fn with_persistence(&mut self, persistence: f64) -> &mut Self;
    fn with_lacunarity(&mut self, lacunarity: f64) -> &mut Self;
    fn with_frequency(&mut self, frequency: f64) -> &mut Self;
    fn with_amplitude(&mut self, amplitude: f64) -> &mut Self;
    fn with_seed(&mut self, seed: i32) -> &mut Self;
    fn with_zoom(&mut self, zoom: f64) -> &mut Self;
    fn with_scale(&mut self, scale: (f64, f64)) -> &mut Self;
    fn with_bias(&mut self, bias: f64) -> &mut Self;
    fn with_output(&mut self, output: String) -> &mut Self;
}

impl ConfBuilder for Conf {
    fn with_size(&mut self, size: i32) -> &mut Self {
        self.size = size;
        self
    }

    fn with_octaves(&mut self, octaves: i32) -> &mut Self {
        self.octaves = octaves;
        self
    }

    fn with_persistence(&mut self, persistence: f64) -> &mut Self {
        self.persistence = persistence;
        self
    }

    fn with_lacunarity(&mut self, lacunarity: f64) -> &mut Self {
        self.lacunarity = lacunarity;
        self
    }

    fn with_frequency(&mut self, frequency: f64) -> &mut Self {
        self.frequency = frequency;
        self
    }

    fn with_amplitude(&mut self, amplitude: f64) -> &mut Self {
        self.amplitude = amplitude;
        self
    }

    fn with_seed(&mut self, seed: i32) -> &mut Self {
        self.seed = seed;
        self
    }

    fn with_zoom(&mut self, zoom: f64) -> &mut Self {
        self.zoom = zoom;
        self
    }

    fn with_scale(&mut self, scale: (f64, f64)) -> &mut Self {
        self.scale = scale;
        self
    }

    fn with_bias(&mut self, bias: f64) -> &mut Self {
        self.bias = bias;
        self
    }

    fn with_output(&mut self, output: String) -> &mut Self {
        self.output = output;
        self
    }
}
