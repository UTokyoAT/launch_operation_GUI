use std::fmt::Debug;
use async_trait::async_trait;
use anyhow::Result;
///通信によって送受信される型に実装するトレイト
pub trait Sendable {
    ///バイナリに変換する
    fn serialize(&self) -> Vec<u8>;
    ///バイナリから元のデータを復元する
    fn deserialize(bytes: &Vec<u8>) -> Self;
    ///シリアライズするのに必要なバイト数
    fn serialized_size() -> usize;
}

#[async_trait]
///データを送信する型に実装するトレイト
pub trait Sender {
    async fn send<T: Sendable + Debug + Send>(&mut self, data: T) -> Result<()>;
}

#[async_trait]
///データを受信する型に実装するトレイト
pub trait Receiver {
    async fn receive<T: Sendable + Debug + Send>(&mut self) -> Result<T>;
}
