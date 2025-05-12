pkgname=nightscape
pkgver=0.1.0
pkgrel=1
pkgdesc="Animación de un cielo nocturno en la terminal, con luna, estrellas y eventos aleatorios."
arch=('x86_64')
url="https://github.com/xhon4/nightscape"
license=('MIT')
depends=()
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/xhon4/nightscape/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('a068a8c8d94ffd8349dc8bccafe6683571eb882ba38606319c03842952543a32')

build() {
  cd "$pkgname-$pkgver"
  cargo build --release --locked
}

package() {
  cd "$pkgname-$pkgver"
  install -Dm755 "target/release/nightscape" "$pkgdir/usr/bin/nightscape"
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
