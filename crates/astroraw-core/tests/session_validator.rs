use astroraw_core::session::validator::validate_session;
use astroraw_models::{FrameType, SessionMetadata};

#[test]
fn empty_session_has_warnings_not_errors() {
    let session = SessionMetadata::default();
    let report = validate_session(&session);
    assert!(report.is_valid(), "Empty session should be valid (warnings only)");
    assert!(!report.warnings.is_empty(), "Empty session should generate warnings");
}

#[test]
fn complete_session_minimal_warnings() {
    let mut session = SessionMetadata::default();
    session.object = Some("M42".to_string());
    session.observer = Some("Test Observer".to_string());
    session.frame_type = Some(FrameType::Light);

    let report = validate_session(&session);
    assert!(report.is_valid());
    // May still warn about missing equipment/location
}

#[test]
fn no_object_triggers_warning() {
    let session = SessionMetadata::default();
    let report = validate_session(&session);
    assert!(report.warnings.iter().any(|w| w.contains("OBJECT")));
}
