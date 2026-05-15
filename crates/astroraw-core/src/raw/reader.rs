use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use exif;
use tracing::{debug, warn};

use astroraw_models::RawMetadata;
use crate::error::{AstroError, Result};

/// Raw pixel data extracted from a CR2/RAW file.
/// For MVP we preserve Bayer data as 16-bit unsigned integers.
#[derive(Debug)]
pub struct RawPixelData {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u16>,
    pub bayer_pattern: Option<String>,
}

pub struct RawReader;

impl RawReader {
    /// Extract metadata from a RAW file without decoding pixel data.
    pub fn read_metadata(path: &Path) -> Result<RawMetadata> {
        if !path.exists() {
            return Err(AstroError::FileNotFound(path.display().to_string()));
        }

        let file = File::open(path).map_err(AstroError::Io)?;
        let mut reader = BufReader::new(file);

        let exif = exif::Reader::new()
            .read_from_container(&mut reader)
            .map_err(|e| AstroError::RawReadError(e.to_string()))?;

        let mut meta = RawMetadata::default();

        for field in exif.fields() {
            let tag = field.tag;
            match tag {
                exif::Tag::Make => {
                    meta.camera_make = Some(field.display_value().to_string().trim_matches('"').to_string());
                }
                exif::Tag::Model => {
                    meta.camera_model = Some(field.display_value().to_string().trim_matches('"').to_string());
                }
                exif::Tag::ExposureTime => {
                    if let exif::Value::Rational(ref v) = field.value {
                        if let Some(r) = v.first() {
                            meta.exposure_time = Some(r.num as f64 / r.denom as f64);
                        }
                    }
                }
                exif::Tag::PhotographicSensitivity => {
                    if let exif::Value::Short(ref v) = field.value {
                        meta.iso_speed = v.first().map(|&x| x as u32);
                    }
                }
                exif::Tag::DateTimeOriginal => {
                    let s = field.display_value().to_string();
                    meta.date_obs = parse_exif_datetime(&s);
                }
                exif::Tag::FocalLength => {
                    if let exif::Value::Rational(ref v) = field.value {
                        if let Some(r) = v.first() {
                            meta.focal_length = Some(r.num as f64 / r.denom as f64);
                        }
                    }
                }
                exif::Tag::FNumber => {
                    if let exif::Value::Rational(ref v) = field.value {
                        if let Some(r) = v.first() {
                            meta.aperture = Some(r.num as f64 / r.denom as f64);
                        }
                    }
                }
                exif::Tag::PixelXDimension | exif::Tag::ImageWidth => {
                    if meta.width.is_none() {
                        if let exif::Value::Long(ref v) = field.value {
                            meta.width = v.first().copied();
                        } else if let exif::Value::Short(ref v) = field.value {
                            meta.width = v.first().map(|&x| x as u32);
                        }
                    }
                }
                exif::Tag::PixelYDimension | exif::Tag::ImageLength => {
                    if meta.height.is_none() {
                        if let exif::Value::Long(ref v) = field.value {
                            meta.height = v.first().copied();
                        } else if let exif::Value::Short(ref v) = field.value {
                            meta.height = v.first().map(|&x| x as u32);
                        }
                    }
                }
                _ => {
                    debug!("Unhandled EXIF tag: {:?}", tag);
                }
            }
        }

        // CR2 files from Canon DSLRs are 14-bit raw, stored in 16-bit containers
        meta.bit_depth = Some(14);

        // Bayer pattern: Canon DSLRs typically use RGGB
        // This is not in standard EXIF; we infer it from make/model for MVP.
        if meta.bayer_pattern.is_none() {
            meta.bayer_pattern = infer_bayer_pattern(meta.camera_make.as_deref(), meta.camera_model.as_deref());
        }

        Ok(meta)
    }

    /// Read raw Bayer pixel data from a CR2 file.
    ///
    /// For MVP this returns a stub — full LibRaw FFI integration is the next step.
    /// The architecture is ready: replace the stub body with actual LibRaw calls.
    pub fn read_raw_bayer(path: &Path) -> Result<RawPixelData> {
        if !path.exists() {
            return Err(AstroError::FileNotFound(path.display().to_string()));
        }

        // Read metadata first to get dimensions
        let meta = Self::read_metadata(path)?;

        let width = meta.width.unwrap_or(0);
        let height = meta.height.unwrap_or(0);

        if width == 0 || height == 0 {
            return Err(AstroError::RawReadError(
                "Could not determine image dimensions. \
                 The universe refused to yield width/height from this RAW file.".to_string(),
            ));
        }

        warn!(
            "LibRaw pixel extraction not yet integrated. \
             Returning zero-filled placeholder data for {}. \
             This is a known limitation — see ROADMAP.md.",
            path.display()
        );

        // Placeholder: zero-filled Bayer plane
        let data = vec![0u16; (width * height) as usize];

        Ok(RawPixelData {
            width,
            height,
            data,
            bayer_pattern: meta.bayer_pattern,
        })
    }
}

fn parse_exif_datetime(s: &str) -> Option<DateTime<Utc>> {
    // EXIF format: "2023:07:15 21:34:00"
    NaiveDateTime::parse_from_str(s.trim_matches('"'), "%Y:%m:%d %H:%M:%S")
        .ok()
        .map(|ndt| Utc.from_utc_datetime(&ndt))
}

fn infer_bayer_pattern(make: Option<&str>, _model: Option<&str>) -> Option<String> {
    match make {
        Some(m) if m.to_uppercase().contains("CANON") => Some("RGGB".to_string()),
        Some(m) if m.to_uppercase().contains("NIKON") => Some("RGGB".to_string()),
        Some(m) if m.to_uppercase().contains("SONY") => Some("RGGB".to_string()),
        _ => None,
    }
}
