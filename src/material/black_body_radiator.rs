use crate::material::material::Radiator;

const PLANCKS_CONSTANT: f64 = 6.62606957e-34;
const BOLTZMANNS_CONSTANT: f64 =  1.3806488e-23;
const SPEED_OF_LIGHT: f64 = 299792458.0;
const WIENS_CONSTANT: f64 = 2.897772126e-3;


pub struct BlackBodyRadiator {
    temperature: f64,
    normalization_factor: f64
}

impl BlackBodyRadiator {
    pub fn new(temperature: f64, intensity: f64) -> BlackBodyRadiator {
        BlackBodyRadiator {
            temperature,
            normalization_factor: intensity / BlackBodyRadiator::boltzmann_distribution((WIENS_CONSTANT / temperature) * 1.0e9, temperature)
        }
    }

    fn boltzmann_distribution(wavelength: f64, temperature: f64) -> f64 {
        let f = SPEED_OF_LIGHT / (wavelength * 1.0e-9);
        return (2.0 * PLANCKS_CONSTANT * f * f * f) /
            (SPEED_OF_LIGHT * SPEED_OF_LIGHT * ((PLANCKS_CONSTANT * f / (BOLTZMANNS_CONSTANT * temperature)).exp() - 1.0));
    }
}

impl Radiator for BlackBodyRadiator {
    fn get_intensity(&self, wavelength: f64) -> f64 {
        BlackBodyRadiator::boltzmann_distribution(wavelength, self.temperature)
    }
}
