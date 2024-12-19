use zino::prelude::*;

mod schedule;

fn main() {
    zino::Cluster::boot().run_with(schedule::apalis_monitor())
}
