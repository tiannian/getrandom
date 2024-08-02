use std::{fmt::Display, num::NonZeroU32};

use risc0_zkvm::{declare_syscall, guest::env};

declare_syscall!(WH3_GETRANDOM);

#[derive(Debug)]
pub enum Error {
    WrongTargetLength,
}

impl Error {
    pub const fn code(self) -> NonZeroU32 {
        unsafe { NonZeroU32::new_unchecked(190001) }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<NonZeroU32> for Error {
    fn from(_code: NonZeroU32) -> Self {
        Self::WrongTargetLength
    }
}

pub fn getrandom(dest: &mut [u8]) -> Result<(), Error> {
    let to_size = dest.len() as u64;
    let to_bytes = to_size.to_be_bytes();

    let target: &[u8] = env::send_recv_slice(WH3_GETRANDOM, &to_bytes);

    if target.len() != dest.len() {
        return Err(Error::WrongTargetLength);
    }

    dest.copy_from_slice(target);

    Ok(())
}
