pub struct SpeciesName {
    pub code: Code,
    pub genus: String,
    pub species: String,
    pub subspecies: String,
}

// See https://en.wikipedia.org/wiki/Nomenclature_codes
pub enum Code {
    ICN,
    ICZN,
    ICNP,
    ICVCN,
}