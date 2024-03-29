
// 这里我声明了一个u32的整数类型,
const MAX_POINT:u32 = 42;

fn main() {
    println!("Hello, world!");
    // 1、
    // 这里使用let关键字声明了一个变量x, 它的类型是i32的整数类型;
    // 在这里，即使我们不写它的类型，也会推断成i32这个类型
    let mut x: i32 = 5;
    println!("the value of x is {}", x);

    // 2、
    // 如果我们没有声明这个值是可变的，那么重新赋值的时候，编译器就会报错
    // cannot assign twice to immutable variable 
    // 翻译过来就是不能对不可变的变量进行重复赋值
    x = 6;
    println!("the value of x is {}", x);

    // 3、
    // 如果我们想要让它不报错，我们需要在x前面添加 mut 关键字 这样就不会报错了
    // let mut x: i32 = 5;


    // 4、变量与常量
    // 常量（contant）,常量在绑定之后也是不可变的，但是它与不可变的变量有很多区别。
    // （1）常量不可以使用mut关键字，因为常量永远都是不可变的
    // （2）声明常量只能使用const关键字，它的类型必须被标注清楚
    // （3）常量可以在任何作用域内进行声明，包括全局作用域
    // （4）常量只能绑定到常量表达式
    // 命名规范方面：使用大写字母，每个单词之间使用下划线进行分割
}
