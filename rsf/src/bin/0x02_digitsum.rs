// 0x02 —— while 与 loop
//
// 运行: cargo run --bin 0x02_digitsum
//
// 题目:
//   1. 写 fn digit_sum(n: u32) -> u32, 把 n 的各位数字相加
//      1234 → 1+2+3+4 = 10, 用 while 写
//   2. main 里用 loop 从 1 往上找, 找到第一个 digit_sum(n) >= 20 的数,
//      用 break 把这个数带出来, 打印它
//
// 提示:
//   * n % 10 是最后一位, n / 10 是去掉最后一位之后剩下的
//   * loop { ... break 值; } 整体是个表达式, 可以 let x = loop { ... };

fn main() {
    let mut i = 1;
    let res: u32 = loop {
        if digit_sum(i) >= 20 {
            break i;
        }
        i += 1;
    };
    println!("第一个各位数之和 >= 20 的数是: {}", res);
}

fn digit_sum(mut n: u32) -> u32 {
    let mut res: u32 = 0;
    while n > 0 {
        res += n % 10;
        n /= 10;
    }
    res
}

