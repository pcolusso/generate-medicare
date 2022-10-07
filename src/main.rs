use rand::Rng;


fn main() {
    let mut rng = rand::thread_rng();
    let mut mn: [u8; 9] = [0; 9];

    mn[0] = rng.gen_range(2..6);
    for i in 1..8 {
        mn[i] = rng.gen_range(0..10);
    }

    mn[8] = (mn[0] + mn[1] * 3 + mn[2] * 7 + mn[3] * 9 + mn[4] + mn[5] * 3 + mn[6] * 7 + mn[7] * 9) % 10;

    println!("{}{}{}{} {}{}{}{}{} 1/1", mn[0], mn[1], mn[2], mn[3], mn[4], mn[5], mn[6], mn[7], mn[8]);
}