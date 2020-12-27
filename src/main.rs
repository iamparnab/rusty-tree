fn main() {
    for i in 1..4 {
        draw_tree_segment(i);
    }
    draw_stem();
    draw_base();
}

const HEIGHT: i32 = 12;
const WIDTH: i32 = 1 + (HEIGHT - 1) * 2; // AP
const STEM_WIDTH: i32 = HEIGHT / 3;
const STEM_HEIGHT: i32 = HEIGHT / 3;
const BASE_WIDTH: i32 = WIDTH * 2;

fn draw_tree_segment(level: i32) {
    let mut stars = 1;

    for i in 1..(HEIGHT + 1) {
        // Chop off the head if level is not 1
        if !(level > 1 && i < HEIGHT * 10 / 15) {
            add_padding();
            // i - 1, because, we need zero space at last row
            for _ in 1..((WIDTH / 2) - (i - 1) + 1) {
                print!(" ");
            }
            for _ in 1..(stars + 1) {
                print!("^");
            }
            println!();
        }

        stars += 2;
    }
}

fn draw_stem() {
    for _ in 1..(STEM_HEIGHT + 1) {
        add_padding();
        for _ in 1..(WIDTH / 2 - STEM_WIDTH / 2 + 1) {
            print!(" ")
        }
        for _ in 1..(STEM_WIDTH + 1) {
            print!("|")
        }
        println!()
    }
}

fn draw_base() {
    for _ in 1..(BASE_WIDTH + 1) {
        print!("*")
    }
    println!()
}

fn add_padding() {
    for _ in 1..BASE_WIDTH / 2 - WIDTH / 2 + 1 {
        print!(" ")
    }
}
