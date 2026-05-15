use astroraw_core::BatchSummary;

pub fn section_header(title: &str) {
    eprintln!();
    eprintln!("  {}", title);
    eprintln!("  {}", "─".repeat(title.len()));
}

pub fn batch_summary(summary: &BatchSummary) {
    eprintln!("  Results");
    eprintln!("  ───────────────────────────────");
    eprintln!("  Total:     {}", summary.total);
    eprintln!("  Succeeded: {}", summary.succeeded);
    eprintln!("  Failed:    {}", summary.failed);
    if summary.skipped > 0 {
        eprintln!("  Skipped:   {}", summary.skipped);
    }
    eprintln!();

    if summary.failed == 0 {
        eprintln!("  Conversion complete. Don't panic.");
    } else {
        eprintln!(
            "  Conversion failed, but at least it failed deterministically. \
             See errors above."
        );
    }
}
