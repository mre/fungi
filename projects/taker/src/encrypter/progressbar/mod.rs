extern crate pbr;

use std::io::Stdout;
use self::pbr::ProgressBar;

pub fn simple(c: u64) -> ProgressBar<Stdout> {
    let mut pb = ProgressBar::new(c * 10);
    
    pb.tick_format("\\|/-");
    pb.format("|#--|");
    pb.show_tick = true;
    pb.show_speed = false;
    pb.show_percent = false;
    pb.show_counter = false;
    pb.show_time_left = false;
    pb.inc();
    // pb.finish_println("done!");
    return pb;
}
