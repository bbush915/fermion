use rand::random;

pub fn random_in_range(minimum: f32, maximum: f32) -> f32 {
    minimum + random::<f32>() * (maximum - minimum)
}
