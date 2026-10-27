// 0x03 —— 读键盘输入
//
// 运行: cargo run --bin 0x03_guess
//
// ── 题目 ────────────────────────────────────────
//
// 猜数字. 秘密数字写死成 42.
//
// 反复做:
//   1. 提示 "猜一个 1-100 的数:"
//   2. 读一行输入, 转成 u32
//   3. 比 42 大 → 提示 "大了"; 比 42 小 → 提示 "小了"
//   4. 等于 42 → 打印 "猜中了! 共猜了 N 次", 结束
//
// 要求:
//   * 统计猜了几次
//   * 输入的不是数字时, 直接让它崩掉就行 (用 expect), 容错下一题再做
//
// ── 会用到的三行 (照抄, 位置自己摆) ──────────────
//
//   use std::io;
//
//   let mut line = String::new();
//   io::stdin().read_line(&mut line).expect("读不到输入");
//   let n: u32 = line.trim().parse().expect("那不是个数字");
//
//   `use` 写在文件最上面.
use std::io;


fn main() {
    let num: u32 = 42;
    let mut count: u32 = 0;

    loop {
        println!("输入你要猜的数~");

        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("没有输入");
        let n: u32 = line.trim().parse().expect("输入不是一个数字");

        count += 1;
        match ( n > num, n == num, n < num) {
            (true, _, _) => println!("猜的太大啦~\n"),
            (_, _, true) => println!("猜的太小啦~\n"),
            (_, true, _) => {
                println!("猜对啦~\n你一共猜了 {} 次", count);
                break
            }
            _ => {}
        }
    }
}
