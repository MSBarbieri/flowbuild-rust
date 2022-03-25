#[macro_export]
macro_rules! js_operator {
    ($func:tt,$($json:tt)+) => ({
        let mut function = String::from("const __method = ");
        function.push_str($func);
        let mut script = js_sandbox::Script::from_string(function.as_str())?;

        let value = serde_json::json!($($json)+);

        script.call("__method",&value)?
    });
}

#[macro_export]
macro_rules! mustache_operator {
    ($text:tt,$($json:tt)+) => ({
        let template = mustache::compile_str($text)?;
        let value = serde_json::json!($($json)+);
        template.render_to_string(&value)
    });
}

#[test]
fn test_js_operator() -> anyhow::Result<()> {
    let js_code = r#"({bag,result,actor_data}) => {
        return `hello World, bag:${bag}, result:${result}, actor_data:${actor_data}`;
    }"#;

    let response: String = js_operator!(js_code,{
        "bag": 1,
        "result": "foo",
        "actor_data": true
    });

    assert_eq!(response, "hello World, bag:1, result:foo, actor_data:true");

    Ok(())
    // println!("{:?}", foo);
    // assert_eq!(foo, 100000);
}

#[test]
fn test_mustace_operator() -> anyhow::Result<()> {
    let text = mustache_operator!("hello {{world}}!",{
        "world":"earth"
    })?;
    assert_eq!(String::from("hello earth!"), text);
    Ok(())
}
