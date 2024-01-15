pub fn convert_temperature(value: f64, from_unit: &str, to_unit: &str) -> Option<f64> {
    //                                          pub:表示這個函數是公開的，可以在其他模塊中使用。
    //                       fn convert_temperature:函數的名稱是 convert_temperature。
    // (value: f64, from_unit: &str, to_unit: &str):函數有三個參數，分別是:
    //                                                  value（待轉換的溫度值），
    //                                                  from_unit（原始溫度的單位），
    //                                                  to_unit（目標溫度的單位）。
    //                               -> Option<f64>:函數的返回類型是 Option<f64>，這表示函數可以返回一個包裝了 f64 類型的 Option。
    //                                              Option用於處理可能為空的情況，它可以是 Some（包裹有值）或 None（空值）。

    match (
        from_unit.to_lowercase().as_str(),
        to_unit.to_lowercase().as_str(),
    ) {
        // match 運算符的模式匹配語句，用於比較兩個字串 (from_unit 和 to_unit) 是否分別等於 "fahrenheit" 和 "celsius"。
        // 字串轉換為小寫，然後再使用 as_str() 方法轉換為 &str 類型。
        // 這麼做的原因是進行比較時通常希望忽略字母的大小寫。

        // 轉換華氏溫度到攝氏溫度
        ("fahrenheit", "celsius") => Some((value - 32.0) * 5.0 / 9.0),
        // 轉換攝氏溫度到華氏溫度
        ("celsius", "fahrenheit") => Some(value * 9.0 / 5.0 + 32.0),
        // 如果單位不匹配，返回 None
        _ => None,
        // 在 Rust 中，Some 是 Option 枚舉類型的一個變體。
        // Option 類型是用來表示一個可能為空值的情況的 Rust 標準函式庫類型。它有兩個變體：Some 和 None。
        // Some: 表示一個包含值的情況，即成功的情況。它是 Option 的一部分，用於將一個值包裝在 Some 中，以表示它存在。
        // None: 表示一個空值的情況，即失敗的情況。它也是 Option 的一部分，用來表示一個缺少的值。
        // 如果轉換成功，函數將傳回 Some 包裹的結果。
        // 如果轉換無法執行，例如單位不匹配，函數將傳回 None，表示失敗或缺失。
        // 使用 Option 的優點在於它提供了一種清晰的方式來處理可能為空的值，避免了在操作中出現 null 或 undefined 的問題。
        // 有助於在編譯時捕獲潛在的空值錯誤，並提高程式碼的安全性。
    }
}
