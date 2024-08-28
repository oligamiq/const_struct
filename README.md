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
So this document may be out of date.

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
It's possible.<br>
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

## Structs (inside declaration const)
Note: Sometimes referred to as the "inside macro."
When the derive macro is applied, a macro with the same name is generated.
When using this macro, it is necessary to import the corresponding struct at the same time.
The member variables of the struct to which this macro is applied must implement the Copy trait. (As indicated by the third error to watch out for.)
By using #[const_struct(macro_export)], the macro can be made publicly available.
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
    tester::<TestSetting!(default())>();
}
```

## Structs (inside declaration const/generics)
In addition to the conditions mentioned, the Copy trait is automatically added to trait bounds.<br>
When generics are present, you must specify types in the order they are defined when invoking the struct macro.<br>
For const generics, if the value can be inferred from the provided arguments, you may use _ as a placeholder.<br>
If all const generics can be omitted, you don't need to write them.<br>
By using the call_with_generics! macro, you can omit const generics when they can be inferred.<br>
Since types are expanded in the order they are defined, you must specify the types in the same order when using the call_with_generics! macro.<br>
Non-const generic types cannot be omitted.<br> When you use #[const_struct] with a type like ???Ty, you can omit non-const generic types as well.<br>
Member variables need not implement the derive macro.<br>
The following is an example:<br>
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
    tester::<5, TestSetting!(5, TestSetting::<5>)>();
    tester::<5, TestSetting!(_, TestSetting::<5>)>();
    tester::<4, TestSetting!(4, TestSetting)>();
    tester::<9, TestSetting!(TestSetting::<9>)>();

    tester::<5, TestSetting!(B)>();
    tester::<5, BTy>();
    call_with_generics!(tester::<TestSetting!(B)>());
    call_with_generics!(tester::<5, BTy>());
    call_with_generics!(tester::<TestSetting!(_, BTy)>());
    call_with_generics!(tester::<TestSetting!(BTy)>());
}
```

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

## Composite Type (Outside into Inside)
You can pass values defined externally into a trait. Options, tuples, and other types can be passed as they are.
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

## Composite Types (Outside into Inside/derive)
It is possible to accept structs generated via derive.<br> For instance, consider the following example:
```rust
use const_struct::{primitive::TupleTy, ConstStruct, F32};

#[derive(ConstStruct, Debug)]
pub struct TestSetting;

pub fn tester<A: TupleTy<(f32, TestSetting)>>() {
    println!("a: {:?}", A::__DATA);
}

fn main() {
    tester::<(F32!(0.5), TestSetting!(TestSetting))>();
}
```

## Composite Types (Outside into Inside/derive/generics)
It is possible to accept structs with generics that are generated via derive. Here's an example:
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
    call_with_generics!(tester::<(F32!(0.5), TestSetting!(BTy))>());
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
## Manual Implementation Without Using Macros: Untested
- ConstStruct for enums, including outside macros, inside macros, etc., has not yet been tested.
## Reducing Dependency Libraries
- The current number of dependencies is 15. Efforts should be made to minimize this number.
## Add document comment.

# Notes for Developers
## Common Errors to Be Aware Of

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
