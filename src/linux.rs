use std::{fmt::Display, fs::File, io::Read, num::NonZeroU32, sync::mpsc::Sender};

use once_cell::sync::OnceCell;

#[derive(Debug)]
pub enum Error {
    FailedToSetSender,
    FailedToSendData,
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
        Self::FailedToSetSender
    }
}

static RANDOM_STATE: OnceCell<Sender<Vec<u8>>> = OnceCell::new();

pub fn set_recoder(sender: Sender<Vec<u8>>) -> Result<(), Error> {
    RANDOM_STATE
        .set(sender)
        .map_err(|_| Error::FailedToSetSender)?;

    Ok(())
}

pub fn getrandom(dest: &mut [u8]) -> Result<(), Error> {
    let mut file = File::open("/dev/urandom").expect("Failed to open random");

    file.read_exact(dest).expect("Failed to read data");

    if let Some(sender) = RANDOM_STATE.get() {
        sender
            .send(dest.to_vec())
            .map_err(|_| Error::FailedToSendData)?;
    }

    Ok(())
}
