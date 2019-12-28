pkg_name=rust-test
pkg_origin=lancewf
pkg_version="0.1.0"
pkg_maintainer="Lance Finfrock <lancewf@gmail.com>"
pkg_license=("Apache-2.0")
pkg_bin_dirs=(bin)

pkg_deps=(
  core/glibc
  core/gcc-libs
)

pkg_build_deps=(
  core/rust
  core/gcc
  core/pkg-config
  core/openssl
  core/make
)

do_build() {
  echo "Building"
  pushd "${PLAN_CONTEXT}/.."
    cargo build --release --verbose
  popd
}

do_install() {
  echo "cache path: ${CACHE_PATH}"
  echo "pkg_prefix: ${pkg_prefix}"
  pushd "${PLAN_CONTEXT}/.."
    cp -r src ${CACHE_PATH}/.
    cp Cargo.toml ${CACHE_PATH}/.
    cargo install --root "${pkg_prefix}" --path "${CACHE_PATH}" --verbose
  popd
}