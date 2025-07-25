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

    let length = array.len();

    if array.is_empty() {
        return Err(MathematicsError::EmptyArray);
    }

    let mut sorted_array = array;
    sorted_array.sort_by(|a, b| a.total_cmp(b));

    let array_even_size = (length % 2) == 0;
    let middle = length / 2;

    if !array_even_size {
        Ok(sorted_array[middle])
    } else {
        Ok((sorted_array[middle - 1] + sorted_array[middle]) / 2.0)
    }
}

fn calculate_mean(array: &[f64]) -> f64 {
    let length = array.len() as f64;
    let sum = array.iter().sum::<f64>();
    sum / length
}
