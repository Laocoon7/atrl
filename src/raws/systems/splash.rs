use crate::game::prelude::*;

#[derive(Component)]
pub struct OnSplashScreen;

#[derive(Deref, DerefMut)]
pub struct SplashTimer(Timer);

pub fn setup_splash(timer: f32, mut commands: Commands, asset_server: Res<AssetServer>) {
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
    commands.insert_resource(SplashTimer(Timer::from_seconds(timer, false)));
}

pub fn countdown(time: Res<Time>, mut timer: ResMut<SplashTimer>) { timer.tick(time.delta()); }
