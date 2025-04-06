use anyhow::{Context, Result, bail};
use bitflags::Flags;
use env_logger::Env;
use log::{debug, info};
use lopdf::{Document, Permissions};
use pdf_perm::{PdfPerm, ShortFlags};
use std::io::Write;

fn main() -> Result<()> {
    // Setup the logger
    setup_logger();

    // Collect command line arguments
    let mut iter = std::env::args();
    let program_name = iter.next().unwrap_or_else(|| "pdf-perm".to_string());
    let args: Vec<_> = iter.collect();
    let mut perm_mod = None;

    // Interpret arguments
    let (input_path, output_path) = match args.len() {
        0 => {
            println!("Usage: {program_name} [PERMISSION] <INPUT> [OUTPUT]\n");
            println!("Supported permissions: {}", Permissions::all().summary());
            display(Permissions::all());
            println!("\nYou can use * to represent all permissions.");
            return Ok(());
        }
        1 => (&args[0], &args[0]), // <INPUT>
        2 => {
            // [PERMISSION] <INPUT>
            perm_mod.replace(&args[0]);
            (&args[1], &args[1])
        }
        3 => {
            // [PERMISSION] <INPUT> [OUTPUT]
            perm_mod.replace(&args[0]);
            (&args[1], &args[2])
        }
        _ => bail!("Too many arguments"),
    };

    // Open the PDF document
    let mut doc = Document::load(&input_path)?;
    debug!("Encryption state: {:?}", doc.encryption_state);

    // Read permissions
    info!("Reading original permissions");
    let mut perm = doc.permissions();
    info!("Original permissions: {}", perm.summary());

    // Early exit if no modifications are specified
    let Some(perm_mod) = perm_mod else {
        info!("No modifications specified, exiting");
        return Ok(());
    };

    // Modify permissions
    perm.apply_modification(perm_mod);
    info!("Modified permissions: {}", perm.summary());

    doc.set_permissions(perm)
        .with_context(|| format!("Failed to set permissions for given document: {input_path}"))?;

    // Save the document
    info!("Saving document to {output_path}");
    doc.save(&output_path)
        .with_context(|| format!("Failed to save document to {output_path}"))?;

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

/// Display permissions in the format of a list.
fn display(permissions: Permissions) {
    for (short, flag) in Permissions::SHORT_FLAGS.iter().zip(Permissions::FLAGS) {
        let perm = flag.value();
        let name = flag.name();
        if permissions.contains(*perm) {
            println!("+ [{short}] {name}");
        } else {
            println!("- [{short}] {name}");
        }
    }
}
