use crate::{game_engine::keyboard_events_provider::{GameAction, KeyboardEventsProvider, KEY_BINDINGS}, lang::localizable::LocalizableText, menus::menu::{Menu, MenuItem, MenuUpdate}, spacing, text, ui::components::{with_backdrop, Spacing, Typography, View}, vstack};

#[derive(Clone)]
pub struct KeyBindingMenuItem {
    pub action: GameAction,
}

impl MenuItem for KeyBindingMenuItem {
    fn title(&self) -> String {
        format!("{:?}", self.action)
    }
}

pub struct KeyBindingMenu {
    pub menu: Menu<KeyBindingMenuItem>,
    waiting_for_key_input: Option<GameAction>,
}

impl KeyBindingMenu {
    pub fn new() -> Self {
        let items = vec![
            KeyBindingMenuItem { action: GameAction::Attack },
            KeyBindingMenuItem { action: GameAction::Confirm },
        ];

        let mut menu = Menu::new("keys.menu.title".localized(), items);
        menu.text = Some("keys.menu.message".localized());
        menu.show();

        Self { 
            menu, 
            waiting_for_key_input: None
        }
    }

    pub fn update(&mut self, keyboard: &KeyboardEventsProvider, time_since_last_update: f32) -> MenuUpdate {
        if let Some(action) = self.waiting_for_key_input {
            if let Some(key) = keyboard.currently_pressed_key {
                let mut key_bindings = KEY_BINDINGS.lock().unwrap();
                key_bindings.set_keys(action, vec![key]);
                self.waiting_for_key_input = None;
            }
            (true, vec![])
        } else {
            let (is_active, updates) = self.menu.update(keyboard, time_since_last_update);

            if self.menu.selection_has_been_confirmed {
                let selected_item = self.menu.selected_item();
                self.waiting_for_key_input = Some(selected_item.action);
                self.menu.selection_has_been_confirmed = false;
            }

            (is_active, updates)
        }
    }

    pub fn ui(&self) -> View {
        if let Some(action) = self.waiting_for_key_input {
            with_backdrop(
                vstack!(
                    Spacing::Zero,
                    text!(
                        Typography::Regular,
                        format!("{} {:?}", "keys.menu.press_to_change".localized(), action)
                    ),
                    spacing!(Spacing::LG)
                )
            )
        } else {
            self.menu.ui()
        }
    }
}