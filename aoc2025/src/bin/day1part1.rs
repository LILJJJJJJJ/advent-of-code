use aoc::read_file_by_line_to_vec;

fn main() {
    let input_steps: Vec<String> = read_file_by_line_to_vec("inputs/day1.txt");

    let mut password_count: u16 = 0;
    let mut current_degree: i16 = 50;

    for step in input_steps {
        println!(" step: {step}");
        let action: i16 = action_parser(step) % 100;

        println!(" action: {action}");
        let result_degree: i16 = current_degree + action;

        if action.is_positive() && result_degree > 99 {
            current_degree = result_degree - 100;
        } else if action.is_negative() && result_degree < 0 {
            current_degree = 100 + result_degree;
        } else {
            current_degree = result_degree
        }
        if current_degree.is_negative() {
            panic!("current_degree shouldnt be negative here!");
        }

        if result_degree == 0 || result_degree == 100 {
            password_count += 1;
        }
    }

    println!("{password_count}");
}

fn action_parser(action_step: String) -> i16 {
    let direction: &str = &action_step[0..1];
    let degree = &action_step[1..];
    let idegree: i16 = degree
        .parse::<i16>()
        .expect("smt wrong with the number man");
    if direction == "L" {
        -1 * idegree
    } else {
        idegree
    }
}
