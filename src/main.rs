use anyhow::{Context, Result, bail};
use bitflags::Flags;
use env_logger::Env;
use log::{debug, info};
use lopdf::{Document, Permissions};
use pdf_perm::{PdfPerm, ShortFlags};
use std::{io::Write, path::Path};

fn main() -> Result<()> {
    // Setup the logger
    setup_logger();

    // Collect command line arguments
    let mut iter = std::env::args();
    let program_path = iter.next().unwrap_or_else(|| "pdf-perm".to_string());
    let args: Vec<_> = iter.collect();
    let mut perm_mod = None;

    // Check if we're running in "DeSec" mode
    if is_desec(&program_path) {
        let input_path = match args.into_iter().next() {
            Some(path) => path,
            None => {
                println!("DeSec mode activated!");
                println!("Usage: {program_path} <INPUT>");
                prompt_input_path()?
            },
        };

        return set_permissions(&input_path, &input_path, Some("=*"));
    }

    // Interpret arguments
    let (input_path, output_path) = match args.len() {
        0 => {
            println!("Usage: {program_path} [PERMISSION] <INPUT> [OUTPUT]\n");
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

    set_permissions(input_path, output_path, perm_mod.map(|x| x.as_str()))
}

/// Set permissions.
fn set_permissions(
    input_path: &str,
    output_path: &str,
    perm_mod: Option<&str>,
) -> Result<()> {
    // Open the PDF document
    info!("Reading document: {input_path}");
    let mut doc = Document::load(input_path)?;
    debug!("Encryption state: {:?}", doc.encryption_state);

    // Read permissions
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
    doc.save(output_path)
        .with_context(|| format!("Failed to save document: {output_path}"))?;

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

/// Determine whether we're running in "DeSec" mode.
fn is_desec(program_path: &str) -> bool {
    // Get program name from path
    let path = Path::new(program_path);
    let file_name = path.file_stem().unwrap_or_default();
    let program_name = file_name.to_str().unwrap_or_default().to_lowercase();

    // (pdf-)?desec(ure)?
    // Strip leading `pdf-` if present
    let program_name = program_name.strip_prefix("pdf-").unwrap_or(&program_name);
    // Strip trailing `ure` if present
    let program_name = program_name.strip_suffix("ure").unwrap_or(&program_name);

    // Check if the program name equals "desec"
    program_name == "desec"
}

/// Prompt for input path.
fn prompt_input_path() -> Result<String> {
    println!("Please enter the input file path:");
    let mut input_path = String::new();
    std::io::stdin()
        .read_line(&mut input_path)
        .with_context(|| format!("Failed to read from stdin"))?;
    Ok(input_path)
}
