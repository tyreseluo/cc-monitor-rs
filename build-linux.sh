#!/bin/bash

# Build script for Linux with proper tray icon support

echo "CC Monitor Build Script for Linux"
echo "================================="

# Detect distribution
if [ -f /etc/os-release ]; then
    . /etc/os-release
    DISTRO=$ID
    VERSION=$VERSION_ID
    echo "Detected: $NAME $VERSION"
else
    echo "Could not detect distribution"
    DISTRO="unknown"
fi

# Function to check if a package is installed
check_package() {
    if command -v dpkg &> /dev/null; then
        dpkg -l | grep -q "^ii  $1"
    elif command -v rpm &> /dev/null; then
        rpm -q "$1" &> /dev/null
    else
        return 1
    fi
}

# Determine which tray library to use
BUILD_FEATURES=""

case "$DISTRO" in
    "linuxmint"|"ubuntu-mate"|"elementary")
        echo "Using Ayatana AppIndicator (default for $DISTRO)"
        if ! check_package "libayatana-appindicator3-dev"; then
            echo "Installing libayatana-appindicator3-dev..."
            sudo apt-get update
            sudo apt-get install -y libayatana-appindicator3-dev
        fi
        # Use default features (ayatana is typically default)
        BUILD_FEATURES=""
        ;;
    
    "neon"|"kubuntu")
        echo "Using libappindicator3 (recommended for KDE)"
        if ! check_package "libappindicator3-dev"; then
            echo "Installing libappindicator3-dev..."
            sudo apt-get update
            sudo apt-get install -y libappindicator3-dev
        fi
        BUILD_FEATURES="--no-default-features --features tray-libappindicator"
        ;;
    
    "ubuntu")
        # Check if KDE is installed
        if [ "$XDG_CURRENT_DESKTOP" = "KDE" ] || [ "$DESKTOP_SESSION" = "plasma" ]; then
            echo "Detected KDE on Ubuntu, using libappindicator3"
            if ! check_package "libappindicator3-dev"; then
                echo "Installing libappindicator3-dev..."
                sudo apt-get update
                sudo apt-get install -y libappindicator3-dev
            fi
            BUILD_FEATURES="--no-default-features --features tray-libappindicator"
        else
            echo "Using Ayatana AppIndicator (default for Ubuntu)"
            if ! check_package "libayatana-appindicator3-dev"; then
                echo "Installing libayatana-appindicator3-dev..."
                sudo apt-get update
                sudo apt-get install -y libayatana-appindicator3-dev
            fi
            BUILD_FEATURES=""
        fi
        ;;
    
    "fedora")
        echo "Using libappindicator for Fedora"
        if ! check_package "libappindicator-gtk3-devel"; then
            echo "Installing libappindicator-gtk3-devel..."
            sudo dnf install -y libappindicator-gtk3-devel
        fi
        BUILD_FEATURES="--no-default-features --features tray-libappindicator"
        ;;
    
    *)
        echo "Unknown distribution. Attempting default build..."
        echo "You may need to install tray dependencies manually:"
        echo "  - For Ayatana: libayatana-appindicator3-dev"
        echo "  - For AppIndicator: libappindicator3-dev"
        BUILD_FEATURES=""
        ;;
esac

# Install other required dependencies
echo ""
echo "Checking other dependencies..."

if command -v apt-get &> /dev/null; then
    # Debian/Ubuntu based
    PACKAGES="build-essential pkg-config libssl-dev"
    for pkg in $PACKAGES; do
        if ! check_package "$pkg"; then
            echo "Installing $pkg..."
            sudo apt-get install -y "$pkg"
        fi
    done
elif command -v dnf &> /dev/null; then
    # Fedora based
    PACKAGES="gcc pkg-config openssl-devel"
    for pkg in $PACKAGES; do
        if ! check_package "$pkg"; then
            echo "Installing $pkg..."
            sudo dnf install -y "$pkg"
        fi
    done
fi

# Build the project
echo ""
echo "Building CC Monitor..."
echo "Build command: cargo build --release $BUILD_FEATURES"

cargo build --release $BUILD_FEATURES

if [ $? -eq 0 ]; then
    echo ""
    echo "Build successful!"
    echo "Binary location: target/release/cc-monitor-rs"
    echo ""
    echo "To run: ./target/release/cc-monitor-rs"
    echo "To install system-wide: sudo cp target/release/cc-monitor-rs /usr/local/bin/"
else
    echo ""
    echo "Build failed!"
    echo ""
    echo "Troubleshooting:"
    echo "1. Check if you have Rust installed: rustc --version"
    echo "2. Try installing both tray libraries:"
    echo "   sudo apt-get install libayatana-appindicator3-dev libappindicator3-dev"
    echo "3. Check the error messages above for missing dependencies"
    echo ""
    echo "For KDE Neon specific issues, see: docs/LINUX_TRAY_DEPENDENCIES.md"
    exit 1
fi