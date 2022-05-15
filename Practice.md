# Rustの勉強

## Dockerの作成
* docker-hubからrust環境（rust:1.31）を入手する
    * 参考：https://doc.rust-jp.rs/book-ja/ch01-01-installation.html

## 単体ビルド・実行
* Helloworldのコード（`helloworld.rs`）を用意
    ```rust
    fn main() {
        println!("Hello, world!");
    }
    ```
* ビルド；`rustc helloworld.rs`
* 実行：`./helloworld`

## Cargo（パッケージ）単位でのビルドと実行
[参考](https://doc.rust-jp.rs/book-ja/ch01-03-hello-cargo.html)
* cargoは依存関係を簡単にするツールのこと
* 環境変数`USER`を設定する必要があるので、Dockerfileをいじる
* プロジェクト'mycargo'の作成：`cargo new mycargo`
    * cargo.toml, src/main.rs　が作成される
    * 依存がある場合は、cargo.tomlの`[dependencies]`以下に依存関係を追記する
        * 例：乱数生成ライブラリの場合
            ```rust
            [dependencies]
            rand = "0.8.3"
            ```
* ビルド：.tomlファイルと同じ階層にて、`cargo build`
    * ビルド可能かチェックしたい場合は、`cargo check`
    * リリース用なら、`cargo build --release`
* 実行：同改装にて、`cargo run`
    * ビルドされたプログラムが軌道する

## 数あてゲームで学んだこと
### 変数の宣言
* `let var` .. 変数varの宣言（これだけだと定数）
* `let mut mvar` .. 可変変数mvarの宣言（letのオプションとして設定

### 変数へのアクセス
* 直接アクセス：C++やPythonなどと変わりなし
    * 代入:`mvar = 1`
    * 参照:`println(mvar)`
* アドレスアクセス
    * TODO: unsafeじゃないとだめ？
* 変数から変数への代入
    * コピーになる場合と、移動になる場合があり、ややこしい
    ```rust
    let a = 1;
    let b = a; // char,intなどの値はCopyされる（Copyトレイト）
    println!("a:{}",a); // OK

    let str_a = String::from("hello");
    let str_b = str_a + "world!"; // Copyではなく移動（str_aは使えなくなる）
    println!("str_a:{}",str_a); // エラー ' borrow of moved value: `str_a`'
    ```

### 変数へのキャスト
```rust
let mut str_a = "21";
let mut var1 :u32 = str_a.parse().expect(); // 変数を数字に変換（parse())し、uint32にキャスト(`:u32`)
```

### 処理結果の判定
* 関数内で`panic(エラーメッセージ)`を呼ぶと、関数呼び出し元に`io::Result`が暗黙的に返される
* 関数呼び出し側で、`expect(panicされたときのメッセージ)`を呼んでおくと、panicした場合はエラーが表示される
    * 上の例だとpanicのエラーメッセージは無視されて、expectの引数のエラーメッセージが表示される
    ```rust
    関数().expect("エラー発生！"
    ```
    * 関数呼び出し元でpanicの対処(=expectの呼び出し）がないと、コンパイラでwarningが出る（ビルドはできる

## テキスト送受信アプリで学んだこと
### 全体アルゴリズムの参考：
* https://riptutorial.com/rust/example/4404/a-simple-tcp-client-and-server-application--echo

### TcpStreamからの文字列読み込み
    ```rust
        let result = stream.read_to_string(&mut line)?;
        match result {
            Ok(n) => println!("Received {} bytes", n),
            _ => {}
        }
    ```

* 参考：[stack overflow](https://stackoverflow.com/questions/30552187/reading-from-a-tcpstream-with-readread-to-string-hangs-until-the-connection-is)

### 配列
* 0でフォーマットされた指定長の配列
    ```rust
    let array: [i32; 512] = [0; 512]; //int32型で512要素ある配列を、0で初期化する
    ```

### コマンドライン引数
```rust
use std::env; // コマンドライン引数の処理を含むライブラリ
fn main() {
    let args: Vec<String> = env::args().collect(); // String型のvector型の変数argsに引数を詰める
    let arg_a = &args[1] // 一番最初の引数
}
```

### 関数への引数の渡し方
* C言語と似ていて、`&`で参照渡し
* 参照でない場合、値の代入と同様に、**引数に所有権が写ってもとの値は使えなくなる**
    * 参照渡しで値を関数にわたす（借用: borrow)と、関数実行後ももとの値が使える
    ```rust
    fn submod(a :&String){ // &が付いてるので実行元から変数を借用している
        a + "world" // aに"world"を足す。関数最後の式なので、これが返り値になる
    }
    ```
* ただし参照渡ししたものを`->`でアクセスせず`.`でアクセス
    ```rust
    let mut src = String::from("abc");
    fn hoge(arg: &String){
        arg.len();
        arg.push_str("edf");
    }
    ```

### 条件文
```rust
    if n < 0 {
        ...
    } else if n > 0 {
        ...
    } else {
        ...
    }
```

### クロージャ（所謂Lambda）
```rust
|x:String| x + "world" // | 引数 | 式
//複数行の場合↓
|x:String| {
    xa = x + "world";
    xa + "!!"
}
```

### Switch文
```rust
let var = 2;
match var
{
    1 => println!("var is ONE");
    2 => println!("var is SECOND");
    _ => println!("var is NOT ANY OF 1 or 2"); // _　はその他を示す
}
```

### 関数からのReturn
* 関数最後尾で扱った式の値が返り値になる（文末セミコロンをつけない）
    ```rust
    fn submod() {
        String::from("final answer") // セミコロンをつけない
    }

    fn main(){
        println!("submod:{}",submod())
    }
    ```
* 関数の途中で値を返す：普通にreturn

### 標準出力と標準エラー出力
* 標準出力：`println!("output:{}",str_a)`
    * `!`がついているのは、printlnが関数でなくマクロだから
* 標準エラー出力：`eprintln!()`

### ループ文
* `loop`
    ```rust
    loop{
        /* 永遠に繰り返す */
    }
    ```
    * 無限ループ。`while(true){}`相当
    * Rustでは`while true {}`はコンパイルできない

* `for`
    ```rust
    for i in 1..10 {
        // 1から10まで繰り返す
    }
    ```
    * 文法以外に、C++のforと異なるような特徴なし

* `while`
    ```rust
    while r == True {
        r = some_process();
        // some_processがTrueである限り繰り返す
    }
    ```
    * 文法以外に、C++のwhileと異なるような特徴なし

* `while let`

### "?"オペレータ
* 文末の?で、Result型に対するmatchの代わりになる（例外っぽく扱う）
    ```rust
    do_something()?;
    // ↑と↓は同じ
    match do_somethind() {
        Ok(s) => println("It worked!");
        Err(e) => return Err(e);
    }
    ```
* 参考:[Rustのエラー処理](https://qiita.com/fujitayy/items/cafe661415b6aa33d884#%E3%82%AA%E3%83%9A%E3%83%AC%E3%83%BC%E3%82%BF%E3%83%BC)

## 構造体
* 構造体ではPythonと違って、変数型を定義する
* 構文：
    ```rust
    struct Point{
        x: i32 // Integer32bit
        y: i32 // コメントはC++とかと同じ構文
        z: i32
    }
    ```

## コンストラクタでインスタンス作成
```rust
let mut instance_a = String::new()
```

## ライブラリのInclude
```rust
use std::io; // import iostreamな位置づけ
```
* C++と異なり、`use`も行末に`;`が必要

## ROS2上でのRust（DDS通信）
* 参考：[RUSTのROS2クライアントを作る](https://zenn.dev/eduidl/articles/rust-ros2-client)

## DNNとRust

# 参考
* [日本語版Rust Tutorial](https://doc.rust-jp.rs/book-ja)
* https://qiita.com/c3drive/items/0008c05998374b6315b4
* [Rust初心者殺しの文法10選](https://qiita.com/muumu/items/8cdcc79fa881912adf51)
* [Rust by example](https://doc.rust-lang.org/rust-by-example/index.html)
* [Rust Programming Langurage](https://doc.rust-lang.org/1.30.0/book/first-edition/)