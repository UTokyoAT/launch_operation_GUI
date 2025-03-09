use std::fmt::Debug;
///通信によって送受信される型に実装するトレイト
pub trait Sendable {
    ///バイナリに変換する
    fn serialize(&self) -> Vec<u8>;
    ///バイナリから元のデータを復元する
    fn deserialize(bytes: &Vec<u8>) -> Self;
    ///シリアライズするのに必要なバイト数
    fn serialized_size() -> usize;
}

///データを送信する型に実装するトレイト
pub trait Sender<E> {
    fn send<T: Sendable + Debug>(&mut self, data: T) -> Result<(), E>;
}

///データを受信する型に実装するトレイト
pub trait Receiver<E> {
    ///データを受信していればそのデータを，していなければエラーを返す
    fn try_receive<T: Sendable + Debug>(&mut self) -> Result<T, E>;
}
