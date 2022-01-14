use glam::Vec3;
use crate::tracer::Photon;

pub struct Plotter {
    width: u16,
    height: u16,
    aspect_ratio: f32,
    buffer: Box<[Vec3]>
}

impl Plotter {

    pub fn new_merged(p1: &Plotter, p2: &Plotter) -> Plotter {
        let merged_buffer = p1.buffer.iter().zip(p2.buffer.iter())
            .map(|a|  a.0.clone() + a.1.clone())
            .collect::<Vec<Vec3>>()
            .into_boxed_slice();
        Plotter { width: p1.width, height: p1.height, aspect_ratio: p1.aspect_ratio, buffer: merged_buffer }
    }

    pub fn new(width: u16, height: u16) -> Plotter{
        let x = width as f32;
        Plotter {width, height, aspect_ratio: (width as f32 / height as f32), buffer: vec![Vec3::new(0.0, 0.0, 0.0); (width as i32 * height as i32) as usize].into_boxed_slice()}
    }

    pub fn merge(&mut self, other: Plotter) {
        self.buffer = self.buffer
            .iter()
            .zip(other.buffer.iter())
            .map(|(a, b)| a.clone() + b.clone())
            .collect::<Vec<Vec3>>()
            .into_boxed_slice();
    }

    pub fn plot_photon(&mut self, photon: Photon) {
        // println!("x: {} y: {} wavelength: {} intensity: {}", photon.x, photon.y, photon.wavelength, photon.strength);
        let cie = Plotter::wavelength_to_cie(photon.wavelength);
        self.plot_pixel(photon.x, photon.y, cie * photon.strength);
    }

    fn plot_pixel(&mut self, x: f32, y: f32, cie: Vec3) {
        let px = (x * 0.5 + 0.5) * (self.width as f32 - 1.0);
        let py = (y * self.aspect_ratio * 0.5 + 0.5) * (self.height as f32 - 1.0);
        let px1 = 0.max((px.floor() as i32).min(self.width as i32 - 1));
        let px2 = 0.max((px.floor() as i32).min(self.width as i32 - 1));
        let py1 = 0.max((py.floor() as i32).min(self.height as i32 - 1));
        let py2 = 0.max((py.floor() as i32).min(self.height as i32 - 1));

        let cx = px - px1 as f32;
        let cy = py - py1 as f32;
        let c11 = (1.0 - cx) * (1.0 - cy);
        let c12 = (1.0 - cx) * cy;
        let c21 = cx * (1.0 - cy);
        let c22 = cx * cy;
        let i11 = (py1 * self.width as i32 + px1) as usize;
        let i12 = (py1 * self.width as i32 + px2) as usize;
        let i21 = (py2 * self.width as i32 + px1) as usize;
        let i22 = (py2 * self.width as i32 + px2) as usize;

        self.buffer[i11] = self.buffer[i11] + cie * c11;
        self.buffer[i12] = self.buffer[i12] + cie * c12;
        self.buffer[i21] = self.buffer[i21] + cie * c21;
        self.buffer[i22] = self.buffer[i22] + cie * c22;
    }

    fn wavelength_to_cie(wavelength: f32) -> Vec3 {
        let indexf = (wavelength - 380.0) / 5.0;
        let index = indexf as i32;
        let remainder = indexf - index as f32;
        if index < -1 || index > 80 {
            return Vec3::new(0.0, 0.0, 0.0);//Wavelength invisible
        }
        else if index == -1 {
            return Vec3::new(
                CIE_X[0] * remainder,
                CIE_Y[0] * remainder,
                CIE_Z[0] * remainder);
        }
        else if index == 80 {
            return Vec3::new(
                CIE_X[80] * remainder,
                CIE_Y[80] * remainder,
                CIE_Z[80] * remainder);
        }
        else {
            return Vec3::new(
                CIE_X[index as usize] * (1.0 - remainder) + CIE_X[index as usize + 1] * remainder,
                CIE_Y[index as usize] * (1.0 - remainder) + CIE_Y[index as usize + 1] * remainder,
                CIE_Z[index as usize] * (1.0 - remainder) + CIE_Z[index as usize + 1] * remainder
            );
        }
    }

    pub fn tone_map(&self) -> Vec<(u8, u8, u8)> {
        let max_intensity = self.calculate_exposure();
        let ln_4 = 4.0_f32.ln();



        let rgb_vec: Vec<(u8, u8, u8)> = self.buffer.iter()
            .map(|cie| Vec3::new(
                (cie.x / max_intensity) / ln_4,
                (cie.y / max_intensity) / ln_4,
                (cie.z / max_intensity) / ln_4
            ))
            .map(|cie| Plotter::cie_to_rgb(&cie))
            .map(|rgb| ((Plotter::clamp(rgb.x) * 255.0) as u8, (Plotter::clamp(rgb.y) * 255.0) as u8, (Plotter::clamp(rgb.z) * 255.0) as u8))
            .collect();

        rgb_vec
    }

