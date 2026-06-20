#[test]
fn eval_crate_does_not_own_judge_runtime() {
    let source = include_str!("../src/lib.rs");

    assert!(
        !source.contains(&["pub trait Judge", "Provider"].concat()),
        "beater-eval must not define a judge provider trait; use beater-judge"
    );
    assert!(
        !source.contains(&["pub struct Judge", "Broker"].concat()),
        "beater-eval must not define a judge broker; use beater-judge"
    );
    assert!(source.contains("pub struct JudgeRequest"));
    assert!(source.contains("pub struct JudgeResponse"));
}
