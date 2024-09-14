use crate::{entities::species::{EntityType, SPECIES_NONE}, game_engine::{entity::Entity, keyboard_events_provider::KeyboardEventsProvider, locks::LockType, state_updates::{EngineStateUpdate, WorldStateUpdate}}, lang::localizable::LocalizableText, ui::components::View};
use super::{menu::{Menu, MenuItem, MenuUpdate}, text_input::TextInput};

#[derive(Debug, Clone)]
pub enum EntityOptionMenuItem {
    Remove,
    Rename,
    PickUp,
    Fight,
    Read(String),
    ChangeLock,
    ChangeDestinationWorld,
    ChangeDestinationX,
    ChangeDestinationY,
}

impl MenuItem for EntityOptionMenuItem {
    fn title(&self) -> String {
        match self {
            EntityOptionMenuItem::Remove => "entity.menu.remove".localized(),
            EntityOptionMenuItem::Rename => "entity.menu.rename".localized(),
            EntityOptionMenuItem::PickUp => "entity.menu.pickup".localized(),
            EntityOptionMenuItem::Read(_) => "entity.menu.read".localized(),
            EntityOptionMenuItem::Fight => "entity.menu.fight".localized(),
            EntityOptionMenuItem::ChangeLock => "entity.menu.change_lock".localized(),
            EntityOptionMenuItem::ChangeDestinationWorld => "entity.menu.change_destination_world".localized(),
            EntityOptionMenuItem::ChangeDestinationX => "entity.menu.change_destination_x".localized(),
            EntityOptionMenuItem::ChangeDestinationY => "entity.menu.change_destination_y".localized(),
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
    ChangingDestinationWorld,
    ChangingDestinationX,
    ChangingDestinationY,
}

pub struct EntityOptionsMenu {
    entity: Box<Entity>,
    time_since_last_closed: f32,
    menu: Menu<EntityOptionMenuItem>,
    state: EntityOptionsMenuState,
    text_input: TextInput,
    lock_menu: Menu<LockType>
}

impl EntityOptionsMenu {
    pub fn new() -> Self {
        Self {
            entity: Box::new(SPECIES_NONE.make_entity()),
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
            ])
        }
    }

