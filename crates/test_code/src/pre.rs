pub trait TestSettingManualTyImpl<A, B, C, D, E, F, G, H, TAIL> {
    const __A: A;
    const __B: B;
    const __C: C;
    const __D: D;
    const __E: E;
    const __F: F;
    const __G: G;
    const __H: H;
    const __TAIL: TAIL;
}

pub trait TestSettingManualTyImplData {
    type __A;
    type __B;
    type __C;
    type __D;
    type __E;
    type __F;
    type __G;
    type __H;
    type __TAIL;
}

pub struct DefaultNone;

pub enum TailSome<T> {
    Some(T),
    None,
}

impl <T: TestSettingManualTyImpl<A, B, C, D, E, F, G, H, TAIL>, A, B, C, D, E, F, G, H, TAIL> TestSettingManualTyImpl<A, B, C, D, E, F, G, H, TAIL> for TailSome<T> {
    const __A: A = T::__A;
    const __B: B = T::__B;
    const __C: C = T::__C;
    const __D: D = T::__D;
    const __E: E = T::__E;
    const __F: F = T::__F;
    const __G: G = T::__G;
    const __H: H = T::__H;
    const __TAIL: TAIL = T::__TAIL;
}

impl<T: TestSettingManualTyImplData> TestSettingManualTyImplData for TailSome<T> {
    type __A = T::__A;
    type __B = T::__B;
    type __C = T::__C;
    type __D = T::__D;
    type __E = T::__E;
    type __F = T::__F;
    type __G = T::__G;
    type __H = T::__H;
    type __TAIL = T::__TAIL;
}
