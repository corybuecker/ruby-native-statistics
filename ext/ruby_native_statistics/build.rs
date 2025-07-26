use rb_sys_env;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    rb_sys_env::activate()?;
    Ok(())
}
