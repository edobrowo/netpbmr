use crate::pbm::{PbmPlain, PbmRaw};
use crate::pgm::{PgmPlain, PgmRaw};
use crate::ppm::{PpmPlain, PpmRaw};

/// PNM (Portable Any Format) generalizes the
/// PBM, PGM, and PPM formats.
pub enum PnmFormat {
    PbmPlain(PbmPlain),
    PgmPlain(PgmPlain),
    PpmPlain(PpmPlain),
    PbmRaw(PbmRaw),
    PgmRaw(PgmRaw),
    PpmRaw(PpmRaw),
}
