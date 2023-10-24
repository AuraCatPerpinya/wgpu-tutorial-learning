use std::error::Error;

use wgpu_learning::run;

fn main() -> Result<(), Box<dyn Error>> {
    pollster::block_on(run())?;

    Ok(())
}
