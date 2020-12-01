mod day;
mod days;

use structopt::StructOpt;

use days::run_day;
use days::run_all_days;

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(short, long, help = "Run all days until day_num")]
    all: bool,

    #[structopt(short, long, default_value = "0")]
    bench_num: usize,

    day_num: usize,
}

fn main() {
    let args = Cli::from_args();
    
    if args.all {
        run_all_days(args.day_num, args.bench_num);
    } else  {
        run_day(args.day_num, args.bench_num);
    }
}
