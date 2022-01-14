use crate::material::material::Radiator;

pub struct SpectrumRadiator {
    min_wavelength: f32,
    max_wavelength: f32
}

impl SpectrumRadiator {
    pub fn new(min_wavelength: f32, max_wavelength: f32) -> SpectrumRadiator {
        SpectrumRadiator { min_wavelength, max_wavelength }
    }
}

impl Radiator for SpectrumRadiator {
    fn get_intensity(&self, wavelength: f32) -> f32 {
        if wavelength >= self.min_wavelength && wavelength < self.max_wavelength {
            1.0
        } else {
            0.0
        }
    }
}
