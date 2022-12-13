use super::Conf;

impl Default for Conf {
    fn default() -> Self {
        Conf {
            size: 75,
            octaves: 1,
            persistence: 0.5,
            lacunarity: 0.5,
            frequency: 0.2,
            amplitude: 0.05,
            seed: 0,
            zoom: 1.0,
            scale: (0.5, 1.0),
            bias: 0.5,
            output: "output.svg".to_string(),
        }
    }
}
