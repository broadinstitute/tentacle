use crate::config::JoinCovariancesConfig;
use crate::error::Error;
use crate::requests::CovariancesRequest;
use crate::http;
use crate::responses::{CovariancesData, Group};
use crate::find_common::find_common_sorted;
use crate::variant::Variant;
use fs_err as fs;
use std::io::BufWriter;
use std::io::Write;

pub(crate) async fn join_covariances(config: JoinCovariancesConfig)
                                     -> Result<(), Error> {
    let request1 =
        CovariancesRequest::new_single_group(config.region.clone(),
                                             config.genome_build.clone(),
                                             config.dataset1);
    let request2 =
        CovariancesRequest::new_single_group(config.region, config.genome_build,
                                             config.dataset2);
    let group1 =
        get_single_group(http::get_covariance_data(request1).await?)?;
    let group2 =
        get_single_group(http::get_covariance_data(request2).await?)?;
    let variants1 = parse_variants(&group1.variants)?;
    let variants2 = parse_variants(&group2.variants)?;
    let indices = find_common_sorted(&variants1, &variants2);
    let n_variants1 = variants1.len();
    let n_variants2 = variants2.len();
    if config.dry {
        println!("{} variants in dataset {}, {} in dataset {}, {} in common",
                 n_variants1, config.dataset1, n_variants2, config.dataset2, indices.len());
        Ok(())
    } else {
        let mut out = BufWriter::new(fs::File::create(config.out)?);
        writeln!(out, "variantX\tvariantY\tcov1\tcov2")?;
        for (x1, x2) in &indices {
            for (y1, y2) in &indices {
                let variant_x = group1.variants.get(*x1).unwrap();
                let variant_y = group1.variants.get(*y1).unwrap();
                let cov1 =
                    pick_half_matrix_element(&group1.covariance, n_variants1, *x1,
                                             *y1)?;
                let cov2 =
                    pick_half_matrix_element(&group2.covariance, n_variants2, *x2,
                                             *y2)?;
                writeln!(out, "{}\t{}\t{}\t{}", variant_x, variant_y, cov1, cov2)?;
            }
        }
        out.flush()?;
        Ok(())
    }
}

fn get_single_group(covariances_data: CovariancesData) -> Result<Group, Error> {
    let mut groups = covariances_data.groups;
    if groups.len() != 1 {
        Err(Error::from(
            format!("Expected exactly one group from data set {}, but got {} groups.",
                    covariances_data.summary_statistic_dataset, groups.len()))
        )
    } else {
        Ok(groups.remove(0))
    }
}

fn parse_variants(strings: &[String]) -> Result<Vec<Variant>, Error> {
    let mut variants = Vec::<Variant>::new();
    for string in strings {
        variants.push(Variant::parse(&*string)?);
    }
    Ok(variants)
}

fn pick_half_matrix_element(matrix: &[f64], n: usize, x: usize, y: usize) -> Result<f64, Error> {
    let n_elements = n * (n + 1) / 2;
    if matrix.len() != n_elements {
        return Err(Error::from(
            format!("For {} variants, need {} covariances, but got {}", n, n_elements,
                    matrix.len())
        ));
    }
    if x >= n {
        return Err(Error::from(format!("Index {} is out of bounds {}.", x, n)));
    }
    if y >= n {
        return Err(Error::from(format!("Index {} is out of bounds {}.", y, n)));
    }
    let (big, small) = if x > y { (x, y) } else { (y, x) };
    let index = (2 * n + 1 - small) * small / 2 + big - small;
    Ok(*matrix.get(index).ok_or_else(
        || {
            Error::from(format!("Index {} out of bounds for {}.", index, matrix.len()))
        }
    )?)
}