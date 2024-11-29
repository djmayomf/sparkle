use std::collections::HashMap;
use std::time::{Duration, Instant};
use rand::{thread_rng, Rng};

pub struct SecurityDefenseSystem {
    // Track authentication attempts and sessions
    auth_attempts: HashMap<String, (u32, Instant)>,
    active_sessions: HashMap<String, SessionInfo>,
    // Configuration
    max_auth_attempts: u32,
    auth_timeout: Duration,
    session_timeout: Duration,
}

struct SessionInfo {
    created_at: Instant,
    last_verified: Instant,
    trust_score: f32,
    permissions: Vec<Permission>,
}

#[derive(Clone, Debug)]
pub enum Permission {
    ReadChat,
    WriteChat,
    ModifyStream,
    AccessAdmin,
    ControlBot,
}

#[derive(Debug)]
pub enum SecurityError {
    TooManyAttempts,
    SessionExpired,
    InsufficientTrustScore,
    UnauthorizedAccess,
    InvalidCredentials,
}

impl std::error::Error for SecurityError {}

impl std::fmt::Display for SecurityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityError::TooManyAttempts => write!(f, "Too many authentication attempts"),
            SecurityError::SessionExpired => write!(f, "Session has expired"),
            SecurityError::InsufficientTrustScore => write!(f, "Insufficient trust score"),
            SecurityError::UnauthorizedAccess => write!(f, "Unauthorized access"),
            SecurityError::InvalidCredentials => write!(f, "Invalid credentials"),
        }
    }
}

impl SecurityDefenseSystem {
    pub fn new() -> Self {
        Self {
            auth_attempts: HashMap::new(),
            active_sessions: HashMap::new(),
            max_auth_attempts: 3,
            auth_timeout: Duration::from_secs(300),  // 5 minutes
            session_timeout: Duration::from_secs(3600), // 1 hour
        }
    }

    pub fn authenticate(&mut self, user_id: &str, credentials: &str) -> Result<String, SecurityError> {
        // Check for too many attempts
        if let Some((attempts, timestamp)) = self.auth_attempts.get(user_id) {
            if *attempts >= self.max_auth_attempts && 
               timestamp.elapsed() < self.auth_timeout {
                return Err(SecurityError::TooManyAttempts);
            }
        }

        // Verify credentials (implement your actual verification logic)
        if !self.verify_credentials(credentials) {
            self.record_failed_attempt(user_id);
            return Err(SecurityError::InvalidCredentials);
        }

        // Generate session token and create session
        let session_token = self.generate_session_token();
        self.create_session(user_id, &session_token);

        Ok(session_token)
    }

    pub fn verify_action(&mut self, session_token: &str, required_permission: Permission) -> Result<(), SecurityError> {
        let session = self.active_sessions.get_mut(session_token)
            .ok_or(SecurityError::UnauthorizedAccess)?;

        // Verify session is still valid
        if session.created_at.elapsed() > self.session_timeout {
            self.active_sessions.remove(session_token);
            return Err(SecurityError::SessionExpired);
        }

        // Check trust score
        if session.trust_score < 0.7 {
            return Err(SecurityError::InsufficientTrustScore);
        }

        // Verify permissions
        if !session.permissions.contains(&required_permission) {
            return Err(SecurityError::UnauthorizedAccess);
        }

        // Update last verified time
        session.last_verified = Instant::now();
        Ok(())
    }

    pub fn revoke_session(&mut self, session_token: &str) {
        self.active_sessions.remove(session_token);
    }

    fn record_failed_attempt(&mut self, user_id: &str) {
        let entry = self.auth_attempts
            .entry(user_id.to_string())
            .or_insert((0, Instant::now()));
        
        entry.0 += 1;
        entry.1 = Instant::now();
    }

    fn verify_credentials(&self, credentials: &str) -> bool {
        // Implement actual credential verification
        // This is a placeholder - replace with actual secure verification
        !credentials.is_empty()
    }

    fn generate_session_token(&self) -> String {
        // Generate a secure random token
        // This is a placeholder - implement proper secure token generation
        let mut rng = thread_rng();
        let token: u128 = rng.gen();
        format!("session_{:x}", token)
    }

    fn create_session(&mut self, user_id: &str, session_token: &str) {
        let session = SessionInfo {
            created_at: Instant::now(),
            last_verified: Instant::now(),
            trust_score: 1.0,
            permissions: vec![
                Permission::ReadChat,
                Permission::WriteChat,
            ],
        };
        self.active_sessions.insert(session_token.to_string(), session);
    }

    pub fn adjust_trust_score(&mut self, session_token: &str, adjustment: f32) {
        if let Some(session) = self.active_sessions.get_mut(session_token) {
            session.trust_score = (session.trust_score + adjustment)
                .max(0.0)
                .min(1.0);
        }
    }

    pub fn cleanup_expired_sessions(&mut self) {
        self.active_sessions.retain(|_, session| {
            session.created_at.elapsed() < self.session_timeout
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authentication_flow() {
        let mut system = SecurityDefenseSystem::new();
        
        // Test successful authentication
        let result = system.authenticate("user1", "valid_credentials");
        assert!(result.is_ok());

        // Test failed authentication
        let result = system.authenticate("user2", "");
        assert!(result.is_err());
    }

    #[test]
    fn test_session_verification() {
        let mut system = SecurityDefenseSystem::new();
        
        // Create a valid session
        let token = system.authenticate("user1", "valid_credentials").unwrap();
        
        // Test permission verification
        let result = system.verify_action(&token, Permission::ReadChat);
        assert!(result.is_ok());

        // Test invalid permission
        let result = system.verify_action(&token, Permission::AccessAdmin);
        assert!(result.is_err());
    }
} 