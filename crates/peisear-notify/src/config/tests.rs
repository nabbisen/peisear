use super::*;

#[test]
fn tls_mode_default_465_is_implicit() {
    assert_eq!(TlsMode::default_for_port(465), TlsMode::Implicit);
}

#[test]
fn tls_mode_default_587_is_starttls() {
    assert_eq!(TlsMode::default_for_port(587), TlsMode::Starttls);
}

#[test]
fn tls_mode_from_str_accepts_aliases() {
    assert_eq!(TlsMode::from_str_loose("implicit"), Some(TlsMode::Implicit));
    assert_eq!(TlsMode::from_str_loose("smtps"), Some(TlsMode::Implicit));
    assert_eq!(TlsMode::from_str_loose("starttls"), Some(TlsMode::Starttls));
    assert_eq!(
        TlsMode::from_str_loose("submission"),
        Some(TlsMode::Starttls)
    );
    assert_eq!(TlsMode::from_str_loose("garbage"), None);
}
