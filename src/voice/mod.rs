pub mod chat_manager;
pub mod speech_recognition;

pub use chat_manager::{VoiceChatManager, VoiceMessage};
pub use speech_recognition::{SpeechRecognizer, VoiceCommand, CommandType};
