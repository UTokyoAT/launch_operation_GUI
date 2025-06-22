#include <arduino.h>
#include <unity.h>
#include "Log.hpp"

void test_setup() {
    Log log(1.0, 2.0);
    uint8_t* bytes = log.serialize();
    Log deserialized_log = Log::deserialize(bytes);
    TEST_ASSERT_EQUAL(log.var1, deserialized_log.var1);
    TEST_ASSERT_EQUAL(log.var2, deserialized_log.var2);
    delete[] bytes;
}

void loop() {
    UNITY_BEGIN();
    RUN_TEST(test_setup);
    UNITY_END();
}

void setup() {
    
}