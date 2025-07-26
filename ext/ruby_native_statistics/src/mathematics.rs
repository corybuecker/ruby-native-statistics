use magnus::{Error, RArray, Ruby};

#[derive(thiserror::Error, Debug)]
pub enum MathematicsError {
    #[error("Array must have at least one element")]
    EmptyArray,

    #[error("Magnus error")]
    MagnusError(magnus::Error),
}

impl magnus::error::IntoError for MathematicsError {
    fn into_error(self, ruby: &Ruby) -> Error {
        match self {
            MathematicsError::EmptyArray => {
                Error::new(ruby.exception_range_error(), self.to_string())
            }
            MathematicsError::MagnusError(err) => err,
        }
    }
}

pub fn calculate_mean(array: &[f64]) -> f64 {
    let length = array.len() as f64;
    let sum = array.iter().sum::<f64>();
    sum / length
}

pub fn calculate_median(array: &[f64]) -> Result<f64, MathematicsError> {
    if array.is_empty() {
        return Err(MathematicsError::EmptyArray);
    }

    let mut sorted_array = array.to_vec();
    sorted_array.sort_by(|a, b| a.total_cmp(b));

    let length = sorted_array.len();
    let array_even_size = (length % 2) == 0;
    let middle = length / 2;

    if !array_even_size {
        Ok(sorted_array[middle])
    } else {
        Ok((sorted_array[middle - 1] + sorted_array[middle]) / 2.0)
    }
}

pub fn mean(rb_self: RArray) -> Result<f64, MathematicsError> {
    let array = rb_self
        .to_vec::<f64>()
        .map_err(|e| MathematicsError::MagnusError(e))?;

    if array.is_empty() {
        return Err(MathematicsError::EmptyArray);
    }

    Ok(calculate_mean(&array))
}

