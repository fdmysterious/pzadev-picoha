/// HA protocol generic types

use crc16;
use heapless::Vec;

#[derive(Debug,PartialEq)]
pub enum MsgCode {
   // Generic request code
   GenericRequest(u16),

   /// Specific interface request, may be processed by downstream interface implementation.
   InterfaceRequest(u16),

   /// Specific interface response code, may be processed by downstream interface implementation
   InterfaceResponse(u16),

   /// Generic response code
   GenericResponse(u16),
}

impl MsgCode {
   pub fn from_slice(ss: &[u8; 2]) -> Self {
      Self::from_u16(u16::from_be_bytes([ss[0], ss[1]]))
   }

   /// Categorize an incoming frame from its code
   pub fn from_u16(code: u16) -> Self {
      match code {
         // Generic requests
         0x0000 ..= 0x0FFF => Self::GenericRequest(code),
         0x1000 ..= 0x7FFF => Self::InterfaceRequest(code),
         0x8000 ..= 0xEFFF => Self::InterfaceResponse(code),
         0xF000 ..= 0xFFFF => Self::GenericResponse(code),
      }
   }

   /// Validate that the code in enum is good
   pub fn validate(&self) -> bool {
      match self {
         Self::GenericRequest(code) => match code {
            0x0000 ..= 0x0FFF => true,
            _                 => false,
         },

         Self::InterfaceRequest(code) => match code {
            0x1000 ..= 0x7FFF => true,
            _                 => false,
         },

         Self::InterfaceResponse(code) => match code {
            0x8000 ..= 0xEFFF => true,
            _                 => false,
         },

         Self::GenericResponse(code) => match code {
            0xF000 ..= 0xFFFF => true,
            _                 => false,
         }
      }
   }

   /// Returns the u16 code of the frame. Ensures that the returned
   /// code is in the correct range.
   pub fn try_into_u16(&self) -> Option<u16> {
      match self {
           Self::GenericRequest(code)
         | Self::InterfaceRequest(code)
         | Self::InterfaceResponse(code)
         | Self::GenericResponse(code) => match self.validate() {
            true  => Some(*code),
            false => None,
         }
      }
   }


   /// Returns wether the frame code designates a request/notification
   pub fn is_request(&self) -> bool {
      match self {
         Self::GenericRequest(_)  | Self::InterfaceRequest(_)  => true,
         Self::GenericResponse(_) | Self::InterfaceResponse(_) => false,
      }
   }

   /// Returns wether the frame code designates a response/acknowledge
   pub fn is_response(&self) -> bool {
      !self.is_request()
   }
}

#[derive(Debug)]
pub enum MsgError {
    InvalidLength,

    InvalidCRC(u16, u16),
    UnknownCode,
    InvalidArg,

    NotARequest(MsgCode),
}

#[derive(Debug,PartialEq)]
pub struct MsgFrame {
   pub code: MsgCode,
   pub data: Vec<u8, 64>,
}

impl MsgFrame {
   /// Creates a frame from a code and a slice of binary data (to build messages in app)
   pub fn new(code: u16, data: &[u8]) -> Self {
      Self {
         code: MsgCode::from_u16(code),
         data: Vec::from_slice(data).unwrap(), // TODO // Error if data is too large?
      }
   }

   /// Creates a frame from a slice of binary data (to process incoming messages)
   pub fn from_slice(ss: &[u8]) -> Result<Self, MsgError> {
      // Initial length check
      // 4: 2 for code + 2 for CRC

      if ss.len() < 4 {
         return Err(MsgError::InvalidLength)
      }

      // Compute and validate CRC
      let crc_frame = u16::from_be_bytes([ss[ss.len()-2], ss[ss.len()-1]]);

      let crc_real: u16 = crc16::State::<crc16::CCITT_FALSE>::calculate(
         &ss[..ss.len()-2]
      );

      if crc_real != crc_frame {
         return Err(MsgError::InvalidCRC(crc_real, crc_frame))
      }

      let code = MsgCode::from_slice(&ss[..2].try_into().unwrap());

      Ok(Self {
         code: code,
         data: match Vec::from_slice(&ss[2..ss.len()-2]) {
            Ok(x) => x,
            Err(_) => {return Err(MsgError::InvalidLength);},
         }
      })
   }

   pub fn to_slice(&self) -> &

   /// Compute the CRC of the frame
   pub fn crc(&self) -> u16 {
      let code_u16 = self.code.try_into_u16().unwrap();
      let mut crc  = crc16::State::<crc16::CCITT_FALSE>::new();

      crc.update(&code_u16.to_be_bytes());
      crc.update(self.data.as_slice());

      crc.get()
   }
}