fn main() {
    //just something i threw together for fun

    let gradiant = colorous::COOL;
    let floor = day09::parse(include_str!("../../day09/src/input.txt"));
    for (idx, depth) in floor.floor.iter().enumerate() {
        if idx % floor.size.0 == 0 {
            println!("\x1b[0m");
        }
        let color = gradiant.eval_rational(*depth as usize, 10);
        print!("\x1b[48;2;{};{};{}m ", color.r, color.g, color.b);
    }
    println!("\x1b[0m ");
}
