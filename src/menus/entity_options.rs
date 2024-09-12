use crate::{entities::species::EntityType, game_engine::{keyboard_events_provider::KeyboardEventsProvider, locks::LockType, state_updates::{EngineStateUpdate, WorldStateUpdate}}, lang::localizable::LocalizableText, ui::components::View};
use super::{menu::{Menu, MenuItem, MenuUpdate}, text_input::TextInput};

#[derive(Debug, Copy, Clone)]
pub enum EntityOptionMenuItem {
    Remove,
    Rename,
    PickUp,
    ChangeLock,
}

impl MenuItem for EntityOptionMenuItem {
    fn title(&self) -> String {
        match self {
            EntityOptionMenuItem::Remove => "entity.menu.remove".localized(),
            EntityOptionMenuItem::Rename => "entity.menu.rename".localized(),
            EntityOptionMenuItem::PickUp => "entity.menu.pickup".localized(),
            EntityOptionMenuItem::ChangeLock => "entity.menu.change_lock".localized(),
        }
    }
}

impl MenuItem for LockType {
    fn title(&self) -> String {
        self.localized_name()
    }
}

pub enum EntityOptionsMenuState {
    Closed,
    ChangingName,
    ChangingLock,
}

pub struct EntityOptionsMenu {
    entity_name: String,
    entity_id: u32,
    species_id: u32,
    time_since_last_closed: f32,
    menu: Menu<EntityOptionMenuItem>,
    state: EntityOptionsMenuState,
    text_input: TextInput,
    lock_menu: Menu<LockType>,
}

impl EntityOptionsMenu {
    pub fn new() -> Self {
        Self {
            entity_name: "".to_owned(),
            entity_id: 0,
            species_id: 0,
            time_since_last_closed: 1.0,
            menu: Menu::new("entity.menu.title".localized(), vec![]),
            state: EntityOptionsMenuState::Closed,
            text_input: TextInput::new(),
            lock_menu: Menu::new("entity.menu.change_lock_title".localized(), vec![
                LockType::None,
                LockType::Yellow,
                LockType::Red,
                LockType::Blue,
                LockType::Green,
                LockType::Silver,
            ]),
        }
    }

    pub fn show(&mut self, entity_name: &str, entity_id: &u32, species_id: &u32, entity_type: &EntityType, creative_mode: bool) {
        if self.time_since_last_closed < 0.5 {
            return;
        }
        self.time_since_last_closed = 0.0;
        self.menu.items = self.available_options(creative_mode, entity_type);

        if self.menu.items.is_empty() {
            return
        }

        self.entity_name = entity_name.to_owned();
        self.entity_id = *entity_id;
        self.species_id = *species_id;
        self.menu.title = entity_name.to_owned();
        self.menu.show();
        self.state = EntityOptionsMenuState::Closed;
    }

    pub fn is_open(&self) -> bool {
        self.menu.is_open
    }

    pub fn update(&mut self, keyboard: &KeyboardEventsProvider, time_since_last_update: f32) -> MenuUpdate {
        if !self.menu.is_open {
            self.time_since_last_closed += time_since_last_update;
        }

        match self.state {
            EntityOptionsMenuState::ChangingName => self.update_from_change_name(keyboard, time_since_last_update),
            EntityOptionsMenuState::ChangingLock => self.update_from_change_lock(keyboard, time_since_last_update),
            EntityOptionsMenuState::Closed => self.update_from_close(keyboard, time_since_last_update),
        }
    }

    fn update_from_change_name(&mut self, keyboard: &KeyboardEventsProvider, time_since_last_update: f32) -> MenuUpdate {
        self.text_input.update(keyboard, time_since_last_update);

        if self.text_input.did_confirm() {
            let new_name = self.text_input.text().trim().to_owned();
            self.menu.close();
            self.state = EntityOptionsMenuState::Closed;
            self.text_input.clear();

            return (false, vec![
                WorldStateUpdate::RenameEntity(self.entity_id, new_name),
                WorldStateUpdate::EngineUpdate(EngineStateUpdate::SaveGame)
            ]);
        } else if self.text_input.did_cancel() {
            self.state = EntityOptionsMenuState::Closed;
            self.text_input.clear();
        }

        (self.menu.is_open, vec![])
    }

