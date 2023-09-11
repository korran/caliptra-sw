/*++

Licensed under the Apache-2.0 license.

File Name:

    lms_kat.rs

Abstract:

    File contains the Known Answer Tests (KAT) for LMS cryptography operations.

--*/

use caliptra_drivers::{CaliptraError, CaliptraResult, Lms, LmsResult, Sha256Hw};
use caliptra_lms_types::{
    bytes_to_words_6, LmotsAlgorithmType, LmotsSignature, LmsAlgorithmType, LmsIdentifier,
    LmsPublicKey, LmsSignature,
};
use zerocopy::{BigEndian, LittleEndian, U32};

#[derive(Default, Debug)]
pub struct LmsKat {}

impl LmsKat {
    /// This function executes the Known Answer Tests (aka KAT) for LMS.
    pub fn execute(&self, sha256_driver: &mut Sha256Hw, lms: &Lms) -> CaliptraResult<()> {
        self.kat_lms_24(sha256_driver, lms)
    }

    fn kat_lms_24(&self, sha256_driver: &mut Sha256Hw, lms: &Lms) -> CaliptraResult<()> {
        const MESSAGE: [u8; 8] = [0, 0, 30, 76, 217, 179, 51, 230];
        const LMS_TYPE: LmsAlgorithmType = LmsAlgorithmType::LmsSha256N24H15;
        const LMOTS_TYPE: LmotsAlgorithmType = LmotsAlgorithmType::LmotsSha256N24W4;
        const Q: U32<BigEndian> = U32::ZERO;
        const LMS_IDENTIFIER: LmsIdentifier = [
            97, 165, 213, 125, 55, 245, 228, 107, 251, 117, 32, 128, 107, 7, 161, 184,
        ];
        const LMS_PUBLIC_HASH: [U32<LittleEndian>; 6] = bytes_to_words_6([
            0xf6, 0xb6, 0xac, 0x88, 0xd6, 0xbf, 0x10, 0x62, 0x13, 0x03, 0x85, 0x8a, 0x85, 0x4e,
            0x19, 0xcb, 0xbc, 0x7f, 0xb6, 0x3c, 0xeb, 0x80, 0x44, 0xfb,
        ]);
        const NONCE: [U32<LittleEndian>; 6] = [U32::ZERO; 6];

        const Y: [[U32<LittleEndian>; 6]; 51] = [
            bytes_to_words_6([
                0x0a, 0x03, 0xc9, 0x4b, 0xb4, 0xb3, 0x1f, 0x2f, 0xaa, 0x24, 0xd1, 0xbd, 0xcc, 0x4f,
                0xef, 0xda, 0x59, 0x83, 0x22, 0x10, 0xc5, 0xa4, 0x0e, 0x8f,
            ]),
            bytes_to_words_6([
                0xdc, 0xb4, 0x87, 0xc4, 0x5d, 0x23, 0xb6, 0x4d, 0x38, 0xf4, 0xc8, 0x09, 0xfd, 0x09,
                0xd2, 0xbd, 0xa8, 0xaa, 0x0f, 0xf6, 0x0f, 0xc4, 0x71, 0x81,
            ]),
            bytes_to_words_6([
                0xf9, 0x8f, 0xaf, 0x2c, 0x69, 0x31, 0x89, 0xd7, 0x1e, 0xdb, 0x87, 0xf6, 0xa5, 0x8c,
                0x01, 0x02, 0x77, 0xa7, 0x76, 0x67, 0xe3, 0x81, 0xf1, 0x38,
            ]),
            bytes_to_words_6([
                0xe8, 0x97, 0xe1, 0x09, 0xc2, 0x9a, 0xa3, 0x26, 0x54, 0xe4, 0x32, 0xda, 0x3b, 0x3e,
                0x25, 0x45, 0x4f, 0x6b, 0xce, 0xd7, 0x88, 0x07, 0xe9, 0x52,
            ]),
            bytes_to_words_6([
                0xf2, 0xf4, 0x48, 0x3d, 0xe4, 0x7b, 0xee, 0x24, 0xc8, 0x24, 0xb7, 0x66, 0xdb, 0x5d,
                0x50, 0x10, 0x2e, 0xbe, 0xa7, 0x56, 0x73, 0x8a, 0x10, 0x52,
            ]),
            bytes_to_words_6([
                0xfe, 0x65, 0x0e, 0xe9, 0xc5, 0x8e, 0x5c, 0xdd, 0xd0, 0x01, 0x70, 0xca, 0x0c, 0x31,
                0x35, 0x41, 0x01, 0x0c, 0x2c, 0xf2, 0xd0, 0x2c, 0x50, 0x44,
            ]),
            bytes_to_words_6([
                0x9c, 0x47, 0x81, 0x51, 0x5d, 0xaf, 0x9c, 0xb1, 0x2e, 0xe7, 0xec, 0xa4, 0x6f, 0x15,
                0x40, 0x6d, 0x2b, 0x7a, 0x3a, 0x68, 0xd1, 0xc1, 0xbc, 0x43,
            ]),
            bytes_to_words_6([
                0x94, 0x6c, 0xa9, 0x9f, 0x4d, 0xea, 0x75, 0x13, 0xf6, 0x0c, 0x29, 0x7d, 0x4f, 0xae,
                0x9b, 0x6c, 0x33, 0x45, 0x7d, 0x1e, 0xda, 0xdc, 0xf0, 0xcf,
            ]),
            bytes_to_words_6([
                0x70, 0xf8, 0x74, 0x81, 0xf4, 0x6a, 0xbc, 0x67, 0x64, 0x7d, 0x22, 0xf4, 0x5d, 0x50,
                0x10, 0x34, 0x5f, 0xa0, 0xf3, 0x5a, 0xe4, 0x4a, 0x31, 0x3a,
            ]),
            bytes_to_words_6([
                0x67, 0x64, 0x06, 0xf2, 0x97, 0xff, 0x53, 0x3e, 0x72, 0x12, 0x9e, 0x29, 0xbb, 0x4e,
                0xc9, 0xcb, 0xa6, 0xfa, 0x9b, 0x8c, 0xd1, 0xdb, 0x8d, 0x0d,
            ]),
            bytes_to_words_6([
                0xa8, 0xa7, 0x9c, 0x6f, 0xf1, 0xcd, 0x58, 0xa4, 0x83, 0xfc, 0xbd, 0x35, 0xb6, 0x33,
                0x2e, 0x9a, 0x89, 0x13, 0x94, 0xa5, 0xb6, 0x1f, 0xf3, 0x1a,
            ]),
            bytes_to_words_6([
                0x88, 0x65, 0xeb, 0x3e, 0x83, 0xf4, 0xed, 0xee, 0xd6, 0x15, 0x04, 0xc4, 0xbb, 0x5e,
                0x30, 0xab, 0x81, 0x00, 0xfa, 0x9f, 0x99, 0x55, 0x7e, 0xf9,
            ]),
            bytes_to_words_6([
                0xb3, 0x0b, 0x61, 0xa4, 0x0e, 0xa0, 0x75, 0xb4, 0x1e, 0x95, 0x80, 0x49, 0xa7, 0x4f,
                0xa4, 0xa5, 0xc4, 0x86, 0x54, 0xb8, 0xe0, 0x03, 0x61, 0x90,
            ]),
            bytes_to_words_6([
                0x07, 0x7c, 0x55, 0x0a, 0x59, 0xd5, 0xe4, 0xb4, 0xaa, 0x8a, 0x6b, 0xa6, 0x1b, 0x27,
                0x02, 0x03, 0x1b, 0xfb, 0x19, 0xf2, 0x90, 0x42, 0x84, 0xb0,
            ]),
            bytes_to_words_6([
                0xb5, 0xbc, 0x90, 0xa7, 0xcf, 0x1a, 0x4a, 0x5b, 0xc2, 0xf2, 0x7e, 0xdd, 0x03, 0x57,
                0x44, 0x67, 0x58, 0x9b, 0x2f, 0x72, 0xdd, 0x8c, 0x79, 0x5e,
            ]),
            bytes_to_words_6([
                0x2f, 0xcb, 0x15, 0xd7, 0x4c, 0x34, 0x73, 0x9b, 0x50, 0x3f, 0xd9, 0x0d, 0x5f, 0xdf,
                0x49, 0xe3, 0x5c, 0x53, 0x9c, 0x9e, 0xd1, 0x83, 0x85, 0xaa,
            ]),
            bytes_to_words_6([
                0xb6, 0x19, 0x6f, 0xb0, 0xc7, 0x8f, 0x06, 0xcc, 0xba, 0x32, 0x65, 0x46, 0xeb, 0x2c,
                0xaf, 0xb8, 0x92, 0x07, 0x8a, 0xd0, 0xa0, 0xe1, 0x07, 0x8a,
            ]),
            bytes_to_words_6([
                0x86, 0x37, 0xed, 0xdc, 0x0b, 0xce, 0xdb, 0x77, 0x11, 0x37, 0x0a, 0x16, 0x5f, 0xfe,
                0x0c, 0x67, 0x79, 0xd1, 0x54, 0x78, 0x40, 0x98, 0x60, 0x10,
            ]),
            bytes_to_words_6([
                0x12, 0x9c, 0x66, 0x89, 0x47, 0xbf, 0x4d, 0xea, 0x6c, 0x8b, 0xfd, 0xa7, 0x05, 0xfb,
                0x33, 0x8d, 0xf9, 0xc2, 0xe0, 0x2d, 0xfd, 0xb3, 0xe6, 0x1b,
            ]),
            bytes_to_words_6([
                0x2c, 0xae, 0x80, 0x41, 0x1e, 0x33, 0x49, 0x64, 0x5b, 0x55, 0x6b, 0xff, 0xea, 0xa8,
                0x89, 0x33, 0x9b, 0xc6, 0xa5, 0xa3, 0x14, 0x95, 0x96, 0x98,
            ]),
            bytes_to_words_6([
                0x31, 0x54, 0xa1, 0x95, 0x7a, 0x87, 0x93, 0x7f, 0x17, 0x4e, 0x2c, 0xb5, 0xfd, 0xff,
                0x62, 0xd9, 0xe0, 0xc8, 0xce, 0x45, 0x90, 0xcf, 0xb6, 0x8e,
            ]),
            bytes_to_words_6([
                0x2e, 0x87, 0x10, 0x6b, 0x25, 0xb7, 0x89, 0x33, 0x77, 0xaa, 0x71, 0xd6, 0x43, 0xe9,
                0xdb, 0x06, 0xab, 0xd9, 0xbd, 0x59, 0x47, 0x54, 0xcc, 0x54,
            ]),
            bytes_to_words_6([
                0xff, 0xd9, 0x58, 0xb3, 0x20, 0x35, 0xd8, 0x1f, 0xf5, 0xb2, 0xfd, 0x2b, 0xab, 0x3e,
                0xc8, 0x36, 0x74, 0x67, 0xe2, 0x43, 0xe5, 0xff, 0xb3, 0x62,
            ]),
            bytes_to_words_6([
                0x79, 0xf1, 0x4f, 0x99, 0x1a, 0xb3, 0x29, 0x3a, 0x36, 0x4d, 0xc5, 0xe7, 0xf0, 0xb1,
                0x04, 0x1c, 0x8e, 0x45, 0x58, 0xe9, 0x21, 0x45, 0xbb, 0x51,
            ]),
            bytes_to_words_6([
                0x0c, 0xf5, 0x05, 0x0a, 0xb0, 0x57, 0x8c, 0x1f, 0x9c, 0xd3, 0x96, 0x4d, 0x46, 0x41,
                0x9d, 0x88, 0x8b, 0xd8, 0x8d, 0x01, 0x73, 0x2c, 0x51, 0xeb,
            ]),
            bytes_to_words_6([
                0x4b, 0x03, 0xe5, 0xf2, 0x6c, 0x89, 0x40, 0x4e, 0xb1, 0x03, 0xe6, 0x1a, 0x85, 0x57,
                0x50, 0x0d, 0x7c, 0xaa, 0x2e, 0x70, 0xac, 0xde, 0x1f, 0xb4,
            ]),
            bytes_to_words_6([
                0xae, 0xe2, 0xa8, 0xfd, 0x0f, 0x78, 0x97, 0x6f, 0xcc, 0x27, 0x1c, 0x90, 0x98, 0x91,
                0x0b, 0xd2, 0x48, 0x5c, 0xff, 0xb1, 0x6d, 0xa0, 0x72, 0xcf,
            ]),
            bytes_to_words_6([
                0x51, 0x73, 0x8e, 0x2c, 0xf4, 0xf8, 0x6a, 0x6c, 0xc3, 0xe1, 0x30, 0x77, 0x37, 0xdc,
                0x75, 0x91, 0xa2, 0x59, 0x2d, 0x38, 0xe7, 0x43, 0x5c, 0xc7,
            ]),
            bytes_to_words_6([
                0x60, 0x06, 0x86, 0x5b, 0x90, 0x20, 0x82, 0xe1, 0x5c, 0x46, 0xbb, 0x70, 0x52, 0x97,
                0x90, 0x8f, 0x78, 0xc8, 0xfb, 0xe8, 0xe8, 0x93, 0x5d, 0x30,
            ]),
            bytes_to_words_6([
                0x20, 0xcc, 0xda, 0xf0, 0x33, 0x59, 0x85, 0xa0, 0xb6, 0xe6, 0x08, 0x0d, 0x2a, 0xf4,
                0x10, 0x38, 0x6b, 0x6e, 0xfc, 0x40, 0xef, 0xe9, 0xe7, 0x69,
            ]),
            bytes_to_words_6([
                0x1d, 0x16, 0xcf, 0xe2, 0x12, 0x80, 0xa5, 0x66, 0x1c, 0x11, 0x10, 0xc8, 0xfc, 0x62,
                0x67, 0xda, 0x33, 0x83, 0x2d, 0xe9, 0xdc, 0x04, 0xce, 0xc9,
            ]),
            bytes_to_words_6([
                0xe3, 0x6b, 0xa5, 0xc6, 0x4c, 0x67, 0xf8, 0xb0, 0xdc, 0xef, 0xe7, 0xd2, 0xcf, 0xb8,
                0xfa, 0xf1, 0xe8, 0x56, 0x73, 0x2a, 0xf5, 0x0a, 0x82, 0x3e,
            ]),
            bytes_to_words_6([
                0x25, 0x10, 0x89, 0x48, 0x2b, 0x75, 0x9c, 0xa8, 0xce, 0x35, 0x68, 0xb7, 0x13, 0xc6,
                0xa4, 0x0e, 0x1c, 0xad, 0xbd, 0x35, 0x4a, 0xe1, 0x1c, 0x33,
            ]),
            bytes_to_words_6([
                0x8c, 0x0d, 0x27, 0xb6, 0xb6, 0x02, 0x29, 0xee, 0x81, 0x63, 0x6a, 0xeb, 0xc0, 0x99,
                0x86, 0x22, 0x57, 0xfa, 0xf9, 0xbc, 0x1f, 0x58, 0x20, 0xf7,
            ]),
            bytes_to_words_6([
                0xd4, 0x27, 0x1d, 0x9f, 0x9b, 0x3d, 0x92, 0xa8, 0x8c, 0x0c, 0x76, 0x3e, 0xa9, 0xc3,
                0x7e, 0xd7, 0xc6, 0x79, 0x64, 0x0f, 0x83, 0x90, 0xdd, 0x97,
            ]),
            bytes_to_words_6([
                0x65, 0x2f, 0xa1, 0x29, 0x3e, 0xb8, 0xc1, 0xad, 0xf9, 0x01, 0xe9, 0x61, 0xaa, 0x1e,
                0x88, 0xdc, 0x0e, 0x58, 0x46, 0xba, 0x3f, 0xb1, 0x45, 0xc6,
            ]),
            bytes_to_words_6([
                0xb4, 0x69, 0x66, 0xd3, 0xdc, 0xd2, 0x4e, 0xc8, 0xbf, 0xf7, 0xa1, 0x40, 0x97, 0x37,
                0x9b, 0xe0, 0x70, 0x85, 0x12, 0x6b, 0x4d, 0x66, 0x33, 0x26,
            ]),
            bytes_to_words_6([
                0xa6, 0xe6, 0x4e, 0x71, 0x6e, 0x63, 0x35, 0x16, 0x9c, 0xfe, 0x20, 0x14, 0x9b, 0xee,
                0xcb, 0xc3, 0xdd, 0xfb, 0xf7, 0x99, 0xe3, 0xa0, 0x18, 0x7d,
            ]),
            bytes_to_words_6([
                0x6d, 0x36, 0x5e, 0xa6, 0xcc, 0xd8, 0x18, 0xad, 0x8a, 0xd8, 0x43, 0x62, 0xe5, 0x09,
                0x1e, 0xaf, 0xb4, 0xe5, 0x19, 0xdc, 0x18, 0x08, 0x5a, 0xe9,
            ]),
            bytes_to_words_6([
                0x57, 0x65, 0x1f, 0xff, 0x94, 0x8b, 0x38, 0x80, 0xb9, 0xcf, 0x08, 0x1d, 0x8d, 0x1b,
                0x36, 0xda, 0xb0, 0x59, 0x14, 0x58, 0x28, 0x1c, 0x30, 0x16,
            ]),
            bytes_to_words_6([
                0x2b, 0xa9, 0x94, 0x9a, 0x2a, 0x8a, 0xa0, 0x43, 0x2e, 0xf1, 0xf9, 0x98, 0x35, 0x6d,
                0x85, 0x5b, 0xa6, 0xbc, 0x62, 0x4f, 0x56, 0x6e, 0x42, 0xb3,
            ]),
            bytes_to_words_6([
                0x78, 0x6f, 0x52, 0xf5, 0x45, 0x39, 0xfa, 0x0d, 0x1d, 0x29, 0xf8, 0xe5, 0xd8, 0x8a,
                0xb4, 0x1d, 0x54, 0xe7, 0x91, 0x84, 0xd4, 0xfb, 0xcc, 0x83,
            ]),
            bytes_to_words_6([
                0xa2, 0x21, 0xbf, 0x3a, 0x08, 0x45, 0xf7, 0xbb, 0x1a, 0x05, 0x1d, 0x59, 0x59, 0xd5,
                0xea, 0x7b, 0xdf, 0x95, 0x3b, 0x37, 0x3b, 0x9a, 0x9d, 0xb2,
            ]),
            bytes_to_words_6([
                0x83, 0xd3, 0x4a, 0xea, 0x0e, 0x84, 0x24, 0xfe, 0x8a, 0x8c, 0x9f, 0xb2, 0x3b, 0xb7,
                0xd3, 0x4a, 0x6d, 0x07, 0xef, 0x9e, 0x76, 0xc7, 0xd9, 0xe0,
            ]),
            bytes_to_words_6([
                0xe6, 0x68, 0x81, 0xf9, 0x51, 0x8c, 0x7f, 0xae, 0xbf, 0x89, 0xa0, 0xc3, 0xcd, 0x55,
                0xe6, 0x9a, 0x3f, 0x93, 0xdc, 0x01, 0xc6, 0x1c, 0x4f, 0xc0,
            ]),
            bytes_to_words_6([
                0x88, 0xd0, 0xc9, 0x89, 0xec, 0xc6, 0x25, 0x19, 0xd3, 0x02, 0x83, 0x70, 0x50, 0xa7,
                0x31, 0x96, 0x62, 0xb2, 0xcb, 0x5d, 0x72, 0x5d, 0xcc, 0x96,
            ]),
            bytes_to_words_6([
                0x33, 0x2a, 0xe6, 0x80, 0x06, 0x87, 0x71, 0x13, 0xf9, 0x78, 0x1e, 0x4f, 0x61, 0xba,
                0x17, 0x9f, 0x04, 0x11, 0x67, 0xd1, 0xc2, 0xee, 0x29, 0x5e,
            ]),
            bytes_to_words_6([
                0xe5, 0x3d, 0xba, 0x38, 0x3f, 0xe4, 0xe5, 0x1f, 0x53, 0x6b, 0xb3, 0x23, 0x9c, 0x03,
                0x34, 0xae, 0x8f, 0x39, 0x72, 0xbd, 0x24, 0x25, 0x37, 0x9f,
            ]),
            bytes_to_words_6([
                0xd0, 0xb0, 0xd9, 0x2a, 0x78, 0xf9, 0xeb, 0xa9, 0x64, 0x8b, 0x3a, 0x9f, 0x3b, 0xca,
                0x44, 0xa3, 0xba, 0xf8, 0x36, 0x32, 0x6d, 0x42, 0x20, 0x6d,
            ]),
            bytes_to_words_6([
                0x5a, 0x8c, 0x8e, 0xb1, 0x8f, 0xbf, 0x5b, 0xf2, 0x8d, 0xa4, 0x9b, 0x7d, 0x2e, 0x89,
                0xc1, 0x57, 0x62, 0x45, 0x87, 0xd7, 0x28, 0x0b, 0x93, 0x82,
            ]),
            bytes_to_words_6([
                0x2f, 0xac, 0xcc, 0x4e, 0x1e, 0xfd, 0xb4, 0xbf, 0xe7, 0xe7, 0x73, 0xd8, 0x9f, 0x35,
                0xde, 0x74, 0x0c, 0x66, 0xa4, 0xc1, 0x43, 0x2d, 0xf7, 0x86,
            ]),
        ];

        const PATH: [[U32<LittleEndian>; 6]; 15] = [
            bytes_to_words_6([
                0x22, 0xa6, 0x0d, 0x20, 0x0a, 0x86, 0x38, 0xac, 0x4f, 0xe0, 0x98, 0x13, 0x34, 0x5b,
                0x3b, 0x89, 0xe5, 0x4c, 0xf3, 0x92, 0x1d, 0x07, 0x6b, 0x1f,
            ]),
            bytes_to_words_6([
                0xcb, 0xbf, 0xbd, 0x1a, 0x2a, 0xb3, 0x72, 0x15, 0x6a, 0x52, 0xb8, 0x81, 0xa3, 0x40,
                0xca, 0x6c, 0xd4, 0x58, 0xd5, 0xa0, 0xd9, 0x11, 0x0a, 0x1d,
            ]),
            bytes_to_words_6([
                0x46, 0xae, 0xa3, 0x45, 0x86, 0xe7, 0x18, 0xaf, 0x12, 0x6a, 0x9c, 0xa2, 0x2c, 0x3d,
                0x3c, 0xdf, 0x10, 0xa5, 0x6b, 0x90, 0xe2, 0xf2, 0x71, 0x6e,
            ]),
            bytes_to_words_6([
                0x4b, 0x34, 0xa2, 0x69, 0x79, 0x05, 0x1b, 0x29, 0x16, 0xea, 0xda, 0x7d, 0x6e, 0x53,
                0x48, 0x59, 0xbf, 0x02, 0x9c, 0x42, 0xdd, 0x79, 0xea, 0xcd,
            ]),
            bytes_to_words_6([
                0x68, 0xff, 0xa6, 0x67, 0x53, 0x0c, 0x99, 0x03, 0x65, 0x60, 0x38, 0xdc, 0x3a, 0x7f,
                0x80, 0x22, 0xb7, 0x52, 0x06, 0x8b, 0x73, 0x1c, 0x83, 0x50,
            ]),
            bytes_to_words_6([
                0xb5, 0x94, 0xc4, 0x6e, 0x11, 0x1f, 0x69, 0x11, 0x3b, 0x4d, 0x4f, 0xee, 0xca, 0x5b,
                0x91, 0x3b, 0x5a, 0x14, 0xe2, 0xda, 0x2a, 0xe0, 0x7a, 0x02,
            ]),
            bytes_to_words_6([
                0x56, 0x7c, 0x23, 0x36, 0x9a, 0x3c, 0x8b, 0x71, 0x79, 0xe7, 0x7e, 0xef, 0x0b, 0xd8,
                0x38, 0x3a, 0x87, 0x74, 0x49, 0xf2, 0x8f, 0xd4, 0x54, 0xf8,
            ]),
            bytes_to_words_6([
                0x9e, 0x54, 0x4b, 0xc0, 0xa6, 0x6d, 0xd3, 0x70, 0x7c, 0xad, 0x3c, 0xbc, 0xa1, 0xae,
                0x7c, 0xff, 0xa4, 0xcb, 0xa6, 0xd7, 0x3e, 0x90, 0x90, 0xed,
            ]),
            bytes_to_words_6([
                0xb1, 0x5f, 0x19, 0xe2, 0x03, 0x3b, 0xe4, 0x24, 0x61, 0x25, 0x12, 0x62, 0xc3, 0x0e,
                0xac, 0x30, 0x92, 0x79, 0xc6, 0x5b, 0x03, 0x61, 0x79, 0xb8,
            ]),
            bytes_to_words_6([
                0x85, 0x4d, 0xc9, 0xa5, 0x8d, 0x70, 0x8f, 0x83, 0x61, 0xa4, 0x83, 0x7b, 0x4a, 0x3e,
                0xfa, 0xdb, 0x8c, 0xa2, 0x3a, 0xaa, 0xb4, 0xcb, 0x69, 0xbe,
            ]),
            bytes_to_words_6([
                0x5a, 0x2d, 0xb9, 0xa9, 0x92, 0xd9, 0x7d, 0xf5, 0x3e, 0xd5, 0xcf, 0xba, 0x60, 0x6e,
                0xa5, 0x60, 0xf8, 0x1a, 0x66, 0x4f, 0x51, 0xce, 0x47, 0xf8,
            ]),
            bytes_to_words_6([
                0x86, 0xf6, 0x80, 0x56, 0x37, 0xc8, 0x9e, 0xea, 0x6f, 0x64, 0x2f, 0x19, 0x3b, 0x67,
                0xda, 0xfb, 0xd6, 0xfc, 0x4e, 0x87, 0x19, 0x15, 0x18, 0xc8,
            ]),
            bytes_to_words_6([
                0x85, 0x26, 0x32, 0xe9, 0x4e, 0xce, 0x78, 0xe3, 0xb9, 0xa4, 0x44, 0xf3, 0x19, 0x2e,
                0x1a, 0x51, 0xf7, 0x9d, 0x69, 0x7d, 0x82, 0xf9, 0x7c, 0xd1,
            ]),
            bytes_to_words_6([
                0x44, 0xe2, 0xa3, 0xe1, 0x26, 0x82, 0xc9, 0x70, 0x91, 0x51, 0x69, 0x33, 0x6b, 0x3b,
                0xb0, 0xda, 0x43, 0x30, 0xf5, 0x01, 0x0a, 0x3e, 0xc1, 0xbd,
            ]),
            bytes_to_words_6([
                0x33, 0x29, 0x53, 0x1c, 0xe7, 0x75, 0xdf, 0xdc, 0xf6, 0x64, 0x46, 0x28, 0x58, 0xfa,
                0x61, 0xa7, 0x97, 0xb8, 0x2a, 0xb1, 0x88, 0xde, 0xca, 0xf1,
            ]),
        ];

        const LMS_SIG: LmsSignature<6, 51, 15> = LmsSignature {
            q: Q,
            ots: LmotsSignature {
                ots_type: LMOTS_TYPE,
                nonce: NONCE,
                y: Y,
            },
            tree_type: LMS_TYPE,
            tree_path: PATH,
        };

        const LMS_PUBLIC_KEY: LmsPublicKey<6> = LmsPublicKey {
            id: LMS_IDENTIFIER,
            digest: LMS_PUBLIC_HASH,
            tree_type: LMS_TYPE,
            otstype: LMOTS_TYPE,
        };

        let success =
            lms.verify_lms_signature(sha256_driver, &MESSAGE, &LMS_PUBLIC_KEY, &LMS_SIG)?;
        if success != LmsResult::Success {
            Err(CaliptraError::ROM_KAT_LMS_DIGEST_MISMATCH)?;
        }

        Ok(())
    }
}
