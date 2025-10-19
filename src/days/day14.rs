use std::fs::read_to_string;

pub struct Platform {
    space: Vec<Vec<char>>,
}

impl Platform {
    pub fn from_file(fname: &str) -> Self {
        let data = read_to_string(fname).unwrap();

        let space = data.lines().map(|line| line.chars().collect()).collect();

        Platform { space }
    }

    pub fn tilt_north(&mut self) {
        // for each column, start at the top and walk down
        // If the field is empty, look south.
        // If the next non-empty field is an O, swap the two fields.
        // Continue.
        let n_rows = self.space.len();
        let n_columns = self.space[0].len();

        for column in 0..n_columns {
            // For rows, the last one can be skipped - there is nothing to swap into it
            for row in 0..n_rows - 1 {
                match self.space[row][column] {
                    '.' => {
                        // field is empty, look down
                        for down_row in row + 1..n_rows {
                            match self.space[down_row][column] {
                                'O' => {
                                    // It's a rock - swap
                                    self.space[row][column] = 'O';
                                    self.space[down_row][column] = '.';
                                    break;
                                }
                                '#' => break, // it's a cube rock, ignore the rest
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn load(&self) -> i32 {
        let n_rows = self.space.len();

        let mut load = 0_i32;

        for (idx, row) in self.space.iter().enumerate() {
            // compute factor as i32 so the mapped values are i32
            let factor = (n_rows - idx) as i32;

            load += row
                .iter()
                .map(|c| match c {
                    'O' => factor,
                    _ => 0,
                })
                .sum::<i32>(); // explicit sum type to avoid ambiguity
        }

        load
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform() {
        let mut platform = Platform::from_file("resources/day14_sample.txt");

        assert_eq!(platform.space.len(), 10);
        assert_eq!(platform.space[0].len(), 10);

        assert_eq!(platform.space[0][0], 'O');
        assert_eq!(platform.space[1][0], 'O');
        assert_eq!(platform.space[2][0], '.');
        assert_eq!(platform.space[1][1], '.');
        assert_eq!(platform.space[1][4], '#');

        platform.tilt_north();
        assert_eq!(platform.space[2][0], 'O');
        assert_eq!(platform.space[7][2], 'O');

        assert_eq!(platform.load(), 136);
    }
}
