use std::rc::Rc;

fn main() {
    //Question 1: Create a variable on the stack and a variable on the heap. Multiply their values and print out the results.
    let a = 3;
    let b = Box::new(5);
    println!("{}", a * *b);

    //Question 2: Create a variable that holds a String
    let rc_value = String::from("s");
    {
        //Create a reference counting smart pointer that points to the above String.
        let rc1 = Rc::new(rc_value);

        //Print out how many references the smart pointer has.
        println!("strong count of rc1: {}", Rc::strong_count(&rc1));
        
        //Code block
        {
            //Create another reference counting smart pointer that points to our first smart pointer
            let rc2 = Rc::clone(&rc1);
            
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
    //Answer) the ownership of rc_value is moved to rc1, and rc1 is dropped already so rc_value is also dropped and we can't access to rc_value.
    // println!("s: {}", rc_value);
}
