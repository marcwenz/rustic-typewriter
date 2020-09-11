pub fn _clear_screen() {
    print!("{}[2J", 27 as char);
}
