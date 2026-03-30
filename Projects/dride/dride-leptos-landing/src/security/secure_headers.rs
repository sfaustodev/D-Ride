pub struct SecurityHeaders;

impl SecurityHeaders {
    /// Get CSP meta tag content
    pub fn get_csp_content() -> String {
        let script_sources = vec![
            "'self'",
            "'unsafe-inline'",
            "'unsafe-eval'",
            "https://api.devnet.solana.com",
            "https://api.mainnet-beta.solana.com",
            "https://*.phantom.app",
            "https://*.solflare.app",
            "https://*.backpack.app",
        ].join(" ");

        let style_sources = vec![
            "'self'",
            "'unsafe-inline'",
        ].join(" ");

        let connect_sources = vec![
            "'self'",
            "https://api.devnet.solana.com",
            "https://api.mainnet-beta.solana.com",
            "https://*.solana.com",
            "wss://*.solana.com",
        ].join(" ");

        format!(
            "default-src 'self'; \
            script-src {}; \
            style-src {}; \
            img-src 'self' data: https:; \
            font-src 'self' data:; \
            connect-src {}; \
            frame-src 'none'; \
            object-src 'none'; \
            base-uri 'self'; \
            form-action 'self'; \
            require-trusted-types-for 'script'",
            script_sources, style_sources, connect_sources
        )
    }

    /// Get security headers map
    pub fn get_security_headers() -> Vec<(&'static str, String)> {
        vec![
            ("Content-Security-Policy", Self::get_csp_content()),
            ("X-Content-Type-Options", "nosniff".to_string()),
            ("X-Frame-Options", "DENY".to_string()),
            ("X-XSS-Protection", "1; mode=block".to_string()),
            ("Strict-Transport-Security", "max-age=31536000; includeSubDomains; preload".to_string()),
            ("Referrer-Policy", "strict-origin-when-cross-origin".to_string()),
            (
                "Permissions-Policy",
                "geolocation=(), \
                 microphone=(), \
                 camera=(), \
                 payment=(), \
                 usb=(), \
                 magnetometer=(), \
                 gyroscope=(), \
                 accelerometer=()"
                .to_string(),
            ),
            ("Cache-Control", "no-store, no-cache, must-revalidate, private".to_string()),
            ("Pragma", "no-cache".to_string()),
        ]
    }
}
