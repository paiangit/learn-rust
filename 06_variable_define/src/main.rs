fn main() {
    // 定义变量的格式
    // let 变量名=值; 不指定变量类型
    // let 变量名:类型=值; 指定变量类型
    // 变量就是给某一个内存地址取名字
    // 变量的命名规范：
    // 变量名只能由字母、数字和下划线组成
    // 变量名的第一个字符不能是数字
    // 变量名不能是 Rust 语言的关键字
    // 变量名区分大小写
    let variable1:&str = "";
    println!("variable1 {}", variable1);

    // 默认情况下，Rust 中的变量在声明了值之后是不可变的，即不允许修改变量的值
    // 可以使用 mut 关键字来声明一个可变变量，mut 是 mutable 的缩写
    let variable2 = 10;
    // 下面这行会报错：cannot assign twice to immutable variable
    // variable2 = 20;
    println!("variable2 {}", variable2);


    // 可变变量：使用 mut 关键字来声明一个可变变量，mut 是 mutable 的缩写
    // let mut 变量名=值; 不指定变量类型
    // let mut 变量名:类型=值; 指定变量类型
    let mut variable3 = 10;
    // 下面这行会报错：cannot assign twice to immutable variable
    variable3 = 20;
    println!("variable3 {}", variable3);
}
