#![no_std]

pub mod transport;
pub mod ha;

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
mod tests {
   use crate::ha;

   #[test]
   fn codes_classify_test() {
      const GENERIC_REQ    : u16 = 0x0293u16;
      const GENERIC_RESP   : u16 = 0xFFEEu16;
      const INTERFACE_REQ  : u16 = 0x1492u16;
      const INTERFACE_RESP : u16 = 0x8736u16;

      let code_generic_req    = ha::MsgCode::from_u16(GENERIC_REQ);
      let code_generic_resp   = ha::MsgCode::from_u16(GENERIC_RESP);
      let code_interface_req  = ha::MsgCode::from_u16(INTERFACE_REQ);
      let code_interface_resp = ha::MsgCode::from_u16(INTERFACE_RESP);

      assert_eq!(code_generic_req,    ha::MsgCode::GenericRequest(GENERIC_REQ  ));
      assert_eq!(code_generic_resp,   ha::MsgCode::GenericResponse(GENERIC_RESP ));
      assert_eq!(code_interface_req,  ha::MsgCode::InterfaceRequest(INTERFACE_REQ));
      assert_eq!(code_interface_resp, ha::MsgCode::InterfaceResponse(INTERFACE_RESP));


      // Here codes are voluntarly inversed to test that the conversion back to u16 gives None
      let invalid_generic_req    = ha::MsgCode::GenericRequest(GENERIC_RESP  );
      let invalid_generic_resp   = ha::MsgCode::GenericResponse(GENERIC_REQ   );
      let invalid_interface_req  = ha::MsgCode::InterfaceRequest(INTERFACE_RESP);
      let invalid_interface_resp = ha::MsgCode::InterfaceResponse(INTERFACE_REQ );

      assert_eq!(invalid_generic_req.try_into_u16(),    None);
      assert_eq!(invalid_generic_resp.try_into_u16(),   None);
      assert_eq!(invalid_interface_req.try_into_u16(),  None);
      assert_eq!(invalid_interface_req.try_into_u16(),  None);
      assert_eq!(invalid_interface_resp.try_into_u16(), None);
   }

   #[test]
   fn frame_decode_test_generic_req() {
      const TEST_GENERIC_REQ: [u8;8] = [
         0x03, 0x29,             // Generic request code
         0x12, 0x34, 0x56, 0x78, // Data
         0xD4, 0x14,             // checksum
      ];

      let frame = ha::MsgFrame::from_slice(&TEST_GENERIC_REQ);
      println!("{:?}", frame);

      let frame = frame.unwrap(); // Ensure frame is OK

      assert_eq!(frame.code, ha::MsgCode::GenericRequest(0x329));
      assert_eq!(*frame.data, [0x12u8, 0x34u8, 0x56u8, 0x78u8]);
      assert_eq!(frame.crc(), 0xD414u16);
   }

   #[test]
   fn frame_encode_test_generic_resp() {
      let frame = ha::MsgFrame::new(
         0xFFFF,
         &[0x12u8, 0x34u8]
      );

      assert_eq!(frame.code.try_into_u16().unwrap().to_be_bytes(), [0xFF, 0xFF]);
      assert_eq!(*frame.data, [0x12, 0x34]);
      assert_eq!(frame.crc().to_be_bytes(), [0x13, 0xC6]);
   }
}
