### 🗑 Trash

A Rust based CLI trash utility implementing [The FreeDesktop.org Trash specification](https://specifications.freedesktop.org/trash-spec/trashspec-1.0.html). Verbs based on [trash-cli](https://github.com/andreafrancia/trash-cli). It replaces the spartan rm alias with something more robust.

### Examples

#### `trash put foo.bar -v`

Moves foo.bar to the trash and prints verbose logs.

#### `trash empty`

Empties the trash directory.

#### `trash restore file -f`

Restores a file, potentially forcing an overwrite.

### `trash --help`

```sh
trash 0.1.0
Tim Cohen <contact@timcohen.dev>
A cli trash utility.

USAGE:
    trash [OPTIONS] <METHOD> [FILE]

ARGS:
    <METHOD>    `p` | `put`     `r` | `restore`     `e` | `empty`
    <FILE>      The target file or directory used with the `put` or `restore` methods

OPTIONS:
    -f, --force      Force non-recoverable deletes/overwrites
    -h, --help       Print help information
    -v, --verbose    Explain all steps
    -V, --version    Print version information
```

# Todo

### Add globular removes

```sh
$ ls
abc.txt foo.txt foz.txt
$ trash put ./fo*
Remove 2 files? y
$ ls
abc.txt

$ touch rawr.rar rawrr.rar
$ trash put -f ./rawr*
$ ls
abc.txt
```

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
Option 1:     | foo.txt.5356c39d-94de-498e-96c0-0d462df92a34
Deletion Date | "2022-06-11T02:31:31.200846+00:00"
File Size     | 60B
──────────────┼───────────────────────────────────────────────────────────────
──────────────┼───────────────────────────────────────────────────────────────
Option 2:     | foo.txt.c4037062-7d6b-47e9-a48b-fd9efb757d6c
Deletion Date | "2022-06-11T02:31:35.722779+00:00"
File Size     | 60B
──────────────┼───────────────────────────────────────────────────────────────

Which would you like to restore? (See at ~/.local/share/Trash/files)
Option: _

```

### Show trash info from Trash files folder

```sh
$ pwd
~/.local/share/Trash/files
$ ls
foz.txt.c16b01c0-116b-4fc5-ab92-8d1a15953ff7
$ trash info foz.txt.c16b01c0-116b-4fc5-ab92-8d1a15953ff7
───────┼───────────────────────────────────────────────────────────────
       │ File: foz.txt.c16b01c0-116b-4fc5-ab92-8d1a15953ff7.trashinfo
   1   │ [Trash Info]
   2   │ Path=/private/tmp/testing/example/foz.txt
   3   │ DeletionDate="2022-06-11T02:31:18.961346+00:00"
───────┼───────────────────────────────────────────────────────────────
```

### Restore from Trash files folder

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
