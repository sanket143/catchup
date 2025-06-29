#[macro_export]
macro_rules! warp_err {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => {
                eprintln!("{:?}", err);
                return Err(warp::reject());
            }
        }
    };
}
