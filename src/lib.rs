use vbs::version::StaticVersion;

pub const MAJOR: u16 = 0;
pub const MINOR: u16 = 1;

pub type SequencerVersion = StaticVersion<MAJOR, MINOR>;

pub const SEQUENCER_VERSION: SequencerVersion = StaticVersion {};
