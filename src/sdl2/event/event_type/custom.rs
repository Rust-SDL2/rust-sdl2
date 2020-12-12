use std::collections::HashMap;
use std::sync::{LockResult, Mutex, MutexGuard};

pub(in crate::event) struct CustomEventTypeMaps {
    pub(in crate::event) sdl_id_to_type_id: HashMap<u32, ::std::any::TypeId>,
    pub(in crate::event) type_id_to_sdl_id: HashMap<::std::any::TypeId, u32>,
}

impl CustomEventTypeMaps {
    fn new() -> Self {
        CustomEventTypeMaps {
            sdl_id_to_type_id: HashMap::new(),
            type_id_to_sdl_id: HashMap::new(),
        }
    }
}

lazy_static! {
    static ref CUSTOM_EVENT_TYPES: Mutex<CustomEventTypeMaps> =
        Mutex::new(CustomEventTypeMaps::new());
}

pub(in crate::event) fn lock() -> LockResult<MutexGuard<'static, CustomEventTypeMaps>> {
    CUSTOM_EVENT_TYPES.lock()
}
