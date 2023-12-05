use tracing::Level;
use tracing_subscriber::EnvFilter;

pub fn init(verbosity: u8) {
    let level = match verbosity {
        0 => Level::INFO,
        1 => Level::DEBUG,
        2 | 3 => Level::TRACE,
        _ => Level::INFO,
    };

    let filter = EnvFilter::from_default_env()
        .add_directive(level.into())
        .add_directive("tokio_util=off".parse().unwrap())
        .add_directive("hyper=info".parse().unwrap())
        .add_directive("warp=info".parse().unwrap())
        .add_directive("warp=warn".parse().unwrap())
        .add_directive("api".parse().unwrap())
        .add_directive("zmq=off".parse().unwrap())
        .add_directive("mio=off".parse().unwrap())
        .add_directive("h2=off".parse().unwrap());
    tracing_subscriber::fmt().with_env_filter(filter).init();
}