    pub fn show(
        &mut self, 
        entity: Box<Entity>, 
        creative_mode: bool,
        inventory: bool
    ) {
        if self.time_since_last_closed < 0.5 {
            return;
        }
        self.entity = entity;
        self.time_since_last_closed = 0.0;
        self.menu.items = self.available_options(creative_mode, inventory);

        if self.menu.items.is_empty() {
            return
        }

        self.menu.title = self.entity.name.to_owned();
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
            EntityOptionsMenuState::ChangingName => {
                self.update_from_text_input(keyboard, time_since_last_update, vec![
                        WorldStateUpdate::RenameEntity(self.entity.id, self.current_text()),
                        WorldStateUpdate::EngineUpdate(EngineStateUpdate::SaveGame)
                    ]
                )
            },
            EntityOptionsMenuState::ChangingDestinationWorld => {
                self.update_from_text_input(keyboard, time_since_last_update, vec![
                        WorldStateUpdate::UpdateDestinationWorld(self.entity.id, self.current_u32()),
                        WorldStateUpdate::EngineUpdate(EngineStateUpdate::SaveGame)
                    ]
                )
            },
            EntityOptionsMenuState::ChangingDestinationX => {
                self.update_from_text_input(keyboard, time_since_last_update, vec![
                        WorldStateUpdate::UpdateDestinationX(self.entity.id, self.current_i32()),
                        WorldStateUpdate::EngineUpdate(EngineStateUpdate::SaveGame)
                    ]
                )
            },
            EntityOptionsMenuState::ChangingDestinationY => {
                self.update_from_text_input(keyboard, time_since_last_update, vec![
                        WorldStateUpdate::UpdateDestinationY(self.entity.id, self.current_i32()),
                        WorldStateUpdate::EngineUpdate(EngineStateUpdate::SaveGame)
                    ]
                )
            },
            EntityOptionsMenuState::ChangingLock => self.update_from_change_lock(keyboard, time_since_last_update),
            EntityOptionsMenuState::Closed => self.update_from_close(keyboard, time_since_last_update),
        }
    }

    fn update_from_text_input(
        &mut self, 
        keyboard: &KeyboardEventsProvider, 
        time_since_last_update: f32,
        updates: Vec<WorldStateUpdate>
    ) -> MenuUpdate {
        self.text_input.update(keyboard, time_since_last_update);

        if self.text_input.did_confirm() {
            self.menu.close();
            self.state = EntityOptionsMenuState::Closed;
            self.text_input.clear();
            return (false, updates);
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
                WorldStateUpdate::ChangeLock(self.entity.id, selected_lock),
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
                    vec![WorldStateUpdate::RemoveEntity(self.entity.id)]
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
                        WorldStateUpdate::EngineUpdate(
                            EngineStateUpdate::AddToInventory(
                                self.entity.clone()
                            )
                        ),
                        WorldStateUpdate::RemoveEntity(self.entity.id),
                        WorldStateUpdate::EngineUpdate(EngineStateUpdate::SaveGame),
                    ]
                },
                EntityOptionMenuItem::Read(contents) => {
                    self.menu.clear_selection();
                    vec![
                        WorldStateUpdate::EngineUpdate(
                            EngineStateUpdate::DisplayLongText(
                                contents
                            )
                        )
                    ]
                },
                EntityOptionMenuItem::Fight => {
                    self.menu.clear_selection();
                    vec![
                        WorldStateUpdate::EngineUpdate(
                            EngineStateUpdate::Fight(
                                self.entity.clone()
                            )
                        )
                    ]
                },
                EntityOptionMenuItem::ChangeLock => {
                    self.menu.clear_selection();
                    self.ask_for_lock_type();
                    vec![]
                },
                EntityOptionMenuItem::ChangeDestinationWorld => {
                    self.menu.clear_selection();
                    self.ask_for_new_destination_world();
                    vec![]
                },
                EntityOptionMenuItem::ChangeDestinationX => {
                    self.menu.clear_selection();
                    self.ask_for_new_destination_x();
                    vec![]
                },
                EntityOptionMenuItem::ChangeDestinationY => {
                    self.menu.clear_selection();
                    self.ask_for_new_destination_y();
                    vec![]
                },
            };
            return (self.menu.is_open, updates);
        }

        (self.menu.is_open, vec![])
    }

    pub fn ui(&self) -> View {
        match self.state {
            EntityOptionsMenuState::ChangingDestinationWorld => self.text_input.ui(),
            EntityOptionsMenuState::ChangingDestinationX => self.text_input.ui(),
            EntityOptionsMenuState::ChangingDestinationY => self.text_input.ui(),
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

    fn ask_for_new_destination_world(&mut self) {
        self.state = EntityOptionsMenuState::ChangingDestinationWorld;
        self.text_input.clear();
        self.text_input.title = "entity.menu.change_destination_world".localized();
    }

    fn ask_for_new_destination_x(&mut self) {
        self.state = EntityOptionsMenuState::ChangingDestinationX;
        self.text_input.clear();
        self.text_input.title = "entity.menu.change_destination_x".localized();
    }

    fn ask_for_new_destination_y(&mut self) {
        self.state = EntityOptionsMenuState::ChangingDestinationY;
        self.text_input.clear();
        self.text_input.title = "entity.menu.change_destination_y".localized();
    }

    fn available_options(&self, creative_mode: bool, inventory: bool) -> Vec<EntityOptionMenuItem> {
        if inventory {
            self.available_options_inventory()
        } else if creative_mode {
            self.available_options_creative()
        } else {
            self.available_options_regular()
        }
    }

    fn available_options_creative(&self) -> Vec<EntityOptionMenuItem> {
        let nothing: Vec<EntityOptionMenuItem> = vec![];

        match self.entity.entity_type {
            EntityType::Hero => nothing,
            EntityType::Npc => vec![
                EntityOptionMenuItem::Fight,
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
                EntityOptionMenuItem::ChangeDestinationWorld,
                EntityOptionMenuItem::ChangeDestinationX,
                EntityOptionMenuItem::ChangeDestinationY,
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

    fn available_options_regular(&self) -> Vec<EntityOptionMenuItem> {
        let pickup = vec![EntityOptionMenuItem::PickUp];
        let fight = vec![EntityOptionMenuItem::Fight];
        let nothing: Vec<EntityOptionMenuItem> = vec![];

        match self.entity.entity_type {
            EntityType::Hero => fight,
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

    fn available_options_inventory(&self) -> Vec<EntityOptionMenuItem> {
        if let Some(contents) = self.entity.contents.clone() {
            vec![EntityOptionMenuItem::Read(contents.localized())]
        } else {
            vec![]
        }
    }
}

impl EntityOptionsMenu {
    fn current_text(&self) -> String {
        self.text_input.text().trim().to_owned()
    }

    fn current_i32(&self) -> i32 {
        self.current_text().parse().unwrap_or_default()
    }

    fn current_u32(&self) -> u32 {
        self.current_text().parse().unwrap_or_default()
    }
}