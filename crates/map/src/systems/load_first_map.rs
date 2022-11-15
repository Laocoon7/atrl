use crate::prelude::*;

pub(crate) fn load_first_map(
    mut commands: Commands,
    game_context: Res<GameContext>,
    state: Res<CurrentGameState>,
) {
    let seed = game_context.get_rng().prht.get(0, 0, 0);

    let mut random = Random::new(seed);

    let (start_x, start_y) = random_start_position(&mut random);
    let chain = BuilderChain::new(
        [GRID_WIDTH, GRID_HEIGHT],
        WorldPosition { position: IVec3 { x: 0, y: 0, z: 0 } },
        "first map",
    )
    .start_with(CellularAutomataArchitect::new())
    .with(RoomMapArchitect::new())
    .with(AreaStartingPosition::new(start_x, start_y))
    .generate(&mut game_context.get_rng());

    commands.spawn(chain.get_map());

    state.set_next(&mut commands);
}
