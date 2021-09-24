/**
 * === 迭代器 === 
 * 
 * 
 */

#[derive(Debug)]
struct Shoes{
    size:i32,
    style:String,
}
 fn main() {
    
    // 消耗迭代器
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let total:i32 = v1_iter.sum();
    println!("{}", total);


    // 产生迭代器的方法
    let v2 = vec![3,4,5];
    let mut v2_iter = v2.iter();
    let v2_coll:Vec<_> = v2_iter.map(|x|x + 1).collect(); // 收集到集合中
    println!("{:?}", &v2_coll);

    let my_shoe = Shoes{size:10, style:"sneaker".to_string()};
    let store_shoe = vec![
        Shoes{size:10, style:"sneaker0".to_string()},
        Shoes{size:11, style:"sneaker1".to_string()},
        Shoes{size:12, style:"sneaker2".to_string()},
        ];

    let my_styles = shoes_check(store_shoe, my_shoe.size);
    println!("{:?}", &my_styles);


    get_iter__sum();
 }

 fn shoes_check(shoes:Vec<Shoes>, shoes_size:i32) -> Vec<Shoes>{
     shoes.into_iter().filter(|x|x.size == shoes_size).collect()
 }

// 创建自定义的迭代器
mod lib;

fn get_iter__sum(){
    let sum = lib::iter_fn();
    println!("自定义迭代器：{}", sum);
}








 #[test]
 fn test_iter(){
    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
 }

