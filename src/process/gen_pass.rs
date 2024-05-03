use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(length:u8,upper:bool,lower:bool,number:bool,symbol:bool)-> anyhow::Result<()>{
  // 随机数
  let mut rng = rand::thread_rng();
  let mut password = Vec::new();
  let mut chars = Vec::new();

  if upper {
    // 区分大小写。将大写字母加入到数组中
    chars.extend_from_slice(UPPER);
    // 从数组中随机选一个数
    password.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty"));
  }
  if lower {
    chars.extend_from_slice(LOWER);
    password.push(*LOWER.choose(&mut rng).expect("LOWER won't be empty"));
  }
  if number {
    chars.extend_from_slice(NUMBER);
    password.push(*NUMBER.choose(&mut rng).expect("NUMBER won't be empty"));
  }
  if symbol {
    chars.extend_from_slice(SYMBOL);
    password.push(*SYMBOL.choose(&mut rng).expect("SYMBOL won't be empty"));
  }
  // 生成密码
  for _ in 0..(length - password.len() as u8) {
    let c = chars.choose(&mut rng).expect("chars won't be empty");
    password.push(*c)
  } 
  password.shuffle(&mut rng);

  let password = String::from_utf8(password)?;
  println!("Generated password: {}", password);

  // 输出强弱程度
  let estimate = zxcvbn(&password, &[])?;
  eprintln!("Estimated strength: {}", estimate.score());
  Ok(())
}