use clap::Parser;
use hyper::Uri;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Where to proxy to, in the format of `[scheme?]://[authority]` or just the authority.
    pub destination: Uri,
}
