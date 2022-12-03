use crate::prelude::*;

// CoreStage::First
//   (after) => BigBrainStage::Scorers
//      (after) => BigBrainStage::Thinkers
// CoreStage::PreUpdate
//   (after) => BigBrainStage::Actions
//      (after) => AtrlStage::ProccessEvents (movement_sys / perform_turns_sys)
// CoreStage::Update
// CoreStage::PostUpdate
// CoreStage::Last => (cull_dead_sys => fov_sys)
//   (before) => AtrlStage::CleanupEvents
//   (after) => BigBrainStage::Cleanup
#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
pub enum AtrlStage {
    AIThinking,
    ProcessTurns,
    ProccessEvents,
    CleanupEvents,
}
