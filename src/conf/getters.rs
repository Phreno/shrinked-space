use super::Conf;

pub trait ConfGetter {
    fn get_size(&self) -> i32;
    fn get_octaves(&self) -> i32;
    fn get_persistence(&self) -> f64;
    fn get_lacunarity(&self) -> f64;
    fn get_frequency(&self) -> f64;
    fn get_amplitude(&self) -> f64;
    fn get_seed(&self) -> i32;
    fn get_zoom(&self) -> f64;
    fn get_scale(&self) -> (f64, f64);
    fn get_bias(&self) -> f64;
    fn get_output(&self) -> String;
}

impl ConfGetter for Conf {
    fn get_size(&self) -> i32 {
        self.size
    }

    fn get_octaves(&self) -> i32 {
        self.octaves
    }

    fn get_persistence(&self) -> f64 {
        self.persistence
    }

    fn get_lacunarity(&self) -> f64 {
        self.lacunarity
    }

    fn get_frequency(&self) -> f64 {
        self.frequency
    }

    fn get_amplitude(&self) -> f64 {
        self.amplitude
    }

    fn get_seed(&self) -> i32 {
        self.seed
    }

    fn get_zoom(&self) -> f64 {
        self.zoom
    }

    fn get_scale(&self) -> (f64, f64) {
        self.scale
    }

    fn get_bias(&self) -> f64 {
        self.bias
    }

    fn get_output(&self) -> String {
        self.output.clone()
    }
}
