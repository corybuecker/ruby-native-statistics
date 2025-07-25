use magnus::{Error, RArray, Ruby};

#[derive(thiserror::Error, Debug)]
pub enum DispersionError {
    #[error("Array must have at least one element")]
    EmptyArray,

    #[error("Input is out of range")]
    RangeError,

    #[error("Magnus error")]
    MagnusError(magnus::Error),
}

impl magnus::error::IntoError for DispersionError {
    fn into_error(self, ruby: &Ruby) -> Error {
        match self {
            DispersionError::EmptyArray => {
                Error::new(ruby.exception_range_error(), self.to_string())
            }
            DispersionError::RangeError => {
                Error::new(ruby.exception_range_error(), self.to_string())
            }
            DispersionError::MagnusError(err) => err,
        }
    }
}

fn calculate_mean(array: &[f64]) -> f64 {
    let length = array.len() as f64;
    let sum = array.iter().sum::<f64>();
    sum / length
}

fn calculate_variance(array: &[f64], population: bool) -> f64 {
    let length = array.len() as f64;
    let distance_from_mean = distance_from_mean(array);
    let divisor = if population { length } else { length - 1.0 };
    distance_from_mean / divisor
}

fn calculate_stdev(array: &[f64], population: bool) -> f64 {
    calculate_variance(array, population).sqrt()
}

fn distance_from_mean(array: &[f64]) -> f64 {
    let mean = calculate_mean(array);

    array.iter().fold(0.0, |acc, x| acc + (x - mean).powi(2))
}

pub fn var(rb_self: RArray) -> Result<f64, DispersionError> {
    let array = rb_self
        .to_vec::<f64>()
        .map_err(|e| DispersionError::MagnusError(e))?;

    if array.is_empty() {
        return Err(DispersionError::EmptyArray);
    }

    Ok(calculate_variance(&array, false))
}

pub fn stdev(rb_self: RArray) -> Result<f64, DispersionError> {
    let array = rb_self
        .to_vec::<f64>()
        .map_err(|e| DispersionError::MagnusError(e))?;

    if array.is_empty() {
        return Err(DispersionError::EmptyArray);
    }

    Ok(calculate_stdev(&array, false))
}

pub fn varp(rb_self: RArray) -> Result<f64, DispersionError> {
    let array = rb_self
        .to_vec::<f64>()
        .map_err(|e| DispersionError::MagnusError(e))?;

    if array.is_empty() {
        return Err(DispersionError::EmptyArray);
    }

    Ok(calculate_variance(&array, true))
}

pub fn stdevp(rb_self: RArray) -> Result<f64, DispersionError> {
    let array = rb_self
        .to_vec::<f64>()
        .map_err(|e| DispersionError::MagnusError(e))?;

    if array.is_empty() {
        return Err(DispersionError::EmptyArray);
    }

    Ok(calculate_stdev(&array, true))
}

fn calculate_percentile(array: &mut [f64], percentile: f64) -> Result<f64, DispersionError> {
    let length = array.len() as f64;

    array.sort_by(|a, b| a.total_cmp(b));

    let h = (length - 1.0) * percentile + 1.0;
    if h.trunc() == h {
        Ok(array[(h as usize) - 1])
    } else {
        let h_floor = h.trunc() as usize;

        Ok((h - h_floor as f64) * (array[h_floor] - array[h_floor - 1]) + array[h_floor - 1])
    }
}
pub fn percentile(rb_self: RArray, percentile: f64) -> Result<f64, DispersionError> {
    let mut array = rb_self
        .to_vec::<f64>()
        .map_err(|e| DispersionError::MagnusError(e))?;

    if array.is_empty() {
        return Err(DispersionError::EmptyArray);
    }

    if !(0.0..=1.0).contains(&percentile) {
        return Err(DispersionError::RangeError);
    }

    calculate_percentile(&mut array, percentile)
}
