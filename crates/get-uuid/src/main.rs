use windows::Win32::System::Com::CoCreateGuid;

fn main() {
    unsafe {
        let lol = CoCreateGuid().unwrap();

        println!("New GUID: {lol:?}\n\t{}", lol.to_u128());
    }
}
