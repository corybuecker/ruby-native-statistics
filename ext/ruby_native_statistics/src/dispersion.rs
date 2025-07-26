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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_mean() {
        assert_eq!(calculate_mean(&[1.0, 2.0, 3.0, 4.0, 5.0]), 3.0);
        assert_eq!(calculate_mean(&[10.0]), 10.0);
        assert_eq!(calculate_mean(&[1.5, 2.5]), 2.0);
        assert_eq!(calculate_mean(&[-1.0, 0.0, 1.0]), 0.0);
        assert_eq!(calculate_mean(&[2.5, 7.5, 15.0, 5.0]), 7.5);
    }

    #[test]
    fn test_distance_from_mean() {
        // For [1, 2, 3], mean = 2, distances = [1, 0, 1], sum of squares = 2
        assert_eq!(distance_from_mean(&[1.0, 2.0, 3.0]), 2.0);

        // For single element, distance should be 0
        assert_eq!(distance_from_mean(&[5.0]), 0.0);

        // For [0, 0, 0], all distances are 0
        assert_eq!(distance_from_mean(&[0.0, 0.0, 0.0]), 0.0);

        // For [-2, 0, 2], mean = 0, distances = [4, 0, 4], sum = 8
        assert_eq!(distance_from_mean(&[-2.0, 0.0, 2.0]), 8.0);
    }

    #[test]
    fn test_calculate_variance_sample() {
        // Sample variance: divide by n-1
        let data = [1.0, 2.0, 3.0, 4.0, 5.0];
        let expected = 2.5; // distance_from_mean = 10, n-1 = 4, variance = 2.5
        assert_eq!(calculate_variance(&data, false), expected);

        // Two elements
        let data = [1.0, 3.0];
        let expected = 2.0; // distance_from_mean = 2, n-1 = 1, variance = 2.0
        assert_eq!(calculate_variance(&data, false), expected);

        // All same values
        let data = [5.0, 5.0, 5.0];
        assert_eq!(calculate_variance(&data, false), 0.0);
    }

    #[test]
    fn test_calculate_variance_population() {
        // Population variance: divide by n
        let data = [1.0, 2.0, 3.0, 4.0, 5.0];
        let expected = 2.0; // distance_from_mean = 10, n = 5, variance = 2.0
        assert_eq!(calculate_variance(&data, true), expected);

        // Single element
        let data = [10.0];
        assert_eq!(calculate_variance(&data, true), 0.0);

        // Two elements
        let data = [1.0, 3.0];
        let expected = 1.0; // distance_from_mean = 2, n = 2, variance = 1.0
        assert_eq!(calculate_variance(&data, true), expected);
    }

    #[test]
    fn test_calculate_stdev_sample() {
        // Sample standard deviation is sqrt of sample variance
        let data = [1.0, 2.0, 3.0, 4.0, 5.0];
        let expected = 2.5_f64.sqrt(); // sample variance = 2.5
        assert_eq!(calculate_stdev(&data, false), expected);

        // All same values
        let data = [7.0, 7.0, 7.0, 7.0];
        assert_eq!(calculate_stdev(&data, false), 0.0);
    }

    #[test]
    fn test_calculate_stdev_population() {
        // Population standard deviation is sqrt of population variance
        let data = [1.0, 2.0, 3.0, 4.0, 5.0];
        let expected = 2.0_f64.sqrt(); // population variance = 2.0
        assert_eq!(calculate_stdev(&data, true), expected);

        // Single element
        let data = [42.0];
        assert_eq!(calculate_stdev(&data, true), 0.0);
    }

    #[test]
    fn test_calculate_percentile_basic() {
        let mut data = [1.0, 2.0, 3.0, 4.0, 5.0];

        // 0th percentile (minimum)
        assert_eq!(calculate_percentile(&mut data, 0.0).unwrap(), 1.0);

        // 50th percentile (median)
        assert_eq!(calculate_percentile(&mut data, 0.5).unwrap(), 3.0);

        // 100th percentile (maximum)
        assert_eq!(calculate_percentile(&mut data, 1.0).unwrap(), 5.0);
    }

    #[test]
    fn test_calculate_percentile_interpolation() {
        let mut data = [1.0, 2.0, 3.0, 4.0];

        // 25th percentile: h = (4-1)*0.25 + 1 = 1.75
        // Interpolate between index 0 (value 1) and index 1 (value 2)
        // Result = 0.75 * (2-1) + 1 = 1.75
        assert_eq!(calculate_percentile(&mut data, 0.25).unwrap(), 1.75);

        // 75th percentile: h = (4-1)*0.75 + 1 = 3.25
        // Interpolate between index 2 (value 3) and index 3 (value 4)
        // Result = 0.25 * (4-3) + 3 = 3.25
        assert_eq!(calculate_percentile(&mut data, 0.75).unwrap(), 3.25);
    }

    #[test]
    fn test_calculate_percentile_single_element() {
        let mut data = [42.0];

        assert_eq!(calculate_percentile(&mut data, 0.0).unwrap(), 42.0);
        assert_eq!(calculate_percentile(&mut data, 0.5).unwrap(), 42.0);
        assert_eq!(calculate_percentile(&mut data, 1.0).unwrap(), 42.0);
    }

    #[test]
    fn test_calculate_percentile_unsorted_data() {
        let mut data = [5.0, 1.0, 3.0, 2.0, 4.0];

        // Should sort internally and return correct percentiles
        assert_eq!(calculate_percentile(&mut data, 0.0).unwrap(), 1.0);
        assert_eq!(calculate_percentile(&mut data, 0.5).unwrap(), 3.0);
        assert_eq!(calculate_percentile(&mut data, 1.0).unwrap(), 5.0);
    }

    #[test]
    fn test_calculate_percentile_with_duplicates() {
        let mut data = [1.0, 2.0, 2.0, 3.0, 4.0];

        assert_eq!(calculate_percentile(&mut data, 0.0).unwrap(), 1.0);
        assert_eq!(calculate_percentile(&mut data, 1.0).unwrap(), 4.0);

        // 50th percentile should handle duplicates correctly
        let result = calculate_percentile(&mut data, 0.5).unwrap();
        assert!(result >= 2.0 && result <= 3.0);
    }

    #[test]
    fn test_calculate_percentile_negative_numbers() {
        let mut data = [-5.0, -2.0, 0.0, 2.0, 5.0];

        assert_eq!(calculate_percentile(&mut data, 0.0).unwrap(), -5.0);
        assert_eq!(calculate_percentile(&mut data, 0.5).unwrap(), 0.0);
        assert_eq!(calculate_percentile(&mut data, 1.0).unwrap(), 5.0);
    }

    #[test]
    fn test_variance_and_stdev_consistency() {
        let data = [1.0, 2.0, 3.0, 4.0, 5.0];

        // Sample variance and stdev should be consistent
        let sample_var = calculate_variance(&data, false);
        let sample_stdev = calculate_stdev(&data, false);
        assert!((sample_stdev * sample_stdev - sample_var).abs() < 1e-14);

        // Population variance and stdev should be consistent
        let pop_var = calculate_variance(&data, true);
        let pop_stdev = calculate_stdev(&data, true);
        assert!((pop_stdev * pop_stdev - pop_var).abs() < 1e-14);
    }

    #[test]
    fn test_mathematical_properties() {
        let data = [2.0, 4.0, 6.0, 8.0, 10.0];
        let mean = calculate_mean(&data);

        // Mean should be the average
        assert_eq!(mean, 6.0);

        // Population variance should be less than sample variance (when n > 1)
        let pop_var = calculate_variance(&data, true);
        let sample_var = calculate_variance(&data, false);
        assert!(pop_var < sample_var);

        // Population stdev should be less than sample stdev
        let pop_stdev = calculate_stdev(&data, true);
        let sample_stdev = calculate_stdev(&data, false);
        assert!(pop_stdev < sample_stdev);
    }
}
