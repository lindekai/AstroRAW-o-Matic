use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use exif;
use tracing::debug;

use astroraw_models::RawMetadata;
use crate::error::{AstroError, Result};

/// Raw pixel data extracted from a CR2/RAW file — preserved as Bayer mosaic.
#[derive(Debug)]
pub struct RawPixelData {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u16>,
    pub bayer_pattern: Option<String>,
    pub black_level: u16,
    pub white_level: u16,
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
                    meta.camera_make = Some(clean_string(&field.display_value().to_string()));
                }
                exif::Tag::Model => {
                    meta.camera_model = Some(clean_string(&field.display_value().to_string()));
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
                exif::Tag::DateTimeOriginal | exif::Tag::DateTime => {
                    if meta.date_obs.is_none() {
                        // Try both display_value and raw ASCII value
                        let s = match &field.value {
                            exif::Value::Ascii(v) => v.first()
                                .and_then(|b| std::str::from_utf8(b).ok())
                                .unwrap_or("")
                                .to_string(),
                            _ => field.display_value().to_string(),
                        };
                        meta.date_obs = parse_exif_datetime(&s);
                    }
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

        // Canon DSLRs: 14-bit RAW in 16-bit container
        meta.bit_depth = Some(14);

        if meta.bayer_pattern.is_none() {
            meta.bayer_pattern = infer_bayer_pattern(meta.camera_make.as_deref());
        }

        Ok(meta)
    }

    /// Read raw Bayer pixel data from a CR2 file using rawler.
    pub fn read_raw_bayer(path: &Path) -> Result<RawPixelData> {
        if !path.exists() {
            return Err(AstroError::FileNotFound(path.display().to_string()));
        }

        let raw = rawler::decode_file(path)
            .map_err(|e| AstroError::RawReadError(e.to_string()))?;

        let cpp = raw.cpp; // components per pixel (1 = Bayer mosaic)
        if cpp != 1 {
            return Err(AstroError::RawReadError(format!(
                "Expected single-channel Bayer data (cpp=1), got cpp={}. \
                 Debayered input is not supported.",
                cpp
            )));
        }

        let full_width = raw.width;
        let full_height = raw.height;

        let all_pixels: Vec<u16> = match raw.data {
            rawler::RawImageData::Integer(pixels) => pixels,
            rawler::RawImageData::Float(_) => {
                return Err(AstroError::RawReadError(
                    "Float RAW data is not supported. Expected 16-bit integer Bayer data.".to_string(),
                ));
            }
        };

        // Crop to crop_area (removes optical black borders).
        // Fall back to active_area, then full sensor.
        let (width, height, data) = if let Some(crop) = raw.crop_area {
            let x = crop.p.x;
            let y = crop.p.y;
            let w = crop.d.w;
            let h = crop.d.h;
            let mut cropped = Vec::with_capacity(w * h);
            for row in y..y + h {
                let start = row * full_width + x;
                cropped.extend_from_slice(&all_pixels[start..start + w]);
            }
            (w as u32, h as u32, cropped)
        } else if let Some(active) = raw.active_area {
            let x = active.p.x;
            let y = active.p.y;
            let w = active.d.w;
            let h = active.d.h;
            let mut cropped = Vec::with_capacity(w * h);
            for row in y..y + h {
                let start = row * full_width + x;
                cropped.extend_from_slice(&all_pixels[start..start + w]);
            }
            (w as u32, h as u32, cropped)
        } else {
            (full_width as u32, full_height as u32, all_pixels)
        };

        let bayer_pattern = raw.camera.cfa.name.clone();
        let black_level = raw.blacklevel.levels.first()
            .map(|r| r.as_f32() as u16)
            .unwrap_or(0);
        let white_level = raw.whitelevel.0.first().copied()
            .map(|v| v as u16)
            .unwrap_or(u16::MAX);

        Ok(RawPixelData {
            width,
            height,
            data,
            bayer_pattern: Some(bayer_pattern),
            black_level,
            white_level,
        })
    }
}

fn clean_string(s: &str) -> String {
    s.trim_matches('"').trim().to_string()
}

fn parse_exif_datetime(s: &str) -> Option<DateTime<Utc>> {
    NaiveDateTime::parse_from_str(s.trim_matches('"'), "%Y:%m:%d %H:%M:%S")
        .ok()
        .map(|ndt| Utc.from_utc_datetime(&ndt))
}

fn infer_bayer_pattern(make: Option<&str>) -> Option<String> {
    match make {
        Some(m) if m.to_uppercase().contains("CANON") => Some("RGGB".to_string()),
        Some(m) if m.to_uppercase().contains("NIKON") => Some("RGGB".to_string()),
        Some(m) if m.to_uppercase().contains("SONY") => Some("RGGB".to_string()),
        _ => None,
    }
}