pub fn median(rb_self: RArray) -> Result<f64, MathematicsError> {
    let array = rb_self
        .to_vec::<f64>()
        .map_err(|e| MathematicsError::MagnusError(e))?;

    calculate_median(&array)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_mean_single_element() {
        assert_eq!(calculate_mean(&[5.0]), 5.0);
    }

    #[test]
    fn test_calculate_mean_multiple_elements() {
        assert_eq!(calculate_mean(&[1.0, 2.0, 3.0, 4.0, 5.0]), 3.0);
    }

    #[test]
    fn test_calculate_mean_with_negative_numbers() {
        assert_eq!(calculate_mean(&[-2.0, -1.0, 0.0, 1.0, 2.0]), 0.0);
    }

    #[test]
    fn test_calculate_mean_with_decimals() {
        let result = calculate_mean(&[1.5, 2.5, 3.5]);
        assert!((result - 2.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_calculate_mean_large_numbers() {
        let result = calculate_mean(&[1000000.0, 2000000.0, 3000000.0]);
        assert_eq!(result, 2000000.0);
    }

    #[test]
    fn test_calculate_median_empty_array() {
        let result = calculate_median(&[]);
        assert!(matches!(result, Err(MathematicsError::EmptyArray)));
    }

    #[test]
    fn test_calculate_median_single_element() {
        assert_eq!(calculate_median(&[42.0]).unwrap(), 42.0);
    }

    #[test]
    fn test_calculate_median_odd_length_sorted() {
        assert_eq!(calculate_median(&[1.0, 2.0, 3.0]).unwrap(), 2.0);
    }

    #[test]
    fn test_calculate_median_odd_length_unsorted() {
        assert_eq!(calculate_median(&[3.0, 1.0, 2.0]).unwrap(), 2.0);
    }

    #[test]
    fn test_calculate_median_even_length_sorted() {
        assert_eq!(calculate_median(&[1.0, 2.0, 3.0, 4.0]).unwrap(), 2.5);
    }

    #[test]
    fn test_calculate_median_even_length_unsorted() {
        assert_eq!(calculate_median(&[4.0, 1.0, 3.0, 2.0]).unwrap(), 2.5);
    }

    #[test]
    fn test_calculate_median_with_duplicates() {
        assert_eq!(calculate_median(&[1.0, 2.0, 2.0, 3.0]).unwrap(), 2.0);
        assert_eq!(calculate_median(&[1.0, 2.0, 2.0, 2.0, 3.0]).unwrap(), 2.0);
    }

    #[test]
    fn test_calculate_median_with_negative_numbers() {
        assert_eq!(calculate_median(&[-3.0, -1.0, 0.0, 1.0, 3.0]).unwrap(), 0.0);
        assert_eq!(calculate_median(&[-4.0, -2.0, -1.0, 1.0]).unwrap(), -1.5);
    }

    #[test]
    fn test_calculate_median_with_decimals() {
        let result = calculate_median(&[1.1, 2.2, 3.3]).unwrap();
        assert!((result - 2.2).abs() < f64::EPSILON);
    }

    #[test]
    fn test_calculate_median_preserves_original_array() {
        let original = vec![3.0, 1.0, 4.0, 2.0];
        let original_copy = original.clone();
        let _result = calculate_median(&original).unwrap();
        assert_eq!(original, original_copy);
    }

    #[test]
    fn test_calculate_median_with_infinity() {
        assert_eq!(
            calculate_median(&[f64::NEG_INFINITY, 0.0, f64::INFINITY]).unwrap(),
            0.0
        );
    }

    #[test]
    fn test_calculate_median_with_nan() {
        // NaN values should be handled by total_cmp, but behavior may be implementation-defined
        let result = calculate_median(&[1.0, f64::NAN, 3.0]);
        // We can't easily test the exact result with NaN, but it shouldn't panic
        assert!(result.is_ok());
    }

    #[test]
    fn test_mathematics_error_display() {
        let empty_error = MathematicsError::EmptyArray;
        assert_eq!(
            empty_error.to_string(),
            "Array must have at least one element"
        );
    }

    #[test]
    fn test_mathematics_error_debug() {
        let empty_error = MathematicsError::EmptyArray;
        let debug_string = format!("{:?}", empty_error);
        assert!(debug_string.contains("EmptyArray"));
    }

    // Integration-style tests for the core functions
    #[test]
    fn test_mean_and_median_consistency_single_element() {
        let data = [7.5];
        let mean_result = calculate_mean(&data);
        let median_result = calculate_median(&data).unwrap();
        assert_eq!(mean_result, median_result);
    }

    #[test]
    fn test_mean_and_median_symmetric_distribution() {
        let data = [1.0, 2.0, 3.0, 4.0, 5.0];
        let mean_result = calculate_mean(&data);
        let median_result = calculate_median(&data).unwrap();
        assert_eq!(mean_result, median_result);
    }

    #[test]
    fn test_large_dataset_performance() {
        let large_data: Vec<f64> = (1..=10000).map(|x| x as f64).collect();

        let mean_result = calculate_mean(&large_data);
        assert_eq!(mean_result, 5000.5);

        let median_result = calculate_median(&large_data).unwrap();
        assert_eq!(median_result, 5000.5);
    }

    #[test]
    fn test_edge_case_very_small_numbers() {
        let data = [
            f64::MIN_POSITIVE,
            f64::MIN_POSITIVE * 2.0,
            f64::MIN_POSITIVE * 3.0,
        ];
        let mean_result = calculate_mean(&data);
        let median_result = calculate_median(&data).unwrap();

        assert!(mean_result > 0.0);
        assert!(median_result > 0.0);
    }

    #[test]
    fn test_edge_case_very_large_numbers() {
        let large_value = f64::MAX / 10.0;
        let data = [large_value, large_value, large_value];
        let mean_result = calculate_mean(&data);
        let median_result = calculate_median(&data).unwrap();

        assert!(!mean_result.is_infinite());
        assert!(!median_result.is_infinite());
        assert_eq!(mean_result, median_result);
        assert_eq!(mean_result, large_value);
    }

    #[test]
    fn test_precision_with_many_small_values() {
        let data: Vec<f64> = vec![0.1; 1000];

        let median_result = calculate_median(&data).unwrap();
        assert!((median_result - 0.1).abs() < f64::EPSILON);

        let mean_result = calculate_mean(&data);
        // Use a reasonable tolerance for accumulated floating point errors
        let tolerance = 1e-10;
        assert!((mean_result - 0.1).abs() < tolerance);
    }
}
