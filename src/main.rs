// fn main() {
//   let mut count = 0;
//   'counting_up: loop {
//       println!("count = {}", count);
//       let mut remaining = 10;

//       loop {
//           println!("remaining = {}", remaining);
//           if remaining == 9 {
//               break;
//           }
//           if count == 2 {
//               break 'counting_up;
//           }
//           remaining -= 1;
//       }

//       count += 1;
//   }
//   println!("End count = {}", count);
// }


// Feature a loop to retry an operation you know might fail, such as checking whether a thread has completed its job. You might also need to pass the result of that operation out of the loop to the rest of your code. 
// fn main() {
//   let mut counter = 0;

//   let result = loop {
//     counter += 1;

//     if counter == 10 {
//       break counter * 2;
//     }
//   };

//   println!("the result is {}", result);
// }

// Conditional loops with while: While a condition holds true, the code runs; otherwise it exits the loop
// fn main() {
//   let mut number = 3;

//   while number != 0 {
//     println!("{}!", number);

//     number -= 1;
//   }

//   println!("LIFTOFF!!!");
// }

// LOOPING through a collection with for: DO NOT USE THIS BELOW CODE, it is prone to errors
// fn main() {
//   let a = [10, 20, 30, 40, 50];
//   let mut index = 0;

//   while index < 5 {
//     println!("The value is: {}", a[index]);

//     index += 1;
//   }
// }

// instead, a better way to write it is;
// fn main() {
//   let a = [10, 20, 30, 40, 50];

//   for element in a {
//     println!("the value is: {}", element);
//   }
// }

// rewriting the liftoff countdown code with for loops
fn main() {
  for number in (1..4).rev() {
    println!("{}!", number);
  }
  println!("LIFTOFF!!!");
}