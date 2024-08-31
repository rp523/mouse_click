use rdev::{Button, EventType, simulate};
use chrono::Local;
use rand::{self, Rng};

fn main() {
    let mut rng = rand::thread_rng();
    loop {
        match simulate(&EventType::ButtonPress(Button::Left)) {
            Ok(()) => println!("Mouse clicked. {}", Local::now().format("%Y-%m-%d %H:%M:%S")),
            Err(e) => println!("Error: {:?}", e),
        }
        let interval = rng.gen_range(3 * 60..10 * 60);
        std::thread::sleep(std::time::Duration::from_secs(interval));
    }
}
