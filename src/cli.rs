use clap::{Parser, ValueEnum};

/// URL-encodes or -decodes the input. If INPUT is given, it encodes or
/// decodes INPUT, otherwise it takes its input fromt stdin.
#[derive(Parser)]
#[command(version, author, about)]
pub struct Args {
    /// The string to encode.
    pub input: Option<String>,

    /// Decode the input, rather than encode.
    #[arg(short, long)]
    pub decode: bool,

    /// Decode the input non-lossily. If set, the program will fail if it
    /// encounters a sequence that does not produce valid UTF-8.
    #[arg(short, long)]
    pub strict_decode: bool,

    /// The encode set to use when encoding.
    /// See https://url.spec.whatwg.org/ for more details.
    #[arg(short, long)]
    pub encode_set: EncodeSet,
}

#[derive(Default, ValueEnum, Clone)]
pub enum EncodeSet {
    #[default]
    Component,
    Control,
    Form,
    Fragment,
    Path,
    Query,
    Squery,
    Userinfo,
}
