#pragma once

#include <Arduino.h>
#include <string.h>

struct Log {
    
    float var1;
    
    double var2;
    

    Log( float var1,  double var2) : var1(var1),  var2(var2) {}

    uint8_t* serialize() const {
        uint8_t* bytes = new uint8_t[12];
        
        const uint8_t* ptr_var1 = reinterpret_cast<const uint8_t*>(&var1);
        memcpy(&bytes[0], ptr_var1, sizeof(float));
        
        const uint8_t* ptr_var2 = reinterpret_cast<const uint8_t*>(&var2);
        memcpy(&bytes[4], ptr_var2, sizeof(double));
        
        return bytes;
    }

    static Log deserialize(const uint8_t* bytes) {
        
        float var1;
        memcpy(&var1, &bytes[0], sizeof(float));
        
        double var2;
        memcpy(&var2, &bytes[4], sizeof(double));
        
        Log result(var1, var2);
        return result;
    }

    static constexpr size_t serialized_size() {
        return 12;
    }
};