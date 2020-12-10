use std::{convert::TryInto, fmt::Debug, ops::Index};

use super::base2d::Base2d;

/// A two-dimensional array using a flat internal representation.
///
/// This is a row major implementation, consecutive elements across the x
/// dimension are next to each other.
///
/// `x` represents variation in row elements (which column the value is in),
/// whereas `y` represents a change in column elements (which row is it in). The
/// grid may be indexed with a tuple using get2d((x,y)) , for example:
///
/// - get2d((5, 0)) returns the sixth element of the first row. It can also be
///   interpreted as the the element at column 5 and row 0.
///
/// - get2d((1, 5)) returns the second element of the sixth row. In other words,
///   the element at column 1 and row 5.
///
/// # Indexing
///
/// Implements the Index trait, so the grid may be read by a tuple inside
/// square brackets. Example:
///
/// ```
/// use adv20::helpers::grid::Grid;
/// let mut grid = Grid::new(5, 5, 0u8);
/// let v = grid.get_mut(2, 2);
/// *v = 100;
/// assert_eq!(grid[(2,2)], 100);
/// ```
///
/// ## Beware
///
/// If no inferring is made, the Default type for tuples in rust is i32.
///
/// # Panics
///
/// Panics if the indexing inside square brackets is done with negative values.
#[derive(Debug, PartialEq, Eq)]
pub struct Grid<T> {
    flat: Vec<T>,
    pub len_x: usize,
    pub len_y: usize,
}

impl<T: Clone> Grid<T> {
    pub fn new(len_x: usize, len_y: usize, init: T) -> Grid<T> {
        Grid {
            flat: vec![init; len_x * len_y],
            len_x,
            len_y,
        }
    }

    pub fn new_with_vec(len_x: usize, len_y: usize, flat: Vec<T>) -> Grid<T> {
        Grid { flat, len_x, len_y }
    }
}

impl<T> Grid<T> {
    /// returns the value at position x,y.
    ///
    /// # Panics
    ///
    /// Panics if either index is out of bounds.
    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.flat[self.index(x, y)]
    }

    /// returns the value at position pos, where pos any type that can be
    /// represented as a Base2d<usize>.
    ///
    /// # Panics
    ///
    /// Panics if indexes are out of bounds.
    pub fn get_from2d<V>(&self, pos: V) -> &T
    where
        V: Into<Base2d<usize>>,
    {
        let pos: Base2d<usize> = pos.into();
        self.get(pos.x, pos.y)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        let i = self.index(x, y); // must have an aux variable coz mutable borrow
        &mut self.flat[i]
    }

    pub fn get_mut_from2d<V>(&mut self, p: V) -> &mut T
    where
        V: Into<Base2d<usize>>,
    {
        let p: Base2d<usize> = p.into();
        let i = self.index(p.x, p.y);
        &mut self.flat[i]
    }

    /// returns the total size of the array (len_x * len_y)
    pub fn size(&self) -> usize {
        self.len_x * self.len_y
    }

    /// returns the value at position `x, y`. Wraps around if either index is
    /// larger than its array dimension.
    pub fn wrap(&self, x: usize, y: usize) -> &T {
        let nx = x % self.len_x;
        let ny = y % self.len_y;
        self.get(nx, ny)
    }

    /// returns the value at position `x, y`. If index `x` is larger than the x
    /// dimension of the grid, the index is wrapped around.
    ///
    /// # Panics
    ///
    /// Panics if the `y` index is out of bounds.
    pub fn wrap_x(&self, x: usize, y: usize) -> &T {
        let nx = x % self.len_x;
        self.get(nx, y)
    }

    /// returns the value at position `x, y`. If index `y` is larger than the y
    /// dimmension of the grid, the index is wrapped around.
    ///
    /// # Panics
    ///
    /// Panics if the `x` index is out of bounds.
    pub fn wrap_y(&self, x: usize, y: usize) -> &T {
        let ny = y % self.len_y;
        self.get(x, ny)
    }

    //--------------------------------------------------------------------
    // Private
    //--------------------------------------------------------------------

    /// returns the index for acessing the `flat` array from the coordinates `x`
    /// and `y`.
    fn index(&self, x: usize, y: usize) -> usize {
        self.len_x * y + x
    }
}

impl<T, V> Index<V> for Grid<T>
where
    V: TryInto<Base2d<usize>>,
    <V as TryInto<Base2d<usize>>>::Error: Debug,
{
    type Output = T;

    fn index(&self, index: V) -> &Self::Output {
        self.get_from2d(index.try_into().unwrap())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::convert::TryFrom;

    pub fn test() {
        let grid = Grid::new(5, 5, 0u8);
        println!("{}", usize::try_from(-3i32).unwrap());
        let _ = grid[(3, 3)];
    }
}
