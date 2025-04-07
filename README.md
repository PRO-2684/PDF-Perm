# PDF-Perm

[![GitHub License](https://img.shields.io/github/license/PRO-2684/PDF-Perm?logo=opensourceinitiative)](https://github.com/PRO-2684/PDF-Perm/blob/main/LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/PRO-2684/PDF-Perm/release.yml?logo=githubactions)](https://github.com/PRO-2684/PDF-Perm/blob/main/.github/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/PRO-2684/PDF-Perm?logo=githubactions)](https://github.com/PRO-2684/PDF-Perm/releases)
[![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/PRO-2684/PDF-Perm/total?logo=github)](https://github.com/PRO-2684/PDF-Perm/releases)
[![Crates.io Version](https://img.shields.io/crates/v/pdf-perm?logo=rust)](https://crates.io/crates/pdf-perm)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/pdf-perm?logo=rust)](https://crates.io/crates/pdf-perm)
[![docs.rs](https://img.shields.io/docsrs/pdf-perm?logo=rust)](https://docs.rs/pdf-perm)

Change the permissions of a PDF file.

## Installation

### Using `binstall`

```shell
cargo binstall pdf-perm
```

### Downloading from Releases

Navigate to the [Releases page](https://github.com/PRO-2684/PDF-Perm/releases) and download respective binary for your platform. Make sure to give it execute permissions.

### Compiling from Source

```shell
cargo install pdf-perm
```

## Examples

The usage is quite similar to `chmod` command:

```shell
pdf-perm +c no-copy.pdf
```

This will modify `no-copy.pdf` and make it copiable. Bugs may arise, so make sure to backup if it matters. Alternatively, by specifying another path, this tool will write to there instead of overwriting the original file:

```shell
pdf-perm +c no-copy.pdf can-copy.pdf
```

To reject a permission, use `-`:

```shell
pdf-perm -c can-copy.pdf no-copy.pdf
```

To set exactly the permissions you want, use `=`:

```shell
pdf-perm =pma my.pdf
```

This will set the permissions to `PRINTABLE`, `MODIFIABLE`, and `ANNOTABLE` and reject all other permissions.

To grant all permissions, use `+*` or `=*`; to reject all permissions, use `-*`:

```shell
pdf-perm +* confidential.pdf declassified.pdf
```

## Usage

```shell
pdf-perm [PERMISSION] <INPUT> [OUTPUT]
```

| Argument Length | Interpretation                  |
| --------------- | ------------------------------- |
| 0               | Print help and permissions      |
| 1               | `<INPUT>`                       |
| 2               | `[PERMISSION] <INPUT>`          |
| 3               | `[PERMISSION] <INPUT> [OUTPUT]` |
| 4+              | Invalid                         |

### Permission

The permission argument is a string that specify the permissions to be set on the PDF file. It must starts with one of the following:

- `+`: to grant permissions
- `-`: to reject permissions
- `=`: to set permissions exactly

Then, you can specify the short flags for the permissions you want to grant, reject, or set exactly. Valid short flags and their [corresponding constant](https://docs.rs/lopdf/0.36.0/lopdf/encryption/struct.Permissions.html#impl-Permissions) in [`lopdf` crate](https://docs.rs/lopdf/0.36.0/lopdf/) are:

| #   | Short Flag | Constant                      |
| --- | ---------- | ----------------------------- |
| 3   | `p`        | **P**RINTABLE                 |
| 4   | `m`        | **M**ODIFIABLE                |
| 5   | `c`        | **C**OPYABLE                  |
| 6   | `a`        | **A**NNOTABLE                 |
| 9   | `f`        | **F**ILLABLE                  |
| 10  | `x`        | COPYABLE_FOR_ACCESSIBILITY    |
| 11  | `s`        | A**S**SEMBLABLE               |
| 12  | `q`        | PRINTABLE_IN_HIGH_**Q**UALITY |
| /   | `*`        | All permissions               |

> [!NOTE]
>
> - The index in the tables are $1$-based.
> - The actual permissions you see on PDF readers may differ from the ones you see using this tool. This is because some permissions imply others.
> - See the [PDF 1.4 ref](https://opensource.adobe.com/dc-acrobat-sdk-docs/pdfstandards/pdfreference1.4.pdf), TABLE 3.15 for more details.

If this argument is not specified, `pdf-perm` will print the permissions of the input file and exit.

### Input

**Required**. Path to the input PDF file. This file will be modified in place unless an output path is specified.

### Output

Path to the output PDF file. If not specified, the input file will be modified in place.

## The DeSec Mode

When the last part of the `argv[0]` matches `(pdf-)?desec(ure)?` (case-insensitive, without extension), the program will run in "DeSec" mode. In this mode, the program will only accept one argument, which is the input PDF file. The program will then grant all permissions to the PDF file, effectively removing all restrictions.

Typically, you can rename the binary or create a symlink as `pdf-desec` or `desec` to enable this mode. In this way, simply dragging and dropping a PDF file onto the binary will remove all restrictions from it.

## Caveats

- This crate does NOT handle **password protected** PDFs. Consider decrypting them first, or using [pdfrip](https://github.com/mufeedvh/pdfrip) to break the password.
- Currently, only PDF 1.4 is supported.

## Credits

- [abatsakidis/PDFDeSecure](https://github.com/abatsakidis/PDFDeSecure/tree/master/Example-PDF), for inspiration and the example PDF file under `tests/` directory.
- [J-F-Liu/lopdf](https://github.com/J-F-Liu/lopdf), for the `lopdf` crate, which is used to read and write PDF files.

## TODO

- [ ] Set to `None` if permissions are default
- [ ] DeSec mode
- [ ] Preserve `EncryptionVersion`
- [ ] Allow specifying `EncryptionVersion` if not present
- [x] Implement `chmod` like syntax (`-`, `+`, `=`)
- [x] Extended syntax
    - [x] `*` for all permissions
