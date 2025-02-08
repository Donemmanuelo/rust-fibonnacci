use std::io;

fn main(){
 
    println!("Input a fibonnacci number");
    let mut fibo  = String::new();
    //let fibo: u32 = fibo.trim().parse().expect("please input a number");
     io::stdin()
       .read_line(&mut fibo)
        .expect("failed to read line");
    let fibo: usize = fibo.trim().parse().expect("please enter a number");
   // let x: u32 = 0;
    //let y: u32 = 1;

    //let mut arr: Vec<arr> = Vec::new();
   let mut arr: Vec<u64> = Vec::new();
    //let i  = 0;
    //arr[0] = x;
    //arr[1]= y;
    
    for i in 0..=fibo {   
      if i == 0 {
        arr.push(0)
      }
      else if i == 1 {
        arr.push(1)
      }
      else {
       arr.push(arr[i-1] + arr[i - 2]);
      
    }
}

    
    for i in 0..arr.len() {

        println!("The fibonnacci of {} is {}", i, arr[i]);
    }

}
