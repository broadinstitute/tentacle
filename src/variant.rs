use crate::error::Error;
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct Variant {
    chrom: String,
    pos: usize,
    ref_allele: String,
    alt_allele: String,
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct VariantPair {
    first: Variant,
    second: Variant,
}

impl Variant {
    pub(crate) fn new(chrom: String, pos: usize, ref_allele: String, alt_allele: String)
                      -> Variant {
        Variant { chrom, pos, ref_allele, alt_allele }
    }
    fn new_parse_error(string: &str) -> Error {
        Error::from(format!("Could not parse {} as a variant id.", string))
    }
    pub(crate) fn parse(string: &str) -> Result<Variant, Error> {
        let mut parts = string.split(&['_', '/', ':'][..]);
        let chrom =
            String::from(parts.next().ok_or_else(|| Variant::new_parse_error(string))?);
        let pos =
            str::parse::<usize>(parts.next().ok_or_else(|| Variant::new_parse_error(string))?)?;
        let ref_allele =
            String::from(parts.next().ok_or_else(|| Variant::new_parse_error(string))?);
        let alt_allele =
            String::from(parts.next().ok_or_else(|| Variant::new_parse_error(string))?);
        Ok(Variant::new(chrom, pos, ref_allele, alt_allele))
    }
}

impl VariantPair {
    pub(crate) fn new(variant1: Variant, variant2: Variant) -> VariantPair {
        let (first, second) =
            if variant1 <= variant2 {
                (variant1, variant2)
            } else {
                (variant2, variant1)
            };
        VariantPair { first, second }
    }
}