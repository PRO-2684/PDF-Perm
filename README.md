# PDF-Perm

Change the permissions of a PDF file.

## Usage

The usage is quite similar to `chmod` command:

```shell
pdf-perm +c no-copy.pdf
```

This will overwrite `no-copy.pdf` and make it copiable. Bugs may arise, so make sure to have a backup if it matters.

By specifying another path, it will write to there instead of overwriting the original file:

```shell
pdf-perm +c no-copy.pdf can-copy.pdf
```

To remove a permission, use `-`:

```shell
pdf-perm -c can-copy.pdf no-copy.pdf
```

To set exactly the permissions you want, use `=`:

```shell
pdf-perm =pma my.pdf
```

This will set the permissions to `PRINTABLE`, `MODIFIABLE`, and `ANNOTABLE` and remove all other permissions. For a complete list of permissions, see the table below.

If multiple permission modifiers are used, they will be combined in the order they are specified.

## Permissions

Here's a list of permissions this crate is capable of handling:

| # | Short Flag | Long Flag |
| - | - | - |
|  2 | `p` | [P]RINTABLE |
|  3 | `m` | [M]ODIFIABLE |
|  4 | `c` | [C]OPYABLE |
|  5 | `a` | [A]NNOTABLE |
|  8 | `f` | [F]ILLABLE |
|  9 | `x` | COPYABLE_FOR_ACCESSIBILITY |
| 10 | `s` | A[S]SEMBLABLE |
| 11 | `q` | PRINTABLE_IN_HIGH_[Q]UALITY |

See the [PDF 1.4 ref](https://opensource.adobe.com/dc-acrobat-sdk-docs/pdfstandards/pdfreference1.4.pdf), TABLE 3.15 for more details. Note that the index in the table is 1-based, while the index in the table above is 0-based.

## Caveats

- This crate does NOT handle **password protected** PDFs. Consider decrypting them first, or using [pdfrip](https://github.com/mufeedvh/pdfrip) to break the password.
- Currently, only PDF 1.4 is supported.

## Credits

- [abatsakidis/PDFDeSecure](https://github.com/abatsakidis/PDFDeSecure/tree/master/Example-PDF), for the example PDF file under `tests/` directory.

## TODO

- [ ] Implement `chmod` like syntax
    - [x] `-`, `+`, `=`
    - [ ] `-R` for recursive
    - [ ] `--reference` for reference file
- [ ] Extended syntax
    - [ ] `!` for negation
- [ ] Handle filenames starting with `-`, `+`, `=`
- [ ] Set to `None` if permissions are default
- [ ] Preserve `EncryptionVersion`
- [ ] Allow specifying `EncryptionVersion` if not present
