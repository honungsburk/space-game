use bevy::prelude::*;

pub trait DespawnAll {
    /// Despawns all entities in the iterator
    fn despawn_all(&mut self, iter: impl IntoIterator<Item = Entity>);
}

impl<'w, 's> DespawnAll for Commands<'w, 's> {
    fn despawn_all(&mut self, iter: impl IntoIterator<Item = Entity>) {
        for entity in iter.into_iter() {
            self.entity(entity).despawn_recursive();
        }
    }
}
