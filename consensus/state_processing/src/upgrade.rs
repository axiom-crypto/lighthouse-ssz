pub mod altair;
pub mod capella;
pub mod eip4844;
pub mod eip6110;
pub mod merge;

pub use altair::upgrade_to_altair;
pub use capella::upgrade_to_capella;
pub use eip4844::upgrade_to_eip4844;
pub use eip6110::upgrade_to_eip6110;
pub use merge::upgrade_to_bellatrix;
