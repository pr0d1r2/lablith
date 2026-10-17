use super::RenderError;
use crate::core::ConfigError;
use crate::kind::KindError;
use crate::system::{System, SystemError};

#[test]
fn each_refusal_prints_its_own_message() {
    let row = ConfigError::new(2, "w", "f");
    assert_eq!(RenderError::from(row.clone()).to_string(), row.to_string());
    let kind = KindError::row(3, "k", "f");
    assert_eq!(
        RenderError::from(kind.clone()).to_string(),
        kind.to_string()
    );
    let sys = SystemError::NoTags(System::X86_64Darwin);
    assert_eq!(RenderError::from(sys.clone()).to_string(), sys.to_string());
}

#[test]
fn a_duplicate_names_the_path_and_the_fix() {
    let e = RenderError::Duplicate("a.yml".into());
    assert_eq!(
        e.to_string(),
        "two files at `a.yml`\n  fix: give each row its own id, and list no \
         generated path as hand-written"
    );
}
