use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use tracing::{info, warn, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub timestamp: DateTime<Utc>,
    pub event_type: SecurityEventType,
    pub severity: EventSeverity,
    pub wallet_address: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub details: serde_json::Value,
    pub outcome: EventOutcome,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityEventType {
    WalletConnected,
    WalletDisconnected,
    PresalePurchaseAttempt,
    PresalePurchaseSuccess,
    PresalePurchaseFailed,
    RateLimitExceeded,
    UnauthorizedAccessAttempt,
    InvalidInputDetected,
    SuspiciousActivity,
    AuthenticationFailure,
    #[serde(rename = "XSSAttempt")]
    XSSAttempt,
    #[serde(rename = "SQLInjectionAttempt")]
    SQLInjectionAttempt,
    #[serde(rename = "CSRFAttempt")]
    CSRFAttempt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventOutcome {
    Success,
    Failure,
    Blocked,
    Flagged,
}

pub struct SecurityMonitor;

impl SecurityMonitor {
    /// Log security event
    pub fn log_event(event: SecurityEvent) {
        match event.severity {
            EventSeverity::Info => {
                info!(
                    timestamp = %event.timestamp,
                    event_type = ?event.event_type,
                    wallet = event.wallet_address,
                    "Security event: {:?}",
                    event.event_type
                );
            }
            EventSeverity::Warning => {
                warn!(
                    timestamp = %event.timestamp,
                    event_type = ?event.event_type,
                    wallet = event.wallet_address,
                    details = %event.details,
                    "Security warning: {:?}",
                    event.event_type
                );
            }
            EventSeverity::Error | EventSeverity::Critical => {
                error!(
                    timestamp = %event.timestamp,
                    event_type = ?event.event_type,
                    wallet = event.wallet_address,
                    details = %event.details,
                    "Security {}: {:?}",
                    match event.severity {
                        EventSeverity::Error => "error",
                        EventSeverity::Critical => "critical",
                        _ => unreachable!(),
                    },
                    event.event_type
                );
            }
        }

        // In production, send to external monitoring service (Sentry, Datadog, etc)
        Self::send_to_monitoring_service(event);
    }

    /// Log presale purchase attempt
    pub fn log_presale_attempt(
        wallet_address: &str,
        sol_amount: f64,
        success: bool,
        error: Option<String>,
    ) {
        let event = SecurityEvent {
            timestamp: Utc::now(),
            event_type: if success {
                SecurityEventType::PresalePurchaseSuccess
            } else {
                SecurityEventType::PresalePurchaseFailed
            },
            severity: EventSeverity::Info,
            wallet_address: Some(wallet_address.to_string()),
            ip_address: None, // Would be populated from request
            user_agent: None,
            details: serde_json::json!({
                "sol_amount": sol_amount,
                "dride_amount": sol_amount * 200.0,
                "error": error,
            }),
            outcome: if success {
                EventOutcome::Success
            } else {
                EventOutcome::Failure
            },
        };

        Self::log_event(event);
    }

    /// Log invalid input detection
    pub fn log_invalid_input(
        wallet_address: Option<&str>,
        field: &str,
        value: &str,
        reason: &str,
    ) {
        let event = SecurityEvent {
            timestamp: Utc::now(),
            event_type: SecurityEventType::InvalidInputDetected,
            severity: EventSeverity::Warning,
            wallet_address: wallet_address.map(|s| s.to_string()),
            ip_address: None,
            user_agent: None,
            details: serde_json::json!({
                "field": field,
                "value": value,
                "reason": reason,
            }),
            outcome: EventOutcome::Blocked,
        };

        Self::log_event(event);
    }

    /// Log rate limit exceeded
    pub fn log_rate_limit(
        wallet_address: &str,
        limit_type: &str,
    ) {
        let event = SecurityEvent {
            timestamp: Utc::now(),
            event_type: SecurityEventType::RateLimitExceeded,
            severity: EventSeverity::Warning,
            wallet_address: Some(wallet_address.to_string()),
            ip_address: None,
            user_agent: None,
            details: serde_json::json!({
                "limit_type": limit_type,
            }),
            outcome: EventOutcome::Blocked,
        };

        Self::log_event(event);
    }

    /// Log XSS attempt
    pub fn log_xss_attempt(
        wallet_address: Option<&str>,
        input: &str,
    ) {
        let event = SecurityEvent {
            timestamp: Utc::now(),
            event_type: SecurityEventType::XSSAttempt,
            severity: EventSeverity::Critical,
            wallet_address: wallet_address.map(|s| s.to_string()),
            ip_address: None,
            user_agent: None,
            details: serde_json::json!({
                "input": input,
            }),
            outcome: EventOutcome::Blocked,
        };

        Self::log_event(event);
    }

    /// Detect suspicious activity patterns
    pub fn detect_suspicious_activity(
        wallet_address: &str,
        recent_events: &[SecurityEvent],
    ) -> bool {
        // Check for rapid successive purchases
        let purchase_attempts = recent_events.iter()
            .filter(|e| {
                e.wallet_address.as_ref() == Some(wallet_address) &&
                matches!(e.event_type, SecurityEventType::PresalePurchaseAttempt)
            })
            .count();

        if purchase_attempts > 10 {
            warn!("Suspicious activity: {} purchase attempts from {} in short time", purchase_attempts, wallet_address);
            return true;
        }

        // Check for repeated failures
        let failures = recent_events.iter()
            .filter(|e| {
                e.wallet_address.as_ref() == Some(wallet_address) &&
                e.outcome == EventOutcome::Failure
            })
            .count();

        if failures > 5 {
            warn!("Suspicious activity: {} failures from {}", failures, wallet_address);
            return true;
        }

        false
    }

    fn send_to_monitoring_service(event: SecurityEvent) {
        // In production, send to Sentry, Datadog, or custom service
        // For now, we log via tracing
        debug!("Sending event to monitoring service: {:?}", event);
    }
}
