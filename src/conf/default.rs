use super::Conf;

impl Default for Conf {
    fn default() -> Self {
        Conf {
            size: 512,
            octaves: 8,
            persistence: 0.5,
            lacunarity: 2.0,
            frequency: 1.0,
            amplitude: 1.0,
            seed: 0,
            zoom: 1.0,
            scale: (1.0, 1.0),
            bias: 0.0,
            output: "output.svg".to_string(),
        }
    }
}
