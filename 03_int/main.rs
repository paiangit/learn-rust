fn main() {
  let price = 100;
  let price2:u32 = 200;
  let price3:i32 = -300;
  let price4:isize = 400;
  let price5:usize = 500;

  println!("price is {}", price);
  println!("price2 is {} and price3 is {}", price2, price3);
  println!("price4 is {} and price5 is {}", price4, price5);

  // 下面这行代码会报错，你明明定义的是i32类型，但却给它赋值为浮点数，类型不匹配，所以会报错：mismatched types
  // let price6:i32 = 66.66;

  // 下面这行代码会报错：error: literal out of range for `i8`
  let price7:i8 = 192;
}
