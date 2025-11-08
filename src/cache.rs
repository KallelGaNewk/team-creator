use serde::{Deserialize, Serialize};

pub trait PersistentCache: Sized + Serialize + for<'de> Deserialize<'de> {
    fn filename() -> &'static str;
    fn save_to_disk(&self)
    where
        Self: Serialize,
    {
        let ron_string = ron::to_string(&self).expect("Failed to serialize data to RON");

        #[cfg(not(target_arch = "wasm32"))]
        std::fs::write(Self::filename(), ron_string).expect("Failed to write cache to disk");

        #[cfg(target_arch = "wasm32")]
        if let Some(storage) = web_sys::window()
            .expect("no global `window` exists")
            .local_storage()
            .ok()
            .flatten()
        {
            storage
                .set_item(Self::filename(), &ron_string)
                .expect("Failed to write cache to localStorage");
        }
    }

    fn read_from_disk() -> Option<Self>
    where
        Self: Sized,
    {
        #[cfg(not(target_arch = "wasm32"))]
        let ron_str = {
            let appdata = std::fs::read(Self::filename()).ok()?;
            String::from_utf8(appdata).ok()?
        };

        #[cfg(target_arch = "wasm32")]
        let ron_str = {
            if let Some(storage) = web_sys::window()
                .expect("no global `window` exists")
                .local_storage()
                .ok()
                .flatten()
            {
                storage.get_item(Self::filename()).ok().flatten()?
            } else {
                return None;
            }
        };

        ron::from_str(&ron_str).ok()
    }

    fn read_or(init: Self) -> Self {
        Self::read_from_disk().unwrap_or(init)
    }
}
