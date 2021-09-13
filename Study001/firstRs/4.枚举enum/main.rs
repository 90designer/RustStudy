// ====[rust 枚举]====

enum ip_addr_kind{
    V4,
    V6,
}
#[derive(Debug)]
// 定义枚举变体的类型
enum ip_addr_kind2{
    V1,
    V4(u8,u8,u8,u8),
    V6(String),
    V9{x:i32, y:i32},
}
// 为枚举定义方法
impl ip_addr_kind2 {
    fn get(&self) {
        println!("枚举定义的方法");
    }
}
struct ip_addr{
    kind: ip_addr_kind,
    addr:String
}
fn main(){
    let enum1 = ip_addr_kind::V4;
    let enum2 = ip_addr_kind::V6;
    route(enum1);
    route(enum2);
    route(ip_addr_kind::V4);

    let ip0 = ip_addr{
        kind:ip_addr_kind::V4,
        addr:String::from("10.0.2.1")
    };

    let ip1 = ip_addr_kind2::V1;
    let ip2 = ip_addr_kind2::V4(127,0,0,1);
    let ip3 = ip_addr_kind2::V6(String::from("::1"));
    let ip4 = ip_addr_kind2::V9{x:10, y:20};
    println!("{:#?}",ip2);
    ip4.get();
}   

fn route(ip_kind:ip_addr_kind){}

// =====Option枚举====
// 标准库中定义
// enum  Option<T> {
//     Some(T),
//     None
// }

fn option_fn(){
    let some_number = Some(5);
    let some_str = Some("StringA");
    let some_string = Some(String::from("AA"));

    let absent_num :Option<i32> = None;

}
