use crate::game::prelude::internal::*;
use crate::prelude::*;
use bevy::app::AppExit;

pub fn setup_main_menu(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    asset_server: Res<AssetServer>,
    mut font_mapping: ResMut<FontMapping>,
    ui_camera: Query<Entity, With<UICamera>>,
) {
    if let Ok(entity) = ui_camera.get_single() {
        info!("Setting up main menu");
        font_mapping.set_default(asset_server.load("fonts/lato/lato-light.kayak_font"));

        let mut widget_context = KayakRootContext::new();
        widget_context.add_plugin(KayakWidgetsContextPlugin);

        // Main Menu Widget
        widget_context.add_widget_data::<MenuWidget, MainMenuState>();
        widget_context.add_widget_system(
            MenuWidget::default().get_name(),
            widget_update::<MenuWidget, MainMenuState>,
            menu_widget_render,
        );

        // Menu Buttons
        widget_context.add_widget_data::<MenuButton, ButtonState>();
        widget_context.add_widget_system(
            MenuButton::default().get_name(),
            widget_update::<MenuButton, ButtonState>,
            menu_button_render,
        );

        let handle_click_close = OnEvent::new(
            move |In((event_dispatcher_context, _, event, _entity)): In<(
                EventDispatcherContext,
                WidgetState,
                Event,
                Entity,
            )>,
                  mut exit: EventWriter<AppExit>| {
                if let EventType::Click(..) = event.event_type {
                    exit.send(AppExit);
                }
                (event_dispatcher_context, event)
            },
        );

        let continue_click = OnEvent::new(
            move |In((event_dispatcher_context, _, mut event, _entity)): In<(
                EventDispatcherContext,
                WidgetState,
                Event,
                Entity,
            )>,
                  mut commands: Commands,
                  game_state: Res<CurrentGameState>| {
                event.prevent_default();
                event.stop_propagation();
                if let EventType::Click(..) = event.event_type {
                    game_state.set_next(&mut commands);
                }
                (event_dispatcher_context, event)
            },
        );

        let parent_id = None;
        let panel1_image = textures.ui_panel.clone();
        let logo_image = textures.logo.clone();

        rsx! {
            <KayakAppBundle>
                <NinePatchBundle
                    nine_patch={NinePatch {
                        handle: panel1_image,
                        border: Edge::all(25.0),
                    }}
                    styles={KStyle {
                        width: Units::Pixels(350.0).into(),
                        height: Units::Pixels(512.0).into(),
                        left: Units::Stretch(1.0).into(),
                        right: Units::Stretch(1.0).into(),
                        top: Units::Stretch(1.0).into(),
                        bottom: Units::Stretch(1.0).into(),
                        padding: Edge::new(
                            Units::Pixels(20.0),
                            Units::Pixels(20.0),
                            Units::Pixels(50.0),
                            Units::Pixels(20.0),
                        ).into(),
                        ..KStyle::default()
                    }}
                >
                    <KImageBundle
                        image={KImage(logo_image)}
                        styles={KStyle {
                            // width: Units::Pixels(310.0).into(),
                            // height: Units::Pixels(78.0).into(),
                            // bottom: Units::Stretch(1.0).into(),
                            ..KStyle::default()
                        }}
                    />
                    <ElementBundle
                        id={"button_area"}
                        styles={KStyle {
                            left: Units::Pixels(50.0).into(),
                            right: Units::Pixels(50.0).into(),
                            ..Default::default()
                        }}
                    >
                        <MenuButtonBundle
                            button={MenuButton { text: "Play".into() }}
                            on_event={continue_click}
                        />
                        <MenuButtonBundle button={MenuButton { text: "Options".into() }} />
                        <MenuButtonBundle
                            button={MenuButton { text: "Quit".into() }}
                            on_event={handle_click_close}
                        />
                    </ElementBundle>
                </NinePatchBundle>
            </KayakAppBundle>
        }

        commands.entity(entity).insert(widget_context);
    }
}
