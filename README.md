# Nali

User-friendly package manager for KhmerOS - a fork of paru

[![nali](https://img.shields.io/badge/nali-v2.1.0-blue.svg)](https://github.com/koompi/nali)

## Description

**Nali** is a fork of paru with intuitive English commands while maintaining full backward compatibility. Built upon the excellent [paru](https://github.com/Morganamilo/paru) codebase, nali preserves all of paru's powerful functionality while introducing simple English commands that eliminate the need to memorize cryptic pacman flags.

This project was **initiated by KOOMPI OS** to make Arch Linux package management more accessible to more users and those who prefer intuitive command syntax.

### Key Features

- **Intuitive English Commands**: Use `nali install firefox` instead of `paru -S firefox`
- **Full Backward Compatibility**: All existing pacman flags continue to work unchanged
- **AUR Support**: Complete AUR helper functionality inherited from paru
- **KhmerOS Integration**: Optimized for KhmerOS distribution
- **Zero Learning Curve**: Simple English verbs for common operations

## English Commands vs Traditional Flags

| English Command | Traditional Flag | Description |
|----------------|------------------|-------------|
| `nali install pkg` | `paru -S pkg` | Install a package |
| `nali update` | `paru -Syu` | Update all packages |
| `nali search pkg` | `paru -Ss pkg` | Search for packages |
| `nali remove pkg` | `paru -R pkg` | Remove a package |
| `nali info pkg` | `paru -Si pkg` | Show package information |
| `nali list` | `paru -Q` | List installed packages |
| `nali clean` | `paru -Sc` | Clean package cache |
| `nali upgrade` | `paru -Syu` | Upgrade all packages |
| `nali download pkg` | `paru -Sw pkg` | Download package only |
| `nali reinstall pkg` | `paru -S --needed pkg` | Reinstall package |
| `nali purge pkg` | `paru -Rns pkg` | Remove package with dependencies |
| `nali autoremove` | `paru -Rns $(pacman -Qtdq)` | Remove orphaned packages |
| `nali installed` | `paru -Q` | List installed packages |
| `nali orphans` | `paru -Qtd` | List orphaned packages |
| `nali foreign` | `paru -Qm` | List foreign packages |
| `nali explicit` | `paru -Qe` | List explicitly installed packages |
| `nali files pkg` | `paru -Ql pkg` | List package files |
| `nali owns file` | `paru -Qo file` | Find package that owns a file |
| `nali depends pkg` | `paru -Qi pkg` | Show package dependencies |
| `nali refresh` | `paru -Sy` | Refresh package databases |
| `nali mirror-update` | `paru -Syy` | Force refresh package databases |
| `nali check` | `paru -Dk` | Check package integrity |
| `nali verify` | `paru -Qk` | Verify package files |
| `nali cache-info` | `paru -Sc --print` | Show cache information |

## Installation

### For KhmerOS Users

```bash
sudo pacman -S nali
```

### From Source

```bash
sudo pacman -S --needed base-devel git

git clone https://github.com/koompi/nali.git
cd nali
makepkg -si
```

### Building for Development

```bash
git clone https://github.com/koompi/nali.git
cd nali
cargo build --release
sudo cp target/release/nali /usr/local/bin/
```

## Quick Start

### Basic Usage

```bash
# Install a package
nali install firefox

# Update all packages
nali update

# Search for packages
nali search libreoffice

# Remove a package
nali remove firefox

# Show package information
nali info firefox

# List all installed packages
nali list
```

### Advanced Usage (All paru features work)

```bash
# Install from AUR
nali install spotify

# Upgrade AUR packages only
nali -Sua

# Interactive search and install
nali libreoffice

# Build and install from local PKGBUILD
nali -Bi .

# Download PKGBUILD
nali -G spotify
```

## Examples

### English Commands
```bash
nali install firefox thunderbird libreoffice
nali update
nali search "office suite"
nali remove firefox
nali clean
```

### Traditional Flags (still supported)
```bash
nali -S firefox thunderbird libreoffice
nali -Syu
nali -Ss "office suite"
nali -R firefox
nali -Sc
```

## Credits

**Nali** is built upon the excellent [paru](https://github.com/Morganamilo/paru) project by [Morganamilo](https://github.com/Morganamilo) and contributors. We extend our sincere gratitude to the paru development team for creating such a robust AUR helper.

- **Original Project**: [paru](https://github.com/Morganamilo/paru) by Morganamilo
- **Maintainer**: KOOMPI OS Team
- **Purpose**: KhmerOS package management enhancement
- **License**: GPL-3.0 (same as paru)

## Contributing

We welcome contributions from the KhmerOS and Arch Linux communities:

1. **Bug Reports**: Please open issues on our [GitHub repository](https://github.com/koompi/nali/issues)
2. **Feature Requests**: Suggest new English commands or improvements
3. **Translations**: Help improve Khmer language support
4. **Documentation**: Contribute to user guides and examples

See [CONTRIBUTING.md](./CONTRIBUTING.md) for detailed contribution guidelines.

## Support

- **Documentation**: Available in both English and Khmer
- **Community**: Join our [Telegram group](https://t.me/khmeros) for support
- **Issues**: Report bugs at [GitHub Issues](https://github.com/koompi/nali/issues)

## Debugging

Nali is built on paru, so debugging follows similar patterns:

1. **Check if makepkg works**: If nali can't build a package, first verify `makepkg` can build it
2. **Enable debug mode**: Set `PARU_DEBUG=1` for verbose output
3. **Check logs**: Review `/var/log/pacman.log` for system-level issues

## Development

### Adding New English Commands

To add new English commands, edit `src/translate.rs`:

```rust
// Add new translations in the CommandTranslator::new() method
translations.insert("newcommand".to_string(), vec!["-NewFlag".to_string()]);
```

### Testing

```bash
# Run tests
cargo test

# Test specific command translations
cargo test -- test_install_translation

# Manual testing
./target/debug/nali install firefox --dry-run
```

## License

This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE) file for details, same as the original paru project.

---

**Made with ❤️ by KOOMPI OS for KhmerOS**