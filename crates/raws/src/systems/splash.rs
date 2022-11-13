use crate::prelude::*;

#[derive(Component)]
pub struct OnSplashScreen;

#[derive(Resource, Deref, DerefMut)]
pub struct SplashTimer(Timer);

pub fn setup_splash(timer: f32, mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("images/splash.png");

    commands.spawn((
        ImageBundle {
            image: UiImage(icon),
            style: Style {
                // This will center the logo
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            ..default()
        },
        OnSplashScreen,
    ));

    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(timer, TimerMode::Once)));
}

pub fn countdown(time: Res<Time>, mut timer: ResMut<SplashTimer>) { timer.tick(time.delta()); }
