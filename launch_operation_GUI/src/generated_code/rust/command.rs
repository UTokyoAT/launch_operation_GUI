use crate::traits::Sendable;

pub struct command {
    
    var1: u8,
    
}

impl Sendable for command {
    fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        bytes.extend_from_slice(&self.var1.to_le_bytes());
        
        bytes
    }

    fn deserialize(bytes: &Vec<u8>) -> Self {
        
        let var1 = u8::from_le_bytes(
            bytes[0..0 + 1].try_into().unwrap(),
        );
        
        command {
            
            var1,
            
        }
    }

    fn serialized_size() -> usize {
        1
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
        
        let var1: u8 = rng.random();
        
        let data = command {
            
            var1,
            
        };
        let serialized = data.serialize();
        assert_eq!(serialized.len(), 1);
        let deserialized = command::deserialize(&serialized);
        
        assert_eq!(data.var1, deserialized.var1);
        

    }
}
