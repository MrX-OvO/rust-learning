fn main() {
    let mut num = 3;
    if num > 5 {
        print!("num={} > 5 is true\n", num);
    } else {
        print!("num={} > 5 is false\n", num);
    }

    num = 6;
    if num % 5 == 0 {
        print!("num={} can be divided by 5\n", num);
    } else if num % 4 == 0 {
        print!("num={} can be divided by 4\n", num);
    } else if num % 3 == 0 {
        print!("num={} can be divided by 3\n", num);
    } else if num % 2 == 0 {
        print!("num={} can be divided by 2\n", num);
    } else if num % 1 == 0 {
        print!("num={} can be divided by 1\n", num);
    } else {
        print!("num={}\n", num);
    }

    let condition = true;
    let num = if condition { 5 } else { 6 };
    print!("num={}\n", num);

    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };
    print!("result of loop:{}\n", result);

    let mut i = 3;
    while i > 0 {
        print!("{}!\n", i);
        i -= 1;
    }
    print!("LIFTOFF!!!\n");

    let arr = [1, 2, 3, 4, 5];
    for elem in arr.iter() {
        print!("{}, ", elem);
    }
    print!("\n");

    for i in (1..4).rev() {
        print!("{}!\n", i)
    }
}
