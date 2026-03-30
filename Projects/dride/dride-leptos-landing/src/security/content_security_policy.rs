pub struct ContentSecurityPolicy;

impl ContentSecurityPolicy {
    /// Generate CSP meta tag HTML
    pub fn meta_tag() -> String {
        format!(
            r#"<meta http-equiv="Content-Security-Policy" content="{}" />"#,
            Self::get_policy()
        )
    }

    /// Get CSP policy as string
    pub fn get_policy() -> String {
        let directives = vec![
            Self::default_src(),
            Self::script_src(),
            Self::style_src(),
            Self::img_src(),
            Self::font_src(),
            Self::connect_src(),
            Self::frame_src(),
            Self::object_src(),
            Self::base_uri(),
            Self::form_action(),
            Self::require_trusted_types(),
        ].join(" ");

        directives
    }

    fn default_src() -> String {
        "default-src 'self'".to_string()
    }

    fn script_src() -> String {
        let sources = vec![
            "'self'",
            "'unsafe-inline'",
            "'unsafe-eval'",
            "https://api.devnet.solana.com",
            "https://api.mainnet-beta.solana.com",
            "https://*.phantom.app",
            "https://*.solflare.app",
            "https://*.backpack.app",
            "https://cdn.jsdelivr.net",
        ];
        format!("script-src {}", sources.join(" "))
    }

    fn style_src() -> String {
        let sources = vec![
            "'self'",
            "'unsafe-inline'",
            "https://cdn.jsdelivr.net",
        ];
        format!("style-src {}", sources.join(" "))
    }

    fn img_src() -> String {
        let sources = vec![
            "'self'",
            "data:",
            "https:",
        ];
        format!("img-src {}", sources.join(" "))
    }

    fn font_src() -> String {
        let sources = vec![
            "'self'",
            "data:",
        ];
        format!("font-src {}", sources.join(" "))
    }

    fn connect_src() -> String {
        let sources = vec![
            "'self'",
            "https://api.devnet.solana.com",
            "https://api.mainnet-beta.solana.com",
            "https://*.solana.com",
            "wss://*.solana.com",
            "https://*.phantom.app",
            "https://*.solflare.app",
        ];
        format!("connect-src {}", sources.join(" "))
    }

    fn frame_src() -> String {
        "frame-src 'none'".to_string()
    }

    fn object_src() -> String {
        "object-src 'none'".to_string()
    }

    fn base_uri() -> String {
        "base-uri 'self'".to_string()
    }

    fn form_action() -> String {
        "form-action 'self'".to_string()
    }

    fn require_trusted_types() -> String {
        "require-trusted-types-for 'script'".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csp_contains_required_directives() {
        let csp = ContentSecurityPolicy::get_policy();

        assert!(csp.contains("script-src"));
        assert!(csp.contains("style-src"));
        assert!(csp.contains("img-src"));
        assert!(csp.contains("connect-src"));
        assert!(csp.contains("frame-src 'none'"));
        assert!(csp.contains("object-src 'none'"));
    }

    #[test]
    fn test_csp_includes_trusted_domains() {
        let csp = ContentSecurityPolicy::get_policy();

        assert!(csp.contains("api.solana.com"));
        assert!(csp.contains("phantom.app"));
        assert!(csp.contains("solflare.app"));
    }

    #[test]
    fn test_meta_tag_format() {
        let meta_tag = ContentSecurityPolicy::meta_tag();

        assert!(meta_tag.contains("<meta"));
        assert!(meta_tag.contains("Content-Security-Policy"));
        assert!(meta_tag.contains("content="));
    }
}
