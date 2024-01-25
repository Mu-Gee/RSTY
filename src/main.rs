fn trial() {
    let _two = 2;
    let _name = "James";
    let mut _my_name = "Njeru";
    let my_half = 0.5;
    let _your_half = my_half;
}

fn add(a: i32, b: i32) -> i32 {
    a+b
}

fn main() {
    println!("Hello, world!");
    trial();
    let x =add(1, 1);
    let y = add(6, 5);
    let z = add(x, y);

    // Debug display version use :?
    println!("{:?}", z);
    println!("The Result is: {:?}", z);
    println!("{y:?}");

    // End user display version
    println!("{x}");

    // use of the if statement
    if z > 10 {
        println!("Big number")
    } else {
        println!("Small number")
    }

    // use of nested if statement
    if z > 10 {
        if z > 12 {
            println!("Huge Number")
        } else {
            println!("Big Number")
        }
    } else {
        println!("small Number")
    }

    // use of if..else if..else
    if z > 12 {
        println!("Huge_number");
    } else if z > 10 {
        println!("Big_number");
        }
    else {
        println!("Small_number");
    }
}