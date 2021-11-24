use crate::error::Error;

#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct Variant {
    chrom: String,
    pos: usize,
    ref_allele: String,
    alt_allele: String,
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
