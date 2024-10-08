# const_struct
[![Latest version](https://img.shields.io/crates/v/const_struct.svg)](https://crates.io/crates/const_struct)
[![Documentation](https://docs.rs/const_struct/badge.svg)](https://docs.rs/const_struct)
![License](https://img.shields.io/crates/l/const_struct.svg)

[![Japanese](https://img.shields.io/badge/lang-ja-blue.svg)](README-ja.md)
[![English](https://img.shields.io/badge/lang-en-blue.svg)](README.md)

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

## 初期化
このパッケージで生成されてエクスポートされたマクロを使う場合は、クレートルートでこの関数を呼び出す必要がある。
そのため、もし使用するライブラリが存在する場合は、この旨を伝える必要がある。
```rust
const_struct::init!();
```

## プリミティブ型(inside declaration const)
※ここではインサイドマクロと呼ぶことがあります。<br>
プリミティブ型を使う場合は、`primitive`モジュールを使います。<br>
そして、渡すときは、名前をキャメルケースに変換した名前で定義されているマクロを使います。<br>
プリミティブ型の場合は、`VALUE`でもアクセスできます。<br>
対応している型は、u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, char, bool, Unitです。<br>

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
可能です。<br>
```rust
use const_struct::{const_struct, primitive::OptionTy};

pub fn tester<A: OptionTy<f64>>() {
    println!("a: {:?}", A::__DATA);
}

#[const_struct]
const PI: Option<f64> = Some(3.14159265358979);

fn main() {
    tester::<PiTy>();
}
```

## 構造体(inside declaration const)
※ここではインサイドマクロと呼ぶことがあります。<br>
deriveマクロを適用すると、同名のマクロが生成されます。<br>
このマクロを使用する際には、構造体も同時にインポートする必要があります。<br>
適用する構造体のメンバ変数は、`Copy`可能である必要があります。（気を付けるべきエラー3つ目より）<br>
`#[const_struct(macro_export)]`とすることで、マクロを外部に公開することができます。<br>
```rust
#[allow(unused)]
#[const_struct::const_struct(macro_export)]
#[derive(const_struct::ConstStruct, Debug)]
pub struct TestSetting {
    a: Option<u32>,
    abc_def: &'static str,
}

pub fn tester<A: TestSettingTy>() {
    println!("a: {:?}", A::__DATA);
}

pub const fn default() -> TestSetting {
    TestSetting {
        a: None,
        abc_def: "hello world",
    }
}

fn main() {
    tester::<macros::TestSetting!(default())>();
}
```

## 構造体(inside declaration const/generics)
上記の条件に加え、トレイト境界にはCopyが勝手に追加されます。<br>
ジェネリクスが存在する場合、構造体名のマクロを呼び出す際に、定義順で型を指定します。<br>
const genericsの場合は、渡した値から推定可能な場合は、`_`を使うことができます。<br>
全てのconst genericsを省略する場合、書かないことができます。<br>
`call_with_generics!`マクロを使うことで、推論可能な場合、const genericsを省略することができます。<br>
定義順に型は展開されるため、`call_with_generics!`マクロを使う場合、受け取る側では定義順に型を指定する必要があります。<br>
const genericsでない型は、省略することはできません。<br>
`#[const_struct]`で定義した`???Ty`を引数に用いた場合、const genericsでない型も省略することができます。<br>
メンバ変数はderiveマクロを実装している必要はありません。<br>
例は以下の通りです。<br>
```rust
use const_struct::{call_with_generics, const_struct, ConstStruct};

#[derive(ConstStruct, Debug)]
pub struct TestSetting<const N: usize>;

pub fn tester<const N: usize, A: TestSettingTy<N>>() {
    println!("a: {:?}", A::__DATA);
}

#[const_struct]
const B: TestSetting<5> = TestSetting;

fn main() {
    tester::<5, macros::TestSetting!(5, TestSetting::<5>)>();
    tester::<5, macros::TestSetting!(_, TestSetting::<5>)>();
    tester::<4, macros::TestSetting!(4, TestSetting)>();
    tester::<9, macros::TestSetting!(TestSetting::<9>)>();

    tester::<5, macros::TestSetting!(B)>();
    tester::<5, BTy>();
    call_with_generics!(tester::<macros::TestSetting!(B)>());
    call_with_generics!(tester::<5, BTy>());
    call_with_generics!(tester::<macros::TestSetting!(_, BTy)>());
    call_with_generics!(tester::<macros::TestSetting!(BTy)>());
}
```

## 複合型(Option)
複合型を受け取る場合には、一番外側の型だけTyを付けます。<br>
```rust
use const_struct::{primitive::OptionTy, F32, Some};

pub fn tester<A: OptionTy<Option<f32>>>() {
    println!("a: {:?}", A::__DATA);
}

fn main() {
    tester::<Some!(Some!(F32!(0.5)))>();
}
```

## 複合型(Tuple)
複合型を受け取る場合には、TupleTyでまとめます。<br>
最大は10個までです。<br>
```rust
use const_struct::{primitive::TupleTy, F32, F64, U32};

pub fn tester<A: TupleTy<(f32, f64, u32)>>() {
    println!("a: {:?}", A::__DATA);
}

fn main() {
    tester::<(F32!(0.5), F64!(0.5), U32!(0))>();
}
```

## 複合型(Outside into Inside)
外部で定義した値をトレイトで受け取ることができます。<br>
optionやtupleなどもそのまま受け取ることができます。<br>
```rust
use const_struct::{const_struct, primitive::F64Ty};

pub fn tester<A: F64Ty>() {
    println!("a: {:?}", A::__DATA);
}

#[const_struct]
const PI: f64 = 3.14159265358979;

fn main() {
    tester::<PiTy>();
}
```

## 複合型(Outside into Inside/derive)
deriveで生成された構造体も受け取ることができます。<br>
```rust
use const_struct::{primitive::TupleTy, ConstStruct, F32};

#[derive(ConstStruct, Debug)]
pub struct TestSetting;

pub fn tester<A: TupleTy<(f32, TestSetting)>>() {
    println!("a: {:?}", A::__DATA);
}

fn main() {
    tester::<(F32!(0.5), macros::TestSetting!(TestSetting))>();
}
```

## 複合型(Outside into Inside/derive/generics)
deriveで生成されたジェネリクス構造体も受け取ることができます。<br>
```rust
use const_struct::{call_with_generics, const_struct, primitive::TupleTy, ConstStruct, F32};

#[derive(ConstStruct, Debug)]
pub struct TestSetting<const N: usize>;

pub fn tester<const N: usize, A: TupleTy<(f32, TestSetting<N>)>>() {
    println!("a: {:?}", A::__DATA);
}

#[const_struct]
const B: TestSetting<0> = TestSetting;

fn main() {
    tester::<0, (F32!(0.5), BTy)>();
    call_with_generics!(tester::<(F32!(0.5), macros::TestSetting!(BTy))>());
}
```

## Non derive
Deriveマクロが付いていない構造体にはかなりの制限があります。<br>
まず、const genericsを用いている構造体には使うことができません。これは、型がわからないためです。<br>
また、マクロを用いて、inner declaration constは行えません。<br>
PrimitiveTraitsを使うことで、型を直接受け取ることができます。<br>
メンバ変数がderiveを実装している必要はないため、既存の構造体をラップする簡単なラッパーを作ることで、一切の制限なく使うことができます。<br>
```rust
use const_struct::{const_struct, primitive::TupleTy, PrimitiveTraits};

#[derive(Debug)]
pub struct TestSetting;

pub fn tester<A: TupleTy<(TestSetting, )>>() {
    println!("a: {:?}", A::__DATA);
}

pub fn tester_alt<A: PrimitiveTraits<DATATYPE = TestSetting>>() {
    println!("a: {:?}", A::__DATA);
}

#[const_struct]
const B: TestSetting = TestSetting;

fn main() {
    tester::<(BTy, )>();
    tester_alt::<BTy>();
}
```

また、const genericsでないジェネリクスは使うことができます。<br>
ただし、call_with_generics!マクロを使うことはできません。<br>
これは、展開するべきジェネリクスの情報がないためです。<br>
```rust
use const_struct::{const_struct, primitive::TupleTy};

pub trait Float {}

impl Float for f32 {}

#[derive(Debug)]
pub struct TestSetting<F: Float> {
    a: F,
}

pub fn tester<F: Float + core::fmt::Debug + Copy, A: TupleTy<(TestSetting<F>, )>>() {
    println!("a: {:?}", A::__DATA);
}

#[const_struct]
const B: TestSetting<f32> = TestSetting { a: 0.5 };

fn main() {
    tester::<f32, (BTy, )>();
}
```

## パス指定
下記のように、`const_struct`を用いてパスを指定することができます。<br>
これを用いて、構造体の絶対パスを指定することで、別のモジュールまたは、外部のライブラリから構造体名のマクロを使用する際に、構造体をインポートする必要がなくなります。<br>
また、トレイトなどのパスは指定しなくて良いです。<br>
渡す値の内部を書き換えるわけではないため、下記のように、ジェネリクスとして渡す型のパスや、渡す値を作る際ははsuperなどを用いて指定する必要があります。<br>
```rust
use const_struct::{const_struct, ConstStruct};
use core::fmt::Debug;

#[derive(Debug, Copy, Clone)]
pub struct Float32;

pub trait Float {}

impl Float for Float32 {}

#[derive(ConstStruct, Debug)]
#[const_struct(
    TestSettingC: crate::test15::TestSettingC,
)]
pub struct TestSettingC<const N: usize, F: Float> {
    _a: F,
}

pub fn tester<const N: usize, F: Float + Copy + Debug, A: TestSettingCTy<N, F>>() {
    println!("a: {:?}", A::__DATA);
}

pub mod module {
    fn main() {
        const_struct::call_with_generics!(super::tester::<
            super::macros::TestSettingC!(
                super::Float32,
                super::TestSettingC::<7, super::Float32> { _a: super::Float32 }
            ),
        >());
    }
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
現在、ReduceMacroReclusionを行っていないため、何重にもした場合、recursion_limitエラーが発生する可能性があります。<br>
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
## マクロを使わない手動実装:未テスト
- Enumに対するConstStruct、アウトサイドマクロ、インサイドマクロ、etc

## 依存ライブラリを減らす: 15（自身を含めて）

## ドキュメントコメントの追加

# 開発者へ
## 気を付けるべきエラー
```rust
pointers cannot be cast to integers during const eval
at compile-time, pointers do not have an integer value
avoiding this restriction via `transmute`, `union`, or raw pointers leads to compile-time undefined behavior
```

```rust
constructing invalid value: encountered a dangling reference (0x48[noalloc] has no provenance)
```

```rust
destructor of `generics::TestStructWithFloatGenerics<T, S>` cannot be evaluated at compile-time
```

```rust
error: reached the recursion limit while instantiating `...`
```

```rust
the placeholder `_` is not allowed within types on item signatures for return types
```

```rust
can't capture dynamic environment in a fn item
```
