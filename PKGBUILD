# Maintainer: Shay K. Stage <ShayKStage@protonmail.com>
pkgname=luxutil
pkgver=1.0.2
pkgrel=1
pkgdesc='Utilities for LUKS encrypted drives/DM-Crypt'
arch=(x86_64)
url='https://github.com/ShayKStage/luxutil'
license=(
    Apache
    MIT
)
depends=('cryptsetup')
makedepends=('cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/ShayKStage/luxutil/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP')

prepare() {
    cd $pkgname-$pkgver
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    cd $pkgname-$pkgver
    cargo build --bins --release --frozen
}

check() {
    cd $pkgname-$pkgver
    cargo test --frozen
}

package() {
    cd $pkgname-$pkgver
    install -Dm 755 "target/release/luksmount" -t "$pkgdir/usr/bin"
    install -Dm 755 "target/release/luksumount" -t "$pkgdir/usr/bin"
    install -Dm 644 "LICENSE-APACHE" -t "$pkgdir/usr/share/licenses/$pkgname"
    install -Dm 644 "LICENSE-MIT" -t "$pkgdir/usr/share/licenses/$pkgname"
    install -Dm 644 README.md -t "$pkgdir/usr/share/doc/$pkgname"
}