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

| Short Flag | Long Flag | Description |
| - | - | - |
| `c` | `copy` | Allows copying |

## Removing Password?

Consider using [pdfrip](https://github.com/mufeedvh/pdfrip) instead.

## TODO

- [ ] Implement `chmod` like syntax
    - [ ] `-`, `+`, `=`
    - [ ] `-R` for recursive
    - [ ] `--reference` for reference file
- [ ] Full PDF permission support
- [ ] Preserve `EncryptionVersion`
- [ ] Allow specifying `EncryptionVersion` if not present
