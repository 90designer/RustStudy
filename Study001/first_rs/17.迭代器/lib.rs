
#[derive(Debug)]
pub struct Counter{
    pub count :u32,
}
impl Counter {
    fn new()->Counter{
        Counter{count:0}
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self)->Option<Self::Item>{
        if self.count < 5{
            self.count += 1;
            Some(self.count)
        }else{
            None
        }
    }

}

pub fn iter_fn ()->u32{
    let c = Counter::new()
                .zip(Counter::new().skip(1))
                .map(|(a, b)|a*b)
                .filter(|x|x%3 == 0);

    println!("{:?}",&c);
    c.sum()
                
}
