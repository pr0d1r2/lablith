use super::KindError;
use crate::core::kind as neutral;
use crate::pin::PinError;
use crate::system::{System, SystemError};

#[test]
fn a_row_error_prints_the_lablith_shape() {
    let e = KindError::row(4, "bad", "fix it");
    assert_eq!(e.to_string(), ".lablith:4: bad\n  fix: fix it");
}

#[test]
fn pipelith_pin_and_system_errors_convert_and_print_as_their_own() {
    let e = KindError::from(neutral::KindError::new(2, "w", "f"));
    assert_eq!(e, KindError::row(2, "w", "f"));
    let pin = PinError { image: "x".into() };
    assert_eq!(KindError::from(pin.clone()).to_string(), pin.to_string());
    let sys = SystemError::NoTags(System::X86_64Darwin);
    assert_eq!(KindError::from(sys.clone()).to_string(), sys.to_string());
}
