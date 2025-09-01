#include <Arduino.h>
#include "Log.hpp"

void setup() {
  // put your setup code here, to run once:
  Serial.begin(115200);
}

void loop() {
  // シリアルからデータを受信
  if (LogSerialInterface::can_receive()) {
    Log received_log = LogSerialInterface::receive();
    received_log.var1 += 10;
    received_log.var2 += 20;
    LogSerialInterface::send(received_log);
  }
}
