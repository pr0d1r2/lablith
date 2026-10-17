use super::quote;

#[test]
fn a_word_is_single_quoted() {
    assert_eq!(
        quote("hk check --all | tee log"),
        "'hk check --all | tee log'"
    );
}

#[test]
fn an_embedded_quote_cannot_end_the_word() {
    assert_eq!(quote("it's $HOME"), "'it'\\''s $HOME'");
}
