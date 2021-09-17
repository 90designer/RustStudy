 // =======[Rust 泛型] =====
/**
 * 1. 函数中是使用泛型
 *  fn largest<T> (list:&[T]) -> T{}
 *  函数前面中，在小括号之前先定义泛型类型，后面使用
 * 
 * 2. 结构体中使用泛型
 *  struct Stu<T,U>{}
 *  可定义多个泛型的类型参数。但不建议过多，不同类型参数，实现可相同。
 * 
 * 3. 枚举中使用泛型
 *  enum Color<T>{
        red(T),
        green(T,T),
    }
 */

struct Stu<T>{
    name:T,
    desc:T,
    age:i32
}
// 针对T类型，定义的泛型类型方法（impl后的泛型参数，表示在类型T上实现方法）
impl<T> Stu<T> {
    fn get_name(&self)->&T{
        &self.name
    }
}

// 针对具体类型定义的方法
impl Stu<String> {
    fn get_name2 (&self)->&String{
        &self.name
    }
}

enum Color<T>{
    red(T),
    green(T,T),
}

fn main() {
    let t1 = Color::red(9);
    let t2 = Color::green('a','c');
    let t3 = Color::green(
        Stu{name:"哈哈1", desc:'a',age:12},
        Stu{name:"哈哈2", desc:'b',age:18}
    );



    let num_list = vec![1,20,30,3,4];
    println!("最大值：{}",largest(&num_list));

    let num_list = vec![55,220,310,3,444];
    println!("最大值：{}",largest(&num_list));
}

fn largest (list:&[i32]) -> i32{
    let mut larg = list[0];
    for item in list {
        if *item > larg{
            larg = *item;
        }
    }
    // for &item in list {
    //     if item > larg{
    //         larg = item;
    //     }
    // }
    larg
}