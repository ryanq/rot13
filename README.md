
# ROT-13

Transform text with ROT-13 to keep your spoilers from ruining someone's day.


## Usage/Examples

```bash
$ # Encode a short text by passing it as input:
$ rot13 "This is a spoiler"
Guvf vf n fcbvyre

$ # Encode output of another command by passing no input and piping:
$ print_a_spoiler | rot13
Ab, Bov-Jna xvyyrq lbhe sngure.

$ # Encode the contents of the pasteboard (on macOS):
$ pbpaste | rot13 | pbcopy
```


## Installation

Install `rot13` with `Cargo`:

```bash
  cargo install --git https://github.com/ryanq/rot13
```
    
## License

This project is licensed under the [MIT license](https://choosealicense.com/licenses/mit/).

