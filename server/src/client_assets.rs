#![allow(unused)]

use include_dir::Dir;

#[cfg(feature = "bundle-client")]
static CLIENT_ASSETS_DIR: Dir = include_dir::include_dir!("$CARGO_MANIFEST_DIR/../client/dist");

pub fn is_active() -> bool {
    cfg!(feature = "bundle-client")
}

pub fn get_assets() -> &'static include_dir::Dir<'static> {
    #[cfg(feature = "bundle-client")]
    return &CLIENT_ASSETS_DIR;
    panic!("Client assets are not bundled. You need to enable the `bundle-client` feature.")
}
