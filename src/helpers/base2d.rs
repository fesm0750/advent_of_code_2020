use std::{
    convert::{TryFrom, TryInto},
    error::Error,
};

/// Helper struct for representing 2d values, i.e: coordinates, indexes, etc.
#[derive(Copy, Clone, Debug)]
pub struct Base2d<U> {
    pub x: U,
    pub y: U,
}

impl<U> Base2d<U>
where
    U: Copy,
{
    /// Constructs a new Base2d
    pub fn new(x: U, y: U) -> Base2d<U> {
        Base2d { x, y }
    }

    /// Returns a tuple `(x, y)`.
    pub fn tuple(&self) -> (U, U) {
        (self.x, self.y)
    }
}

/// Defines how to convert a tuple (U, U) to a Base2d. In practice, allows to
/// use tuples, in some situations, as a more handy alternative instead of
/// instaciating the Base2d struct.
impl<U> TryFrom<(U, U)> for Base2d<usize>
where
    U: TryInto<usize>,
    <U as TryInto<usize>>::Error: std::error::Error + 'static,
{
    type Error = Box<dyn Error>;

    fn try_from(item: (U, U)) -> Result<Self, Self::Error> {
        Ok(Base2d {
            x: item.0.try_into()?,
            y: item.1.try_into()?,
        })
    }
}
