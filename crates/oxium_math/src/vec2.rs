/// A 2-dimensional vector with [`f32`] components.
pub struct Vec2 {
    /// The x-component of the vector.
    pub x: f32,
    /// The y-component of the vector.
    pub y: f32,
}

impl Vec2 {
    /// Creates a new 2-dimensional vector from the given components.
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[cfg(test)]
mod tests {
    use crate::Vec2;

    #[test]
    fn new() {
        let vec = Vec2::new(-72.82515, 25.319649);

        assert_eq!(vec.x, -72.82515);
        assert_eq!(vec.y, 25.319649);
    }
}
