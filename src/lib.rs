use versioned_binary_serialization::version::StaticVersion;

pub const MAJOR: u16 = 0;
pub const MINOR: u16 = 1;

pub const SEQUENCER_VERSION: StaticVersion<MAJOR, MINOR> = StaticVersion;
