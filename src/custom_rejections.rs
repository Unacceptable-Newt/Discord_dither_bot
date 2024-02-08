use warp::reject;

#[derive(Debug)]
    pub struct InvalidSignautre;
    impl reject::Reject for InvalidSignautre {}

#[derive(Debug)]
    pub struct BadSignautre;
    impl reject::Reject for BadSignautre {}

#[derive(Debug)]
    pub struct InvalidTimestamp;
    impl reject::Reject for InvalidTimestamp {}

#[derive(Debug)]
    pub struct BadBodyError;
    impl reject::Reject for BadBodyError {}
