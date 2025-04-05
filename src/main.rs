use anyhow::{Context, Result, bail};
use env_logger::Env;
use log::{debug, info};
use lopdf::{Document, Permissions};
use pdf_perm::PdfPerm;
use std::io::Write;

fn main() -> Result<()> {
    // Setup the logger
    setup_logger();

    let path = std::env::args()
        .nth(1)
        .unwrap_or("tests/no-copy.pdf".to_string());

    // Open a PDF file
    let mut doc = Document::load(&path)?;
    if doc.is_encrypted() {
        bail!("Encrypted PDFs are not supported");
    }

    // Read the encryption state
    debug!("Encryption State: {:?}", doc.encryption_state);

    // Read permissions
    info!("Reading original permissions");
    let allowed = doc.permissions().unwrap_or_else(|| {
        debug!("No permissions found, using default");
        Permissions::default()
    });
    let disallowed = Permissions::from_bits_truncate(!allowed.bits());

    info!("Allowed Permissions: {allowed:?}");
    info!("Disallowed Permissions: {disallowed:?}");

    // Allow all permissions (Permissions::all())
    info!("Setting permissions to all");
    doc.set_permissions(Permissions::all())
        .with_context(|| format!("Failed to set permissions for given document: {path}"))?;

    // Save the document if an output path is provided
    if let Some(output_path) = std::env::args().nth(2) {
        info!("Saving document to {output_path}");
        doc.save(&output_path)
            .with_context(|| format!("Failed to save document to {output_path}"))?;
    } else {
        info!("No output path provided, not saving");
    }

    Ok(())
}

/// Setup the logger.
fn setup_logger() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format(|buf, record| {
            let level = record.level();
            let style = buf.default_level_style(level);
            writeln!(buf, "[{style}{level}{style:#}] {}", record.args())
        })
        .init();
}
