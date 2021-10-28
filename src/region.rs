use crate::error::Error;

pub(crate) struct Region {
    pub(crate) chrom: String,
    pub(crate) start: usize,
    pub(crate) stop: usize,
}

fn cannot_parse_error(region: &str) -> Error {
    Error::from(format!("Cannot parse '{}' as a region. Need format 1:1000-2000.", region))
}

impl Region {
    pub(crate) fn parse(string: &str) -> Result<Region, Error> {
        let mut iter1 = string.split(':');
        let chrom =
            String::from(iter1.next().ok_or_else(|| cannot_parse_error(string))?);
        let interval = iter1.next().ok_or_else(|| cannot_parse_error(string))?;
        let mut iter2 = interval.split('-');
        let start =
            str::parse::<usize>(
                iter2.next().ok_or_else(|| cannot_parse_error(string))?
            )?;
        let stop =
            str::parse::<usize>(
                iter2.next().ok_or_else(|| cannot_parse_error(string))?
            )?;
        Ok(Region { chrom, start, stop })
    }
}