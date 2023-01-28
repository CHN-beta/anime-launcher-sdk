#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Resolution {
    // qHD; 960x540
    MiniHD,

    // 1280x720
    HD,

    // 1920x1080
    FullHD,

    // 2560x1440
    QuadHD,

    // 3840x2160
    UltraHD,

    Custom(u64, u64)
}

impl Resolution {
    pub fn list() -> Vec<Self> {
        vec![
            Self::MiniHD,
            Self::HD,
            Self::FullHD,
            Self::QuadHD,
            Self::UltraHD
        ]
    }

    pub fn from_pair(width: u64, height: u64) -> Self {
        for res in Self::list() {
            let pair = res.get_pair();

            if pair.0 == width && pair.1 == height {
                return res;
            }
        }

        Self::Custom(width, height)
    }

    pub fn get_pair(&self) -> (u64, u64) {
        match self {
            Self::MiniHD  => (960,  540),
            Self::HD      => (1280, 720),
            Self::FullHD  => (1920, 1080),
            Self::QuadHD  => (2560, 1440),
            Self::UltraHD => (3840, 2160),

            Self::Custom(w, h) => (*w, *h)
        }
    }
}

impl TryFrom<u32> for Resolution {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::MiniHD),
            1 => Ok(Self::HD),
            2 => Ok(Self::FullHD),
            3 => Ok(Self::QuadHD),
            4 => Ok(Self::UltraHD),

            5 => Ok(Self::Custom(0, 0)),

            _ => Err(String::from("Failed to convert number to Resolution enum"))
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<u32> for Resolution {
    fn into(self) -> u32 {
        match self {
            Self::MiniHD  => 0,
            Self::HD      => 1,
            Self::FullHD  => 2,
            Self::QuadHD  => 3,
            Self::UltraHD => 4,

            _ => 5 // Custom resolution
        }
    }
}

impl std::fmt::Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (w, h) = self.get_pair();

        f.write_str(&format!("{w}x{h}"))
    }
}
