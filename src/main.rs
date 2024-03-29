use std::env;
mod day01;
mod day02;
mod day03;
mod day04;

fn advent_of_code_title() {
    println!("     ________   ______   __   __   ______   ___   __    _________  ");
    println!("    /_______/\\ /_____/\\ /_/\\ /_/\\ /_____/\\ /__/\\ /__/\\ /________/\\ ");
    println!("    \\::: _  \\ \\\\:::_ \\ \\\\:\\ \\\\ \\ \\\\::::_\\/_\\::\\_\\\\  \\ \\\\__.::.__\\/ ");
    println!("     \\::(_)  \\ \\\\:\\ \\ \\ \\\\:\\ \\\\ \\ \\\\:\\/___/\\\\:. `-\\  \\ \\  \\::\\ \\   ");
    println!("      \\:: __  \\ \\\\:\\ \\ \\ \\\\:\\_/.:\\ \\\\::___\\/_\\:. _    \\ \\  \\::\\ \\  ");
    println!("       \\:.\\ \\  \\ \\\\:\\/.:| |\\ ..::/ / \\:\\____/\\\\. \\`-\\  \\ \\  \\::\\ \\ ");
    println!("     ___\\__\\/\\__\\/_\\____/_/ \\___/_(   \\_____\\/ \\__\\/ \\__\\/   \\__\\/ ");
    println!("    /_____/\\ /_____/\\                                              ");
    println!("    \\:::_ \\ \\\\::::_\\/_                                             ");
    println!("     \\:\\ \\ \\ \\\\:\\/___/\\                                            ");
    println!("      \\:\\ \\ \\ \\\\:::._\\/                                            ");
    println!("       \\:\\_\\ \\ \\\\:\\ \\                                              ");
    println!("     ___\\_____\\/_\\_\\/  ______   ______                             ");
    println!("    /_____/\\ /_____/\\ /_____/\\ /_____/\\                            ");
    println!("    \\:::__\\/ \\:::_ \\ \\\\:::_ \\ \\\\::::_\\/_                           ");
    println!("     \\:\\ \\  __\\:\\ \\ \\ \\\\:\\ \\ \\ \\\\:\\/___/\\                          ");
    println!("      \\:\\ \\/_/\\\\:\\ \\ \\ \\\\:\\ \\ \\ \\\\::___\\/_                         ");
    println!("       \\:\\_\\ \\ \\\\:\\_\\ \\ \\\\:\\/.:| |\\:\\____/\\                        ");
    println!("        \\_____\\/ \\_____\\/ \\____/_/ \\_____\\/                        ");
    println!("                                                                   ");
}

fn main() {
    advent_of_code_title();
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "1" => {
                println!("Day 01 a: {:?}", day01::calculate_fuel());
                println!("Day 01 b: {:?}", day01::calculate_fuel_requirement());
            },
            "2" => day02::print_result(),
            "3" => day03::print_result(),
            "4" => day04::print_result(),
            _ => println!("unsolved day"),
        };
    } else {
        println!("Bitte als erstes Argument den Tag angeben");
    }
}
