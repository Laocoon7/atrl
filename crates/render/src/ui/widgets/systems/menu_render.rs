use crate::prelude::*;

pub fn menu_widget_render(
    In((widget_context, entity,),): In<(KayakWidgetContext, Entity,),>,
    mut commands: Commands,
    query: Query<&MainMenuState,>,
    query2: Query<(&KChildren, &MenuWidget,),>,
) -> bool {
    let state_entity = widget_context.use_state(&mut commands, entity, MainMenuState::default(),);

    if let Ok(state,) = query.get(state_entity,) {
        // Activate Children if show is true
        if let Ok((children, _,),) = query2.get(entity,) {
            if state.show {
                children.process(&widget_context, Some(entity,),);
            }
        }
    }

    true
}
