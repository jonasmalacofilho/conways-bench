pub struct Game<Backing> {
    current: Grid<Backing>,
    previous: Grid<Backing>,
}

impl<B> Game<B>
where
    Grid<B>: GameGrid + Clone,
{
    pub fn new(starting: Grid<B>) -> Self {
        let dimensions = starting.dimensions();
        Game {
            current: starting,
            previous: Grid::<B>::new(dimensions),
        }
    }

    pub fn update(&mut self) {
        std::mem::swap(&mut self.current, &mut self.previous);
        for (cell, state) in self.previous.iter() {
            let next_state = match self.previous.neighborhood(cell) {
                3 => true,
                4 => state,
                _ => false,
            };
            self.current.set(cell, next_state);
        }
    }

    pub fn current(&self) -> &Grid<B> {
        &self.current
    }

    pub fn previous(&self) -> &Grid<B> {
        &self.previous
    }
}

pub type Pair = (usize, usize);

pub type Grid<Backing> = (Backing, Pair);

pub trait GameGrid {
    type Iter<'a>: Iterator<Item = (Pair, bool)>
    where
        Self: 'a;

    fn new(dimensions: Pair) -> Self;

    fn dimensions(&self) -> Pair;

    fn get(&self, cell: Pair) -> Option<bool>;

    fn set(&mut self, cell: Pair, alive: bool);

    fn iter(&self) -> Self::Iter<'_>;

    #[inline]
    fn index(&self, cell: Pair) -> Option<usize> {
        let dim = self.dimensions();
        if cell.0 < dim.0 && cell.1 < dim.1 {
            Some(cell.0 * dim.1 + cell.1)
        } else {
            None
        }
    }

    #[inline]
    fn neighborhood(&self, cell: Pair) -> usize {
        let mut alive = 0;
        for i in 0..3 {
            for j in 0..3 {
                let neighbor = (
                    cell.0.wrapping_add(i).wrapping_sub(1),
                    cell.1.wrapping_add(j).wrapping_sub(1),
                );
                alive += self.get(neighbor).unwrap_or(false) as usize;
            }
        }
        alive
    }
}

pub struct GridIter<I> {
    inner: I,
    position: Pair,
    dimensions: Pair,
}

impl GameGrid for Grid<Vec<bool>> {
    type Iter<'a> = GridIter<std::slice::Iter<'a, bool>> where Self: 'a;

    fn new(dimensions: Pair) -> Self {
        (vec![false; dimensions.0 * dimensions.1], dimensions)
    }

    #[inline]
    fn dimensions(&self) -> Pair {
        self.1
    }

    #[inline]
    fn get(&self, cell: Pair) -> Option<bool> {
        let index = self.index(cell)?;
        unsafe { Some(*self.0.get_unchecked(index)) }
    }

    #[inline]
    fn set(&mut self, cell: Pair, alive: bool) {
        let index = self.index(cell).unwrap();
        unsafe { *self.0.get_unchecked_mut(index) = alive };
    }

    fn iter(&self) -> Self::Iter<'_> {
        GridIter {
            inner: self.0.iter(),
            position: Default::default(),
            dimensions: self.1,
        }
    }
}

// impl<'a, I: Iterator<Item = &'a bool>> Iterator for GridIter<I> {
impl<'a> Iterator for GridIter<std::slice::Iter<'a, bool>> {
    type Item = (Pair, bool);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(&next) = self.inner.next() {
            let current = self.position;
            self.position.1 += 1;
            if self.position.1 >= self.dimensions.1 {
                self.position.0 += 1;
                self.position.1 = 0;
            }
            Some((current, next))
        } else {
            None
        }
    }
}

impl GameGrid for Grid<bit_vec::BitVec> {
    type Iter<'a> = GridIter<bit_vec::Iter<'a>> where Self: 'a;

    fn new(dimensions: Pair) -> Self {
        (
            bit_vec::BitVec::from_elem(dimensions.0 * dimensions.1, false),
            dimensions,
        )
    }

    #[inline]
    fn dimensions(&self) -> Pair {
        self.1
    }

    #[inline]
    fn get(&self, cell: Pair) -> Option<bool> {
        let index = self.index(cell)?;
        Some(self.0[index])
    }

    #[inline]
    fn set(&mut self, cell: Pair, alive: bool) {
        let index = self.index(cell).unwrap();
        self.0.set(index, alive);
    }

    fn iter(&self) -> Self::Iter<'_> {
        GridIter {
            inner: self.0.iter(),
            position: Default::default(),
            dimensions: self.1,
        }
    }
}

impl<'a> Iterator for GridIter<bit_vec::Iter<'a>> {
    type Item = (Pair, bool);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.inner.next() {
            let current = self.position;
            self.position.1 += 1;
            if self.position.1 >= self.dimensions.1 {
                self.position.0 += 1;
                self.position.1 = 0;
            }
            Some((current, next))
        } else {
            None
        }
    }
}

impl GameGrid for Grid<bitvec::vec::BitVec> {
    type Iter<'a> = GridIter<bitvec::slice::Iter<'a, usize, bitvec::order::Lsb0>> where Self: 'a;

    fn new(dimensions: Pair) -> Self {
        (
            bitvec::vec::BitVec::repeat(false, dimensions.0 * dimensions.1),
            dimensions,
        )
    }

    #[inline]
    fn dimensions(&self) -> Pair {
        self.1
    }

    #[inline]
    fn get(&self, cell: Pair) -> Option<bool> {
        let index = self.index(cell)?;
        unsafe { Some(*self.0.get_unchecked(index)) }
    }

    #[inline]
    fn set(&mut self, cell: Pair, alive: bool) {
        let index = self.index(cell).unwrap();
        unsafe { *self.0.get_unchecked_mut(index) = alive };
    }

    fn iter(&self) -> Self::Iter<'_> {
        GridIter {
            inner: self.0.iter(),
            position: Default::default(),
            dimensions: self.1,
        }
    }
}

impl<'a> Iterator for GridIter<bitvec::slice::Iter<'a, usize, bitvec::order::Lsb0>> {
    type Item = (Pair, bool);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.inner.next() {
            let current = self.position;
            self.position.1 += 1;
            if self.position.1 >= self.dimensions.1 {
                self.position.0 += 1;
                self.position.1 = 0;
            }
            Some((current, *next))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_index() {
        let grid = Grid::<Vec<bool>>::new((10, 10));
        assert_eq!(grid.index((3, 6)), Some(36));
        assert_eq!(grid.index((10, 0)), None);
        assert_eq!(grid.index((0, 10)), None);
    }

    #[test]
    fn smoke_vec_grid() {
        let mut grid = Grid::<Vec<bool>>::new((10, 10));
        assert_eq!(grid.get((3, 5)), Some(false));

        grid.set((3, 5), true);
        assert_eq!(grid.get((3, 5)), Some(true));

        grid.set((2, 6), true);
        assert_eq!(grid.neighborhood((3, 5)), 2);
        assert_eq!(grid.neighborhood((2, 6)), 2);
        assert_eq!(grid.neighborhood((3, 6)), 2);
        assert_eq!(grid.neighborhood((3, 7)), 1);

        assert_eq!(grid.neighborhood((0, 0)), 0);
        assert_eq!(grid.neighborhood((9, 9)), 0);

        assert_eq!(
            grid.iter()
                .filter_map(|(cell, alive)| alive.then_some(cell))
                .collect::<Vec<_>>(),
            vec![(2, 6), (3, 5)],
        );
    }
}
