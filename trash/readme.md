## 🗑 Trash

A Rust based CLI trash utility implementing [The FreeDesktop.org Trash specification](https://specifications.freedesktop.org/trash-spec/trashspec-1.0.html). Verbs based on [trash-cli](https://github.com/andreafrancia/trash-cli). It replaces the spartan rm alias with something more robust.

# TODO UPDATE WITH GITHUB ACTIONS WHEN PUBLIC

## Installation

### Option 1: Download the binary in this repo at binary/trash

### Option 2: Compile with cargo

```sh
$ git clone https://github.com/timothycohen/*****PLACEHOLDER*****
$ cd *****PLACEHOLDER*****
$ cargo build --release
$ cp target/release/trash /path/to/your_bin_folder/in_PATH # example ~/.local/my_bin
```

### Alias

I personally alias over rm (no silly mistakes :)) and the trash command from my_bin

```sh
# in .zsh_aliases or wherever you keep your aliases
rm='~/.local/my_bin/trash put'
rmc='~/.local/my_bin/trash'
trash='mkdir -p ~/.local/share/Trash/files; cd ~/.local/share/Trash/files;'
```

## Examples

### `trash put file` <br/> Moves file to the trash

```sh
$ ls
bar.txt foo.txt
$ trash put bar.txt
$ trash put foo.txt -v
Info: Canonicalizing file paths.
Info: Writing info file to ~/.local/share/Trash/info/foo.txt.e6ba2602-6886-4ee3-851a-a27b3a64c135.trashinfo
Info: Moving trashed file to ~/.local/share/Trash/files/foo.txt.e6ba2602-6886-4ee3-851a-a27b3a64c135
$ ls
$ _
```

### `trash empty` <br/> Empties the trash directory

```sh
$ trash empty
Warn: Permanently delete all 103 files at ~/.local/share/Trash/files? [y/n] y
$ ls ~/.local/share/Trash/files
$ _
```

### `trash restore file` <br/> Restores a file

```sh
$ ls -lhAF
-rw-r--r--   1 tco  staff    13K Jun 11 21:01 bar.txt
-rw-r--r--   1 tco  staff    0B  Jun 11 21:01 foo.txt
$ trash put foo.txt
$ trash put bar.txt
$ touch bar.txt
$ trash restore foo.txt
$ trash restore bar.txt
Err: Will not overwrite file: "~/bar.txt"
$ trash restore bar.txt -f # force
$ ls -lhAF
-rw-r--r--   1 tco  staff    13K Jun 11 21:01 bar.txt # trash copy has overwritten local copy
-rw-r--r--   1 tco  staff    0B  Jun 11 21:01 foo.txt
```

### `trash info file` <br/> Show trash info from any folder

```sh
$ pwd
~/.local/share/Trash/files
$ ls
foo.txt.e6ba2602-6886-4ee3-851a-a27b3a64c135
$ trash info foo.txt.e6ba2602-6886-4ee3-851a-a27b3a64c135.trashinfo

[Trash Info]
FileName=foo.txt.e6ba2602-6886-4ee3-851a-a27b3a64c135
Path=/private/tmp/testing/example/foo.txt
DeletionDate=2022-06-12T01:01:09.012176+00:00
FileSize=0 B
```

### `trash info` <br/> Show all trash info

```sh
$ pwd
~/.local/share/Trash/files
$ ls
foo.txt.e6ba2602-6886-4ee3-851a-a27b3a64c135
bar.txt.4cb89234-a921-428a-97b0-2edfc0326422
$ trash info

[Trash Info]
FileName=foo.txt.e6ba2602-6886-4ee3-851a-a27b3a64c135
Path=/private/tmp/testing/example/foo.txt
DeletionDate=2022-06-12T01:01:09.012176+00:00
FileSize=0 B

[Trash Info]
FileName=bar.txt.4cb89234-a921-428a-97b0-2edfc0326422
Path=/private/tmp/testing/example/bar.txt
DeletionDate=2022-06-12T01:01:10.803454+00:00
FileSize=13.11 KB
```

### Handles files/directories, single/multiple, absolute/relative paths

```sh
$ pwd
~/dev/testing_trash
$ tree
.
├── nest
│   ├── really_nested
│   │   └── deep.txt
│   └── foz.txt
└── abc.txt
$ trash put abc.txt ~/dev/testing_trash/nest/really_nested nest/foz.txt
$ tree
.
└── nest
$ trash restore ~/dev/testing_trash/abc.txt ./nest/really_nested nest/foz.txt
$ tree
├── nest
│   ├── really_nested
│   │   └── deep.txt
│   └── foz.txt
└── abc.txt
```

### `trash --help`

```sh
trash 0.1.0
Tim Cohen <contact@timcohen.dev>
A cli trash utility.

USAGE:
    trash [OPTIONS] <METHOD> [FILE]

ARGS:
    <METHOD>    `e` | `empty`   `i` | 'info'   `p` | `put`   `r` | `restore`
    <FILE>      The target file or directory

OPTIONS:
    -f, --force      Force non-recoverable deletes/overwrites
    -h, --help       Print help information
    -v, --verbose    Explain all steps
    -V, --version    Print version information
```

## Future plans

### Add selection for restoring conflicting paths

```sh
$ ls
foo.txt
$ trash put ./foo.txt
$ touch foo.txt
$ trash put ./foo.txt
$ ls ~/.local/share/Trash/files
foo.txt.419fe43c-5273-4f4c-a648-b13256f330b8
foo.txt.c4037062-7d6b-47e9-a48b-fd9efb757d6c
$ trash restore foo.txt

──────────────┼───────────────────────────────────────────────────────────────
Option 1:     | foo.txt.e6ba2602-6886-4ee3-851a-a27b3a64c135.trashinfo
Path:         | private/tmp/testing/example/foo.txt
Deletion Date | 2022-06-12T01:01:09.012176+00:00
File Size     | 0 B
──────────────┼───────────────────────────────────────────────────────────────
──────────────┼───────────────────────────────────────────────────────────────
Option 2:     | foo.txt.c4037062-7d6b-47e9-a48b-fd9efb757d6c
Path:         | private/tmp/testing/example/foo.txt
Deletion Date | 2022-06-11T02:31:35.722779+00:00
File Size     | 6.42 KB
──────────────┼───────────────────────────────────────────────────────────────

Which would you like to restore? (See at ~/.local/share/Trash/files)
Option: _

```

### Restore from Trash files folder directly

```sh
$ pwd
~/.local/share/Trash/files
$ ls
foz.txt.c16b01c0-116b-4fc5-ab92-8d1a15953ff7
$ trash restore foz.txt.c16b01c0-116b-4fc5-ab92-8d1a15953ff7
$ cd /private/tmp/testing/example
$ ls
foz.txt
```

### Handle symlinks

```sh
tmp@ -> private/tmp

$ pwd
/tmp/testing
$ ls

$ trash restore foz.txt
$ ls
foz.txt
```

### Recursive Info

### Globular info/restore (put already supported by virtue of shell expansion)
