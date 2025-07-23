# Linux System Tray Dependencies

## Problem

Different Linux distributions use different system tray indicator libraries:

- **Ayatana-based** (Linux Mint, Ubuntu MATE, etc.): `libayatana-appindicator3`
- **Traditional AppIndicator** (KDE Neon, Ubuntu 24.04 with KDE): `libappindicator3`

This creates a dependency conflict when building or running cc-monitor-rs on different distributions.

## Solutions

### For End Users

#### On KDE Neon / Ubuntu 24.04 with KDE:
```bash
# Install the required dependency
sudo apt-get install libappindicator3-dev libappindicator3-1

# If you encounter conflicts, you may need to:
sudo apt-get install --reinstall libappindicator3-1
```

#### On Linux Mint / Ubuntu MATE:
```bash
# Install the required dependency
sudo apt-get install libayatana-appindicator3-dev libayatana-appindicator3-1
```

### For Developers

#### Option 1: Build with Specific Features

We can add feature flags to Cargo.toml to support both:

```toml
[features]
default = ["tray-ayatana"]
tray-ayatana = ["tray-icon/ayatana"]
tray-libappindicator = ["tray-icon/libappindicator"]
```

Then build with:
```bash
# For KDE Neon
cargo build --release --no-default-features --features tray-libappindicator

# For Linux Mint (default)
cargo build --release
```

#### Option 2: Runtime Detection

The tray-icon crate tries to detect the appropriate library at runtime. However, both libraries need to be available during compilation.

#### Option 3: Static Linking

Consider statically linking the tray dependencies to avoid runtime issues.

## Workaround for KDE Neon

If you're on KDE Neon and experiencing issues:

1. **Install both libraries** (if possible):
   ```bash
   sudo apt-get install libappindicator3-dev libayatana-appindicator3-dev
   ```

2. **Create symbolic links** (temporary fix):
   ```bash
   # This is a workaround - use with caution
   sudo ln -s /usr/lib/x86_64-linux-gnu/libappindicator3.so.1 /usr/lib/x86_64-linux-gnu/libayatana-appindicator3.so.1
   ```

3. **Use AppImage or Flatpak** distribution which bundles dependencies.

## Building Universal Binary

To create a binary that works on both systems:

1. Build on a system with both libraries installed
2. Use `ldd` to check dependencies:
   ```bash
   ldd target/release/cc-monitor-rs | grep indicator
   ```

3. Consider using AppImage for distribution:
   ```bash
   # This bundles all dependencies
   ./create-appimage.sh
   ```

## Testing

Test on different distributions:
- KDE Neon (Ubuntu 24.04 base)
- Linux Mint
- Ubuntu with GNOME
- Fedora with KDE

## References

- [tray-icon issue #110](https://github.com/tauri-apps/tray-icon/issues/110)
- [AppIndicator vs Ayatana](https://github.com/AyatanaIndicators/libayatana-appindicator)