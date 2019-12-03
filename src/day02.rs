fn int_program_code() -> Vec<i64> {
    vec![
        1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,9,1,19,1,19,5,23,1,9,23,27,2,27,6,31,1,5,31,35,2,9,35,39,2,6,39,43,2,43,13,47,2,13,47,51,1,10,51,55,1,9,55,59,1,6,59,63,2,63,9,67,1,67,6,71,1,71,13,75,1,6,75,79,1,9,79,83,2,9,83,87,1,87,6,91,1,91,13,95,2,6,95,99,1,10,99,103,2,103,9,107,1,6,107,111,1,10,111,115,2,6,115,119,1,5,119,123,1,123,13,127,1,127,5,131,1,6,131,135,2,135,13,139,1,139,2,143,1,143,10,0,99,2,0,14,0
    ]
}

fn calculate() -> i64 {
    let mut code = int_program_code();
    'calculateprogram: for i in 0..(code.len() / 4) {
        let pos = i * 4;
        let target = code[pos + 3] as usize;
        let f1 = code[pos + 1] as usize;
        let f2 = code[pos + 2] as usize;
        match code[pos] {
            99 => break 'calculateprogram,
            1 => code[target] = code[f1] + code[f2],
            2 => code[target] = code[f1] * code[f2],
            _ => continue 'calculateprogram,
        }
    }
    code[0]
}

pub fn print_result() {
    println!("{:?}", calculate());
}