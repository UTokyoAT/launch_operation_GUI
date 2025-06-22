#include <Arduino.h>
#include "Log.hpp"

void setup() {
  // put your setup code here, to run once:
  Serial.begin(115200);
  while (!Serial) {
    ; // シリアルポートが開くのを待つ
  }
}

void loop() {
  // シリアルからデータを受信
  if (Serial.available() >= static_cast<int>(Log::serialized_size())) {
    uint8_t received_bytes[Log::serialized_size()];

    // 必要なバイト数を読み込む
    for (size_t i = 0; i < Log::serialized_size(); i++) {
      while (!Serial.available()) {
        ; // データが利用可能になるのを待つ
      }
      received_bytes[i] = Serial.read();
    }

    // 受信したデータをLogオブジェクトに変換
    Log received_log = Log::deserialize(received_bytes);

    // データを再度シリアライズして送信
    uint8_t* response_bytes = received_log.serialize();
    Serial.write(response_bytes, Log::serialized_size());
    delete[] response_bytes; // メモリリークを防ぐ
  }
}
