/* -------------------------------- */
/* ------- Slides example --------- */
/* -------------------------------- */
/*
// Hello world example (A simple example for async syntax familiarity)
use futures::executor::block_on; // executor to run future(task)

fn main(){
    let task_hello = hello(); //this call will return a future not the message
    block_on(task_hello);     // passing the future in executor
}

// heelo world async function
async fn hello(){      
    print!("Hello Pakistan\n");          
}

*/


// A basic async tasks example(This should run tasks simultaneously)
use futures::executor::block_on; // executor to run future(task)
use std::time::Instant; // timer

fn main(){
    let task = async_main(); //this call will return a future not the message
    block_on(task);     // passing the future in executor
}

async fn async_main(){
    let task1 = learn_sing_song();
    let task2 = dance();
    futures::join!(task2, task1);
}

async fn learn_sing_song() {
    // let start = Instant::now();
    // println!("{:?}", start.elapsed());
    
    learn_song().await;
    
    println!("Start singing song");
    sing_song().await;
    println!("Ends singing song");
    // let elapsed = start.elapsed();
    // println!("{:?}", elapsed); 
}

async fn sing_song(){
}

async fn learn_song(){

    println!("Start learning song");
    let mut counter : u32 = 0;
    for _i in 1..100000000 {
        counter = counter;
    }
    println!("End learning song");
    
}

async fn dance(){
    println!("Dance starts");
    // let start = Instant::now();
    // println!("{:?}", start.elapsed());
    let mut counter : u32 = 0;
    for _i in 1..200000000 {
        counter = counter;
    }
    println!("Dance ends");
    // let elapsed = start.elapsed(); 
    // let diff = elapsed -start;
    // println!("{:?}", diff.seconds()); 
}