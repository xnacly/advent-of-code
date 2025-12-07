use std::ops::{Index, IndexMut};

use crate::point::Point;

#[derive(Default, Clone, PartialEq)]
pub struct Grid {
    vec: Vec<Vec<u8>>,
    pub width: i32,
    pub height: i32,
}

impl Grid {
    pub fn parse(s: &str) -> Self {
        Grid::from_vec(&s.lines().map(String::from).collect::<Vec<_>>())
    }

    pub fn from_vec(v: &[String]) -> Self {
        let height = v.len() as i32;
        let vec: Vec<Vec<u8>> = v
            .into_iter()
            .filter(|x| !x.is_empty())
            .map(|x| x.chars().map(|x| x as u8).collect())
            .collect();
        let width = vec.get(0).unwrap_or(&vec![]).len() as i32;
        Grid { vec, height, width }
    }

    pub fn columns(&self) -> Vec<Vec<u8>> {
        let mut cols = vec![Vec::with_capacity(self.height as usize); self.width as usize];

        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                cols[x].push(self.vec[y][x]);
            }
        }

        cols
    }

    pub fn rows(&self) -> Vec<Vec<u8>> {
        self.vec.clone()
    }

    pub fn get(&self, x: usize, y: usize) -> Option<u8> {
        self.vec.get(y).and_then(|row| row.get(x)).copied()
    }

    pub fn get_point(&self, p: Point) -> Option<u8> {
        if p.x < 0 || p.y < 0 {
            return None;
        }

        let (xu, yu) = (p.x as usize, p.y as usize);
        self.vec.get(yu).and_then(|row| row.get(xu)).copied()
    }

    pub fn transpose(&mut self) {
        let old_height = self.vec.len();
        let old_width = self.vec.iter().map(|r| r.len()).max().unwrap_or(0);
        let mut new_vec = vec![vec![0u8; old_height]; old_width];

        for i in 0..old_height {
            let row = &self.vec[i];
            for j in 0..old_width {
                new_vec[j][i] = *row.get(j).unwrap_or(&0);
            }
        }

        self.vec = new_vec;
        std::mem::swap(&mut self.width, &mut self.height);
    }
}

impl Index<Point> for Grid {
    type Output = u8;

    #[inline]
    fn index(&self, index: Point) -> &Self::Output {
        &self.vec[index.y as usize][index.x as usize]
    }
}

impl IndexMut<Point> for Grid {
    fn index_mut(&mut self, index: Point) -> &mut u8 {
        &mut self.vec[index.y as usize][index.x as usize]
    }
}

impl Index<usize> for Grid {
    type Output = [u8];

    fn index(&self, row: usize) -> &Self::Output {
        &self.vec[row]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.vec[row]
    }
}

impl Index<i32> for Grid {
    type Output = [u8];

    fn index(&self, row: i32) -> &Self::Output {
        &self.vec[row as usize]
    }
}

impl IndexMut<i32> for Grid {
    fn index_mut(&mut self, row: i32) -> &mut Self::Output {
        &mut self.vec[row as usize]
    }
}

use std::fmt;

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Grid {{ width: {}, height: {} }}",
            self.width, self.height
        )?;
        for row in &self.vec {
            let line = row.iter().map(|&b| b as char).collect::<String>();
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_grid() {
        let grid = Grid::default();
        assert_eq!(grid.width, 0);
        assert_eq!(grid.height, 0);
        assert_eq!(grid.rows().len(), 0);
        assert_eq!(grid.columns().len(), 0);
        assert_eq!(grid.get(0, 0), None);
    }

    #[test]
    fn test_square_grid() {
        let grid = Grid::from_vec(&vec![
            "123".to_string(),
            "456".to_string(),
            "789".to_string(),
        ]);

        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 3);
        assert_eq!(grid.get(0, 0), Some(b'1'));
        assert_eq!(grid.get(2, 2), Some(b'9'));
        assert_eq!(grid.get(3, 0), None);

        let cols = grid.columns();
        assert_eq!(cols.len(), 3);
        assert_eq!(cols[0], vec![b'1', b'4', b'7']);

        let rows = grid.rows();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[1], vec![b'4', b'5', b'6']);
    }

    #[test]
    fn test_rectangular_grid_wide() {
        let mut grid = Grid::from_vec(&vec!["12345".to_string(), "67890".to_string()]);

        assert_eq!(grid.width, 5);
        assert_eq!(grid.height, 2);

        grid.transpose();
        assert_eq!(grid.width, 2);
        assert_eq!(grid.height, 5);

        let expected = vec![
            vec![b'1', b'6'],
            vec![b'2', b'7'],
            vec![b'3', b'8'],
            vec![b'4', b'9'],
            vec![b'5', b'0'],
        ];
        assert_eq!(grid.vec, expected);
    }

    #[test]
    fn test_rectangular_grid_tall() {
        let mut grid = Grid::from_vec(&vec![
            "12".to_string(),
            "34".to_string(),
            "56".to_string(),
            "78".to_string(),
        ]);

        assert_eq!(grid.width, 2);
        assert_eq!(grid.height, 4);

        grid.transpose();
        assert_eq!(grid.width, 4);
        assert_eq!(grid.height, 2);

        let expected = vec![vec![b'1', b'3', b'5', b'7'], vec![b'2', b'4', b'6', b'8']];
        assert_eq!(grid.vec, expected);
    }

    #[test]
    fn test_jagged_grid() {
        let mut grid = Grid::from_vec(&vec!["12".to_string(), "345".to_string(), "6".to_string()]);

        assert_eq!(grid.width, 2);
        assert_eq!(grid.height, 3);

        grid.transpose();
        // transpose fills missing entries with 0
        let expected = vec![
            vec![b'1', b'3', b'6'],
            vec![b'2', b'4', 0],
            vec![0, b'5', 0],
        ];
        assert_eq!(grid.vec, expected);
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 3);
    }

    #[test]
    fn test_single_row() {
        let mut grid = Grid::from_vec(&vec!["abcdef".to_string()]);
        assert_eq!(grid.width, 6);
        assert_eq!(grid.height, 1);

        grid.transpose();
        assert_eq!(grid.width, 1);
        assert_eq!(grid.height, 6);

        let expected: Vec<Vec<u8>> = "abcdef".chars().map(|c| vec![c as u8]).collect();
        assert_eq!(grid.vec, expected);
    }

    #[test]
    fn test_single_column() {
        let mut grid = Grid::from_vec(&vec!["a".to_string(), "b".to_string(), "c".to_string()]);

        assert_eq!(grid.width, 1);
        assert_eq!(grid.height, 3);

        grid.transpose();
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 1);

        let expected: Vec<Vec<u8>> = vec![vec![b'a', b'b', b'c']];
        assert_eq!(grid.vec, expected);
    }

    #[test]
    fn test_indexing() {
        let mut grid = Grid::from_vec(&vec!["12".to_string(), "34".to_string()]);

        use crate::point::Point;
        assert_eq!(grid[Point { x: 0, y: 0 }], b'1');
        assert_eq!(grid[Point { x: 1, y: 1 }], b'4');

        grid[Point { x: 0, y: 0 }] = b'9';
        assert_eq!(grid[Point { x: 0, y: 0 }], b'9');
    }

    #[test]
    fn test_parse_empty_lines() {
        let s = "\n12\n\n34\n";
        let grid = Grid::parse(s);
        assert_eq!(grid.height, 2);
        assert_eq!(grid.width, 2);
        assert_eq!(grid.vec, vec![vec![b'1', b'2'], vec![b'3', b'4']]);
    }
}
