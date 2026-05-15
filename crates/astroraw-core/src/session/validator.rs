use astroraw_models::SessionMetadata;

#[derive(Debug, Default)]
pub struct ValidationReport {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationReport {
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }
}

pub fn validate_session(session: &SessionMetadata) -> ValidationReport {
    let mut report = ValidationReport::default();

    // Schema version check
    if session.schema_version != "1.0" {
        report.warnings.push(format!(
            "Unknown schema_version '{}'. The Guide might not approve of future-schema files.",
            session.schema_version
        ));
    }

    // Warn about commonly missing but important fields
    if session.object.is_none() {
        report.warnings.push(
            "OBJECT not specified. The FITS header will omit it. \
             Consider adding 'object' to your session JSON."
                .to_string(),
        );
    }

    if session.observer.is_none() {
        report.warnings.push("OBSERVER not specified.".to_string());
    }

    if session.equipment.is_none() {
        report.warnings.push(
            "No equipment data in session JSON. \
             Camera pixel size, telescope, and focal length will be inferred or omitted."
                .to_string(),
        );
    }

    if session.frame_type.is_none() {
        report.warnings.push(
            "Frame type not specified. Defaulting to 'light'. \
             Use --type or add 'frame_type' to session JSON if this is wrong."
                .to_string(),
        );
    }

    report
}
