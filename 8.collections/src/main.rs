
// #![allow(unused)]
// fn main() {
//     let mut v = vec![100, 32, 57];
//     for i in &mut v {
//         *i += 50;
//     }
// 
//     let mut j = 50;
//     let k = &mut j;
//     *k += 1;
//     j += 1;
//     println!("{}", j);
//     // println!("{}", k);
// 
//     println!("{:?}", v);
// 
//     let mut v = vec![100, 32, 57];
//     let r = &mut v;
//     (*r).push(42);       // 使用可变引用
//     println!("{:?}", v);
// 
// 
//     // 定义一个枚举，以便能在 vector 中存放不同类型的数据
//     enum SpreadsheetCell {
//         Int(i32),
//         Float(f64),
//         Text(String),
//     }
// 
//     let row = vec![
//         SpreadsheetCell::Int(3),
//         SpreadsheetCell::Text(String::from("blue")),
//         SpreadsheetCell::Float(10.12),
//     ];
// }



#![allow(unused)]
fn main() {
    // data 的类型是 &'static str
    // &str：这是一个字符串切片，是对字符串数据的不可变引用，它可以指向 String 或者字符串字面量。
    let data = "initial contents";

    let s = data.to_string();

    // 该方法也可直接用于字符串字面量：
    let s = "initial contents".to_string();

    // 等同于
    let s = String::from("initial contents");

    // 请记住，字符串是 UTF-8 编码的，所以可以包含任何正确编码的数据
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str("bar");

    // push_str 方法采用字符串 slice，因为我们并不需要获取参数的所有权
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s1 = String::from("Hello");
    let s2 = " World";
    println!("s1 初始地址 : {:p}", s1.as_ptr());
    println!("s2 初始地址 : {:p}", s2.as_ptr());

    s1.push_str(s2); // 追加 s2 到 s1
    println!("s1 追加后地址: {:p}", s1.as_ptr()); // 地址可能变化（扩容）
    println!("s2 的地址    : {:p}", s2.as_ptr()); // s2 的地址不变
    println!("结果: {}", s1); // 输出 "Hello World"

    let mut s1 = String::from("Hello");
    let s2 = String::from(" World");
    s1.push_str(&s2); // 通过 &String 自动转为 &str


    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
    // println!("s1: {}", s1);
    println!("s2: {}", s2);

    let s1 = "12";
    let s2 = "123";

    println!("\"12\" 的字节数: {}", s1.len()); // 应输出 6
    println!("\"123\" 的字节数: {}", s2.len()); // 应输出 9

    // 打印每个字符的字节表示
    println!("\"12\" 的字节表示: {:x?}", s1.as_bytes());
    println!("\"123\" 的字节表示: {:x?}", s2.as_bytes());

    loop {
        let mut input :String = String::new();
        println!("请输入：");
        match std::io::stdin().read_line(&mut input) {
            Ok(n)=> {
                let cleaned_input = input.trim_end();
                println!("{} bytes read", cleaned_input.len());
                println!("{cleaned_input}");
            },
            Err(error) => println!("error: {error}"),
        }
    }
}

