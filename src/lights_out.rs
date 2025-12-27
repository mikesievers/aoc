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
//! guassion eliminationi s performed.
//!
use std::ops::{Add, Div, Sub};

type LightIndex = usize;
type Button = Vec<LightIndex>;

// Define a LightState that supports modular arithmetic
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct LightState<const N: i32>(i32);

// Allow initialization from i32
impl<const N: i32> From<i32> for LightState<N> {
    fn from(value: i32) -> Self {
        LightState(value.rem_euclid(N))
    }
}

// implement modular airithmmetic
impl<const N: i32> Add for LightState<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        LightState((self.0 + rhs.0).rem_euclid(N))
    }
}

impl<const N: i32> Sub for LightState<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        LightState((self.0 - rhs.0).rem_euclid(N))
    }
}
// Division is multiplication by multiplicative inverse:
impl<const N: i32> Div for LightState<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        // Determine multiplicative inverse
        let mut mult_inv: i32 = -1;
        for i in 0..N {
            if (rhs.0 * i).rem_euclid(N) == 1 {
                mult_inv = i;
            }
        }
        if mult_inv == -1 {
            panic!("Multiplicative inverse in base could not be found.");
        } else {
            LightState((self.0 * mult_inv).rem_euclid(N))
        }
    }
}

#[cfg(test)]
mod test_light_state {
    use super::LightState;

    #[test]
    fn test_addition() {
        let ls1: LightState<3> = 1.into();
        let ls2: LightState<3> = 2.into();
        let ls3: LightState<3> = 5.into();

        assert_eq!(ls1 + ls1, ls2); // 1+1 == 2
        assert_eq!(ls3 - ls1, ls1); // 2 - 1 == 1
    }

    #[test]
    fn test_division() {
        let ls1: LightState<7> = 6.into();
        let ls2: LightState<7> = 3.into();
        assert_eq!(ls1 / ls2, 2.into());
    }

    #[test]
    #[should_panic]
    fn test_division_panic() {
        // 2 in mod 10 does not have a multiplicative inverse
        let ls1: LightState<10> = 5.into();
        let ls2: LightState<10> = 2.into();

        let this_should_panic = ls1 / ls2;
    }
}

pub const DIM: i32 = 3;

pub struct LightsOut {
    dimension: usize,
    buttons: Vec<Button>,
    initial_state: Vec<LightState<DIM>>,
    target_state: Vec<LightState<DIM>>,
}

impl LightsOut {
    pub fn new(
        dimension: usize,
        buttons: Vec<Button>,
        initial_state: Vec<LightState<DIM>>,
        target_state: Vec<LightState<DIM>>,
    ) -> LightsOut {
        LightsOut {
            dimension,
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
        Some(vec![0, 1])
    }
}

#[cfg(test)]
mod tests {
    use super::{Button, DIM, LightState, LightsOut};

    #[test]
    fn test_lo() {
        let dimension = 2;
        let initial_state: Vec<LightState<DIM>> = vec![0.into(), 1.into()]; // Light 0 is off, 1 is on
        let target_state: Vec<LightState<DIM>> = vec![0.into(), 0.into()]; // All lights should be out
        // Button 0 switches light 0,
        // button 1 switches lights 0 and 1
        let buttons: Vec<Button> = vec![vec![0], vec![0, 1]];
        let lo = LightsOut::new(dimension, buttons, initial_state, target_state);

        // Both buttons have to be pressed once:
        assert_eq!(lo.solution(), Some(vec![1, 1]));
    }
}
