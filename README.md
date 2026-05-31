# show

A small, fast, `bat`-like syntax highlighter for the terminal, written in Rust.

`show` prints any source file to your terminal with **syntax highlighting** and
**line numbers** — like `cat`, but readable. It supports a wide range of
languages out of the box (Rust, Python, JavaScript, Nix, TOML, Dockerfile, and
many more) thanks to [`syntect`](https://github.com/trishume/syntect) and
[`two-face`](https://github.com/CosmicHorrorDev/two-face).

```text
   1 │ { pkgs ? import <nixpkgs> {} }:
   2 │
   3 │ pkgs.mkShell {
   4 │   name = "python-dev-shell";
   5 │   ...
```

---

## 1. What is this and why?

When you `cat` a source file in the terminal you get a flat wall of
uncolored text. `show` solves that:

- **Syntax highlighting** for dozens of languages, auto-detected from the file
  extension.
- **Line numbers** down the left margin, so you can quickly reference code.
- **~30 built-in color themes** (Dracula, Nord, Catppuccin, Monokai, Gruvbox, Solarized…) with fuzzy name matching.
- **Single self-contained binary** — no runtime, no dependencies to install.

It's a lightweight alternative to [`bat`](https://github.com/sharkdp/bat) for
when you just want quick, colorful file output.

---

## 2. Installation (for developers)

### Option A — Download a prebuilt binary (recommended)

Every tagged release ships prebuilt binaries for Linux, macOS (Intel & Apple
Silicon), and Windows on the
[**Releases page**](https://github.com/Behruz-s-organization/bat_in_rust/releases).

1. Open the [latest release](https://github.com/Behruz-s-organization/bat_in_rust/releases/latest).
2. Download the asset matching your platform:

   | Platform              | Asset                                   |
   | --------------------- | --------------------------------------- |
   | Linux (x86_64)        | `show-x86_64-unknown-linux-gnu`         |
   | macOS (Intel)         | `show-x86_64-apple-darwin`              |
   | macOS (Apple Silicon) | `show-aarch64-apple-darwin`             |
   | Windows (x86_64)      | `show-x86_64-pc-windows-msvc.exe`       |

3. Make it executable and put it on your `PATH` (Linux/macOS):

   ```sh
   chmod +x show-x86_64-unknown-linux-gnu
   sudo mv show-x86_64-unknown-linux-gnu /usr/local/bin/show
   ```

### Option B — Install with Cargo

If you have a [Rust toolchain](https://rustup.rs/) installed:

```sh
cargo install --git https://github.com/Behruz-s-organization/bat_in_rust
```

### Option C — Build from source

```sh
git clone https://github.com/Behruz-s-organization/bat_in_rust
cd bat_in_rust
cargo build --release
# binary is at ./target/release/show
```

---

## 3. Usage

```text
show [OPTIONS] <FILE>
```

### Arguments

| Argument | Description                  |
| -------- | ---------------------------- |
| `<FILE>` | Path to the file to display. |

### Options

| Option            | Description                                                   | Default        |
| ----------------- | ------------------------------------------------------------- | -------------- |
| `-t, --theme <T>` | Color theme (see [Themes](#themes)). Names are fuzzy-matched. | `gruvbox-dark` |
| `--list-themes`   | List all available themes and exit.                           |                |
| `-h, --help`      | Print help.                                                   |                |
| `-V, --version`   | Print version.                                                |                |
| `-u, --update`    | Update to the latest version.                                 |                |

### Themes

`show` ships with ~30 themes via [`two-face`](https://github.com/CosmicHorrorDev/two-face).
List them all with:

```sh
show --list-themes
```

Available themes include:

```text
1337                      Monokai Extended          base16
Catppuccin Frappe         Monokai Extended Bright   base16-256
Catppuccin Latte          Monokai Extended Light    base16-eighties.dark
Catppuccin Macchiato      Monokai Extended Origin   base16-mocha.dark
Catppuccin Mocha          Nord                      base16-ocean.dark
Coldark-Cold              OneHalfDark               base16-ocean.light
Coldark-Dark              OneHalfLight              gruvbox-dark  (default)
DarkNeon                  Solarized (dark)          gruvbox-light
Dracula                   Solarized (light)         zenburn
GitHub                    Sublime Snazzy
InspiredGitHub            TwoDark
```

Theme names are **fuzzy-matched** — case and punctuation are ignored — so all of
these select the same theme:

```sh
show --theme "Solarized (dark)" file.rs
show --theme solarized_dark      file.rs
show --theme solarizeddark       file.rs
```

If an unknown theme is given, `show` warns and falls back to the default.

### Examples

Highlight a file with the default theme:

```sh
show src/main.rs
```

Use a specific theme:

```sh
show --theme dracula src/main.rs
```

List all available themes:

```sh
show --list-themes
```

Highlight a config file (extension is auto-detected):

```sh
show shell.nix
show Cargo.toml
```

---

## License

MIT
