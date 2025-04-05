use anyhow::{Context, Result, bail};
use env_logger::Env;
use log::{debug, info};
use lopdf::{Document, Permissions};
use pdf_perm::{PdfPerm, PermissionExt};
use std::io::Write;

/// Modification of permissions
enum PermissionMod {
    /// Set to exactly the given permissions
    Exact(Permissions),
    /// Insert the given permissions
    Insert(Permissions),
    /// Remove the given permissions
    Remove(Permissions),
}

fn main() -> Result<()> {
    // Setup the logger
    setup_logger();

    // Parsing arguments
    let mut mods = Vec::new();
    let mut paths = (None, None);
    for arg in std::env::args().skip(1) {
        match arg.chars().next().expect("Argument is empty") {
            '+' => {
                let permission = Permissions::from_str(&arg[1..]);
                mods.push(PermissionMod::Insert(permission));
            }
            '-' => {
                // Check if the rest of the argument consists entirely of lowercase letters
                let is_lowercase = arg[1..].chars().all(|c| c.is_ascii_lowercase());
                if is_lowercase {
                    let permission = Permissions::from_str(&arg[1..]);
                    mods.push(PermissionMod::Remove(permission));
                } else {
                    todo!("Handle other arguments");
                }
            }
            '=' => {
                let permission = Permissions::from_str(&arg[1..]);
                mods.push(PermissionMod::Exact(permission));
            }
            _ => {
                if paths.0.is_none() {
                    paths.0.replace(arg);
                } else if paths.1.is_none() {
                    paths.1.replace(arg);
                } else {
                    bail!("Too many paths provided");
                }
            }
        }
    }

    // Unwrapping paths
    let Some(input_path) = &paths.0 else {
        bail!("No input path provided");
    };
    let output_path = paths.1.as_ref().unwrap_or(&input_path);

    // Open a PDF file
    let mut doc = Document::load(&input_path)?;
    debug!("Encryption state: {:?}", doc.encryption_state);

    // Read permissions
    info!("Reading original permissions");
    let mut allowed = doc.permissions().unwrap_or_else(|| {
        debug!("No permissions found, using default");
        Permissions::default()
    });
    let disallowed = Permissions::from_bits_truncate(!allowed.bits());

    info!("Allowed Permissions: {allowed:?}");
    info!("Disallowed Permissions: {disallowed:?}");

    // Early exit if no modifications are specified
    if mods.is_empty() {
        info!("No modifications specified, exiting");
        return Ok(());
    }

    // Modify permissions
    for permission_mod in mods {
        match permission_mod {
            PermissionMod::Exact(exact) => {
                allowed = exact;
            }
            PermissionMod::Insert(permissions) => {
                allowed.insert(permissions);
            }
            PermissionMod::Remove(permissions) => {
                allowed.remove(permissions);
            }
        }
    }
    info!("Modified Permissions: {allowed:?}");
    doc.set_permissions(allowed)
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
