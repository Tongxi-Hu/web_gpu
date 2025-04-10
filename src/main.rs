use app::run;
use pollster::block_on;

mod app;
mod configuration;
mod constant;
mod math;
mod state;
mod util;

fn main() {
    block_on(run());
}
