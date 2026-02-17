use NeuroQuant_SafeCity::actuation::secure_trigger::MechanicalInterlock;
use NeuroQuant_SafeCity::safety::fail_safe::{FailSafe, SystemMode};

#[test]
fn interlock_blocks_invalid_pattern() {
    let interlock = MechanicalInterlock::new(0x0A);
    let packet = vec![0x11, 0x22, 0x33];
    assert!(!interlock.check(&packet));
}

#[test]
fn fail_safe_blocks_in_safe_mode() {
    let fs = FailSafe::new();
    fs.set_mode(SystemMode::SafeMode);
    assert!(!fs.allow_actuation());
}
