use crate::prelude::*;

pub const WAIT_TIME: u32 = SECONDS;

#[derive(Debug)]
pub struct WaitAction;

impl Action for WaitAction {
    fn get_base_time_to_perform(&self) -> u32 { WAIT_TIME }

    fn perform(&mut self, _: &mut World, _: Entity) -> Result<u32, Box<dyn Action>> {
        info!("Waiting");
        Ok(self.get_base_time_to_perform())
    }
}
