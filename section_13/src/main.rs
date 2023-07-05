use std::rc::Rc;

fn main() {
    //Question 1: Create a variable on the stack and a variable on the heap. Multiply their values and print out the results.
    let a = 3;
    let b = Box::new(5);
    println!("{}", a * *b);

    //Question 2: Create a variable that holds a String
    let s = String::from("s");
    {
        //Create a reference counting smart pointer that points to the above String.
        let rc1 = Rc::new(s.clone());

        //Print out how many references the smart pointer has.
        println!("strong count of rc1: {}", Rc::strong_count(&rc1));
        
        //Code block
        {
            //Create another reference counting smart pointer that points to our first smart pointer
            let rc2 = Rc::new(rc1.clone());
            
            //Print out how many references each smart pointer has
            println!("strong count of rc1: {}", Rc::strong_count(&rc1));
            println!("strong count of rc2: {}", Rc::strong_count(&rc2));
        }
        //What value is dropped here?
        //Answer) rc2 is dropped here.
        
        //Print out how many references out first smart pointer has
        println!("strong count of rc1: {}", Rc::strong_count(&rc1));
    } //What value is dropped here?
    //Answer) rc1 is dropped here.
    
    //Comment out the line below. What do you think will happen when you try to run the program now?
    //Answer) String s is still alive but rc1 is already dropped and no longer exist in stack memory. So if i try to access to the rc1, the rust compiler will say that can not find rc1.
    // println!("rc1: {}", rc1);
}
