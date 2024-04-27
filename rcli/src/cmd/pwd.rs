use clap::Parser;
use rand::prelude::SliceRandom;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

#[derive(Debug, Parser)]
pub struct PwdOpts {
    #[arg(long, default_value_t = true)]
    pub upper: bool,

    #[arg(long, default_value_t = true)]
    pub lower: bool,

    #[arg(long, default_value_t = true)]
    pub number: bool,

    #[arg(long, default_value_t = true)]
    pub symbol: bool,

    #[arg(short, long, default_value_t = 6)]
    pub length: u8,
}

pub fn process_pwd(opts: &PwdOpts) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    // 随机选择大写字母
    if opts.upper {
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty"));
    }
    // 随机选择小写字母
    if opts.lower {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("LOWER won't be empty"));
    }
    // 随机选择数字
    if opts.number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("NUMBER won't be empty"));
    }
    // 随机选择特殊符号
    if opts.symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("SYMBOL won't be empty"));
    }

    // 根据命令行参数长度来补齐剩余的密码
    for _ in 0..(opts.length - password.len() as u8) {
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        password.push(*c);
    }

    // 将已经生成的密码进行洗牌打乱
    password.shuffle(&mut rng);

    let password = String::from_utf8(password)?;
    println!("{}", password);

    Ok(())
}
