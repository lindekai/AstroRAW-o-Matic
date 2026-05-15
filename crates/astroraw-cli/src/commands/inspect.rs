use astroraw_core::raw::reader::RawReader;
use crate::args::InspectArgs;

pub fn run(args: InspectArgs) -> i32 {
    match RawReader::read_metadata(&args.input) {
        Ok(meta) => {
            if args.json {
                match serde_json::to_string_pretty(&meta) {
                    Ok(json) => println!("{}", json),
                    Err(e) => {
                        eprintln!("  Error serializing metadata: {}", e);
                        return 1;
                    }
                }
            } else {
                println!("File:        {}", args.input.display());
                println!("Camera:      {} {}",
                    meta.camera_make.as_deref().unwrap_or("Unknown"),
                    meta.camera_model.as_deref().unwrap_or("Unknown"),
                );
                println!("Exposure:    {}", meta.exposure_time.map(|t| format!("{:.4}s", t)).unwrap_or_else(|| "Unknown".to_string()));
                println!("ISO:         {}", meta.iso_speed.map(|i| i.to_string()).unwrap_or_else(|| "Unknown".to_string()));
                println!("Date:        {}", meta.date_obs.map(|d| d.to_string()).unwrap_or_else(|| "Unknown".to_string()));
                println!("Dimensions:  {}x{}", meta.width.unwrap_or(0), meta.height.unwrap_or(0));
                println!("Bit depth:   {}", meta.bit_depth.map(|b| format!("{}-bit", b)).unwrap_or_else(|| "Unknown".to_string()));
                println!("Bayer:       {}", meta.bayer_pattern.as_deref().unwrap_or("Unknown"));
                println!("Focal len:   {}", meta.focal_length.map(|f| format!("{:.1}mm", f)).unwrap_or_else(|| "Unknown".to_string()));
                println!("Aperture:    {}", meta.aperture.map(|a| format!("f/{:.1}", a)).unwrap_or_else(|| "Unknown".to_string()));
            }
            0
        }
        Err(e) => {
            eprintln!("  Could not read RAW metadata. {}", e);
            1
        }
    }
}
