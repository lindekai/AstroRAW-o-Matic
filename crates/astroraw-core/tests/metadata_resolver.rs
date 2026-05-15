use astroraw_core::metadata::resolver::MetadataResolver;
use astroraw_models::{FrameType, RawMetadata, SessionMetadata};

fn make_raw_meta() -> RawMetadata {
    RawMetadata {
        camera_make: Some("Canon".to_string()),
        camera_model: Some("EOS 600D".to_string()),
        exposure_time: Some(120.0),
        iso_speed: Some(800),
        bit_depth: Some(14),
        bayer_pattern: Some("RGGB".to_string()),
        width: Some(5202),
        height: Some(3464),
        ..Default::default()
    }
}

#[test]
fn resolves_mandatory_keywords() {
    let session = SessionMetadata::default();
    let raw = make_raw_meta();
    let resolver = MetadataResolver::new(&session, raw, "test.CR2");
    let resolved = resolver.resolve(None, None);

    let keywords: Vec<&str> = resolved.header.records.iter().map(|r| r.keyword.as_str()).collect();
    assert!(keywords.contains(&"SIMPLE"));
    assert!(keywords.contains(&"BITPIX"));
    assert!(keywords.contains(&"NAXIS"));
    assert!(keywords.contains(&"NAXIS1"));
    assert!(keywords.contains(&"NAXIS2"));
    assert!(keywords.contains(&"EXTEND"));
}

#[test]
fn resolves_camera_and_exposure() {
    let session = SessionMetadata::default();
    let raw = make_raw_meta();
    let resolver = MetadataResolver::new(&session, raw, "test.CR2");
    let resolved = resolver.resolve(None, None);

    let instrume = resolved.header.records.iter().find(|r| r.keyword == "INSTRUME");
    assert!(instrume.is_some());

    let exptime = resolved.header.records.iter().find(|r| r.keyword == "EXPTIME");
    assert!(exptime.is_some());
}

#[test]
fn cli_frame_type_overrides_session() {
    let mut session = SessionMetadata::default();
    session.frame_type = Some(FrameType::Light);
    let raw = make_raw_meta();
    let resolver = MetadataResolver::new(&session, raw, "test.CR2");
    let resolved = resolver.resolve(Some(FrameType::Dark), None);

    let imagetyp = resolved.header.records.iter().find(|r| r.keyword == "IMAGETYP");
    assert!(imagetyp.is_some());
    if let astroraw_models::FitsValue::Str(ref v) = imagetyp.unwrap().value {
        assert_eq!(v, "DARK");
    }
}

#[test]
fn cli_object_overrides_session() {
    let mut session = SessionMetadata::default();
    session.object = Some("M31".to_string());
    let raw = make_raw_meta();
    let resolver = MetadataResolver::new(&session, raw, "test.CR2");
    let resolved = resolver.resolve(None, Some("NGC 7000"));

    let object = resolved.header.records.iter().find(|r| r.keyword == "OBJECT");
    assert!(object.is_some());
    if let astroraw_models::FitsValue::Str(ref v) = object.unwrap().value {
        assert_eq!(v, "NGC 7000");
    }
}

#[test]
fn missing_exptime_generates_warning() {
    let session = SessionMetadata::default();
    let raw = RawMetadata {
        camera_model: Some("EOS 600D".to_string()),
        width: Some(100),
        height: Some(100),
        ..Default::default()
    };
    let resolver = MetadataResolver::new(&session, raw, "test.CR2");
    let resolved = resolver.resolve(None, None);

    assert!(resolved.warnings.iter().any(|w| w.contains("EXPTIME")));
}
