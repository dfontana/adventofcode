mod util;

use util::read_file;

fn main() -> Result<(), std::io::Error> {
  p1()?;
  p2()?;
  Ok(())
}

fn p2() -> Result<(), std::io::Error> {
  let input_file = std::env::args().nth(1).expect("No input path given");
  let contents = read_file(&input_file)?;
  let nums: Vec<i32> = contents.split('\n')
    .map(|f| f.parse::<i32>().unwrap())
    .collect();
  for (idx, v) in nums.iter().enumerate() {
    for (odx, ov) in nums[idx+1..].iter().enumerate() {
      for ev in &nums[odx+1..] {
        let sum = v + ov + ev;
        if sum == 2020 {
          println!("{:?}", v * ov * ev);
          return Ok(());
        }
      }
    }
  }
  Ok(())
}

fn p1() -> Result<(), std::io::Error> {
  let input_file = std::env::args().nth(1).expect("No input path given");
  let contents = read_file(&input_file)?;
  let nums: Vec<i32> = contents.split('\n')
    .map(|f| f.parse::<i32>().unwrap())
    .collect();
  for (idx, v) in nums.iter().enumerate() {
    for ov in &nums[idx+1..] {
      let sum = v + ov;
      if sum == 2020 {
        println!("{:?}", v * ov);
        return Ok(());
      }
    }
  }
  Ok(())
}