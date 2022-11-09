use crate::game::prelude::*;
use banana_bevy_utils::{despawn_with_recursive, state::StateNext};
use smart_default::SmartDefault;

#[derive(Component)]
struct OnSplashScreen;

#[derive(Deref, DerefMut)]
pub struct SplashTimer(Timer);

pub fn setup_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("images/splash.png");

    commands
        .spawn_bundle(ImageBundle {
            image: UiImage(icon),
            style: Style {
                // This will center the logo
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            ..default()
        })
        .insert(OnSplashScreen);

    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(2.0, false)));
}

fn countdown(time: Res<Time>, mut timer: ResMut<SplashTimer>) { timer.tick(time.delta()); }

////////////////////////////////////////////////////////////////////////////////

#[derive(SmartDefault)]
pub struct SplashPlugin<T: std::default::Default> {
    #[default(2.0)]
    pub loading_time: f32,
    pub state_asset_load: T,
}

impl<T: StateNext + std::default::Default> Plugin for SplashPlugin<T> {
    fn build(&self, app: &mut atrl_engine::App) {
        app.add_enter_system(self.state_asset_load.clone(), setup_splash)
            .add_system(countdown.run_in_state(self.state_asset_load.clone()))
            .add_exit_system(
                self.state_asset_load.clone(),
                despawn_with_recursive::<OnSplashScreen>,
            );
    }
}
