use async_std;
use applications::web;

fn main() -> std::result::Result<(), std::io::Error> {
    async_std::task::block_on(web::main())
}

