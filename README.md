# Usage

```shell
prepend_line ~/path/to/some/file.txt "Some New Head Text" "pattern to match"
```

Also accepts piped arguments:

```shell
find <path> -name "<pattern>" -type f | xargs -I{} -P4 -n1 prepend_line {} "Some New Head Text" "pattern to match"
```

The `-n1` is important, to ensure only one file name gets passed to
`prepend_line` at once.

# Example

Say you've got a bunch of JS files and you want to see if there's an annotation
at the top, like `// @flow`. If there is NOT, you want to add a comment to the
file. You can do that like so:

```shell
find src -name "*.js" -type f |\
  xargs -I{} -P4 -n1 \
  prepend_file {} "// If you edit this ensure you add the '// @flow' annotation to the top" "@flow"
```
