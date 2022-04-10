use at_spi::*;

#[async_std::main]
async fn main() {
    println!("{:#?}", CONNECTION.await);
}
