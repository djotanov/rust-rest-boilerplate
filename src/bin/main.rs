use {{project-name}}::config::ApplicationConfig;
use {{project-name}}::bootstrap::ApplicationBootstrap;

fn main() -> std::io::Result<()> {
    let config = ApplicationConfig::new().unwrap();

    ApplicationBootstrap::start(config)
}