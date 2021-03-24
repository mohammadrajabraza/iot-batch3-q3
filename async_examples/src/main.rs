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

/*
// A basic async tasks example(This should run tasks simultaneously)
use futures::executor::block_on; // executor to run future(task)
use async_std::task;
use std::time::Duration;

fn main(){
    // let task = async_main(); //this call will return a future not the message
    // block_on(task);     // passing the future in executor
    
    let task1 = learn_sing_song();
    let task2 = dance();
    block_on(task2);
    block_on(task1);
}

async fn async_main(){
    // let task1 = learn_sing_song();
    // let task2 = dance();
    // futures::join!(task2, task1);

    
}

async fn learn_sing_song() {
    
    learn_song().await;
    println!("Start singing song");
    sing_song().await;
    println!("Ends singing song");
}

async fn sing_song(){
}

async fn learn_song(){

    println!("Start learning song");
    let mut counter : u32 = 0;
    for _i in 1..200000000 {
        // counter = counter;
    }
    // task::sleep(Duration::from_secs(1)).await;
    // println!("we are back for learning");

    println!("End learning song");
    
}
async fn dance(){
    println!("Dance starts");
    let mut counter : u32 = 0;
    for _i in 1..2 {
        task::sleep(Duration::from_millis(1)).await;
    } 
    println!("Dance ends");
}

*/

// use std::time::Duration;
// use futures::executor::block_on;
// use std::thread;
// fn main(){
//     hello();        
//     world();    
// }

// fn world(){
//     thread::sleep(Duration::from_secs(7));
//     println!(" World");        
    
// }

// fn hello(){                
//     thread::sleep(Duration::from_secs(2));  
//     println!("Hello");   
       
// }


use futures::executor::block_on;
use std::time::Duration;
use std::thread;

async fn learn_song() -> String {
    thread::sleep(Duration::from_secs(5));
    "Songs".to_string() 
}
async fn sing_song(song: String) { //println!("{}",song); 
}

async fn dance() { println!("Dance"); }

async fn learn_and_sing() {
    // Wait until the song has been learned before singing it. We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    
    println!("Learning start");
    let song = learn_song().await;

    println!("Learning end");

    println!("Singing start");
    sing_song(song).await;   
    println!("Singing start");
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();
    // `join!` is like `.await` but can wait for multiple futures concurrently. If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked, `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.
    futures::join!(f1, f2);
}
fn main() {
    block_on(async_main());
}