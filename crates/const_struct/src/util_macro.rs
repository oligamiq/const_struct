#[macro_export]
macro_rules! match_underscore {
    (_, $tt_is_underscore:expr) => {
        $tt_is_underscore
    };
    ($input:expr, $tt_is_underscore:expr) => {
        $input
    };
}
