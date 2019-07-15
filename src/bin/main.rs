use boilerplate::config::ApplicationConfig;
use boilerplate::bootstrap::ApplicationBootstrap;

fn main() -> std::io::Result<()> {
    let config = ApplicationConfig::new().unwrap();

    ApplicationBootstrap::start(config)
}