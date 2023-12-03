pub struct ItemDropPlugin;

use bevy::prelude::*;

use super::item_pickups::{self, Pickup};

impl Plugin for ItemDropPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ItemDropEvent>()
            .add_systems(Update, spawn_on_event);
    }
}

pub struct ItemDropBuilder {
    items: Vec<Pickup>,
}

impl ItemDropBuilder {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_item(mut self, item: Pickup) -> Self {
        self.items.push(item);
        return self;
    }

    pub fn add_experience(mut self, amount: u64) -> Self {
        self.items.push(Pickup::Experience(amount));
        return self;
    }

    pub fn build(self) -> ItemDrop {
        return ItemDrop::new(self.items);
    }
}
////////////////////////////////////////////////////////////////////////////////
/// Components
////////////////////////////////////////////////////////////////////////////////

#[derive(Component, Debug)]
pub struct ItemDrop(Vec<Pickup>);

impl ItemDrop {
    pub fn new(items: Vec<Pickup>) -> Self {
        Self(items)
    }

    pub fn items(&self) -> &Vec<Pickup> {
        return &self.0;
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Event
////////////////////////////////////////////////////////////////////////////////

#[derive(Event, Debug, Clone, PartialEq)]
pub struct ItemDropEvent {
    pub position: Vec2,
    pub items: Vec<Pickup>,
}

impl ItemDropEvent {
    pub fn new(position: Vec2, items: Vec<Pickup>) -> Self {
        Self { position, items }
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Systems
////////////////////////////////////////////////////////////////////////////////

fn spawn_on_event(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<ItemDropEvent>,
) {
    for event in events.read() {
        for item in event.items.iter() {
            let position = event.position;
            let rotation = 0.0;
            item_pickups::spawn(
                &mut commands,
                &asset_server,
                item.clone(),
                position,
                rotation,
            );
        }
    }
}
