#![feature(iterator_fold_self)]
#![feature(linked_list_remove)]

mod day;
mod days;

use std::path::PathBuf;
use structopt::StructOpt;

use days::run_all_days;
use days::run_day;

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(short, long, help = "Run all days until day_num")]
    all: bool,

    #[structopt(short, long, help = "Time the solving")]
    timed: bool,

    #[structopt(short, long, default_value = "0")]
    bench_num: usize,

    #[structopt(help = "The day to run")]
    day_num: usize,

    #[structopt(help = "Specify which part to run (leave blank for all)")]
    part_num: Option<usize>,

    #[structopt(help = "Specify which input to use")]
    inputfile: Option<PathBuf>,
}

fn main() {
    let args = Cli::from_args();

    if args.all {
        run_all_days(args.day_num, args.bench_num, args.timed);
    } else {
        run_day(
            args.day_num,
            args.bench_num,
            args.part_num,
            args.inputfile,
            args.timed,
        );
    }
}
