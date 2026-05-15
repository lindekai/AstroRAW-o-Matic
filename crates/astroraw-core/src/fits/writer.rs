use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

use astroraw_models::{FitsHeader, FitsValue};
use crate::error::{AstroError, Result};
use crate::raw::reader::RawPixelData;

/// FITS block size: 2880 bytes, 80 chars per record, 36 records per block.
const FITS_BLOCK_SIZE: usize = 2880;
const FITS_RECORD_SIZE: usize = 80;
const FITS_RECORDS_PER_BLOCK: usize = 36;

pub struct FitsWriter;

impl FitsWriter {
    pub fn new() -> Self {
        Self
    }

    pub fn write(&self, path: &Path, header: &FitsHeader, pixels: &RawPixelData) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(AstroError::Io)?;
        }

        let file = File::create(path).map_err(AstroError::Io)?;
        let mut writer = BufWriter::new(file);

        self.write_header(&mut writer, header)?;
        self.write_data(&mut writer, pixels)?;

        writer.flush().map_err(AstroError::Io)?;
        Ok(())
    }

    fn write_header<W: Write>(&self, writer: &mut W, header: &FitsHeader) -> Result<()> {
        let mut records: Vec<[u8; FITS_RECORD_SIZE]> = Vec::new();

        for rec in &header.records {
            records.push(self.format_record(rec)?);
        }

        // END card
        let mut end_card = [b' '; FITS_RECORD_SIZE];
        end_card[..3].copy_from_slice(b"END");
        records.push(end_card);

        // Pad to block boundary
        while records.len() % FITS_RECORDS_PER_BLOCK != 0 {
            records.push([b' '; FITS_RECORD_SIZE]);
        }

        for record in &records {
            writer.write_all(record).map_err(AstroError::Io)?;
        }

        Ok(())
    }

    fn format_record(&self, rec: &astroraw_models::FitsHeaderRecord) -> Result<[u8; FITS_RECORD_SIZE]> {
        let mut card = [b' '; FITS_RECORD_SIZE];

        let kw = rec.keyword.as_bytes();
        let kw_len = kw.len().min(8);
        card[..kw_len].copy_from_slice(&kw[..kw_len]);

        // COMMENT / HISTORY / BLANK — value fills rest of card
        if matches!(rec.keyword.as_str(), "COMMENT" | "HISTORY" | "CONTINUE") {
            if let FitsValue::Str(ref s) = rec.value {
                let s_bytes = s.as_bytes();
                let copy_len = s_bytes.len().min(FITS_RECORD_SIZE - 8);
                card[8..8 + copy_len].copy_from_slice(&s_bytes[..copy_len]);
            }
            return Ok(card);
        }

        // Standard value indicator
        card[8] = b'=';
        card[9] = b' ';

        let value_str = match &rec.value {
            FitsValue::Bool(b) => format!("{:>20}", if *b { "T" } else { "F" }),
            FitsValue::Int(i) => format!("{:>20}", i),
            FitsValue::Float(f) => format!("{:>20.6E}", f),
            FitsValue::Str(s) => {
                // FITS strings: left-justified, single-quoted, max 68 chars in value field
                let trimmed = if s.len() > 18 { &s[..18] } else { s.as_str() };
                format!("'{:<8}'", trimmed)
            }
        };

        let val_bytes = value_str.as_bytes();
        let val_len = val_bytes.len().min(20);
        card[10..10 + val_len].copy_from_slice(&val_bytes[..val_len]);

        // Comment
        if let Some(ref comment) = rec.comment {
            let comment_start = 32;
            card[comment_start - 2] = b' ';
            card[comment_start - 1] = b'/';
            let c_bytes = comment.as_bytes();
            let c_len = c_bytes.len().min(FITS_RECORD_SIZE - comment_start);
            card[comment_start..comment_start + c_len].copy_from_slice(&c_bytes[..c_len]);
        }

        Ok(card)
    }

    fn write_data<W: Write>(&self, writer: &mut W, pixels: &RawPixelData) -> Result<()> {
        let total_pixels = (pixels.width * pixels.height) as usize;
        let mut bytes_written = 0usize;

        // FITS stores big-endian 16-bit integers, with BZERO=32768 for unsigned mapping.
        // For now we write raw u16 as signed i16 big-endian (BITPIX=16 means signed).
        for &px in &pixels.data[..total_pixels.min(pixels.data.len())] {
            let signed = px as i16;
            let be = signed.to_be_bytes();
            writer.write_all(&be).map_err(AstroError::Io)?;
            bytes_written += 2;
        }

        // Pad data block to 2880-byte boundary
        let remainder = bytes_written % FITS_BLOCK_SIZE;
        if remainder != 0 {
            let padding = FITS_BLOCK_SIZE - remainder;
            let zeros = vec![0u8; padding];
            writer.write_all(&zeros).map_err(AstroError::Io)?;
        }

        Ok(())
    }
}
