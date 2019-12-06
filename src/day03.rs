use std::env;
use std::fs;

struct Direction
{
    x: isize,
    y: isize,
}

struct Point
{
    x: isize,
    y: isize,
}

fn data() -> String
{
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        fs::read_to_string(String::from(args[2].to_string())).expect("")
    }
    else
    {
        println!("USAGE: {} {} <filename-of-input>", args[0], args[1]);
        String::from("")
    }
}

fn get_direction(order: &str) -> Direction
{
    match order
    {
        "U" => Direction { x: 0, y: -1 },
        "D" => Direction { x: 0, y: 1 },
        "L" => Direction { x: -1, y: 0 },
        "R" => Direction { x: 1, y: 0 },
        _ => {
            println!("Error on move {:?}", order);
            Direction { x: 0, y: 0 }
        },
    }
}

fn get_steps(stepstring: &str) -> isize
{
    match isize::from_str_radix(stepstring, 10)
    {
        Ok(v) => v,
        Err(_) => {
            println!("Error on move {:?}", stepstring);
            0
        },
    }
}

pub fn print_result()
{
    let datastring = data();
    if datastring.len() > 0
    {
        let splitdata: Vec<&str> = datastring.split("\n").collect();
        if splitdata.len() == 2
        {
            let path_a: Vec<&str> = splitdata[0].split(",").collect();
            let path_b: Vec<&str> = splitdata[1].split(",").collect();

            let mut crosswire = Point { x: 0, y: 0 };

            let mut position_a = Point { x: 0, y: 0 };
            for step_a_str in path_a.clone()
            {
                let step_a = step_a_str.split_at(1);
                let direction_a = get_direction(step_a.0);

                let mut steps_to_go_a = get_steps(step_a.1);

                while (direction_a.x + direction_a.y) != 0 && steps_to_go_a > 0
                {
                    position_a.x += direction_a.x;
                    position_a.y += direction_a.y;
                    steps_to_go_a -= 1;

                    if position_a.x == 0 && position_a.y == 0
                    {
                        continue;
                    }

                    let mut position_b = Point { x: 0, y: 0 };
                    for step_b_str in path_b.clone()
                    {
                        let step_b = step_b_str.split_at(1);
                        let direction_b = get_direction(step_b.0);

                        let mut steps_to_go_b = get_steps(step_b.1);

                        while (direction_b.x + direction_b.y) != 0 && steps_to_go_b > 0
                        {
                            position_b.x += direction_b.x;
                            position_b.y += direction_b.y;
                            steps_to_go_b -= 1;

                            if position_b.x == 0 && position_b.y == 0
                            {
                                continue;
                            }

                            if position_a.x == position_b.x && position_a.y == position_b.y
                            {
                                let posdis = position_b.x.abs() + position_b.y.abs();
                                let lastdis = crosswire.x.abs() + crosswire.y.abs();
                                if posdis < lastdis || lastdis == 0
                                {
                                    println!("{} {} distance {}", position_b.x, position_b.y, posdis);
                                    crosswire = Point { x: position_b.x, y: position_b.y };
                                }
                            }
                        }
                    }
                }
            }
        }
        else
        {
            println!("Data must be 2 Lines");
        }
    }
}
