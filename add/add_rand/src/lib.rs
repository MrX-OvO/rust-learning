use rand::Rng;

pub fn add_rand(x:i32)->(i32,i32){
    let rand = rand::thread_rng().gen_range(1..101);
    let result = x+rand;
(result,rand)
} 

