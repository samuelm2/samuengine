
pub const PI: f32 = 3.1415926535;

pub fn degrees_to_radians(degrees: f32) -> f32 {
   return degrees * (PI / 180.0);
}

pub fn radians_to_degrees(radians: f32) -> f32 {
    return radians * (180.0 / PI);
}