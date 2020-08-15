use is_root::is_root;

fn main() {
    if is_root() {
        println!("You are root");
    } else {
        println!("You are not root")
    }
}
