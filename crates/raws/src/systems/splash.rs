use crate::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct SplashTimer(Timer);

pub fn setup_splash(
    timer: f32,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut font_mapping: ResMut<FontMapping>,
    mut ctx: Query<&mut KayakRootContext>,
) {
    if let Ok(mut widget_context) = ctx.get_single_mut() {
        font_mapping.set_default(asset_server.load("fonts/lato/lato-light.kayak_font"));
        let splash_image = asset_server.load("images/splash.png");

        widget_context.add_plugin(KayakWidgetsContextPlugin);
        let parent_id = None;
        rsx! {
            <KayakAppBundle>
                <KImageBundle image={KImage(splash_image)} />
                <TextWidgetBundle
                    text={TextProps {
                        size: 50.0,
                        alignment: Alignment::Middle,
                        content: "Loading".to_string(),
                        ..Default::default()
                    }}
                    styles={KStyle{
                        z_index: StyleProp::Value(1),
                        top: StyleProp::Value(Units::Percentage(0.5 * 100.)),
                        position_type: StyleProp::Value(KPositionType::SelfDirected),
                        ..Default::default()
                    }}
                />
            </KayakAppBundle>
        }

        // commands.entity(root_entity).insert(OnSplashScreen);
        commands.insert_resource(SplashTimer(Timer::from_seconds(timer, TimerMode::Once)));
    }
}

pub fn countdown(time: Res<Time>, mut timer: ResMut<SplashTimer>) { timer.tick(time.delta()); }
