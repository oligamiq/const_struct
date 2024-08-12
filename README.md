# const_struct
<!-- [![Latest version](https://img.shields.io/crates/v/bitflags.svg)](https://crates.io/crates/bitflags) -->
<!-- [![Documentation](https://docs.rs/bitflags/badge.svg)](https://docs.rs/bitflags) -->
<!-- ![License](https://img.shields.io/crates/l/bitflags.svg) -->

`const_struct` is a macro that allows you to pass structs, f32, and other types in a manner similar to const generics. This is a different feature from `generic_const_expr`, and it is unclear whether there will be a standard replacement.<br>
https://github.com/rust-lang/rust/issues/76560<br>
https://hackmd.io/OZG_XiLFRs2Xmw5s39jRzA?view<br>

Official Const-related features are proposed here:
https://github.com/rust-lang/const-eval<br>

It can be used in a `no_std` environment.<br>
If additional features are needed, or if you find any bugs, please open an issue.<br>
It is currently under development, and the `main` branch may break.<br>
It can be made public, but there might be no compatibility between major versions.<br>
If compatibility is broken, the major version will be increased. However, at this point, there are no plans to increase the major version.
※This is Translated with ChatGPT [The original README.md](README-ja.md) is written in Japanese.

# Usage
When receiving a struct, add `Ty` to the end of its name.<br>
There are exceptions, but the macro and trait are designed so that nothing else needs to be imported.<br>
The data is stored in `__DATA`.<br>
For structs, you can directly access members by converting their names to camelCase.<br>
Additionally, the data extracted via `::__DATA` is type-inferred by the compiler.<br>
For more detailed code, please refer to `/crates/test_code/`.<br>
*The test code is complex to verify if it works even with unusual references.

## Primitive Types (inside declaration const)
Here, it may be referred to as the inside macro.<br>
When using primitive types, use the `primitive` module.<br>
When passing them, use the macro defined with a camelCase name.<br>
For primitive types, you can also access them using `VALUE`.<br>

```rust
use const_struct::{primitive::F32Ty, F32};

pub fn tester<A: F32Ty>() {
    println!("a: {:?}", A::__DATA);
}

fn main() {
    tester::<F32!(0.5)>();
}
```

## Structs (outside declaration const)
There are no restrictions on the values that can be defined.<br>
For structs, you can receive them by using derive(ConstStruct).<br>
To pass a value as a const, define it using the const_struct macro.<br>
The const_struct macro defines the name by converting it to camelCase and appending Ty.<br>
It is recommended to manage these collectively using files like setting.rs.

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

## Structs (outside declaration const/generics)
Generics are probably not yet supported (I forgot about them).

## Structs (inside declaration const)
Here, it may be referred to as the inside macro.<br>
The basic theory is established, but there are limitations.<br>
Since it expands a lot and there are many trait interactions, it is believed to affect compile time if used extensively.<br>

## Structs (inside declaration const/generics)
The basic theory is established, but there are limitations.<br>
Since it expands a lot and there are many trait interactions, it is believed to affect compile time if used extensively.<br>
The type passed as generics must implement the inside macro, and since it is used internally, it needs to be imported.<br>
You can use trait bounds, but since trait bounds are expanded internally to check the type of the received argument, it is recommended to specify the absolute path using $crate if using trait bounds. Otherwise, import errors may occur.<br>

## Composite Types(Option)
When receiving composite types, only the outermost type should have Ty appended.
Support for tuples is postponed and not yet available.
```rust
use const_struct::{primitive::OptionTy, F32, Some};

pub fn tester<A: OptionTy<Option<f32>>>() {
    println!("a: {:?}", A::__DATA);
}

fn main() {
    tester::<Some!(Some!(F32!(0.5)))>();
}
```

## Composite Types(Tuple)
When receiving a composite type, wrap it with `TupleTy`.
The maximum number of elements is 10.

```rust
use const_struct::{primitive::TupleTy, F32, F64, U32};

pub fn tester<A: TupleTy<(f32, f64, u32)>>() {
    println!("a: {:?}", A::__DATA);
}

fn main() {
    tester::<(F32!(0.5), F64!(0.5), U32!(0))>();
}
```

## ConstCompat
This is an attribute macro that changes normal functions to receive generics based on a cfg flag.<br>
It works by rewriting the internals, so it operates as is.<br>
In the first argument, specify the variable.
When the cfg flag is present, it calls the original function, so when the cfg flag is absent, it calls the original function.<br>
By adding another cfg flag, you can add cfg flags to the corresponding code.<br>
Currently, only regular functions are supported.<br>

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

# Unimplemented Features
## Manual Implementation Without Using Macros: Tested
- Inside macros for structs
- Inside macros for generics on structs

## Manual Implementation Without Using Macros: Untested
- Primitive types (outside macros)
- Generics in outside macros for structs
- ConstStruct, outside macros, and inside macros for enums
