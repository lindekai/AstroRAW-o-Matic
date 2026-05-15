use astroraw_models::{FrameType, SessionMetadata};

#[test]
fn parses_full_session_json() {
    let json = r#"
    {
        "schema_version": "1.0",
        "object": "M31",
        "observer": "Test Observer",
        "frame_type": "light",
        "equipment": {
            "telescope": "80ED",
            "focal_length": 600.0,
            "aperture": 7.5,
            "filter": "L",
            "camera": {
                "make": "Canon",
                "model": "EOS 600Da",
                "pixel_size_x": 4.3,
                "pixel_size_y": 4.3
            }
        },
        "location": {
            "latitude": 52.52,
            "longitude": 13.40,
            "elevation": 34.0
        },
        "output": {
            "raw_mode": "raw_bayer",
            "header_mode": "astro",
            "overwrite": false,
            "write_history": true
        }
    }
    "#;

    let session: SessionMetadata = serde_json::from_str(json).expect("should parse");
    assert_eq!(session.object.as_deref(), Some("M31"));
    assert_eq!(session.frame_type, Some(FrameType::Light));

    let eq = session.equipment.as_ref().unwrap();
    assert_eq!(eq.focal_length, Some(600.0));
    assert_eq!(eq.filter.as_deref(), Some("L"));

    let cam = eq.camera.as_ref().unwrap();
    assert_eq!(cam.pixel_size_x, Some(4.3));
}

#[test]
fn file_overrides_parse_correctly() {
    let json = r#"
    {
        "schema_version": "1.0",
        "frame_type": "light",
        "file_overrides": {
            "dark_001.CR2": {
                "frame_type": "dark"
            }
        }
    }
    "#;

    let session: SessionMetadata = serde_json::from_str(json).expect("should parse");
    let ovr = session.file_overrides.get("dark_001.CR2").unwrap();
    assert_eq!(ovr.frame_type, Some(FrameType::Dark));
}

#[test]
fn minimal_session_uses_defaults() {
    let json = r#"{"schema_version": "1.0"}"#;
    let session: SessionMetadata = serde_json::from_str(json).expect("should parse");
    assert_eq!(session.schema_version, "1.0");
    assert!(session.object.is_none());
    assert!(session.frame_type.is_none());
}
