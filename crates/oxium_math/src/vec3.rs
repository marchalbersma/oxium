/// A 3-dimensional vector with [`f32`] components.
pub struct Vec3 {
    /// The x-component of the vector.
    pub x: f32,
    /// The y-component of the vector.
    pub y: f32,
    /// The z-component of the vector.
    pub z: f32,
}

impl Vec3 {
    /// Creates a new 3-dimensional vector from the given components.
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use crate::Vec3;

    #[test]
    fn new() {
        let vec = Vec3::new(-20.157623, -22.457527, 6.0027847);

        assert_eq!(vec.x, -20.157623);
        assert_eq!(vec.y, -22.457527);
        assert_eq!(vec.z, 6.0027847);
    }
}
