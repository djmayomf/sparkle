use std::collections::HashMap;
use enigo::{Enigo, Key, KeyboardControllable, MouseControllable};
use tokio::sync::mpsc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBinding {
    pub game: String,
    pub action: String,
    pub key: Key,
    pub modifiers: Vec<Key>,
    pub hold_duration: Option<u64>, // Duration in milliseconds
}

#[derive(Debug)]
pub struct GameInputSystem {
    enigo: Enigo,
    key_bindings: HashMap<String, KeyBinding>,
    current_game: String,
    push_to_talk_key: Key,
    is_push_to_talk_active: bool,
    input_tx: mpsc::Sender<GameInput>,
}

#[derive(Debug, Clone)]
pub enum GameInput {
    KeyPress(Key),
    KeyRelease(Key),
    MouseMove(i32, i32),
    MouseClick,
    PushToTalk(bool),
}

impl GameInputSystem {
    pub fn new(input_tx: mpsc::Sender<GameInput>) -> Self {
        let mut system = Self {
            enigo: Enigo::new(),
            key_bindings: HashMap::new(),
            current_game: String::new(),
            push_to_talk_key: Key::Alt,
            is_push_to_talk_active: false,
            input_tx,
        };
        
        system.init_key_bindings();
        system
    }

    fn init_key_bindings(&mut self) {
        // Overwatch bindings
        self.add_overwatch_bindings();
        
        // Valorant bindings
        self.add_valorant_bindings();
        
        // Apex Legends bindings
        self.add_apex_bindings();
        
        // Minecraft bindings
        self.add_minecraft_bindings();
    }

    fn add_overwatch_bindings(&mut self) {
        // Movement
        self.add_key_binding("overwatch", "move_forward", Key::Layout('w'), vec![], None);
        self.add_key_binding("overwatch", "move_back", Key::Layout('s'), vec![], None);
        self.add_key_binding("overwatch", "move_left", Key::Layout('a'), vec![], None);
        self.add_key_binding("overwatch", "move_right", Key::Layout('d'), vec![], None);
        self.add_key_binding("overwatch", "jump", Key::Space, vec![], None);
        self.add_key_binding("overwatch", "crouch", Key::Layout('c'), vec![], None);

        // Abilities
        self.add_key_binding("overwatch", "primary_fire", Key::Layout('1'), vec![], None);
        self.add_key_binding("overwatch", "secondary_fire", Key::Layout('2'), vec![], None);
        self.add_key_binding("overwatch", "ability_1", Key::Layout('e'), vec![], None);
        self.add_key_binding("overwatch", "ability_2", Key::Layout('q'), vec![], None);
        self.add_key_binding("overwatch", "ultimate", Key::Layout('y'), vec![], None);
        self.add_key_binding("overwatch", "melee", Key::Layout('v'), vec![], None);
        
        // Communication
        self.add_key_binding("overwatch", "voice_chat", Key::Layout('v'), vec![], None);
        self.add_key_binding("overwatch", "team_chat", Key::Enter, vec![], None);
        self.add_key_binding("overwatch", "quick_melee", Key::Layout('v'), vec![], None);
        self.add_key_binding("overwatch", "emote_wheel", Key::Layout('c'), vec![Key::Control], None);
    }

    fn add_valorant_bindings(&mut self) {
        // Movement
        self.add_key_binding("valorant", "move_forward", Key::Layout('w'), vec![], None);
        self.add_key_binding("valorant", "move_back", Key::Layout('s'), vec![], None);
        self.add_key_binding("valorant", "move_left", Key::Layout('a'), vec![], None);
        self.add_key_binding("valorant", "move_right", Key::Layout('d'), vec![], None);
        self.add_key_binding("valorant", "jump", Key::Space, vec![], None);
        self.add_key_binding("valorant", "walk", Key::Shift, vec![], None);
        self.add_key_binding("valorant", "crouch", Key::Control, vec![], None);

        // Abilities
        self.add_key_binding("valorant", "primary_weapon", Key::Layout('1'), vec![], None);
        self.add_key_binding("valorant", "secondary_weapon", Key::Layout('2'), vec![], None);
        self.add_key_binding("valorant", "ability_1", Key::Layout('c'), vec![], None);
        self.add_key_binding("valorant", "ability_2", Key::Layout('q'), vec![], None);
        self.add_key_binding("valorant", "signature_ability", Key::Layout('e'), vec![], None);
        self.add_key_binding("valorant", "ultimate", Key::Layout('x'), vec![], None);

        // Communication
        self.add_key_binding("valorant", "voice_chat", Key::Layout('v'), vec![], None);
        self.add_key_binding("valorant", "team_chat", Key::Enter, vec![], None);
        self.add_key_binding("valorant", "ping", Key::Layout('z'), vec![], None);
        self.add_key_binding("valorant", "radio_wheel", Key::Layout('y'), vec![], None);
    }

