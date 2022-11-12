use crate::game::prelude::internal::*;
use crate::prelude::*;

pub fn draw_map_tiles(
    mut commands: Commands,
    q_render_actions: Query<(Entity, &RenderActions)>,
    mut q_tiles: Query<(Entity, &mut TextureAtlasSprite), (With<RenderTile>, Without<Sprite>)>,
    mut q_background_tiles: Query<
        (Entity, &mut Sprite),
        (With<RenderTile>, Without<TextureAtlasSprite>),
    >,
) {
    for (render_actions_entity, render_actions) in q_render_actions.iter() {
        for action in &render_actions.actions {
            match action {
                RenderAction::SetAtlasIndex(entity, index) => {
                    if let Ok((_, mut sprite)) = q_tiles.get_mut(*entity) {
                        sprite.index = *index;
                    }
                }
                RenderAction::SetAtlasHandle(entity, handle) => {
                    if let Ok((tile_entity, _)) = q_tiles.get_mut(*entity) {
                        commands
                            .entity(tile_entity)
                            .remove::<Handle<TextureAtlas>>()
                            .insert(handle.clone());
                    }
                }
                RenderAction::SetForegroundColor(entity, color) => {
                    if let Ok((_, mut sprite)) = q_tiles.get_mut(*entity) {
                        sprite.color = *color;
                    }
                }
                RenderAction::SetBackgroundColor(entity, color) => {
                    if let Ok((_, mut sprite)) = q_background_tiles.get_mut(*entity) {
                        sprite.color = *color;
                    }
                }
            };
        }
        commands.entity(render_actions_entity).despawn_recursive();
    }
}
