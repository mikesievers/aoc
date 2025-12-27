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
//! - an initial Vec of states of the lights
//! - a vec of the target state
//!
//! Determine the buttons to be pressed to reach the target state
//! Return as a Vec of buttons to be pressed with the necessary amount of times
//!

type LightIndex = usize;
type Button = Vec<LightIndex>;
// Possibly needs to be i32 to allow for differences
type LightState = i32;

pub struct LightsOut {
    dimension: usize,
    buttons: Vec<Button>,
    initial_state: Vec<LightState>,
    target_state: Vec<LightState>,
}

impl LightsOut {
    pub fn new(
        dimension: usize,
        buttons: Vec<Button>,
        initial_state: Vec<LightState>,
        target_state: Vec<LightState>,
    ) -> LightsOut {
        LightsOut {
            dimension,
            buttons,
            initial_state,
            target_state,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LightsOut;

    #[test]
    fn test_lo() {
        let lo = LightsOut::new();
        assert_eq!(true, false);
    }
}
