fn main() {
    // x という名前を 5 という値に束縛する（デフォルトは不変）。
    let x = 5;

    // xは不変のため、変更できない。
    // 以下のように変更しようとすると、コンパイルエラーを生じる。
    // error[E0384]: cannot assign twice to immutable variable `x`
    // x = 6;

    println!("x = {}, xのアドレス: {:p}", x, &x);

    // yを可変として束縛する(mut)。
    let mut y = 5;
    println!("y = {}, yのアドレス: {:p}", y, &y);
    // yは可変のため、変更できる。アドレスはそのまま。
    y = 6;
    println!("y = {}, yのアドレス: {:p}", y, &y);

    // シャドーイング
    let z = 5;
    println!("z = {}, zのアドレス: {:p}", z, &z);
    // letを重ねることで、同じ名前の変数を再定義（シャドーイング）できる。
    // mutと異なり、変更後も変数は不変であり、型を変更することもできる。
    let z = z + 1;
    println!("z = {}, zのアドレス: {:p}", z, &z);
    let z = z * 2;
    println!("z = {}, zのアドレス: {:p}", z, &z);
    // 型の変更も行える
    let z = "hello";
    println!("z = {}, zのアドレス: {:p}", z, &z);

    // スコープとシャドーイング
    let zz = 10;
    println!("外側 zz の値: {}, アドレス: {:p}", zz, &zz);
    {
        // シャドーイングにより、zzは再定義される。
        let zz = zz * 2;
        println!("内側 zz の値: {}, アドレス: {:p}", zz, &zz);
    }
    // {}のスコープを抜けると、内側のzzはドロップされ、外側のzzが再び参照される。
    println!("外側 zz の値: {}, アドレス: {:p}", zz, &zz);
}
