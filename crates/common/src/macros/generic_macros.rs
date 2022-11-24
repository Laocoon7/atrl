use crate::prelude::*;

#[macro_export]
macro_rules! impl_new{
  ($to:ty,$($v:ident: $t:ty),*)  => {
      impl $to {
          pub fn new($($v: $t),*) -> $to
          {
              Self {
                  $($v),*
              }
          }
      }
  };
}

#[macro_export]
macro_rules! impl_default {
    ($to:ty) => {
        impl Default for $to {
            fn default() -> Self { Self::new() }
        }
    };
}

#[macro_export]
macro_rules! insert_resource {
    ($r:expr) => {
        |mut commands: Commands| {
            commands.insert_resource($r);
        }
    };

    ($commands:ident, $r:expr) => {
        $commands.insert_resource($r);
    };
}

#[macro_export]
macro_rules! remove_resource {
    ($t:ty) => {
        |mut commands: Commands| {
            commands.remove_resource::<$t>();
        }
    };

    ($commands:ident, $t:ty) => {
        $commands.remove_resource::<$t>();
    };
}

#[macro_export]
macro_rules! spawn_component {
    ($c:expr) => {
        |mut commands: Commands| {
            commands.spawn($c);
        }
    };

    ($commands:ident, $c:expr) => {
        $commands.spawn($c);
    };
}

//////////////////////////////////////////////////////////////////////////////////////////
// Query Manipulation
//////////////////////////////////////////////////////////////////////////////////////////

/// Despawn all entities with a specific marker component
///
/// Useful when exiting states
pub fn despawn_with<T: Component>(mut cmd: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        cmd.entity(e).despawn();
    }
}

/// Despawn all entities with a specific marker component
///
/// Useful when exiting states
pub fn despawn_children<T: Component>(mut cmd: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        cmd.entity(e).despawn_descendants();
    }
}

/// Despawn all entities with a specific marker component
///
/// Useful when exiting states
pub fn despawn_with_recursive<T: Component>(mut cmd: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        cmd.entity(e).despawn_recursive();
    }
}

/// Remove a component type from all entities that have it
pub fn remove_from_all<T: Component>(mut cmd: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        cmd.entity(e).remove::<T>();
    }
}

/// Remove a component type from any entities with some other component
pub fn remove_from_all_with<T: Component, W: Component>(
    mut cmd: Commands,
    q: Query<Entity, (With<T>, With<W>)>,
) {
    for e in q.iter() {
        cmd.entity(e).remove::<T>();
    }
}
