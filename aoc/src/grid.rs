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
        Grid::from_vec(s.lines().map(String::from).collect())
    }

    pub fn from_vec(v: Vec<String>) -> Self {
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

    pub fn get(&self, x: i32, y: i32) -> Option<u8> {
        if y >= 0 && (y as usize) < self.vec.len() {
            let row = &self.vec[y as usize];
            if x >= 0 && (x as usize) < row.len() {
                return Some(row[x as usize]);
            }
        }
        None
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
