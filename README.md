# fastfetch-rs

A fast system information tool written in Rust inspired by fastfetch.

[![Rust](https://github.com/liberodark/fastfetch-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/liberodark/fastfetch-rs/actions/workflows/rust.yml)

## Features

- Fast and lightweight system information display
- Customizable ASCII art logos with color support
- Support for custom logo files
- Configurable logo colors (up to 9 different colors)

## Installation

### Via cargo
```bash
cargo install fastfetch-rs
```

### Manual build
```bash
git clone https://github.com/liberodark/fastfetch-rs.git
cd fastfetch-rs
cargo build --release
sudo cp target/release/fastfetch-rs /usr/local/bin/
```

### Precompiled binaries
Precompiled binaries are available in the [Releases](https://github.com/liberodark/fastfetch-rs/releases) section.

## Usage

Simply run the command:
```bash
fastfetch-rs
```

### Options

#### Logo Selection
- `--logo <LOGO>`: Use a specific predefined logo
- `--logo-file <PATH>`: Use a custom logo file
- `--list-logos`: List all available predefined logos

#### Logo Color Customization
You can customize the logo colors using the following options:
- `--logo-color-1 <COLOR>`: Set color for $1 placeholder
- `--logo-color-2 <COLOR>`: Set color for $2 placeholder
- `--logo-color-3 <COLOR>`: Set color for $3 placeholder
- `--logo-color-4 <COLOR>`: Set color for $4 placeholder
- `--logo-color-5 <COLOR>`: Set color for $5 placeholder
- `--logo-color-6 <COLOR>`: Set color for $6 placeholder
- `--logo-color-7 <COLOR>`: Set color for $7 placeholder
- `--logo-color-8 <COLOR>`: Set color for $8 placeholder
- `--logo-color-9 <COLOR>`: Set color for $9 placeholder

#### Color Format Support
Colors can be specified in multiple formats:
- **Color names**: `red`, `blue`, `green`, `yellow`, `cyan`, `magenta`, `white`, `black`
- **Bright variants**: `brightred`, `brightblue`, etc.
- **Dark variants**: `darkred`, `darkblue`, etc.
- **Special colors**: `orange`, `pink`
- **Hex colors**: `#FF0000`, `#00FF00`, `#0000FF`
- **ANSI 256 colors**: `0` to `255`

### Examples

```bash
# Use default OS detection
fastfetch-rs

# Use a specific logo
fastfetch-rs --logo arch

# List available logos
fastfetch-rs --list-logos

# Use a custom logo file
fastfetch-rs --logo-file ~/my-custom-logo.txt

# Customize logo colors
fastfetch-rs --logo-color-1 "#FF0000" --logo-color-2 blue --logo-color-3 128

# Arch Linux with custom colors
fastfetch-rs --logo arch --logo-color-1 cyan --logo-color-2 "#1793D1"

# Use orange and pink colors
fastfetch-rs --logo debian --logo-color-1 orange --logo-color-2 pink

# Try christmas example
fastfetch-rs --logo-file christmas.txt --logo-color-1 green --logo-color-2 red --logo-color-3 yellow
```

## Custom Logo Format

Custom logos use a simple text format with color placeholders:
- `$1` through `$9`: Color placeholders that will be replaced with the specified colors
- ASCII art using regular text characters

Example custom logo:
```
$3★
             $1▲▲▲
            $1▲▲▲▲▲
           $1▲▲▲$2●$1▲▲▲
          $1▲▲▲▲▲▲▲▲▲
         $1▲▲$2●$1▲▲▲$2●$1▲▲
        $1▲▲▲▲▲▲▲▲▲▲▲
       $1▲▲▲$2●$1▲▲▲$2●$1▲▲▲
      $1▲▲▲▲▲▲▲▲▲▲▲▲▲
     $1▲▲$2●$1▲▲▲$2●$1▲▲▲$2●$1▲▲
    $1▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲
   $1▲▲▲$2●$1▲▲$2●$1▲▲$2●$1▲▲$2●$1▲▲▲
  $1▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲
 $1▲▲$2●$1▲▲$2●$1▲▲▲$2●$1▲▲▲$2●$1▲▲$2●$1▲▲
$1▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲▲
             $2███
             $2███
          $2▀▀▀▀▀▀▀
```
