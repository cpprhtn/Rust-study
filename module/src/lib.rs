/*
//계층구조
module
 ├── network
 └── client
mod network {
    fn connect() {}
}
mod client {
    fn connect() {}
}


//계층구조
module
 └── network
     └── client
mod network {
    fn connect() {}

    mod client {
        fn connect() {}
    }
}


//계층구조
module
 ├── client
 └── network
     └── server
mod client {
    fn connect() {}
}

mod network {
    fn connect() {}

    mod server {
        fn connect() {}
    }
}
*/

//
mod client; // client 모듈을 다른 위치에서 찾으라는 의미.

mod network; // network 모듈을 다른 위치에서 찾으라는 의미.

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

