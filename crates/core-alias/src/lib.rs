#[cfg(test)]
mod tests {
    #[test]
    fn web_key_helper() {
        let key =
            amategeko_core::platform::web_storage_key(amategeko_core::platform::SETTINGS_FILE);
        assert_eq!(key, "amategeko:settings.json");
        assert_eq!(
            amategeko_core::platform::web_storage_key_to_filename(&key),
            Some(amategeko_core::platform::SETTINGS_FILE)
        );
    }
}
