use std::{fmt::Display, fs::File, io::Read, num::NonZeroU32, sync::RwLock};

#[derive(Debug)]
pub struct Error {}

impl Error {
    pub const fn code(self) -> NonZeroU32 {
        unsafe { NonZeroU32::new_unchecked(1) }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<NonZeroU32> for Error {
    fn from(_code: NonZeroU32) -> Self {
        Self {}
    }
}

struct RandomState {
    pub read_mode: bool,
    pub data: Vec<u8>,
    pub offset: usize,
}

impl RandomState {
    pub const fn new() -> Self {
        Self {
            read_mode: false,
            data: Vec::new(),
            offset: 0,
        }
    }
}

static RANDOM_STATE: RwLock<RandomState> = RwLock::new(RandomState::new());

pub fn getrandom_set_read_mode() {
    let mut state = RANDOM_STATE.write().expect("Failed to read state");

    state.read_mode = true;
}

pub fn getrandom_read_data() -> Vec<u8> {
    let reader = RANDOM_STATE.read().expect("Failed to read state");

    reader.data.clone()
}

pub fn getrandom(dest: &mut [u8]) -> Result<(), Error> {
    let read_mode = {
        let reader = RANDOM_STATE.read().expect("Failed to read state");

        reader.read_mode
    };

    let mut state = RANDOM_STATE.write().expect("Failed to read state");

    if read_mode {
        let begin = state.offset;
        let end = state.offset + dest.len();

        dest.copy_from_slice(&state.data[begin..end]);

        state.offset = end;
    } else {
        let mut file = File::open("/dev/random").expect("Failed to open random");

        file.read_exact(dest).expect("Failed to read data");
        state.data.extend_from_slice(dest);

        println!("Generated send random: {}", hex::encode(dest));
    }

    Ok(())
}
