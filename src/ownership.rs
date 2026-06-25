
fn main() {

    let s1 = String::from("123");
    let s2 = s1;

    // println!("{}",s1); 报错,此时s1被移动到了s2,不同于浅拷贝和深拷贝

    let int1 = 32;
    let int2 = int1;

    // 基本数据类型都实现了Copy trait,这个trait可以让移动之后的变量依然能使用
    println!("int1:{} int2:{}",int1,int2);
}