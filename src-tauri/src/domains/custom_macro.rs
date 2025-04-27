#[macro_export]
macro_rules! service_trait {
    ($n:ident$(: $el1:path$(, $eln:path)*)?) => {
        pub trait $n$(: Send + Sync + $el1 $(+ $eln)*)? {}
        impl<T$(: Send + Sync + $el1 $(+ $eln)?)?> $n for T {}
    };
}
