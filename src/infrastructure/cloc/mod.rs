use std::path::Path;

use tokei::{Config, Languages};

pub fn by_dir<P: AsRef<Path>>(path: P) -> Languages {
    let paths = &[path];
    let excluded = &["target"];

    let config = Config::default();

    let mut languages = Languages::new();

    languages.get_statistics(paths, excluded, &config);

    languages
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use tokei::LanguageType;

    use crate::infrastructure::cloc::by_dir;

    fn fixtures_dir() -> PathBuf {
        return PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("_fixtures");
    }

    #[test]
    fn should_cloc_in_dir() {
        let buf = fixtures_dir().join("projects").join("java").join("hello");
        let languages = by_dir(buf);
        let java = &languages[&LanguageType::Java];

        assert_eq!(1, java.blanks);
    }
}
