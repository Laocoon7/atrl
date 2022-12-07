use crate::prelude::*;
pub fn menu_button_render(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    textures: Res<TextureAssets>,
    menu_button_query: Query<&MenuButton>,
    state_query: Query<&ButtonState>,
) -> bool {
    let state_entity = widget_context.use_state(&mut commands, entity, ButtonState { hovering: false });

    let button_text = menu_button_query.get(entity).unwrap().text.clone();
    let button_image = textures.button.clone();
    let button_image_hover = textures.button_hover.clone();

    let on_event = OnEvent::new(
        move |In((event_dispatcher_context, _, mut event, _entity)): In<(
            EventDispatcherContext,
            WidgetState,
            Event,
            Entity,
        )>,
              mut query: Query<&mut ButtonState>| {
            if let Ok(mut button) = query.get_mut(state_entity) {
                match event.event_type {
                    EventType::MouseIn(..) => {
                        event.stop_propagation();
                        button.hovering = true;
                    },
                    EventType::MouseOut(..) => {
                        button.hovering = false;
                    },
                    _ => {},
                }
            }
            (event_dispatcher_context, event)
        },
    );

    if let Ok(button_state) = state_query.get(state_entity) {
        let button_image_handle = if button_state.hovering { button_image_hover } else { button_image };
        let parent_id = Some(entity);

        rsx! {
            <NinePatchBundle
                nine_patch={NinePatch {
                    handle: button_image_handle,
                    border: Edge::all(10.0),
                }}
                styles={KStyle {
                    width: Units::Stretch(1.0).into(),
                    height: Units::Pixels(40.0).into(),
                    ..KStyle::default()
                }}
                on_event={on_event}
            >
                <TextWidgetBundle
                    text={TextProps {
                        alignment: Alignment::Middle,
                        content: button_text,
                        size: 28.0,
                        ..Default::default()
                    }}
                    styles={KStyle {
                        width: Units::Stretch(1.0).into(),
                        height: Units::Stretch(1.0).into(),
                        ..KStyle::default()
                    }}
                />
            </NinePatchBundle>
        };
    }

    true
}
