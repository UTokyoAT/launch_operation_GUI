use crate::traits::Sendable;

pub struct Log {
    
    var1: f32,
    
    var2: f64,
    
}

impl Sendable for Log {
    fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        bytes.extend_from_slice(&self.var1.to_le_bytes());
        
        bytes.extend_from_slice(&self.var2.to_le_bytes());
        
        bytes
    }

    fn deserialize(bytes: &Vec<u8>) -> Self {
        
        let var1 = f32::from_le_bytes(
            bytes[0..0 + 4].try_into().unwrap(),
        );
        
        let var2 = f64::from_le_bytes(
            bytes[4..4 + 8].try_into().unwrap(),
        );
        
        Log {
            
            var1,
            
            var2,
            
        }
    }

    fn serialized_size() -> usize {
        12
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Sendable;
    use rand::{self, Rng};

    #[test]
    fn test_reverse() {
        let mut rng = rand::rng();
        
        let var1: f32 = rng.random();
        
        let var2: f64 = rng.random();
        
        let data = Log {
            
            var1,
            
            var2,
            
        };
        let serialized = data.serialize();
        assert_eq!(serialized.len(), 12);
        let deserialized = Log::deserialize(&serialized);
        
        assert_eq!(data.var1, deserialized.var1);
        
        assert_eq!(data.var2, deserialized.var2);
        

    }
}
