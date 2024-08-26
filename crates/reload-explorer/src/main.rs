use windows::Win32::UI::Shell::{SHChangeNotify, SHFlushSFCache, SHCNE_ALLEVENTS, SHCNF_IDLIST};

fn main() {
    unsafe {
        SHChangeNotify(SHCNE_ALLEVENTS, SHCNF_IDLIST, None, None);
        SHFlushSFCache()
    }
}
