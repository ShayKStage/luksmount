# Maintainer: Shay K. Stage <ShayKStage@protonmail.com>
pkgname=luksmount
pkgver=1.0.2
pkgrel=2
pkgdesc='Utilities for LUKS encrypted drives/DM-Crypt'
arch=(x86_64)
url='https://github.com/ShayKStage/luksmount'
license=(
    Apache
    MIT
)
depends=('cryptsetup')
makedepends=('cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/ShayKStage/luksmount/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP')

prepare() {
    cd $pkgname-$pkgver
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    cd $pkgname-$pkgver
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --bins --release --all-features
}

check() {
    cd $pkgname-$pkgver
    export RUSTUP_TOOLCHAIN=stable
    cargo test --frozen --all-features
}

package() {
    cd $pkgname-$pkgver
    find target/release \
        -maxdepth 1 \
        -executable \
        -type f \
        -exec install -Dm0755 -t "$pkgdir/usr/bin/" {} +
    install -Dm 644 "LICENSE-APACHE" -t "$pkgdir/usr/share/licenses/$pkgname"
    install -Dm 644 "LICENSE-MIT" -t "$pkgdir/usr/share/licenses/$pkgname"
    install -Dm 644 README.md -t "$pkgdir/usr/share/doc/$pkgname"
}
