# zed-fortran

Fortran language support for the [Zed editor](https://zed.dev) via
[fortls](https://fortls.fortran-lang.org), a language server for Fortran.

## Installation

The [`fortls`](https://fortls.fortran-lang.org) language server must be
installed on your system and available in your `$PATH`.


### Installing the Extension

This extension can be installed directly from the Zed extensions marketplace.

## Configuration

If the `fortls` binary is included in your `$PATH` and you are satisfied with
the default configuration, no further action is necessary. If not, you can
configure `fortls` under `lsp.fortls.binary` in the Zed settings by providing
additional arguments as needed.

For a comprehensive list of available configuration options using command line
arguments, see the [fortls documentation](https://fortls.fortran-lang.org/options.html#configuration-using-the-command-line).

### Example Configuration

```json
{
  "lsp": {
    "fortls": {
      "binary": {
        "path": "/opt/homebrew/bin/fortls",
        "arguments": ["--lowercase_intrinsics", "--debug_log"]
      }
    }
  }
}
```
