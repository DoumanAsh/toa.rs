# toa-find

GNU find replacement

## Usage

```
Kawaii Toa shall find all your files(porn) recursively.

USAGE:
    toa-find.exe [FLAGS] [OPTIONS] <PATTERN> [-- <PATH>...]

FLAGS:
    -d, --dir        Prints directories.
    -f, --file       Prints files.
    -h, --help       Prints help information
    -q, --quiet      Ignore errors during search.
    -s, --sym        Follow symbolic links. By default they are not followed.
    -V, --version    Prints version information

OPTIONS:
        --hop <hop>          Specifies depth of recursion.
        --minhop <minhop>    Minimum number of hops before starting to look.

ARGS:
    <PATTERN>    Regex pattern to filter by
    <PATH>...    Folders on which to perform find
```
