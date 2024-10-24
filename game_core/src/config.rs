pub struct Config {
    pub current_lang: String,
    pub levels_path: String,
    pub species_path: String,
    pub inventory_path: String,
    pub key_value_storage_path: String,
    pub localized_strings_path: String,
}

static mut CONFIG: *mut Config = std::ptr::null_mut();

pub fn config() -> &'static Config {
    unsafe {
        &*CONFIG
    }
}

pub fn initialize_config_strings(
    current_lang: String,
    levels_path: String,
    species_path: String,
    inventory_path: String,
    key_value_storage_path: String,
    localized_strings_path: String,
) {
    unsafe {
        let config = Config {
            current_lang,
            levels_path,
            species_path,
            inventory_path,
            key_value_storage_path,
            localized_strings_path
        };
        let boxed = Box::new(config);
        CONFIG = Box::into_raw(boxed);      
    }
}
