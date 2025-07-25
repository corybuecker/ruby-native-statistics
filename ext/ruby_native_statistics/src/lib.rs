use magnus::{Error, Module, Ruby, method};

mod dispersion;
mod mathematics;

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let array = ruby.class_array();

    array.define_method("mean", method!(mathematics::mean, 0))?;
    array.define_method("median", method!(mathematics::median, 0))?;
    array.define_method("stdev", method!(dispersion::stdev, 0))?;
    array.define_method("stdevs", method!(dispersion::stdev, 0))?;
    array.define_method("stdevp", method!(dispersion::stdevp, 0))?;
    array.define_method("var", method!(dispersion::var, 0))?;
    array.define_method("varp", method!(dispersion::varp, 0))?;
    array.define_method("percentile", method!(dispersion::percentile, 1))?;

    Ok(())
}
