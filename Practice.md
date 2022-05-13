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

## 変数の宣言
* `let var` .. 変数varの宣言（これだけだと定数）
* `let mut mvar` .. 可変変数mvarの宣言（letのオプションとして設定

## 変数へのアクセス
* 直接アクセス：C++やPythonなどと変わりなし
    * 代入:`mvar = 1`
    * 参照:`println(mvar)`
* アドレスアクセス
    * TODO: unsafeじゃないとだめ？

## 変数へのキャスト
```rust
let mut str_a = "21";
let mut var1 :u32 = str_a.parse().expect(); // 変数を数字に変換（parse())し、uint32にキャスト(`:u32`)
```

## 処理結果の判定
* 関数内で`panic(エラーメッセージ)`を呼ぶと、関数呼び出し元に`io::Result`が暗黙的に返される
* 関数呼び出し側で、`expect(panicされたときのメッセージ)`を呼んでおくと、panicした場合はエラーが表示される
    * 上の例だとpanicのエラーメッセージは無視されて、expectの引数のエラーメッセージが表示される
    ```rust
    関数().expect("エラー発生！"
    ```
    * 関数呼び出し元でpanicの対処(=expectの呼び出し）がないと、コンパイラでwarningが出る（ビルドはできる

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
* https://qiita.com/c3drive/items/0008c05998374b6315b4
