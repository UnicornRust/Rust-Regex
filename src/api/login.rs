use lazy_static::lazy_static;
use regex::Regex;


pub fn login_regex_check(){
    let text1 = r"I♥️email@example.com";
    let text2 = r"sdf+sdfsdff.as.@.jkkk.d.r1";
    let text3 = r"Not_an_eamil@email";
    match extract_login(&text1) {
        Some(login) => println!("text1={}",login),
        None => println!("text1 is not a right name"),
    };
    match extract_login(&text2) {
        Some(login) => println!("text2={}",login),
        None => println!("text2 is not a right name"),
    };
    match extract_login(&text3) {
        Some(login) => println!("text3={}",login),
        None => println!("text3 is not a right name"),
    };
}


fn extract_login(input: &str) -> Option<&str> {
    // lazy_static 是第三方库，实现灵活的静态量的初始化
    // 你的代码并不会在编译时初始化静态量，它会在首次调用，执行代码时，
    // 来初始化，也就是所谓的延迟计算
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?x)
            ^(?P<login>[^@\s]+)@
            ([[:word:]]+\.)*
            [[:word:]]+$
        ").unwrap();
    };

    // 1. ?P<login>     : 命名捕获组 login, 
    // 2. [^@\s]+       : 限定用户名中不能出现空格和 `@` 符号
    // 2. [[:word:]]+   : 匹配[a-zA-Z0-9_]+ 字符

    RE.captures(input).and_then(|cap|{
        cap.name("login").map(|login|login.as_str())
    })
}



