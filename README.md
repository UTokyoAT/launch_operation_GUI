# launh_operation_GUI

開発中
マイコン-PC間の通信ができます。

## コード生成
通信に使うstructの生成ができます。
### コンフィグの書き方
データの名前の後に{}をおき、その中にcなどと同様、<型> <識別子>;を必要な数だけ並べる。
#### 例
```
log {
    float pressure;
    double speed;
}
```
#### 型
`float`:32 bit浮動小数点

`double`:64 bit浮動小数点

`i8`: 8 bit符号付き整数

`i16`: 16 bit符号付き整数

`i32`: 32 bit符号付き整数

`i64`: 64 bit符号付き整数

`u8`: 8 bit符号なし整数

`u16`: 16 bit符号なし整数

`u32`: 32 bit符号なし整数

`u64`: 64 bit符号なし整数

`bool`: 真偽値
### 実行
```bash
(cd code_generator; cargo run <code_generatorから見たコンフィグの相対パス>)
```
### 出力
launch_operation_GUI/src/generated_code/に生成される。
