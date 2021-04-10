use core::convert::TryFrom;

use crate::{error, Result};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum EntityType {
    Coil,
    DiscreteInput,
    InputRegister = 3,
    HoldingRegister,
}

impl From<EntityType> for char {
    fn from(et: EntityType) -> Self {
        match et {
            EntityType::Coil => '0',
            EntityType::DiscreteInput => '1',
            EntityType::InputRegister => '3',
            EntityType::HoldingRegister => '4',
        }
    }
}

/**
entity numbers are the long form ids of inputs/coils/registers
always 5 or 6 digits

The first digit identifies the type:
- 0 => coil
- 1 => discrete input
- 3 => input register
- 4 => holding register

The remaining digits identify the location in a 1-indexed format
- NOTE: address is 0-indexed

examples:
- 30001 -> input register at address 99
- 30100 -> input register at address 99
- 40001 -> holding register at address 0
- 40100 -> holding register at address 99
*/
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Entity {
    t: EntityType,
    address: u16,
}

impl Entity {
    pub fn from_address(t: EntityType, address: u16) -> Entity {
        Entity { t, address }
    }

    /// type decides which functions are valid for this entity
    pub fn etype(&self) -> EntityType {
        self.t.clone()
    }

    /// address is used in commands directly
    pub fn address(&self) -> u16 {
        self.address
    }

    /// location in 1-indexed address
    pub fn location(&self) -> u32 {
        self.address as u32 + 1
    }

    /// 5 digit entity encoding, supports addresses in range \[0, 9998\]
    pub fn encode_to_str<'a>(&self, bytes: &'a mut [u8]) -> Result<&'a [u8]> {
        if bytes.len() < 5 || self.address > 9998 {
            // buffer too short to encode the bytes
            Err(error::Error::InvalidLength)
        } else {
            bytes[0] = char::from(self.t.clone()) as u8;
            encode_decimal_to(self.address + 1, &mut bytes[1..5]);
            Ok(&bytes[0..5])
        }
    }

    /// 6 digit entity encoding, supports addresses in range \[0, 65535\]
    pub fn encode_to_str_ext<'a>(&self, bytes: &'a mut [u8]) -> Result<&'a [u8]> {
        if bytes.len() < 6 {
            Err(error::Error::InvalidLength)
        } else {
            bytes[0] = char::from(self.t.clone()) as u8;
            encode_decimal_to(self.address + 1, &mut bytes[1..6]);
            Ok(&bytes[0..6])
        }
    }
}

fn encode_decimal_to(mut val: u16, out: &mut [u8]) {
    for c in out.iter_mut().rev() {
        *c = (val % 10) as u8 + '0' as u8;
        val /= 10;
    }
    // assert that the length was enough to encode the value
    debug_assert!(val == 0);
}

/**
Succeeds if entity string is valid
- starts with 0/1/3/4 (valid entity types)
- has 4 or 5 decimal digits following, with a value in the range 1-65536

```rust
use core::convert::TryFrom;
use modbus_frames as modbus;
use modbus::entity::{Entity, EntityType};
assert_eq!(
    Entity::try_from("40001"),
    Ok(Entity::from_address(EntityType::HoldingRegister, 0))
);
```
 */
impl TryFrom<&str> for Entity {
    type Error = error::Error;

    fn try_from(value: &str) -> Result<Self> {
        if value.len() != 5 && value.len() != 6 {
            return Err(Self::Error::InvalidLength);
        }
        let bytes = value.as_bytes();
        // first byte is the type
        match bytes[0] as char {
            '0' => Ok(EntityType::Coil),
            '1' => Ok(EntityType::DiscreteInput),
            '3' => Ok(EntityType::InputRegister),
            '4' => Ok(EntityType::HoldingRegister),
            _ => Err(Self::Error::InvalidEncoding),
        }
        // rest is location
        .and_then(|et| {
            // ASCII to number
            let mut address: u32 = 0; // must accumulate to u32 because max location is 65536 (u16::MAX + 1)
            for &b in &bytes[1..] {
                const MIN: u8 = '0' as u8;
                const MAX: u8 = '9' as u8;
                if b < MIN || b > MAX {
                    return Err(Self::Error::InvalidEncoding);
                }
                address *= 10;
                address += (b - MIN) as u32;
            }
            if address == 0 || address > 65536 {
                // location cannot be 0
                Err(Self::Error::InvalidEncoding)
            } else {
                Ok(Entity::from_address(et, (address - 1) as u16))
            }
        })
    }
}

#[cfg(test)]
mod test {
    use core::convert::TryFrom;

    use super::{Entity, EntityType};
    use crate::error;

    #[test]
    fn test_entity_encode() {
        let entity = Entity::from_address(EntityType::HoldingRegister, 1876);
        let mut buffer = [0; 8];

        assert_eq!(entity.encode_to_str(&mut buffer), Ok("41877".as_bytes()));
        assert_eq!(
            entity.encode_to_str_ext(&mut buffer),
            Ok("401877".as_bytes())
        );

        let entity = Entity::from_address(EntityType::DiscreteInput, 3);
        assert_eq!(entity.encode_to_str(&mut buffer), Ok("10004".as_bytes()));
        assert_eq!(
            entity.encode_to_str_ext(&mut buffer),
            Ok("100004".as_bytes())
        );
        // buffer is too small, returns error
        assert_eq!(
            entity.encode_to_str(&mut buffer[..4]),
            Err(error::Error::InvalidLength)
        );
        assert_eq!(
            entity.encode_to_str_ext(&mut buffer[..5]),
            Err(error::Error::InvalidLength)
        );
    }

    #[test]
    fn test_entity_from_str() {
        assert_eq!(
            Ok(Entity::from_address(EntityType::Coil, 0)),
            Entity::try_from("00001")
        );
        assert_eq!(
            Ok(Entity::from_address(EntityType::DiscreteInput, 32)),
            Entity::try_from("10033")
        );
        assert_eq!(
            Ok(Entity::from_address(EntityType::InputRegister, 10032)),
            Entity::try_from("310033")
        );
        assert_eq!(
            Ok(Entity::from_address(EntityType::HoldingRegister, 65535)),
            Entity::try_from("465536")
        );
        // errors
        assert_eq!(
            Err(error::Error::InvalidEncoding),
            Entity::try_from("90001") // unknown type
        );
        assert_eq!(
            Err(error::Error::InvalidEncoding),
            Entity::try_from("1a001") // invalid character in location
        );
        assert_eq!(
            Err(error::Error::InvalidEncoding),
            Entity::try_from("10000") // 1-65536 is the allowed range of locations
        );
        assert_eq!(
            Err(error::Error::InvalidEncoding),
            Entity::try_from("165537") // 1-65536 is the allowed range of locations
        );
    }

    #[test]
    fn test_entity_location() {
        let test_ent = Entity::try_from("40001").unwrap();
        assert_eq!(test_ent.address(), 0);
        assert_eq!(test_ent.location(), 1);

        let test_ent = Entity::try_from("465536").unwrap();
        assert_eq!(test_ent.address(), 65535u16);
        assert_eq!(test_ent.location(), 65536);
    }
}
