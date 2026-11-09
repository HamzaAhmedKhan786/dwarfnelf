use subtle::ConstantTimeEq;

pub struct LocalAccess {
    authority: String,
    bearer: String,
}

impl LocalAccess {
    pub fn new(port: u16, token: &str) -> Result<Self, &'static str> {
        if token.len() != 64 || !token.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err("DWARFNELF_TOKEN must contain 64 hex characters from a secure generator");
        }
        Ok(Self {
            authority: format!("127.0.0.1:{port}"),
            bearer: format!("Bearer {token}"),
        })
    }

    pub fn allows_host(&self, host: Option<&str>, browser_request: bool) -> bool {
        host == Some(self.authority.as_str()) && !browser_request
    }

    pub fn authenticates(&self, authorization: Option<&str>) -> bool {
        authorization
            .is_some_and(|value| bool::from(self.bearer.as_bytes().ct_eq(value.as_bytes())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_only_the_configured_bearer_and_native_authority() {
        assert!(LocalAccess::new(8765, "short").is_err());
        let token = "a".repeat(64);
        let access = LocalAccess::new(8765, &token).unwrap();
        assert!(access.authenticates(Some(&format!("Bearer {token}"))));
        assert!(!access.authenticates(Some(&format!("Bearer {}", "b".repeat(64)))));
        assert!(access.allows_host(Some("127.0.0.1:8765"), false));
        assert!(!access.allows_host(Some("127.0.0.1:8765"), true));
        assert!(!access.allows_host(Some("localhost:8765"), false));
    }
}
