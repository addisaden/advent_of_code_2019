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
                let direction_a = match step_a.0
                {
                    "U" => Direction { x: 0, y: -1 },
                    "D" => Direction { x: 0, y: 1 },
                    "L" => Direction { x: -1, y: 0 },
                    "R" => Direction { x: 1, y: 0 },
                    _ => {
                        println!("Error on move {:?} on step_a", step_a);
                        Direction { x: 0, y: 0 }
                    },
                };

                let mut position_a = Point { x: 0, y: 0 };
                let mut steps_to_go_a = match isize::from_str_radix(step_a.1, 10)
                {
                    Ok(v) => v,
                    Err(_) => {
                        println!("Error on move {:?} on step_a", step_a);
                        0
                    },
                };

                for step_b_str in path_b.clone()
                {
                    let step_b = step_b_str.split_at(1);
                    let direction_b = match step_b.0
                    {
                        "U" => Direction { x: 0, y: -1 },
                        "D" => Direction { x: 0, y: 1 },
                        "L" => Direction { x: -1, y: 0 },
                        "R" => Direction { x: 1, y: 0 },
                        _ => {
                            println!("Error on move {:?} on step_b", step_b);
                            Direction { x: 0, y: 0 }
                        },
                    };

                    let mut position_b = Point { x: 0, y: 0 };
                    let mut steps_to_go_b = match isize::from_str_radix(step_b.1, 10)
                    {
                        Ok(v) => v,
                        Err(_) => {
                            println!("Error on move {:?} on step_b", step_b);
                            0
                        },
                    };
                }
            }
        }
        else
        {
            println!("Data must be 2 Lines");
        }
    }
}
