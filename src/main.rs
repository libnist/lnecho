use std::env;
fn main() {
    let config = lnecho::Config::from(env::args());

    lnecho::run(config);
}
