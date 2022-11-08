use crate::game::prelude::*;

pub fn draw_tilemaps(
    mut commands: Commands,
    q_tilemap_contexts: Query<(Entity, &TilemapContext)>,
    mut q_tiles: Query<(Entity, &mut TextureAtlasSprite), With<TilemapTile>>,
) {
    for (context_entity, context) in q_tilemap_contexts.iter() {
        for action in &context.actions {
            match action {
                TilemapAction::SetIndex(entity_index, index) => {
                    if let Ok((_, mut sprite)) = q_tiles.get_mut(*entity_index) {
                        sprite.index = *index;
                    }
                }
                TilemapAction::SetAtlas(entity_index, handle) => {
                    if let Ok((tile_entity, _)) = q_tiles.get_mut(*entity_index) {
                        commands
                            .entity(tile_entity)
                            .remove::<Handle<TextureAtlas>>()
                            .insert(handle.clone());
                    }
                }
                TilemapAction::SetColor(entity_index, color) => {
                    if let Ok((_, mut sprite)) = q_tiles.get_mut(*entity_index) {
                        sprite.color = *color;
                    }
                }
            };
        }
        commands.entity(context_entity).despawn_recursive();
    }
}
