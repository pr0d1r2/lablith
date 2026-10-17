use super::{DEFAULT_TOKEN, Secret};
use crate::core::kind::Purpose;

#[test]
fn cachix_reads_the_auth_token_to_push() {
    let s = Secret::cachix();
    assert_eq!(
        (s.name.as_str(), s.purpose),
        ("CACHIX_AUTH_TOKEN", Purpose::CachePush)
    );
    assert_eq!(s.why(), "cachix push");
}

#[test]
fn open_change_takes_the_row_token_or_lablith_token() {
    assert_eq!(Secret::open_change(None).name, DEFAULT_TOKEN);
    assert_eq!(DEFAULT_TOKEN, "LABLITH_TOKEN");
    let s = Secret::open_change(Some("BUMP"));
    assert_eq!((s.name.as_str(), s.purpose), ("BUMP", Purpose::OpenChange));
    assert_eq!(s.why(), "push a branch and open a merge request");
}
