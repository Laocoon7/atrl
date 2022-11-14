use crate::prelude::*;

#[derive(Component, Default)]
pub struct OnSplashScreen;

#[derive(Resource, Deref, DerefMut)]
pub struct SplashTimer(Timer);

pub fn setup_splash(
    timer: f32,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ctx: Query<(Entity, &mut KayakRootContext)>,
) {
    if let Ok((root_entity, mut widget_context)) = ctx.get_single_mut() {
        let splash_image = asset_server.load("images/splash.png");

        widget_context.add_plugin(KayakWidgetsContextPlugin);
        let parent_id = None;
        rsx! {
            <KayakAppBundle>
                <KImageBundle image={KImage(splash_image)} />
            </KayakAppBundle>
        }

        commands.entity(root_entity).insert(OnSplashScreen);
        commands.insert_resource(SplashTimer(Timer::from_seconds(timer, TimerMode::Once)));
    }
}

pub fn countdown(time: Res<Time>, mut timer: ResMut<SplashTimer>) { timer.tick(time.delta()); }
