use app::run;
use pollster::block_on;

mod app;
mod state;

fn main() {
    block_on(run())
}
