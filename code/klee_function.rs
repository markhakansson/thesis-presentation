fn function(mut input: u32) -> u32 {
    if input < 2000 {
        input = input + 1000;
        if input == 1234 {
            panic!("invalid number");
        }
        if input == 5000 {
            panic!("this can never happen");
        }
        return input;
    } else {
        return input;
    }
}
