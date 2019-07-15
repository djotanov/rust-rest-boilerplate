pub fn init_logging() -> Result<(), log4rs::Error> {

    log4rs::init_file("config/log4rs.yaml", Default::default())?;

    Ok(())
}