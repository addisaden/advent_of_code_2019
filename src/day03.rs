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

            for step_a_str in path_a.clone()
            {
                let step_a = step_a_str.split_at(1);
                let mut direction_a = Direction { x: 0, y: 0 };
                let mut position_a = Point { x: 0, y: 0 };
                let mut steps_to_go = 0;
                match isize::from_str_radix(step_a.1, 10)
                {
                    Ok(v) => steps_to_go = v,
                    Err(_) => continue,
                }

                match step_a.0
                {
                    "U" => direction_a = Direction { x: 0, y: -1 },
                    "D" => direction_a = Direction { x: 0, y: 1 },
                    "L" => direction_a = Direction { x: -1, y: 0 },
                    "R" => direction_a = Direction { x: 1, y: 0 },
                    _ => println!("Error on move {:?} on step_a", step_a),
                }

                for step_b_str in path_b.clone()
                {
                    let step_b = step_b_str.split_at(1);
                    let mut direction_b = Direction { x: 0, y: 0};
                    let mut position_b = Point { x: 0, y: 0 };

                    match step_b.0
                    {
                        "U" => direction_b = Direction { x: 0, y: -1},
                        "D" => direction_b = Direction { x: 0, y: 1},
                        "L" => direction_b = Direction { x: -1, y: 0},
                        "R" => direction_b = Direction { x: 1, y: 0},
                        _ => println!("Error on move {:?} on step_b", step_b),
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
