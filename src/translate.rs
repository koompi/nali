use std::collections::HashMap;

/// Maps English commands to their equivalent pacman flags
pub struct CommandTranslator {
    translations: HashMap<String, Vec<String>>,
}

impl CommandTranslator {
    pub fn new() -> Self {
        let mut translations = HashMap::new();
        
        // Core package operations
        translations.insert("install".to_string(), vec!["-S".to_string()]);
        translations.insert("update".to_string(), vec!["-Syu".to_string()]);
        translations.insert("upgrade".to_string(), vec!["-Syu".to_string()]);
        translations.insert("search".to_string(), vec!["-Ss".to_string()]);
        translations.insert("remove".to_string(), vec!["-R".to_string()]);
        translations.insert("info".to_string(), vec!["-Si".to_string()]);
        translations.insert("list".to_string(), vec!["-Q".to_string()]);
        translations.insert("clean".to_string(), vec!["-Sc".to_string()]);
        
        // Additional operations
        translations.insert("download".to_string(), vec!["-Sw".to_string()]);
        translations.insert("reinstall".to_string(), vec!["-S".to_string(), "--needed".to_string()]);
        translations.insert("purge".to_string(), vec!["-Rns".to_string()]);
        translations.insert("autoremove".to_string(), vec!["-Rns".to_string()]);
        
        // Package queries
        translations.insert("installed".to_string(), vec!["-Q".to_string()]);
        translations.insert("orphans".to_string(), vec!["-Qtd".to_string()]);
        translations.insert("foreign".to_string(), vec!["-Qm".to_string()]);
        translations.insert("explicit".to_string(), vec!["-Qe".to_string()]);
        translations.insert("files".to_string(), vec!["-Ql".to_string()]);
        translations.insert("owns".to_string(), vec!["-Qo".to_string()]);
        translations.insert("depends".to_string(), vec!["-Qi".to_string()]);
        
        // Repository operations
        translations.insert("refresh".to_string(), vec!["-Sy".to_string()]);
        translations.insert("mirror-update".to_string(), vec!["-Syy".to_string()]);
        
        // Package verification
        translations.insert("check".to_string(), vec!["-Dk".to_string()]);
        translations.insert("verify".to_string(), vec!["-Qk".to_string()]);
        translations.insert("cache-info".to_string(), vec!["-Sc".to_string(), "--print".to_string()]);
        
        CommandTranslator { translations }
    }

    /// Translates English commands to pacman flags
    pub fn translate(&self, args: &[String]) -> Vec<String> {
        if args.is_empty() {
            return args.to_vec();
        }

        let first_arg = &args[0];
        
        // Check if it's already a pacman flag (starts with -)
        if first_arg.starts_with('-') {
            return args.to_vec();
        }

        // Check if it's an English command we need to translate
        if let Some(pacman_args) = self.translations.get(&first_arg.to_lowercase()) {
            let mut result = pacman_args.clone();
            result.extend_from_slice(&args[1..]);
            result
        } else {
            // Not a recognized English command, return as-is
            args.to_vec()
        }
    }

    /// Returns a list of all supported English commands
    #[cfg(test)]
    pub fn supported_commands(&self) -> Vec<&str> {
        self.translations.keys().map(|k| k.as_str()).collect()
    }

    /// Checks if a command is a supported English command
    #[cfg(test)]
    pub fn is_english_command(&self, cmd: &str) -> bool {
        self.translations.contains_key(&cmd.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_install_translation() {
        let translator = CommandTranslator::new();
        let args = vec!["install".to_string(), "firefox".to_string()];
        let translated = translator.translate(&args);
        assert_eq!(translated, vec!["-S", "firefox"]);
    }

    #[test]
    fn test_update_translation() {
        let translator = CommandTranslator::new();
        let args = vec!["update".to_string()];
        let translated = translator.translate(&args);
        assert_eq!(translated, vec!["-Syu"]);
    }

    #[test]
    fn test_search_translation() {
        let translator = CommandTranslator::new();
        let args = vec!["search".to_string(), "firefox".to_string()];
        let translated = translator.translate(&args);
        assert_eq!(translated, vec!["-Ss", "firefox"]);
    }

    #[test]
    fn test_backward_compatibility() {
        let translator = CommandTranslator::new();
        let args = vec!["-S".to_string(), "firefox".to_string()];
        let translated = translator.translate(&args);
        assert_eq!(translated, vec!["-S", "firefox"]);
    }

    #[test]
    fn test_unknown_command() {
        let translator = CommandTranslator::new();
        let args = vec!["unknown".to_string(), "firefox".to_string()];
        let translated = translator.translate(&args);
        assert_eq!(translated, vec!["unknown", "firefox"]);
    }
}