    fn add_apex_bindings(&mut self) {
        // Movement
        self.add_key_binding("apex", "move_forward", Key::Layout('w'), vec![], None);
        self.add_key_binding("apex", "move_back", Key::Layout('s'), vec![], None);
        self.add_key_binding("apex", "move_left", Key::Layout('a'), vec![], None);
        self.add_key_binding("apex", "move_right", Key::Layout('d'), vec![], None);
        self.add_key_binding("apex", "jump", Key::Space, vec![], None);
        self.add_key_binding("apex", "sprint", Key::Shift, vec![], None);
        self.add_key_binding("apex", "crouch", Key::Control, vec![], None);
        self.add_key_binding("apex", "slide", Key::Control, vec![], Some(300)); // Hold duration for sliding

        // Combat
        self.add_key_binding("apex", "primary_weapon", Key::Layout('1'), vec![], None);
        self.add_key_binding("apex", "secondary_weapon", Key::Layout('2'), vec![], None);
        self.add_key_binding("apex", "tactical_ability", Key::Layout('q'), vec![], None);
        self.add_key_binding("apex", "ultimate_ability", Key::Layout('z'), vec![], None);
        self.add_key_binding("apex", "melee", Key::Layout('v'), vec![], None);
        self.add_key_binding("apex", "grenade", Key::Layout('g'), vec![], None);

        // Interaction
        self.add_key_binding("apex", "interact", Key::Layout('e'), vec![], None);
        self.add_key_binding("apex", "reload", Key::Layout('r'), vec![], None);
        self.add_key_binding("apex", "inventory", Key::Tab, vec![], None);
        self.add_key_binding("apex", "map", Key::Layout('m'), vec![], None);

        // Communication
        self.add_key_binding("apex", "voice_chat", Key::Layout('v'), vec![], None);
        self.add_key_binding("apex", "ping", Key::Layout('f'), vec![], None);
        self.add_key_binding("apex", "ping_wheel", Key::Layout('f'), vec![Key::Alt], None);
    }

    fn add_minecraft_bindings(&mut self) {
        // Movement
        self.add_key_binding("minecraft", "move_forward", Key::Layout('w'), vec![], None);
        self.add_key_binding("minecraft", "move_back", Key::Layout('s'), vec![], None);
        self.add_key_binding("minecraft", "move_left", Key::Layout('a'), vec![], None);
        self.add_key_binding("minecraft", "move_right", Key::Layout('d'), vec![], None);
        self.add_key_binding("minecraft", "jump", Key::Space, vec![], None);
        self.add_key_binding("minecraft", "sneak", Key::Shift, vec![], None);
        self.add_key_binding("minecraft", "sprint", Key::Control, vec![], None);

        // Actions
        self.add_key_binding("minecraft", "attack", Key::Layout('1'), vec![], None); // Left click
        self.add_key_binding("minecraft", "use", Key::Layout('2'), vec![], None);    // Right click
        self.add_key_binding("minecraft", "drop", Key::Layout('q'), vec![], None);
        self.add_key_binding("minecraft", "inventory", Key::Layout('e'), vec![], None);
        self.add_key_binding("minecraft", "hotbar_1", Key::Layout('1'), vec![], None);
        self.add_key_binding("minecraft", "hotbar_2", Key::Layout('2'), vec![], None);
        self.add_key_binding("minecraft", "hotbar_3", Key::Layout('3'), vec![], None);
        self.add_key_binding("minecraft", "hotbar_4", Key::Layout('4'), vec![], None);
        self.add_key_binding("minecraft", "hotbar_5", Key::Layout('5'), vec![], None);
        self.add_key_binding("minecraft", "hotbar_6", Key::Layout('6'), vec![], None);
        self.add_key_binding("minecraft", "hotbar_7", Key::Layout('7'), vec![], None);
        self.add_key_binding("minecraft", "hotbar_8", Key::Layout('8'), vec![], None);
        self.add_key_binding("minecraft", "hotbar_9", Key::Layout('9'), vec![], None);

        // Interface
        self.add_key_binding("minecraft", "chat", Key::Layout('t'), vec![], None);
        self.add_key_binding("minecraft", "command", Key::Forward, vec![], None);
        self.add_key_binding("minecraft", "advancements", Key::Layout('l'), vec![], None);
        self.add_key_binding("minecraft", "swap_hands", Key::Layout('f'), vec![], None);
    }

    fn add_key_binding(&mut self, game: &str, action: &str, key: Key, modifiers: Vec<Key>, hold_duration: Option<u64>) {
        let binding = KeyBinding {
            game: game.to_string(),
            action: action.to_string(),
            key,
            modifiers,
            hold_duration,
        };
        self.key_bindings.insert(format!("{}:{}", game, action), binding);
    }

    pub async fn perform_action(&mut self, game: &str, action: &str) -> Result<(), Box<dyn std::error::Error>> {
        let binding_key = format!("{}:{}", game, action);
        if let Some(binding) = self.key_bindings.get(&binding_key) {
            // Press modifier keys
            for modifier in &binding.modifiers {
                self.enigo.key_down(*modifier);
            }

            // Press main key
            self.enigo.key_down(binding.key);
            
            // Hold if duration specified
            if let Some(duration) = binding.hold_duration {
                tokio::time::sleep(tokio::time::Duration::from_millis(duration)).await;
            }

            // Release main key
            self.enigo.key_up(binding.key);

            // Release modifier keys
            for modifier in binding.modifiers.iter().rev() {
                self.enigo.key_up(*modifier);
            }

            // Send input event
            self.input_tx.send(GameInput::KeyPress(binding.key)).await?;
        }
        Ok(())
    }

    pub async fn toggle_push_to_talk(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.is_push_to_talk_active = !self.is_push_to_talk_active;
        
        if self.is_push_to_talk_active {
            self.enigo.key_down(self.push_to_talk_key);
        } else {
            self.enigo.key_up(self.push_to_talk_key);
        }

        self.input_tx.send(GameInput::PushToTalk(self.is_push_to_talk_active)).await?;
        Ok(())
    }

    pub async fn move_mouse(&mut self, x: i32, y: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.enigo.mouse_move_to(x, y);
        self.input_tx.send(GameInput::MouseMove(x, y)).await?;
        Ok(())
    }

    pub fn set_current_game(&mut self, game: &str) {
        self.current_game = game.to_string();
    }
} 