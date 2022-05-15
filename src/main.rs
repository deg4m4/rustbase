mod comperar;
mod tool;

use rand::Rng;

fn main() {
    let mut c = 0;
    let ( mut win, mut out) = (0, 0);
    while c < 10 {

        let rand_cond: u16 = rand::thread_rng().gen_range(1..4);
        let rand_num: u16 = rand::thread_rng().gen_range(1..1000);

        match rand_cond {
            1 => comperar::min(rand_num, &mut win, &mut out),
            2 => comperar::max(rand_num, &mut win, &mut out),
            3 => comperar::equal(rand_num, &mut win, &mut out),
            _ => {}
        }

        c += 1;

    }

    println!("Win: {}", win);
    println!("Out: {}", out);

}