    fn update_from_change_lock(&mut self, keyboard: &KeyboardEventsProvider, time_since_last_update: f32) -> MenuUpdate {
        self.lock_menu.update(keyboard, time_since_last_update);

        if self.lock_menu.selection_has_been_confirmed {
            let selected_lock = self.lock_menu.selected_item();
            self.lock_menu.clear_selection();
            self.lock_menu.close();
            self.menu.clear_selection();
            self.menu.close();
            self.state = EntityOptionsMenuState::Closed;

            return (false, vec![
                WorldStateUpdate::ChangeLock(self.entity_id, selected_lock),
                WorldStateUpdate::EngineUpdate(EngineStateUpdate::SaveGame)
            ]);
        }
        if !self.lock_menu.is_open {
            self.lock_menu.clear_selection();
            self.lock_menu.close();
            self.menu.clear_selection();
            self.menu.close();
            self.state = EntityOptionsMenuState::Closed;
        }

        (self.menu.is_open, vec![])
    }

    fn update_from_close(&mut self, keyboard: &KeyboardEventsProvider, time_since_last_update: f32) -> MenuUpdate {
        self.menu.update(keyboard, time_since_last_update);

        if self.is_open() && self.menu.selection_has_been_confirmed {
            let updates = match self.menu.selected_item() {
                EntityOptionMenuItem::Remove => {
                    self.menu.clear_selection();
                    self.menu.close();
                    vec![WorldStateUpdate::RemoveEntity(self.entity_id)]
                },
                EntityOptionMenuItem::Rename => {
                    self.menu.clear_selection();
                    self.ask_for_new_name();
                    vec![]
                },
                EntityOptionMenuItem::PickUp => {
                    self.menu.clear_selection();
                    self.menu.close();
                    vec![
                        WorldStateUpdate::EngineUpdate(EngineStateUpdate::AddToInventory(self.species_id)),
                        WorldStateUpdate::RemoveEntity(self.entity_id),
                        WorldStateUpdate::EngineUpdate(EngineStateUpdate::SaveGame),
                    ]
                },
                EntityOptionMenuItem::ChangeLock => {
                    self.menu.clear_selection();
                    self.ask_for_lock_type();
                    vec![]
                },
            };
            return (self.menu.is_open, updates);
        }

        (self.menu.is_open, vec![])
    }

    pub fn ui(&self) -> View {
        match self.state {
            EntityOptionsMenuState::ChangingName => self.text_input.ui(),
            EntityOptionsMenuState::ChangingLock => self.lock_menu.ui(),
            EntityOptionsMenuState::Closed => self.menu.ui(),
        }
    }

    fn ask_for_lock_type(&mut self) {
        self.state = EntityOptionsMenuState::ChangingLock;
        self.lock_menu.show();
    }

    fn ask_for_new_name(&mut self) {
        self.state = EntityOptionsMenuState::ChangingName;
        self.text_input.clear();
        self.text_input.title = "entity.menu.rename_title".localized();
    }

    fn available_options(&self, creative_mode: bool, entity_type: &EntityType) -> Vec<EntityOptionMenuItem> {
        if creative_mode {
            self.available_options_creative(entity_type)
        } else {
            self.available_options_regular(entity_type)
        }
    }

    fn available_options_creative(&self, entity_type: &EntityType) -> Vec<EntityOptionMenuItem> {
        let nothing: Vec<EntityOptionMenuItem> = vec![];

        match entity_type {
            EntityType::Hero => nothing,
            EntityType::Npc => vec![
                EntityOptionMenuItem::Rename,
                EntityOptionMenuItem::Remove,
            ],
            EntityType::Building => vec![
                EntityOptionMenuItem::Remove,
            ],
            EntityType::HouseholdObject => vec![
                EntityOptionMenuItem::PickUp,
                EntityOptionMenuItem::Rename,
                EntityOptionMenuItem::Remove,
            ],
            EntityType::PickableObject => vec![
                EntityOptionMenuItem::PickUp,
                EntityOptionMenuItem::Remove,
            ],
            EntityType::Teleporter => vec![
                EntityOptionMenuItem::ChangeLock
            ],
            EntityType::PushableObject => vec![
                EntityOptionMenuItem::Remove,
            ],
            EntityType::Gate => vec![
                EntityOptionMenuItem::Remove,
            ],
            EntityType::InverseGate => vec![
                EntityOptionMenuItem::Remove,
            ],
            EntityType::PressurePlate => vec![
                EntityOptionMenuItem::Remove,
            ],
        }
    }

    fn available_options_regular(&self, entity_type: &EntityType) -> Vec<EntityOptionMenuItem> {
        let pickup = vec![
            EntityOptionMenuItem::PickUp,
        ];
        let nothing: Vec<EntityOptionMenuItem> = vec![];

        match entity_type {
            EntityType::Hero => nothing,
            EntityType::Npc => nothing,
            EntityType::Building => nothing,
            EntityType::HouseholdObject => pickup,
            EntityType::PickableObject => pickup,
            EntityType::Teleporter => nothing,
            EntityType::PushableObject => nothing,
            EntityType::Gate => nothing,
            EntityType::InverseGate => nothing,
            EntityType::PressurePlate => nothing,
        }
    }
}
