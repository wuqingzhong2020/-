
use std::io; 
use rand::Rng;
use std::cmp::Ordering;
// use std::sync::atomic::Ordering;

fn test() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Hello, world! : {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guessed1: {guess}");
        // 输入非数字字符， 退出循环， 报告原因， 怎么写
        // let result = guess.trim().parse();
        // let guess: u32 = result.expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue
            }
        };
        println!("You guessed2: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn test2() {
    let x = 5;
    {
        let x = x + 1;

        {
            let x = x * 2;
            println!("The value of x in the inner scope is: {x}");
        }
        println!("The value of x is: {x}");
    }
    println!("The value of x is: {x}");
}

fn test3()
{
    use std::env;
    use std::fs;

    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    // 获取当前工作目录
    let current_dir = env::current_dir().
        expect("cannot get current os run dir");
    let absolute_path = current_dir.join(file_path);
    println!("文件的绝对路径是: {:?}", absolute_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

fn main() {
    // test()
    // test2()
    test3()
}

/*
遇到包下载太慢就改一下源
# 放到 `$HOME/.cargo/config` 文件中
【source.crates-io】
registry = "https://github.com/rust-lang/crates.io-index"

# 替换成你偏好的镜像源
replace-with = 'sjtu'
#replace-with = 'ustc'

# 清华大学
【source.tuna】
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

# 中国科学技术大学
【source.ustc】
registry = "git://mirrors.ustc.edu.cn/crates.io-index"

# 上海交通大学
【source.sjtu】
registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index"

# rustcc社区
【source.rustcc】
registry = "git://crates.rustcc.cn/crates.io-index"
*/