use std::fs;

/// Saves the language setting to a configuration file.
///
/// # Arguments
/// * `language` - The language to save.
///
/// # Errors
/// Returns an error if the file cannot be written.
pub fn save_language(language: &str) -> Result<(), std::io::Error> {
    let contents = format!("language={}", language);
    fs::write("config.ini", contents)?;
    Ok(())
}
