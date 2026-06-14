use std::{
    borrow::Borrow,
    error::Error,
    io::{self, BufRead},
    iter,
};

use clap::Parser;
use pe::AsciiSet;
use percent_encoding as pe;

mod cli;
mod encode_sets;

use crate::cli::{Args, EncodeSet};

fn main() {
    let args = Args::parse();

    if let Err(e) = run(args) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut stdout_handle = io::stdout().lock();
    let mut stdin_handle = io::stdin().lock();

    let encode_set = get_encode_set(&args.encode_set);

    if let Some(input) = &args.input {
        return transform_line(input, &mut stdout_handle, encode_set, &args);
    }

    let mut buf = String::new();

    while stdin_handle.read_line(&mut buf)? > 0 {
        transform_line(buf.trim_end(), &mut stdout_handle, encode_set, &args)?;
        buf.clear();
    }

    Ok(())
}

fn get_encode_set(encode_set: &EncodeSet) -> &'static AsciiSet {
    match encode_set {
        EncodeSet::Control => encode_sets::CONTROLS,
        EncodeSet::Fragment => encode_sets::FRAGMENT,
        EncodeSet::Query => encode_sets::QUERY,
        EncodeSet::Squery => encode_sets::SPECIAL_QUERY,
        EncodeSet::Path => encode_sets::PATH,
        EncodeSet::Userinfo => encode_sets::USERINFO,
        EncodeSet::Component => encode_sets::COMPONENT,
        EncodeSet::Form => encode_sets::FORM,
    }
}

fn transform_line(
    line: &str,
    output: &mut impl io::Write,
    encode_set: &'static AsciiSet,
    args: &Args,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let decode_mode = args.decode || args.strict_decode;
    let lossy = !args.strict_decode;

    if decode_mode {
        decode(line.as_bytes(), output, lossy)
    } else {
        encode(line, encode_set, output)?;
        Ok(())
    }
}

fn decode(
    line: &[u8],
    output: &mut impl io::Write,
    lossy: bool,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let decoder = pe::percent_decode(line);

    let decoded = if lossy {
        decoder.decode_utf8_lossy()
    } else {
        decoder.decode_utf8()?
    };

    let result = write_output(iter::once(decoded.borrow()), output);

    match result {
        Err(e) => Err(Box::new(e)),
        _ => Ok(()),
    }
}

fn encode(
    line: &str,
    encode_set: &'static AsciiSet,
    output: &mut impl io::Write,
) -> io::Result<()> {
    let encoded = pe::utf8_percent_encode(line, encode_set);
    write_output(encoded, output)
}

fn write_output<'a, B, W>(strings: B, output: &mut W) -> io::Result<()>
where
    B: IntoIterator<Item = &'a str>,
    W: io::Write,
{
    for string in strings {
        output.write_all(string.as_bytes())?;
    }

    output.write_all("\n".as_bytes())?;

    Ok(())
}
