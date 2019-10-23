use multibase::Base;
use std::io::{Read as _, Write as _};
use structopt::StructOpt;

#[derive(StructOpt)]
/// Convert to and from multibase encodings
enum Args {
    /// Decode multi-base encoded input. If encoded agrument is provided, it is used as input.
    /// Otherwise input is read from stdin.
    Decode {
        /// A multi-base encoded input
        encoded: Option<String>,
    },
    /// convert stdin to base2          0 binary (01010101)
    Base2,
    /// convert stdin to base8          7 octal
    Base8,
    /// convert stdin to base10         9 decimal
    Base10,
    /// convert stdin to base16         f hexadecimal
    Base16,
    /// convert stdin to base16upper    F hexadecimal
    Base16upper,
    /// convert stdin to base32hex      v rfc4648 no padding - highest char
    Base32hex,
    /// convert stdin to base32hexupper V rfc4648 no padding - highest char
    Base32hexupper,
    /// convert stdin to base32         b rfc4648 no padding
    Base32,
    /// convert stdin to base32upper    B rfc4648 no padding
    Base32upper,
    /// convert stdin to base32z        h z-base-32 (used by Tahoe-LAFS)
    Base32z,
    /// convert stdin to base58flickr   Z base58 flicker
    Base58flickr,
    /// convert stdin to base58btc      z base58 bitcoin
    Base58btc,
    /// convert stdin to base64         m rfc4648 no padding
    Base64,
    /// convert stdin to base64url      u rfc4648 no padding
    Base64url,
}

impl Args {
    fn read_input(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(match self {
            Args::Decode { encoded: Some(enc) } => multibase::decode(enc)?.1,
            Args::Decode { encoded: None } => {
                let inp = read_stdin()?;
                let enc: &str = core::str::from_utf8(&inp)?;
                multibase::decode(enc)?.1
            }
            _ => read_stdin()?,
        })
    }

    fn write_output(&self, input: &[u8]) -> Result<(), std::io::Error> {
        match self {
            Args::Decode { .. } => std::io::stdout().write_all(input),
            Args::Base2 => write_as_base(Base::Base2, input),
            Args::Base8 => write_as_base(Base::Base8, input),
            Args::Base10 => write_as_base(Base::Base10, input),
            Args::Base16 => write_as_base(Base::Base16, input),
            Args::Base16upper => write_as_base(Base::Base16Upper, input),
            Args::Base32hex => write_as_base(Base::Base32hex, input),
            Args::Base32hexupper => write_as_base(Base::Base32hexUpper, input),
            Args::Base32 => write_as_base(Base::Base32, input),
            Args::Base32upper => write_as_base(Base::Base32Upper, input),
            Args::Base32z => write_as_base(Base::Base32z, input),
            Args::Base58flickr => write_as_base(Base::Base58flickr, input),
            Args::Base58btc => write_as_base(Base::Base58btc, input),
            Args::Base64 => write_as_base(Base::Base64, input),
            Args::Base64url => write_as_base(Base::Base64url, input),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::from_args();
    let inp = args.read_input()?;
    args.write_output(&inp)?;
    Ok(())
}

fn read_stdin() -> Result<Vec<u8>, std::io::Error> {
    let mut inp: Vec<u8> = Vec::with_capacity(65536);
    std::io::stdin().read_to_end(&mut inp)?;
    Ok(inp)
}

fn write_as_base(base: Base, inp: &[u8]) -> Result<(), std::io::Error> {
    let out = multibase::encode(base, &inp);
    print!("{}", out);
    Ok(())
}