    fn cie_to_rgb(cie: &Vec3) -> Vec3 {
        let r = 3.2406 * cie.x - 1.5372 * cie.y - 0.4986 * cie.z;
        let g = -0.9689 * cie.x + 1.8758 * cie.y + 0.0415 * cie.z;
        let b =  0.0557 * cie.x - 0.2040 * cie.y + 1.0570 * cie.z;
        Vec3::new(Plotter::gamma_correct(r), Plotter::gamma_correct(g), Plotter::gamma_correct(b))
    }

    fn gamma_correct(d: f32) -> f32 {
        if d < 0.0031308 {
            return 12.92 * d;
        }
        return 1.055 * d.powf(1.0/2.2) - 0.055;
    }

    fn calculate_exposure(&self) -> f32 {
        let mean = self.buffer.iter()
            .map(|x| x.y)
            .sum::<f32>() as f32 / self.buffer.len() as f32;
        let sqr_mean = self.buffer.iter()
            .map(|x| x.y * x.y)
            .sum::<f32>() as f32 / self.buffer.len() as f32;
        let variance = sqr_mean - mean * mean;

        mean + variance.sqrt()
    }

    fn clamp(v: f32) -> f32 {
        return if v < 0.0 {
            0.0
        } else if v > 1.0 {
            1.0
        } else {
            v
        }
    }
}

const CIE_X: [f32;81] = [
    0.001368,
    0.002236,
    0.004243,
    0.007650,
    0.014310,
    0.023190,
    0.043510,
    0.077630,
    0.134380,
    0.214770,
    0.283900,
    0.328500,
    0.348280,
    0.348060,
    0.336200,
    0.318700,
    0.290800,
    0.251100,
    0.195360,
    0.142100,
    0.095640,
    0.057950,
    0.032010,
    0.014700,
    0.004900,
    0.002400,
    0.009300,
    0.029100,
    0.063270,
    0.109600,
    0.165500,
    0.225750,
    0.290400,
    0.359700,
    0.433450,
    0.512050,
    0.594500,
    0.678400,
    0.762100,
    0.842500,
    0.916300,
    0.978600,
    1.026300,
    1.056700,
    1.062200,
    1.045600,
    1.002600,
    0.938400,
    0.854450,
    0.751400,
    0.642400,
    0.541900,
    0.447900,
    0.360800,
    0.283500,
    0.218700,
    0.164900,
    0.121200,
    0.087400,
    0.063600,
    0.046770,
    0.032900,
    0.022700,
    0.015840,
    0.011359,
    0.008111,
    0.005790,
    0.004109,
    0.002899,
    0.002049,
    0.001440,
    0.001000,
    0.000690,
    0.000476,
    0.000332,
    0.000235,
    0.000166,
    0.000117,
    0.000083,
    0.000059,
    0.000042
];

/// CIE Y tristimulus values, at 5nm intervals, starting at 380 nm.
const CIE_Y: [f32;81] = [
    0.000039,
    0.000064,
    0.000120,
    0.000217,
    0.000396,
    0.000640,
    0.001210,
    0.002180,
    0.004000,
    0.007300,
    0.011600,
    0.016840,
    0.023000,
    0.029800,
    0.038000,
    0.048000,
    0.060000,
    0.073900,
    0.090980,
    0.112600,
    0.139020,
    0.169300,
    0.208020,
    0.258600,
    0.323000,
    0.407300,
    0.503000,
    0.608200,
    0.710000,
    0.793200,
    0.862000,
    0.914850,
    0.954000,
    0.980300,
    0.994950,
    1.000000,
    0.995000,
    0.978600,
    0.952000,
    0.915400,
    0.870000,
    0.816300,
    0.757000,
    0.694900,
    0.631000,
    0.566800,
    0.503000,
    0.441200,
    0.381000,
    0.321000,
    0.265000,
    0.217000,
    0.175000,
    0.138200,
    0.107000,
    0.081600,
    0.061000,
    0.044580,
    0.032000,
    0.023200,
    0.017000,
    0.011920,
    0.008210,
    0.005723,
    0.004102,
    0.002929,
    0.002091,
    0.001484,
    0.001047,
    0.000740,
    0.000520,
    0.000361,
    0.000249,
    0.000172,
    0.000120,
    0.000085,
    0.000060,
    0.000042,
    0.000030,
    0.000021,
    0.000015
];

/// CIE Z tristimulus values, at 5nm intervals, starting at 380 nm.
const CIE_Z: [f32;81] = [
    0.006450,
    0.010550,
    0.020050,
    0.036210,
    0.067850,
    0.110200,
    0.207400,
    0.371300,
    0.645600,
    1.039050,
    1.385600,
    1.622960,
    1.747060,
    1.782600,
    1.772110,
    1.744100,
    1.669200,
    1.528100,
    1.287640,
    1.041900,
    0.812950,
    0.616200,
    0.465180,
    0.353300,
    0.272000,
    0.212300,
    0.158200,
    0.111700,
    0.078250,
    0.057250,
    0.042160,
    0.029840,
    0.020300,
    0.013400,
    0.008750,
    0.005750,
    0.003900,
    0.002750,
    0.002100,
    0.001800,
    0.001650,
    0.001400,
    0.001100,
    0.001000,
    0.000800,
    0.000600,
    0.000340,
    0.000240,
    0.000190,
    0.000100,
    0.000050,
    0.000030,
    0.000020,
    0.000010,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000,
    0.000000
];