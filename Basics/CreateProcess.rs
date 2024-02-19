// Pointer concept to understand


fn main(){
  let number = Arc::new(Mutex::new(0));
  let my_number = Arc::clone(&number);
  let my_number2 = Arc::clone(&number);

  let thread1 = std::thread::spawn(move||{
    for _ in 0..10{
      *my_number.lock().unwrap() += 1;
    }
  });

  let thread2 = std::thread::spawn(move ||{
    for _ in 0..10{
      *my_number2.lock().unwrap() += 1;
    }
  });

  thread1.join().unwrap();
  thread2.join().unwrap();
  println!("Value is : {:?}",number);
  println!("(+) Exiting...");
}
