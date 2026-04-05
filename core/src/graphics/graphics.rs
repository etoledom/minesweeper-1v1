use crate::{Point, Size};

#[derive(Debug, Clone)]
pub struct Vec2<T> {
    data: Vec<T>,
    size: Size,
}

impl<T> Vec2<T> {
    pub fn new(data: Vec<T>, size: Size) -> Self {
        debug_assert_eq!(data.len(), size.width * size.height, "Wrong size");
        Self { data, size }
    }
    fn index(&self, x: usize, y: usize) -> usize {
        x * self.size.height + y
    }

    pub fn iter<'a>(&'a self) -> Vec2Iter<'a, T> {
        Vec2Iter {
            data: &self,
            index: Point::zero(),
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> Vec2IterMut<'a, T> {
        Vec2IterMut {
            data: Some(&mut self.data),
            height: self.size.height,
            index: Point::zero(),
        }
    }

    pub fn get_width(&self) -> usize {
        self.size.width
    }

    pub fn get_height(&self) -> usize {
        self.size.height
    }

    pub fn get_element(&self, coordinates: Point) -> Option<&T> {
        if coordinates.x >= self.size.width || coordinates.y >= self.size.height {
            return None;
        }
        let index = self.index(coordinates.x, coordinates.y);
        self.data.get(index)
    }

    pub fn get_element_mut(&mut self, coordinates: Point) -> Option<&mut T> {
        if coordinates.x >= self.size.width || coordinates.y >= self.size.height {
            return None;
        }
        let index = self.index(coordinates.x, coordinates.y);
        self.data.get_mut(index)
    }

    pub fn replace_at(&mut self, element: T, coordinates: Point) {
        let index = self.index(coordinates.x, coordinates.y);
        if index < self.data.len() {
            self.data[index] = element;
        }
    }

    pub fn neighbors(&self, coordinates: Point) -> impl Iterator<Item = (Point, &T)> {
        #[rustfmt::skip]
        // Neighbor distances to be checked around the given coordinate.
        const OFFSETS: [(i32, i32); 8] = [
            (-1, -1), (-1, 0), (-1, 1),
            ( 0, -1),          ( 0, 1),
            ( 1, -1), ( 1, 0), ( 1, 1),
        ];

        let (x, y) = (coordinates.x as i32, coordinates.y as i32);
        OFFSETS.iter().filter_map(move |&(dx, dy)| {
            let nx = x + dx;
            let ny = y + dy;
            // Skip coordinates that would underflow usize
            if nx < 0 || ny < 0 {
                return None;
            }
            // Returns an element if it exists
            let point = Point {
                x: nx as usize,
                y: ny as usize,
            };
            let cell = self.get_element(point)?;
            Some((point, cell))
        })
    }
}

//===== ITERATORS =====

pub struct Vec2Iter<'a, T> {
    data: &'a Vec2<T>,
    index: Point,
}

impl<'a, T> Iterator for Vec2Iter<'a, T> {
    type Item = (Point, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let element = self.data.get_element(self.index)?;
        let index = self.index;
        self.index.y += 1;
        if self.index.y >= self.data.get_height() {
            self.index.y = 0;
            self.index.x += 1;
        }

        Some((index, element))
    }
}

pub struct Vec2IterMut<'a, T> {
    data: Option<&'a mut [T]>,
    height: usize,
    index: Point,
}

impl<'a, T> Iterator for Vec2IterMut<'a, T> {
    type Item = (Point, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        let slice = self.data.take()?;
        let (element, rest) = slice.split_first_mut()?;
        let index = self.index;
        self.data = Some(rest);
        self.index.y += 1;
        if self.index.y >= self.height {
            self.index.y = 0;
            self.index.x += 1;
        }
        Some((index, element))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neightbors() {
        let grid = Vec2::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], Size::new(3, 3));
        let neighbors: Vec<_> = grid.neighbors(Point { x: 2, y: 2 }).collect();
        assert_eq!(neighbors.len(), 3);
    }
    #[test]
    fn replace_at() {
        let mut grid = Vec2::new(vec![0, 0, 0, 0, 0, 0], Size::new(3, 2));
        grid.replace_at(1, Point { x: 0, y: 0 });

        let replaced = grid.get_element(Point::zero());

        assert_eq!(*replaced.unwrap(), 1);
    }

    #[test]
    fn test_iter_mut() {
        let mut grid = Vec2::new(vec![1, 1, 1, 1, 1, 1], Size::new(3, 2));

        // Mutate every element
        grid.iter_mut().for_each(|(_, element)| {
            *element = 2;
        });

        // Verify every element was set correctly
        for (point, element) in grid.iter() {
            assert_eq!(*element, 2, "Mismatch at ({}, {})", point.x, point.y);
        }
    }

    #[test]
    fn test_iter_mut_visits_all() {
        let mut grid = Vec2::new(vec![0, 0, 0, 0, 0, 0], Size::new(3, 2));
        let mut count = 0;

        grid.iter_mut().for_each(|(_, element)| {
            *element += 1;
            count += 1
        });

        assert_eq!(count, 6, "Should visit exactly 6 elements");
        for (_, element) in grid.iter() {
            assert_eq!(*element, 1, "Every element should have been visited exactly once");
        }
    }
}
