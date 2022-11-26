use std::fs;

fn main() {
    let input = fs::read_to_string("/home/dragut/Projects/advent-of-code-2015/day2/input.txt")
        .expect("File could not be read");

    let lines = input.lines();

    let mut total = 0;

    for line in lines {
        let side_dimensions = line
            .split('x')
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        fn calculate_areas(dimensions: Vec<i32>) -> Vec<i32> {
            let multiple_all: i32 = dimensions.iter().product();

            dimensions.iter().map(|d| multiple_all / d).collect()
        }

        let dimensions = calculate_areas(side_dimensions);

        let minimum = dimensions.iter().min().unwrap();

        let area_sum = dimensions.iter().fold(0, |mut acc, area| {
            acc += 2 * area;
            acc
        });

        total += area_sum + minimum;
    }
    println!("{}", total);
}
