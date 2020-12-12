fn main() {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }

    // 아래 코드를 돌리려면..
    /*
    - 부모 모듈의 이름에 해당하는, network라는 이름의 새로운 디렉토리를 만드세요.
    - src/network.rs 파일을 이 새로운 network 디렉토리 안으로 옮기고, 파일 이름을 src/network/mod.rs로 고치세요.
    - 서브모듈 파일 src/server.rs를 network 디렉토리 안으로 옮기세요.
    */
    //mod server;
}
