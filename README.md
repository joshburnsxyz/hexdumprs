# Hexdump

Basic hexdump utility implemented in Rust. This was built as a learning exercise.

# Usage

The utlity takes a file using the `--file` or `-f` flag. Optionally you can specify if you want to only print the hex values (`--hex` or `-H`) or ascii values (`ascii` or `-a`);

```
Usage:
        hexdumprs [-f FILE] [OPTIONS]

Flags:
        -f, --file <string> : Specify target file
        -a, --ascii         : Only print the ascii values
        -H, --hex           : Only print the hex values
        -h, --help          : Show help
```
