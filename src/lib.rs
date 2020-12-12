
use std::collections::HashMap;

pub struct Settings {
    data: HashMap<String, String>,
}

impl Settings {
    pub fn new() -> Settings {
        let data: HashMap<String, String> = HashMap::new();
        return Settings{data};
    }

    pub fn get(&mut self, name: &str, default: &str) -> String {
        let name = name.to_string();
        let default = default.to_string();
        return self.data.entry(name).or_insert(default).to_string();
    }

    pub fn set(&mut self, name: &str, value: &str) {
        self.data.insert(name.to_string(), value.to_string());
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_getting_with_default() {
        let mut settings = crate::Settings::new();
        let result = settings.get("test", "moose");
        assert_eq!(result, "moose");
        let result2 = settings.get("test2", "orange");
        assert_eq!(result2, "orange");
    }

    #[test]
    fn test_getting_after_setting() {
        let mut settings = crate::Settings::new();
        settings.set("test", "monkey");
        let result = settings.get("test", "moose");
        assert_eq!(result, "monkey");

    }
}