use std::fs;
use std::path::Path;

use astroraw_models::SessionMetadata;
use crate::error::{AstroError, Result};

pub fn load_session(path: &Path) -> Result<SessionMetadata> {
    if !path.exists() {
        return Err(AstroError::FileNotFound(path.display().to_string()));
    }

    let content = fs::read_to_string(path).map_err(AstroError::Io)?;
    let session: SessionMetadata = serde_json::from_str(&content)
        .map_err(|e| AstroError::InvalidSession(format!("{} (in {})", e, path.display())))?;

    Ok(session)
}
