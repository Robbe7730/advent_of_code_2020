use std::fmt::Display;
use std::fs::File;
use std::io::Read;

pub trait Day {
    type Input;
    type Output1: Display;
    type Output2: Display;

    fn parse_input(&self, content: String) -> Self::Input;
    fn solve_part1(&self, input: &Self::Input) -> Self::Output1;
    fn solve_part2(&self, input: &Self::Input) -> Self::Output2;

    fn solve(&self, day_number: usize) {
        let test_input_unparsed = self.read_test_input(day_number);
        let test_input = self.parse_input(test_input_unparsed);
        let input_unparsed = self.read_input(day_number);
        let input = self.parse_input(input_unparsed);

        println!("---TEST INPUT---");
        println!("Part 1: {}", self.solve_part1(&test_input));
        println!("Part 2: {}", self.solve_part2(&test_input));

        println!("---FULL INPUT---");
        println!("Part 1: {}", self.solve_part1(&input));
        println!("Part 2: {}", self.solve_part2(&input));
    }

    fn read_input(&self, day_number: usize) -> String {
        let mut ret = String::new();
        let mut f = File::open(format!("input/day{}", day_number)).expect("Could not open file");
        f.read_to_string(&mut ret).expect("Could not parse file");
        ret
    }

    fn read_test_input(&self, day_number: usize) -> String {
        let mut ret = String::new();
        let mut f = File::open(format!("input/day{}_test", day_number)).expect("Could not open file");
        f.read_to_string(&mut ret).expect("Could not parse file");
        ret
    }
}
