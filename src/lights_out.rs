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
use std::ops::{Add, Div, Sub};

// Light cycle delta
type LightStateDelta = i32;
// Which cycle changes does a button press cause
type Button = Vec<LightStateDelta>;

// Define a LightState that supports modular arithmetic
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct LightState {
    v: i32,
    dim: i32,
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

pub const DIM: i32 = 2;

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

    pub fn solution(&self) -> Option<Vec<usize>> {
        //! Determine solution to the lights out problem.
        //! The output is a Vec of button presses.
        //! [1,2] means press button 0 once and button 1 twice.
        //!

        // Set up a Matrix to represent the lights out problem
        // A*x = p-b
        Self::setup_matrix(&self.buttons, &self.initial_state, &self.target_state);

        // TODO: While the Matrix is not in row echelon form,
        // perform gaussian elimination
        Some(vec![0, 1])
    }

    pub fn setup_matrix(
        buttons: &[Button],
        initial_state: &[LightState],
        target_state: &[LightState],
    ) -> Vec<Vec<LightStateDelta>> {
        // From buttons and number of lights, determine the initial state of the matrix
        // The final column will be the right side of the equation A*x = p-b
        // The COLUMNs of the matrix correspond to the buttons and their wiring (i.e. lights
        // affected)
        // The ROWs correspond to the lights
        let mut matrix: Vec<Vec<LightStateDelta>> = Vec::with_capacity(target_state.len());
        for light_idx in 0..target_state.len() {
            // Create a row by the wirings of the buttons and the expected state minus the initial
            // state
            let mut row: Vec<LightStateDelta> = Vec::with_capacity(buttons.len() + 1);
            for button in buttons {
                row.push(button[light_idx]);
            }
            // Add the right side of the equation system as final column
            row.push(target_state[light_idx] - initial_state[light_idx]);

            matrix.push(row);
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

        println!("Matrix: {:?}", matrix);

        assert_eq!(matrix, vec![vec![1, 1, 0], vec![0, 1, 1]]);
    }

    #[test]
    fn test_lo() {
        let initial_state = vec![LightState { v: 0, dim: 2 }, LightState { v: 1, dim: 2 }]; // Light 0 is off, 1 is on
        let target_state = vec![LightState { v: 0, dim: 2 }, LightState { v: 0, dim: 2 }]; // All lights should be out
        // Button 0 switches light 0,
        // button 1 switches lights 0 and 1
        let buttons: Vec<Button> = vec![vec![1, 0], vec![1, 1]];
        let lo = LightsOut::new(buttons, initial_state, target_state);

        // Both buttons have to be pressed once:
        assert_eq!(lo.solution(), Some(vec![1, 1]));
    }
}
