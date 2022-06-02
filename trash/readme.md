### 🗑 Trash

A Rust based CLI trash utility implementing [The FreeDesktop.org Trash specification](https://specifications.freedesktop.org/trash-spec/trashspec-1.0.html). Verbs based on [trash-cli](https://github.com/andreafrancia/trash-cli). It replaces the spartan rm alias with something more robust.

### Examples

#### `trash put foo.bar -v`

Moves foo.bar to the trash and prints verbose logs.

#### `trash empty`

Empties the trash directory.

#### `trash restore file -f`

Restores a file, potentially forcing an overwrite.
