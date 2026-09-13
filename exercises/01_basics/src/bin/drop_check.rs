// cargo run -p basics --bin drop_check で実行する

// メモリ解放（Drop）を可視化するための構造体
struct Inspector {
    name: &'static str,
}

impl Drop for Inspector {
    fn drop(&mut self) {
        println!("[Drop実行]{} is が解放されました!", self.name);
    }
}

fn main() {
    println!("\n--- シャドーイングとメモリ解放の実験開始 ---");
    {
        let a = Inspector { name: "1st a" };
        println!("1番目のaを作成しました");
        println!("a.nameの値 : {}", a.name);

        //同じスコープでシャドーイングを行う
        let a = Inspector { name: "2nd a" };
        println!("2番目のaを作成しました");
        println!("a.nameの値 : {}", a.name);

        println!("まだスコープの中にいます");
    }
    println!("スコープを抜けました");
}
