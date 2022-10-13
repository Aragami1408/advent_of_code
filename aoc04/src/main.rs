fn main() {
    let input1 = "yzbqklnj";
    let mut result: u32 = 0;

    loop {
        let hashed = format!(
            "{:x}",
            md5::compute(String::from(input1) + &result.to_string())
        );

        if &hashed[0..5] == "00000" {
            break;
        } else {
            result += 1;
        }
    }

    println!("{}", result);
}
