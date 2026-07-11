#[test]
fn eval_crate_does_not_own_judge_runtime() {
    let source = include_str!("../src/lib.rs");

    assert!(
        !source.contains(&["pub trait Judge", "Provider"].concat()),
        "palette-eval must not define a judge provider trait; use palette-judge"
    );
    assert!(
        !source.contains(&["pub struct Judge", "Broker"].concat()),
        "palette-eval must not define a judge broker; use palette-judge"
    );
    assert!(source.contains("pub struct JudgeRequest"));
    assert!(source.contains("pub struct JudgeResponse"));
}
