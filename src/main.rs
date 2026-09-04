mod graphics;
pub mod level;
pub mod state;
pub mod math;

use crate::state::State;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut state = State::new()?;
    state.run()?;

    Ok(())
}
