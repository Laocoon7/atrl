use crate::prelude::*;

pub fn atlr_log_plugin() -> bevy::log::LogPlugin {
    bevy::log::LogPlugin {
        level: get_log_level(),
        filter: get_log_filters(),
    }
}

pub const fn get_log_level() -> bevy::log::Level {
    if cfg!(feature = "trace") {
        bevy::log::Level::TRACE
    } else if cfg!(feature = "debug") {
        bevy::log::Level::DEBUG
    } else if cfg!(feature = "dev") {
        bevy::log::Level::INFO
    } else {
        bevy::log::Level::ERROR
    }
}

pub fn get_log_filters() -> String {
    match cfg!(feature = "trace") || cfg!(feature = "debug") {
        true => "wgpu_hal=warn,gfx_backend_metal=warn,wgpu_core=warn,bevy_render=info,lain=debug,\
                 bevy_render::render_resource::pipeline_cache=warn,bevy_app=debug,big_brain=debug,\
                 sequence=debug"
            .to_string(),
        false => "".to_string(),
    }
}
