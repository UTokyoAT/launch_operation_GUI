///通信によって送受信される型に実装するトレイト
pub trait Sendable {
    ///バイナリに変換する
    fn serialize(&self) -> Vec<u8>;
    ///バイナリから元のデータを復元する
    fn deserialize(bytes: &Vec<u8>) -> Self;
}

///データを送信する型に実装するトレイト
pub trait Sender<T: Sendable, E> {
    fn send(&mut self, data: T) -> Result<(), E>;
}

///データを受信する型に実装するトレイト
pub trait Receiver<T: Sendable, E> {
    ///データを受信していればそのデータを，していなければエラーを返す
    fn try_receive(&mut self) -> Result<T, E>;
}
