use rs_wgpu_engine::run;
use futures::executor::block_on;

fn main() {
    block_on(run());
}
