use magnus::{Error, Module, RArray, Ruby, method};

fn calculate_mean(array: &Vec<f64>) -> f64 {
    let length = array.len() as f64;
    let sum = array.iter().sum::<f64>();
    sum / length
}

fn distance_from_mean(array: &Vec<f64>) -> f64 {
    let mean = calculate_mean(array);

    array
        .into_iter()
        .fold(0.0, |acc, x| acc + (x - mean).powi(2))
}

fn mean(rb_self: RArray) -> Result<f64, Error> {
    let array = rb_self.to_vec::<f64>()?;
    let length = array.len() as f64;

    if length == 0.0 {
        return Err(Error::new(
            magnus::exception::range_error(),
            "array must have at least one element",
        ));
    }

    Ok(calculate_mean(&array))
}

fn median(rb_self: RArray) -> Result<f64, Error> {
    let array = rb_self.to_vec::<f64>()?;
    let length = array.len();

    if length == 0 {
        return Err(Error::new(
            magnus::exception::range_error(),
            "array must have at least one element",
        ));
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

fn var(rb_self: RArray) -> Result<f64, Error> {
    let array = rb_self.to_vec::<f64>()?;
    let length = array.len() as f64;

    if length == 0.0 {
        return Err(Error::new(
            magnus::exception::range_error(),
            "Cannot perform operation on empty array",
        ));
    }

    let distance_from_mean = distance_from_mean(&array);
    let average_distance = distance_from_mean / (length - 1.0);

    Ok(average_distance)
}

fn stdev(rb_self: RArray) -> Result<f64, Error> {
    let array = rb_self.to_vec::<f64>()?;
    let length = array.len() as f64;

    if length == 0.0 {
        return Err(Error::new(
            magnus::exception::range_error(),
            "Cannot perform operation on empty array",
        ));
    }

    let distance_from_mean = distance_from_mean(&array);
    let average_distance = distance_from_mean / (length - 1.0);

    Ok(average_distance.sqrt())
}

fn varp(rb_self: RArray) -> Result<f64, Error> {
    let array = rb_self.to_vec::<f64>()?;
    let length = array.len() as f64;

    if length == 0.0 {
        return Err(Error::new(
            magnus::exception::range_error(),
            "Cannot perform operation on empty array",
        ));
    }

    let distance_from_mean = distance_from_mean(&array);
    let average_distance = distance_from_mean / (length);

    Ok(average_distance)
}

fn stdevp(rb_self: RArray) -> Result<f64, Error> {
    let array = rb_self.to_vec::<f64>()?;
    let length = array.len() as f64;

    if length == 0.0 {
        return Err(Error::new(
            magnus::exception::range_error(),
            "Cannot perform operation on empty array",
        ));
    }

    let distance_from_mean = distance_from_mean(&array);
    let average_distance = distance_from_mean / length;

    Ok(average_distance.sqrt())
}

fn percentile(rb_self: RArray, percentile: f64) -> Result<f64, Error> {
    let mut array = rb_self.to_vec::<f64>()?;
    let length = array.len() as f64;

    if length == 0.0 {
        return Err(Error::new(
            magnus::exception::range_error(),
            "Cannot perform operation on empty array",
        ));
    }

    if percentile > 1.0 || percentile < 0.0 {
        return Err(Error::new(
            magnus::exception::range_error(),
            "Percentile must be between 0 and 1",
        ));
    }

    array.sort_by(|a, b| a.total_cmp(b));

    let h = (length - 1.0) * percentile + 1.0;
    if h.trunc() == h {
        Ok(array[(h as usize) - 1])
    } else {
        let h_floor = h.trunc() as usize;

        Ok((h - h_floor as f64) * (array[h_floor] - array[h_floor - 1]) + array[h_floor - 1])
    }
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let array = ruby.class_array();

    array.define_method("mean", method!(mean, 0))?;
    array.define_method("median", method!(median, 0))?;
    array.define_method("stdev", method!(stdev, 0))?;
    array.define_method("stdevs", method!(stdev, 0))?;
    array.define_method("stdevp", method!(stdevp, 0))?;
    array.define_method("var", method!(var, 0))?;
    array.define_method("varp", method!(varp, 0))?;
    array.define_method("percentile", method!(percentile, 1))?;

    Ok(())
}
