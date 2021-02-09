/* -------------------------------- */
/* ------- Slides example --------- */
/* -------------------------------- */

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