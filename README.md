# Usage

```shell
prepend_line ~/path/to/some/file.txt
```

Also accepts piped arguments:

```shell
find <path> -name "<pattern>" -type f -print0 | xargs -0 -P4 -n1 prepend_line
```

The `-n1` is important, to ensure only one file name gets passed to `prepend_line` at once.
