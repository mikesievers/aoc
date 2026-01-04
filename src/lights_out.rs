//! Lights out solver
//! Solve a lights out problem with n states
//! (2 states corresponds to the classic lights out problem)
//!
//! Given:
//!
//! - the dimanion of the space (2,3,4,...)
//! - a vec of buttons (which lights are affected)
//!   For now, represent as i32 to allow for subtraction
//!   Better would be n explicit struct with llimited stated
//! - p: an initial Vec of states of the lights
//! - b: a vec of the target state
//!
//! Determine the buttons to be pressed to reach the target state
//! Return as a Vec x of buttons to be pressed with the necessary amount of times
//!
//! The transformation can be written as
//!  A*x = p-b
//! Rather than trying to determine an inverse of the matrix,
//! guassion elimination is performed.
//!
//! ```
//! use aoc::lights_out::{Button, LightState, LightsOut};
//!
//! let initial_state = vec![LightState { v: 0, dim: 2 }, LightState { v: 1, dim: 2 }]; // Light 0 is off, 1 is on
//! let target_state = vec![LightState { v: 0, dim: 2 }, LightState { v: 0, dim: 2 }]; // All lights should be out
//! // Button 0 switches light 0,
//! // button 1 switches lights 0 and 1
//! let buttons: Vec<Button> = vec![vec![1, 0], vec![1, 1]];
//! let lo = LightsOut::new(buttons, initial_state, target_state);
//!
//! // Both buttons have to be pressed once:
//! assert_eq!(lo.solution(), Some(vec![1, 1]));
//! ```

use std::ops::{Add, Div, Sub};

// Light cycle delta
type LightStateDelta = i32;
// Which cycle changes does a button press cause
pub type Button = Vec<LightStateDelta>;

// Define a LightState that supports modular arithmetic
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct LightState {
    pub v: i32,
    pub dim: i32,
}

impl LightState {
    pub fn new(v: i32, dim: i32) -> Self {
        LightState {
            v: v.rem_euclid(dim),
            dim,
        }
    }
}

// implement modular airithmmetic
impl Add<LightState> for LightState {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        if self.dim != rhs.dim {
            panic!("Can't add LightStates with different dimension")
        }
        LightState {
            v: (self.v + rhs.v).rem_euclid(self.dim),
            dim: self.dim,
        }
    }
}

impl Add<LightStateDelta> for LightState {
    type Output = LightState;

    fn add(self, rhs: LightStateDelta) -> Self {
        LightState {
            v: (self.v + rhs).rem_euclid(self.dim),
            dim: self.dim,
        }
    }
}

impl Sub for LightState {
    type Output = LightStateDelta;

    fn sub(self, rhs: Self) -> LightStateDelta {
        if self.dim != rhs.dim {
            panic!("Can't subtract LightStates with different dimension")
        }
        (self.v - rhs.v).rem_euclid(self.dim)
    }
}
// Division is multiplication by multiplicative inverse:
impl Div<LightStateDelta> for LightState {
    type Output = Self;

    fn div(self, rhs: LightStateDelta) -> Self {
        // Determine multiplicative inverse
        let mut mult_inv: i32 = -1;
        for i in 0..self.dim {
            if (rhs * i).rem_euclid(self.dim) == 1 {
                mult_inv = i;
            }
        }
        if mult_inv == -1 {
            panic!("Multiplicative inverse in base could not be found.");
        } else {
            LightState {
                v: (self.v * mult_inv).rem_euclid(self.dim),
                dim: self.dim,
            }
        }
    }
}

#[cfg(test)]
mod test_light_state {
    use super::LightState;
    use super::LightStateDelta;

    #[test]
    fn test_addition() {
        let ls1 = LightState { v: 1, dim: 3 };
        let ls2 = LightState { v: 2, dim: 3 };
        let ls3 = LightState { v: 5, dim: 3 };

        assert_eq!(ls1 + ls1, ls2); // 1+1 == 2
        assert_eq!(ls3 - ls1, 1); // 2 - 1 == 1
    }

    #[test]
    fn test_division() {
        let ls1 = LightState { v: 6, dim: 7 };
        let ls2: LightStateDelta = 3;
        assert_eq!(ls1 / ls2, LightState { v: 2, dim: 7 });
    }

    #[test]
    #[should_panic]
    fn test_division_panic() {
        // 2 in mod 10 does not have a multiplicative inverse
        let ls1: LightState = LightState { v: 5, dim: 10 };
        let ls2: LightStateDelta = 2;

        let _this_should_panic = ls1 / ls2;
    }
}

// The Lights Out struct

pub struct LightsOut {
    buttons: Vec<Button>,
    initial_state: Vec<LightState>,
    target_state: Vec<LightState>,
}

impl LightsOut {
    pub fn new(
        buttons: Vec<Button>,
        initial_state: Vec<LightState>,
        target_state: Vec<LightState>,
    ) -> LightsOut {
        LightsOut {
            buttons,
            initial_state,
            target_state,
        }
    }

