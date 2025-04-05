use env_logger::Env;
use log::{debug, error, info};
use lopdf::{Document, Permissions, Result as PdfResult};
use pdf_perm::PdfPerm;
use std::io::Write;

fn main() -> PdfResult<()> {
    setup_logger();
    let path = std::env::args()
        .nth(1)
        .unwrap_or("tests/no-copy.pdf".to_string());
    // Open a PDF file
    let mut doc = Document::load(path)?;
    if doc.is_encrypted() {
        error!("Does not support encrypted PDFs");
        unimplemented!("Does not support encrypted PDFs");
    }

    // Read the encryption state
    debug!("Encryption State: {:?}", doc.encryption_state);

    // Read permissions
    let allowed = doc.permissions().unwrap_or_else(|| {
        info!("No permissions found, using default");
        Permissions::default()
    });
    let disallowed = Permissions::from_bits_truncate(!allowed.bits());

    info!("Allowed Permissions: {allowed:?}");
    info!("Disallowed Permissions: {disallowed:?}");

    // Allow all permissions (Permissions::all())
    doc.set_permissions(Permissions::all())?;
    info!("Set permissions to all");

    // Save the document if an output path is provided
    if let Some(output_path) = std::env::args().nth(2) {
        info!("Saving document to {output_path}");
        doc.save(output_path)?;
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
