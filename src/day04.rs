fn is_valid(password: isize) -> bool
{
    // Going from left to right, the digits never decrease;
    // they only ever increase or stay the same (like 111123 or 135679).
    let mut valid = true;
    let mut duplicate = false;
    let mut last = password;
    
    while last >= 10
    {
        // println!("{} -> {} {}", password, ((last / 10) % 10), last % 10);
        if ((last / 10) % 10) > (last % 10)
        {
            valid = false;
        }
        if ((last / 10) % 10) == (last % 10)
        {
            duplicate = true;
        }
        last = last / 10;
    }

    valid && duplicate
}

fn password_counts(lower: isize, upper: isize) -> isize
{
    let mut count = 0;
    
    for combination in lower..(upper+1)
    {
        if is_valid(combination)
        {
            count += 1;
        }
    }

    return count;
}

pub fn print_result()
{
    println!("Possible password combinations: {}", password_counts(153517, 630395));
    println!("Wrong: 1758, 300");
}