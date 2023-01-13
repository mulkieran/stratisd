// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use data_encoding::BASE32_NOPAD;
use std::{
    env,
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug)]
struct ExecutableError(String);

impl Display for ExecutableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ExecutableError {}

fn base32_decode(var_name: &str, base32_str: &str) -> Result<(), Box<dyn Error>> {
    let base32_decoded = String::from_utf8(BASE32_NOPAD.decode(base32_str.as_bytes())?)?;
    println!("{}={}", var_name, base32_decoded);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();

    let _ = args.next();
    let key = args.next().unwrap();
    let value = args.next().unwrap();

    base32_decode(&key, &value)?;

    Ok(())
}
