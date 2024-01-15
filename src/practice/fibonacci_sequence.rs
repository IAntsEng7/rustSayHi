// 費波那契數(Fibonacci sequence): 由0和1開始，之後的費波那契數就是由之前的兩數相加而得出。
// F(0)=0, F(1)=1, F(n)=F(n−1)+F(n−2) for n≥2
pub fn fibonacci_at_n(n: u64) -> u64 {
    // 1. 參數n使用u64限制傳入的參數不為負數或是浮點數
    // 2. 回傳值也是一個u64的數值
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    // 3. 宣告兩個可變變數來存放前兩個變數, 因為費波那契數就是由之前的兩數相加而得出
    let mut prev = 0;
    let mut current = 1;

    // 4. for迴圈將迭代從 2 到 n（包括 n）的所有整數值。
    //    i(或可以寫成_) 是一個佔位符，表示在這個for迴圈中其實不關心每個元素的具體值，只關心循環的次數。
    //    此處也可寫成for _ in 2..=n， i是為了在最後一次列印出prev與current的值。
    for i in 2..=n {
        // 5-a. prev + current = next
        // 5-b. prev(5-a current), current(5-b next)
        let next = prev + current;
        if i == n {
            println!("The prev is {} in {}th.", prev, n - 2);
            println!("The current is {} in {}th.", current, n - 1);
        }
        prev = current;
        current = next;
    }

    return current;
}
