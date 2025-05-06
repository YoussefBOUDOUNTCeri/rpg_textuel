use rpg_textuel::{prompt, prompt_string};

#[test]
fn test_prompt_string_default() {
    let default = "DefaultValue";
    let result = prompt_string("", default);
    assert_eq!(result, default.to_string());
}

#[test]
fn test_prompt_default_number() {
    let default = 42;
    let result: i32 = prompt("", default);
    assert_eq!(result, default);
}