    pub fn solution(&self) -> Option<Vec<i32>> {
        //! Determine solution to the lights out problem.
        //! The output is a Vec of button presses.
        //! [1,2] means press button 0 once and button 1 twice.
        //!

        // Set up a Matrix to represent the lights out problem
        // A*x = p-b
        let matrix = Self::setup_matrix(&self.buttons, &self.initial_state, &self.target_state);

        println!(" pre matrix: {:?}", matrix);

        let re_matrix = Self::to_row_echelon(&matrix);
        // TODO: nothing is curently testing the matrix to really be in in row exchelon
        // form, but the following expects it to be in diagonal form

        println!("  re matrix: {:?}", re_matrix);

        // Calculate the solution from the matrix
        let mut solution = Vec::with_capacity(self.buttons.len());
        for row_idx in 0..re_matrix.len() {
            solution.push(re_matrix[row_idx][re_matrix[0].len() - 1].v);
        }

        Some(solution)
    }

    pub fn setup_matrix(
        buttons: &[Button],
        initial_state: &[LightState],
        target_state: &[LightState],
    ) -> Vec<Vec<LightState>> {
        // From buttons and number of lights, determine the initial state of the matrix
        // The final column will be the right side of the equation A*x = p-b
        // The COLUMNs of the matrix correspond to the buttons and their wiring (i.e. lights
        // affected)
        // The ROWs correspond to the lights

        // All states must have the same dimension, use the first one
        let dim = target_state[0].dim;

        let mut matrix: Vec<Vec<LightState>> = Vec::with_capacity(target_state.len());

        for light_idx in 0..target_state.len() {
            // Create a row by the wirings of the buttons and the expected state minus the initial
            // state
            let mut row: Vec<LightState> = Vec::with_capacity(buttons.len() + 1);
            for button in buttons {
                row.push(LightState {
                    v: button[light_idx],
                    dim,
                });
            }
            // Add the right side of the equation system as final column
            row.push(LightState {
                v: target_state[light_idx] - initial_state[light_idx],
                dim,
            });

            matrix.push(row);
        }

        matrix
    }

    pub fn to_row_echelon(orig_matrix: &Vec<Vec<LightState>>) -> Vec<Vec<LightState>> {
        // Change matrix to row echelon form
        let mut matrix = orig_matrix.clone();
        let dim = matrix[0][0].dim; // All dims must be equal, use the first

        for row_idx in 0..matrix.len() {
            // Ensure that the current row has a non-zero entry in the column of the same index
            if matrix[row_idx][row_idx].v == 0 {
                // Swap rows if possible
                for row_cand in row_idx..matrix.len() {
                    if matrix[row_cand][row_idx].v != 0 {
                        matrix.swap(row_idx, row_cand);
                        break; // non-zero entry found, continue
                    }
                }
            }
            // The matrix row starts with non-zero entry, scale to start with a 1
            if matrix[row_idx][row_idx].v != 1 {
                let scale = matrix[row_idx][row_idx].v;
                // We have to scale
                if let Some(row) = matrix.get_mut(row_idx) {
                    for val in row {
                        *val = *val / scale;
                    }
                }
            }
            // Now ensure all other rows start with 0
            let scaled_row = matrix[row_idx].clone(); // need for subtraction below
            for other_row_idx in 0..matrix.len() {
                if other_row_idx == row_idx {
                    continue;
                }
                let other_row_factor = matrix[other_row_idx][row_idx].v;
                if other_row_factor != 0
                    && let Some(other_row) = matrix.get_mut(other_row_idx)
                {
                    for (col, val) in other_row.iter_mut().enumerate() {
                        *val = LightState::new(val.v - scaled_row[col].v * other_row_factor, dim)
                    }
                }
            }
        }

        matrix
    }
}

#[cfg(test)]
mod tests {
    use super::{Button, LightState, LightsOut};

    #[test]
    fn test_setup_matrix() {
        let initial_state: Vec<LightState> =
            vec![LightState { v: 0, dim: 2 }, LightState { v: 1, dim: 2 }]; // Light 0 is off, 1 is on
        let target_state: Vec<LightState> =
            vec![LightState { v: 0, dim: 2 }, LightState { v: 0, dim: 2 }]; // All lights should be out
        // Button 0 switches light 0,
        // button 1 switches lights 0 and 1
        let buttons: Vec<Button> = vec![vec![1, 0], vec![1, 1]];
        let matrix = LightsOut::setup_matrix(&buttons, &initial_state, &target_state);

        let ls0 = LightState { v: 0, dim: 2 };
        let ls1 = LightState { v: 1, dim: 2 };

        assert_eq!(matrix, vec![vec![ls1, ls1, ls0], vec![ls0, ls1, ls1]]);
    }

    #[test]
    fn test_row_echelon() {
        let ls0 = LightState { v: 0, dim: 3 };
        let ls1 = LightState { v: 1, dim: 3 };
        let ls2 = LightState { v: 2, dim: 3 };

        let matrix: Vec<Vec<LightState>> = vec![vec![ls2, ls0, ls2], vec![ls1, ls1, ls1]];
        let re_matrix_expected = vec![vec![ls1, ls0, ls1], vec![ls0, ls1, ls0]];
        let re_matrix = LightsOut::to_row_echelon(&matrix);

        assert_eq!(re_matrix, re_matrix_expected);
    }
}
