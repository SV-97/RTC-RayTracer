#[macro_export]
macro_rules! timeit {
    ( $a: expr ) => {{
        use std::time::Instant;
        let t1 = Instant::now();
        let temp = $a;
        let t2 = Instant::now();
        dbg!(t2.duration_since(t1));
        temp
    }};
}
