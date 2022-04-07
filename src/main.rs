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
fn main() {
  let mut counter = 0;

  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 2;
    }
  };

  println!("the result is {}", result);
}