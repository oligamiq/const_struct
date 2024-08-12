# const_struct
<!-- [![Latest version](https://img.shields.io/crates/v/bitflags.svg)](https://crates.io/crates/bitflags) -->
<!-- [![Documentation](https://docs.rs/bitflags/badge.svg)](https://docs.rs/bitflags) -->
<!-- ![License](https://img.shields.io/crates/l/bitflags.svg) -->

`const_struct`は、const genericsの機能のように、構造体やf32などを渡すことができるようにするためのマクロです。
これはgeneric_const_exprとは別の機能であり、標準の置き換えがあるかは不明です。
https://github.com/rust-lang/rust/issues/76560
https://hackmd.io/OZG_XiLFRs2Xmw5s39jRzA?view

公式のConst関連の機能はこちらで提案されます。
https://github.com/rust-lang/const-eval

no_stdで使うことができます。<br>
もし、追加の機能が必要な場合、もしくはバグを見つけた場合は、issueを立ててください。<br>
現在開発中であり、mainも壊す可能性があります。<br>
外部に公開することもできますが、メジャーバージョン間で互換性がない可能性があります。<br>
互換性がなくなった場合、メジャーバージョンを上げます。
なお、現時点でメジャーバージョンを上げる予定はありません。

# 使い方
構造体を受け取るときは、その名前の後ろに`Ty`をつけます。<br>
例外は存在しますが、マクロやトレイトを使う際には、他にインポートしなければならないものが存在しないように作られています。<br>
データは`__DATA`に格納されています。<br>
構造体の場合、メンバ変数の名前をキャメルケースに変換した名前を用いることで直接アクセスできます。<br>
また、`::__DATA`で取り出したデータは、コンパイラが型推論できるようになっています。<br>
詳しいコードは、`/crates/test_code/`を参照してください。<br>
※変な参照の仕方をしても動くかを確認するため、テストコードが複雑です。

## プリミティブ型(inside declaration const)
※ここではインサイドマクロと呼ぶことがあります。<br>
プリミティブ型を使う場合は、`primitive`モジュールを使います。<br>
そして、渡すときは、名前をキャメルケースに変換した名前で定義されているマクロを使います。<br>
プリミティブ型の場合は、`VALUE`でもアクセスできます。<br>

```rust
use const_struct::{primitive::F32Ty, F32};

pub fn tester<A: F32Ty>() {
    println!("a: {:?}", A::__DATA);
}

fn main() {
    tester::<F32!(0.5)>();
}
```
## 構造体(outside declaration const)
定義できる値に制限はありません。<br>
構造体の場合は、`derive(ConstStruct)`で受け取ることができるようになります。<br>
Constとして渡したい値は、`const_struct`マクロを使って定義します。<br>
`const_struct`マクロは、名前をキャメルケースに変換し、Tyをつけた名前で定義します。<br>
`setting.rs`などを用いて、まとめて管理することをお勧めします。

```rust
use const_struct::{const_struct, ConstStruct};

#[derive(ConstStruct, Debug)]
pub struct TestSetting {
    pub a: Option<u32>,
    abc_def: &'static str,
}

pub fn tester<A: TestSettingTy>() {
    println!("a: {:?}", A::__DATA);
}

#[const_struct]
const WINDOW_SETTING: TestSetting = {
    let mut c = TestSetting {
        a: Some(0),
        abc_def: "abc_def",
    };
    c.a = Some(5);
    c.abc_def = "hello world";
    c
};

fn main() {
    tester::<WindowSettingTy>();
}

```
## 構造体(outside declaration const/generics)
ジェネリクスはまだ対応していないはずです（忘れてました）。

## 構造体(inside declaration const)
※ここではインサイドマクロと呼ぶことがあります。<br>
基礎理論は出来上がっていますが、制限が存在します。<br>
配列は80bytesまでしか渡されません。文字列は80bytesで途切れ、配列の場合は0埋めされます。<br>
また、かなり展開する上、traitの相互作用が多く存在するため、多く使うとコンパイル時間に影響が出ると思われます。<br>
```rust
```

## 構造体(inside declaration const/generics)
基礎理論は出来上がっていますが、制限が存在します。<br>
また、かなり展開する上、traitの相互作用が多く存在するため、多く使うとコンパイル時間に影響が出ると思われます。<br>
ジェネリクスで渡す型は、インサイドマクロを実装している必要があり、また、内部で使うためインポートする必要があります。<br>
トレイト境界を用いることができますが、受け取った引数の型の確認のため、内部でトレイト境界を展開します。よって、トレイト境界を使う場合は、$crateなどを用いて絶対パスで指定することをお勧めします。インポートエラーが出ます<br>
```rust
```

## 複合型
複合型を受け取る場合には、一番外側の型だけTyを付けます。
タプルの対応は後回しにしているため、できていません。
```rust
use const_struct::{primitive::OptionTy, F32, Some};

pub fn tester<A: OptionTy<Option<f32>>>() {
    println!("a: {:?}", A::__DATA);
}

fn main() {
    tester::<Some!(Some!(F32!(0.5)))>();
}
```

## ConstCompat
通常の関数などをcfgフラグに基づいて、ジェネリクス受け取りに変更する属性マクロです。
<br>
内部も書き換えることにより、そのまま動きます。<br>
一つ目の引数で、変数を指定します。
cfgフラグのときに元の関数を呼び出すため、cfgフラグがないときは、元の関数を呼び出すようになります。<br>
もう一つcfgフラグを追加することで、対応コードに対してもcfgフラグを付けれます。<br>
現在、通常の関数のみ対応しています。<br>

```rust
use const_struct::ConstCompat;

#[const_compat(test_setting, #[cfg(not(feature = "dynamic"))])]
pub fn tester(test_setting: TestSetting) {
    let t = test_setting.abc_def;
    println!("{:?}", t);

    tester_inner(test_setting.a.unwrap());

    let test_setting = TestSetting::default();

    println!("{:?}", test_setting);
}
```

# 未実装機能
## マクロを使わない手動実装:テスト済み
- 構造体に対するインサイドマクロ
- 構造体に対するインサイドマクロのジェネリクス

## マクロを使わない手動実装:未テスト
- プリミティブ型（アウトサイドマクロ）
- 複合型（タプル）
- 構造体に対するアウトサイドマクロでのジェネリクス
- Enumに対するConstStruct、アウトサイドマクロ、インサイドマクロ