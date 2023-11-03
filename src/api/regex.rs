use regex::{Regex, RegexSet};

pub fn rust_regex() {
    build_regex();
    regex_group();
    regex_name_group();
    match_multi_regex();
}

// 构建一个正则表达式的方法
fn build_regex() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    assert!(re.is_match("2014-01-01"));
}

// 源自组的使用
fn regex_group() {
    let text = "2023-04-04, 2023-06-17 and 2023-06-01";
    println!("{}", text);
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();

    for cap in re.captures_iter(text) {
        println!(
            "{}:->>---Month:{}, Day: {}, year: {}",
            &cap[0], &cap[2], &cap[3], &cap[1]
        );
    }
}

// 原子组的命名于使用
fn regex_name_group() {
    // 可以对原子组进行命名
    let re = Regex::new(
        r"(?x)
        (?P<y>\d{4})
        -
        (?P<m>\d{2})
        -
        (?P<d>\d{2})
    ",
    )
    .unwrap();
    let before = "2023-04-04, 2023-06-17 and 2023-06-01";
    let after = re.replace_all(before, "$m/$d/$y");
    println!("{}", after);
    assert_eq!(after, "04/04/2023, 06/17/2023 and 06/01/2023");
}

// 同时匹配多个正则表达式
fn match_multi_regex() {
    let set = RegexSet::new(&[
        r"\w+", r"\d+", r"\pL+", r"foo", r"bar", r"barfoo", r"foobar",
    ])
    .unwrap();
    // 获取的是匹配项的下标
    let matches: Vec<_> = set.matches("foobar").into_iter().collect();
    println!("{:?}", matches);
    assert_eq!(matches, vec![0, 2, 3, 4, 6]);

    // 可以测试特定某个正则表达式是否匹配
    let pick = set.matches("12344bar");
    assert!(pick.matched(1));
    assert!(!pick.matched(5));
}
