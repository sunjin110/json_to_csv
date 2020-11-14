// cargo test -- --nocapture

#[test]
fn test() {
    println!("test...あああああああああああああ");

    let list = vec!["あ", "い", "う", "え", "お"];

    // 長さはないんね...
    for v in list {
        println!("v:{}", v);
    }
}
