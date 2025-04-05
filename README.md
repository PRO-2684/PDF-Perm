# PDF-Perm

## Usage

The usage is quite simple for those familimar with `chmod` command:

```shell
pdf-perm +c no-copy.pdf
```

This will overwrite the original file, so make sure to have a backup if it's important.

By specifying another path, it will write to there instead of overwriting the original file:

```shell
pdf-perm +c no-copy.pdf copy.pdf
```

## Permissions

Here's a list of permissions this crate is capable of handling:

| Short Flag | Long Flag |
| - | - |
| `p` | [P]RINTABLE |
| `m` | [M]ODIFIABLE |
| `c` | [C]OPYABLE |
| `a` | [A]NNOTABLE |
| `f` | [F]ILLABLE |
| `x` | COPYABLE_FOR_ACCESSIBILITY |
| `s` | A[S]SEMBLABLE |
| `q` | PRINTABLE_IN_HIGH_[Q]UALITY |

## Removing Password?

Consider using [pdfrip](https://github.com/mufeedvh/pdfrip) instead.

## TODO

- [ ] Implement `chmod` like syntax
    - [ ] `-`, `+`, `=`
    - [ ] `-R` for recursive
    - [ ] `--reference` for reference file
- [ ] Handle filenames starting with `-`, `+`, `=`
- [ ] Set to `None` if permissions are default
- [ ] Preserve `EncryptionVersion`
- [ ] Allow specifying `EncryptionVersion` if not present
