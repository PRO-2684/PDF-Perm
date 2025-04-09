use lopdf::Document;
use pdf_perm::{PdfPerm, ShortFlags};
use std::path::Path;

fn check_permission(path: &Path) {
    let file_stem = path.file_stem().unwrap().to_str().unwrap();
    let (expected_summary, name) = file_stem.split_once('.').unwrap();

    let doc = Document::load(path).unwrap();
    let permissions = doc.permissions();
    let summary = permissions.summary();

    assert_eq!(expected_summary, summary);
    println!("{name}: {summary}");
}

#[test]
fn check_permissions() {
    let test_dir = Path::new("samples");
    let entries = std::fs::read_dir(test_dir).unwrap();
    // Check for all files
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            check_permission(&path);
        }
    }
}
