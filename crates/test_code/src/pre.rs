pub trait TestSettingManualTyImpl<A, B, C, D, TAIL> {
    const __A: A;
    const __B: B;
    const __C: C;
    const __D: D;
    const __TAIL: TAIL;
}

pub trait TestSettingManualTyImplData {
    type __A;
    type __B;
    type __C;
    type __D;
    type __TAIL;
}

pub struct DefaultNone;

pub enum TailSome<T> {
    Some(T),
    None,
}

impl <T: TestSettingManualTyImpl<A, B, C, D, TAIL>, A, B, C, D, TAIL> TestSettingManualTyImpl<A, B, C, D, TAIL> for TailSome<T> {
    const __A: A = T::__A;
    const __B: B = T::__B;
    const __C: C = T::__C;
    const __D: D = T::__D;
    const __TAIL: TAIL = T::__TAIL;
}

impl<T: TestSettingManualTyImplData> TestSettingManualTyImplData for TailSome<T> {
    type __A = T::__A;
    type __B = T::__B;
    type __C = T::__C;
    type __D = T::__D;
    type __TAIL = T::__TAIL;
}
