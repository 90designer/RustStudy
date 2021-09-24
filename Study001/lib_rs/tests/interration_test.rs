use lib_rs;
#[test]
fn its(){
    assert_eq!(10,lib_rs::jicheng(5));
}
#[test]
fn its2(){
    assert_eq!(10,lib_rs::jicheng(5));
}

mod  help;
#[test]
pub fn mod_add(){
    help::add();
}

