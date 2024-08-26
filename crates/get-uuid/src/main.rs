use windows::core::{GUID};
use windows::Win32::System::Com::CoCreateGuid;

fn main() {
    println!("{}", GUID::from("3C2BC851-2131-48AD-8D55-BDBE5B2FAFC9").to_u128());

    unsafe {
        let lol = CoCreateGuid().unwrap();

        println!("New GUID: {lol:?}");
    }
}
