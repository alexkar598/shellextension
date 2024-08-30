use crate::utils::get_property_key_from_name;
use crate::TEST_PROPERTY_GUID;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::ffi::OsString;
use std::ops::{Deref, DerefMut};
use std::sync::RwLock;
use windows::Win32::System::Com::StructuredStorage::PID_FIRST_USABLE;
use windows::Win32::UI::Shell::Common::{SHCOLSTATE, SHCOLSTATE_ONBYDEFAULT, SHCOLSTATE_TYPE_STR};
use windows::Win32::UI::Shell::PropertiesSystem::PROPERTYKEY;

lazy_static! {
    pub static ref virtual_fs_columns: Vec<(OsString, PROPERTYKEY, SHCOLSTATE)> = vec![
        (
            OsString::from("Name"),
            get_property_key_from_name("System.ItemNameDisplay").unwrap(),
            SHCOLSTATE(SHCOLSTATE_TYPE_STR.0 | SHCOLSTATE_ONBYDEFAULT.0)
        ),
        (
            OsString::from("Test"),
            PROPERTYKEY {
                fmtid: TEST_PROPERTY_GUID,
                pid: PID_FIRST_USABLE
            },
            SHCOLSTATE(SHCOLSTATE_TYPE_STR.0 | SHCOLSTATE_ONBYDEFAULT.0)
        ),
    ];
}

type FileId = u64;

enum FsNodes {
    File(File),
    Folder(Folder),
}
trait FsNode {}
impl FsNode for Folder {}
impl FsNode for File {}

struct Folder {
    children: HashMap<FileId, RwLock<FsNodes>>,
}
struct File {
    name: String,
}

struct Child<'a> {
    parent: &'a Folder,
    id: FileId,
}

impl Deref for Child<'_> {
    type Target = FsNodes;

    fn deref(&self) -> &Self::Target {
        self.parent.children.get(&self.id).unwrap().read().unwrap().
    }
}

impl DerefMut for Child<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.parent.children.get_mut(&self.id).unwrap()
    }
}
