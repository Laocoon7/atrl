use crate::prelude::*;

#[derive(Component)]
pub struct LastUpdate {
    pub value: f64,
}

pub(crate) fn build_map(
    mut commands: Commands,
    state: Res<CurrentGameState>,
    tilesets: Tilesets,
    array_texture_loader: Res<ArrayTextureLoader>,
) {
    if let Some(tileset) = tilesets.get_by_id(&0) {
        create_tilemap(&mut commands, [80, 45], tileset, &array_texture_loader);
        state.set_next(&mut commands);
    }
}

pub(crate) fn set_tile_custom_size(mut q_map: Query<&mut Sprite, With<TileTextureIndex>>) {
    for mut sprite in q_map.iter_mut() {
        println!("HERE");
        sprite.custom_size = Some(Vec2::ONE);
    }
}

pub(crate) fn test_display(
    mut commands: Commands,
    time: Res<Time>,
    mut q_map: ParamSet<(
        Query<Entity, (With<TileTextureIndex>, Without<LastUpdate>)>,
        Query<(&mut TileTextureIndex, &mut LastUpdate)>,
    )>,
) {
    for entity in q_map.p0().iter() {
        commands.entity(entity).insert(LastUpdate { value: 0.0 });
    }

    let current_time = time.elapsed_seconds_f64();

    for (mut tile, mut last_update) in q_map.p1().iter_mut() {
        tile.0 = 1;
        if (current_time - last_update.value) > 0.2 {
            last_update.value = current_time;
        }
    }
}
