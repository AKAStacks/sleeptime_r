# Maintainer: Dennis Zarger (github: AKAStacks)
pkgname=sleeptime_r
pkgver=0.1.0
pkgrel=1
pkgdesc="Simple GTK+ application for scheduling a system shutdown."
license=("MIT")
arch=("x86_64")
makedepends=("cargo" "git")
depends=("gtk3")
source=("git+https://github.com/AKAStacks/sleeptime_r.git")
md5sums=("SKIP")

pkgver() {
    (git describe --long --tags || echo "$pkgver") | sed 's/^v//;s/\([^-]*-g\)/r\1/;s/-/./g'
}

build() {
    cd "${srcdir}/${pkgname}"
    #export CARGO_TARGET_DIR="${srcdir}/${pkgname}"
    cargo build --release
}

package() {
    builtdir="${srcdir}/${pkgname}"
    cd ..
    usrdir="$pkgdir/usr"
    mkdir -p $usrdir
    mkdir -p "$usrdir/share/licenses/$pkgname"
    cp "${builtdir}/LICENSE" "$usrdir/share/licenses/$pkgname/"
    mkdir -p "$usrdir/bin"
    cp "${builtdir}/target/release/${pkgname}" "$usrdir/bin/"
    #cargo install --path "${srcdir}/${pkgname}" --root "$usrdir"
    #rm -f $usrdir/.crates.toml
}

