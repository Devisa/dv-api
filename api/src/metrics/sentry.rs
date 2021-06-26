use sentry::*;
use tracing::{Level, span, Span, };
use sentry_actix::*;


pub fn verify() {
    sentry_opts();
    panic!("Panicking to verify!")
}

// TODO: Use automatic dev/prod designation
pub fn sentry_opts() -> sentry::ClientInitGuard {
    let span = span!(Level::TRACE, "my span");

    let _guard = sentry::init((
        "https://61cdc23d523c49a683f71a9c5ae01a6b@o558281.ingest.sentry.io/5827382",
        sentry::ClientOptions {
            environment: Some("dev".into()),
            release: sentry::release_name!(),
            debug: true,
            ..Default::default()
        }
    ));
    _guard
}

pub fn capture()  {

}
