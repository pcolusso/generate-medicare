use rand::Rng;


fn main() {
    let mut rng = rand::thread_rng();
    let mut mn: [u8; 10] = [0; 10];

    for i in 0..8 {
        mn[i] = rng.gen_range(0..10);
    }

    mn[8] = (mn[0] + mn[1] * 3 + mn[2] * 7 + mn[3] * 9 + mn[4] + mn[5] * 3 + mn[6] * 7 + mn[7] * 9) % 10;
    mn[9] = 1;

    // let as_string = mn.iter().fold(String::new(), |mut output, &number| {
    //     write!(output, "{}", number).ok();
    //     output
    // });

    println!("{}{}{}{} {}{}{}{}{} 1/1", mn[0], mn[1], mn[2], mn[3], mn[4], mn[5], mn[6], mn[7], mn[8]);
}