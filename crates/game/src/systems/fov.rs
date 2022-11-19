use crate::prelude::*;

pub fn fov(
    mut map: Query<&mut Map>,
    manager: Res<MapManager>,
    player_q: Query<&Transform, With<Player>>,
) {
    let player_pos = player_q.single();
    if let Some(mut map) = map.iter_mut().find(|m| m.world_position == manager.current_map) {
        fov::compute(player_pos.get(), 8, &mut map.visibility_map);
    }
}
