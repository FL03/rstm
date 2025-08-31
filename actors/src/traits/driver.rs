/*
    Appellation: driver <module>
    Created At: 2025.08.31:07:54:48
    Contrib: @FL03
*/

/// The [`RawDriver`] trait establishes the base interface for all compatible drivers within
/// the system. Each driver is expected to provide its own internal storage, however, the
/// remainder of the context must be _injected_ externally by the associated engine.
///
/// The internal storage of each driver is a specialized _tape_ that allows the driver to
/// manage the interactions between the head and the tape itself.
pub trait RawDriver {
    type Store;

    private! {}
}

/// A [`Driver`] defines an isolated computational entity that operates using a defined set of
/// rules and
pub trait Driver: RawDriver {}
