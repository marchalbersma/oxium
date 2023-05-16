/// A 4-dimensional vector with [`f32`] components.
pub struct Vec4 {
    /// The x-component of the vector.
    pub x: f32,
    /// The y-component of the vector.
    pub y: f32,
    /// The z-component of the vector.
    pub z: f32,
    /// The w-component of the vector.
    pub w: f32,
}

impl Vec4 {
    /// Creates a new 4-dimensional vector from the given components.
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

#[cfg(test)]
mod tests {
    use crate::Vec4;

    #[test]
    fn new() {
        let vec = Vec4::new(-27.499603, 90.0137, -33.023926, 96.00694);

        assert_eq!(vec.x, -27.499603);
        assert_eq!(vec.y, 90.0137);
        assert_eq!(vec.z, -33.023926);
        assert_eq!(vec.w, 96.00694);
    }
}
