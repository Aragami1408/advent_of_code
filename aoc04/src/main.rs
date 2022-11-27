fn main() {
    let input1 = "yzbqklnj";
    let zeros = 5;
    let mut result: u32 = 0;

    loop {
        let hashed = format!(
            "{:x}",
            md5::compute(String::from(input1) + &result.to_string())
        );

        if &hashed[0..6] == "000000" {
            break;
        } else {
            result += 1;
        }
    }

    println!("{}", result);

}
