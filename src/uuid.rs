use rand::Rng;

#[derive(Debug, Clone)]
pub struct Uuid {
    pub uuid: String,
}

impl Uuid {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            uuid: format!(
                "{:04x}{:04x}-{:04x}-{:04x}-{:04x}-{:04x}{:04x}{:04x}",
                rng.gen::<u16>(),
                rng.gen::<u16>(),
                rng.gen::<u16>(),
                (rng.gen::<u16>() & 0x0fff) | 0x4000,
                (rng.gen::<u16>() & 0x3fff) | 0x8000,
                rng.gen::<u16>(),
                rng.gen::<u16>(),
                rng.gen::<u16>()
            ),
        }
    }
}

impl Default for Uuid {
    fn default() -> Self {
        Self::new()
    }
}

impl From<String> for Uuid {
    fn from(value: String) -> Self {
        Self { uuid: value }
    }
}

impl From<&str> for Uuid {
    fn from(value: &str) -> Self {
        Self { uuid: value.into() }
    }
}
