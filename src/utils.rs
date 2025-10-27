pub const PI: f64 = std::f64::consts::PI;
#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
#[inline]
pub fn random_double() -> f64 {
    rand::random::<f64>()
}
#[inline]
pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max-min) * random_double()
}

#[inline]
pub fn random_int(min: i64, max: i64) -> i64 {
    rand::random::<i64>() % (max - min) + min
}