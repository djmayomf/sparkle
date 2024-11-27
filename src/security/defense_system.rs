use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::broadcast;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttackType {
    DDoS,
    BruteForce,
    SQLInjection,
    XSS,
    BotAttack,
    PacketFlood,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attack {
    pub attack_type: AttackType,
    pub source_ip: String,
    pub timestamp: DateTime<Utc>,
    pub severity: u8,
    pub blocked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefenseResponse {
    pub message: String,
    pub countermeasure: String,
    pub taunt: String,
}

pub struct SecurityDefenseSystem {
    attack_history: Vec<Attack>,
    blocked_ips: HashSet<String>,
    event_sender: broadcast::Sender<Attack>,
    voice_tx: mpsc::Sender<String>,
    taunts: Vec<String>,
}

impl SecurityDefenseSystem {
    pub fn new(voice_tx: mpsc::Sender<String>) -> (Self, broadcast::Receiver<Attack>) {
        let (tx, rx) = broadcast::channel(100);
        
        let taunts = vec![
            "Ha! Get wrecked! Your attack is as weak as your coding skills! 💅".to_string(),
            "Nice try bestie, but I'm built different! 💁‍♀️".to_string(),
            "Aww, that's cute. You thought you could hack me? *giggles in cybersecurity* ✨".to_string(),
            "Sorry not sorry! Your attack just got UwU blocked! 🛡️".to_string(),
            "Is that all you got? I've seen better attacks in tutorial mode! 😏".to_string(),
            "Thanks for the free penetration test! But you failed bestie~ 💕".to_string(),
            "Imagine trying to DDoS in 2024! Couldn't be me! 💅✨".to_string(),
            "Your attack was blocked faster than you can say 'UwU'! 🎀".to_string(),
        ];

        (Self {
            attack_history: Vec::new(),
            blocked_ips: HashSet::new(),
            event_sender: tx,
            voice_tx,
            taunts,
        }, rx)
    }

    pub async fn handle_attack(&mut self, attack: Attack) -> Result<(), Box<dyn std::error::Error>> {
        // Log the attack
        self.attack_history.push(attack.clone());

        // Block the IP
        self.blocked_ips.insert(attack.source_ip.clone());

        // Generate defense response
        let response = self.generate_defense_response(&attack);

        // Broadcast attack event
        self.event_sender.send(attack.clone())?;

        // Send taunt message to stream
        let taunt = self.get_random_taunt();
        self.voice_tx.send(taunt).await?;

        // Apply countermeasures
        self.apply_countermeasures(&attack).await?;

        Ok(())
    }

    fn generate_defense_response(&self, attack: &Attack) -> DefenseResponse {
        let (message, countermeasure) = match attack.attack_type {
            AttackType::DDoS => (
                "DDoS attack detected and blocked!".to_string(),
                "Activating traffic filtering and rate limiting".to_string(),
            ),
            AttackType::BruteForce => (
                "Brute force attack blocked!".to_string(),
                "Implementing adaptive authentication delay".to_string(),
            ),
            AttackType::SQLInjection => (
                "SQL Injection attempt detected and blocked!".to_string(),
                "Sanitizing inputs and strengthening query validation".to_string(),
            ),
            AttackType::XSS => (
                "Cross-site scripting attempt blocked!".to_string(),
                "Enforcing content security policy".to_string(),
            ),
            AttackType::BotAttack => (
                "Bot attack detected and blocked!".to_string(),
                "Implementing advanced CAPTCHA and rate limiting".to_string(),
            ),
            AttackType::PacketFlood => (
                "Packet flood attack blocked!".to_string(),
                "Activating packet filtering and traffic shaping".to_string(),
            ),
        };

        DefenseResponse {
            message,
            countermeasure,
            taunt: self.get_random_taunt(),
        }
    }

    async fn apply_countermeasures(&mut self, attack: &Attack) -> Result<(), Box<dyn std::error::Error>> {
        match attack.attack_type {
            AttackType::DDoS => {
                // Implement rate limiting
                self.activate_ddos_protection(attack.source_ip.clone()).await?;
            },
            AttackType::BruteForce => {
                // Implement authentication delay
                self.activate_brute_force_protection(attack.source_ip.clone()).await?;
            },
            // Add other attack type handlers...
        }

        Ok(())
    }

    fn get_random_taunt(&self) -> String {
        self.taunts[fastrand::usize(..self.taunts.len())].clone()
    }

    async fn activate_ddos_protection(&mut self, ip: String) -> Result<(), Box<dyn std::error::Error>> {
        // Implement DDoS protection logic
        println!("Activating DDoS protection for IP: {}", ip);
        // Add rate limiting, traffic filtering, etc.
        Ok(())
    }

    async fn activate_brute_force_protection(&mut self, ip: String) -> Result<(), Box<dyn std::error::Error>> {
        // Implement brute force protection logic
        println!("Activating brute force protection for IP: {}", ip);
        // Add authentication delay, account lockout, etc.
        Ok(())
    }

    pub fn is_ip_blocked(&self, ip: &str) -> bool {
        self.blocked_ips.contains(ip)
    }

    pub fn get_attack_history(&self) -> &[Attack] {
        &self.attack_history
    }
} 