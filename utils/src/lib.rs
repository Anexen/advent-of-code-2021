pub enum Neighbors {
    Quadratic,
    Diagonal,
    Custom(Vec<(i32, i32)>),
}

pub struct Grid<T> {
    pub data: Vec<Vec<T>>,
    pub shape: (usize, usize),
}

impl<T> Grid<T> {
    pub fn from_vec(data: Vec<Vec<T>>) -> Self {
        let shape = (data.len(), data.get(0).unwrap_or(&Vec::new()).len());
        Self { data, shape }
    }

    pub fn get(&self, i: usize, j: usize) -> &T {
        &self.data[i][j]
    }

    pub fn set(&mut self, i: usize, j: usize, value: T) {
        self.data[i][j] = value
    }

    pub fn get_neighbors(&self, i: usize, j: usize, neighbors: Neighbors) -> Vec<(usize, usize)> {
        let neighbors = match neighbors {
            Neighbors::Quadratic => vec![(0, -1), (1, 0), (0, 1), (-1, 0)],
            Neighbors::Diagonal => vec![
                (0, -1),
                (1, -1),
                (1, 0),
                (1, 1),
                (0, 1),
                (-1, 1),
                (-1, 0),
                (-1, -1),
            ],
            Neighbors::Custom(v) => v,
        };

        let mut result = Vec::with_capacity(neighbors.len());
        let (i, j) = (i as i32, j as i32);
        for (di, dj) in neighbors {
            if i + di >= 0
                && i + di < self.shape.0 as i32
                && j + dj >= 0
                && j + dj < self.shape.1 as i32
            {
                result.push(((i + di) as usize, (j + dj) as usize));
            }
        }
        result
    }
}

impl Grid<u8> {
    pub fn from_str(input: &str) -> Self {
        Grid::from_vec(
            input
                .lines()
                .map(|line| line.bytes().map(|x| x - b'0').collect())
                .collect(),
        )
    }
}
