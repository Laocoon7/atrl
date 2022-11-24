use crate::prelude::*;

#[derive(SystemParam,)]
pub struct PlayerQuery<'w, 's,> {
    pub entity: Query<'w, 's, Entity, With<Player,>,>,
    pub world_position: Query<'w, 's, &'static mut WorldPosition, With<Player,>,>,
    pub name: Query<'w, 's, &'static mut Name, With<Player,>,>,
    pub movement: Query<'w, 's, &'static mut Movement, With<Player,>,>,
    pub vision: Query<'w, 's, &'static mut Vision, With<Player,>,>,
    pub health: Query<'w, 's, &'static mut Health, With<Player,>,>,
    pub ai_component: Query<'w, 's, &'static mut AIComponent, With<Player,>,>,
    pub sprite: Query<'w, 's, &'static mut TextureAtlasSprite, With<Player,>,>,
    pub texture_atlas: Query<'w, 's, &'static mut TextureAtlasSprite, With<Player,>,>,
}
