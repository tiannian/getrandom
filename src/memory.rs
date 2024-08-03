use std::{fmt::Display, num::NonZeroU32, sync::RwLock};

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

struct RandomState {
    pub offset: usize,
    pub data: Vec<u8>,
}

static RANDOM_BYTES: RwLock<RandomState> = RwLock::new(RandomState {
    offset: 0,
    data: Vec::new(),
});

pub fn set_random_bytes(bytes: Vec<u8>) {
    let mut writer = RANDOM_BYTES.write().unwrap();

    writer.data = bytes;
}

pub fn getrandom(dest: &mut [u8]) -> Result<(), Error> {
    let len = dest.len();

    let mut writer = RANDOM_BYTES.write().unwrap();

    let begin = writer.offset;

    dest.copy_from_slice(&writer.data[begin..begin + len]);

    writer.offset += len;

    Ok(())
}
