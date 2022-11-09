use atrl_engine::*;
use banana_bevy_utils::{
    despawn_with_recursive, remove_resource,
    state::{StateNext, StateSetNext},
};
use smart_default::SmartDefault;

use crate::game::prelude::CurrentGameState;

#[derive(Component)]
struct OnSplashScreen;

// Newtype to use a `Timer` for this screen as a resource
#[derive(Deref, DerefMut)]
struct SplashTimer(Timer);

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Splash Setup");
    let icon = asset_server.load("images/splash.png");

    commands
        .spawn_bundle(ImageBundle {
            style: Style {
                // This will center the logo
                margin: UiRect::all(Val::Auto),
                // This will set the logo to be 200px wide, and auto adjust its height
                // size: Size::new(Val::Px(200.0), Val::Auto),
                ..default()
            },
            image: UiImage(icon),
            ..default()
        })
        .insert(OnSplashScreen);

    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(2.0, false)));
}

// Tick the timer, and change state when finished
fn countdown(
    time: Res<Time>,
    mut commands: Commands,
    mut timer: ResMut<SplashTimer>,
    mut state: ResMut<CurrentGameState>,
) {
    if timer.tick(time.delta()).finished() {
        state.set_next(&mut commands);
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(SmartDefault)]
pub struct SplashPlugin<T: std::default::Default> {
    #[default(2.0)]
    pub loading_time: f32,
    pub state_splash: T,
}

impl<T: StateNext + std::default::Default> Plugin for SplashPlugin<T> {
    fn build(&self, app: &mut atrl_engine::App) {
        app.add_enter_system(self.state_splash.clone(), splash_setup)
            .add_system(countdown.run_in_state(self.state_splash.clone()))
            .add_exit_system(self.state_splash.clone(), despawn_with_recursive::<OnSplashScreen>);
    }
}
