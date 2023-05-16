/*++

Licensed under the Apache-2.0 license.

File Name:

    lms_24_tests.rs

Abstract:

    File contains test cases for LMS signature verification using SHA256/192.

--*/

#![no_std]
#![no_main]

use caliptra_drivers::{
    get_lms_parameters, hash_message, lookup_lmots_algorithm_type, lookup_lms_algorithm_type,
    verify_lms_signature, HashValue, LmotsAlgorithmType, LmotsSignature, LmsAlgorithmType,
    LmsIdentifier, LmsSignature, Sha192Digest, Sha256,
};
use caliptra_registers::sha256::Sha256Reg;
use caliptra_test_harness::test_suite;

fn test_lms_lookup() {
    let result = lookup_lms_algorithm_type(0);
    assert_eq!(LmsAlgorithmType::LmsReserved, result.unwrap())
}

fn test_get_lms_parameters() {
    // Full size SHA256 hashes
    let (width, height) = get_lms_parameters(&LmsAlgorithmType::LmsSha256N32H5).unwrap();
    assert_eq!(32, width);
    assert_eq!(5, height);
    let (width, height) = get_lms_parameters(&LmsAlgorithmType::LmsSha256N32H10).unwrap();
    assert_eq!(32, width);
    assert_eq!(10, height);
    let (width, height) = get_lms_parameters(&LmsAlgorithmType::LmsSha256N32H15).unwrap();
    assert_eq!(32, width);
    assert_eq!(15, height);
    let (width, height) = get_lms_parameters(&LmsAlgorithmType::LmsSha256N32H20).unwrap();
    assert_eq!(32, width);
    assert_eq!(20, height);
    let (width, height) = get_lms_parameters(&LmsAlgorithmType::LmsSha256N32H25).unwrap();
    assert_eq!(32, width);
    assert_eq!(25, height);

    // Truncated 192 bit SHA256 hashes
    let (width, height) = get_lms_parameters(&LmsAlgorithmType::LmsSha256N24H5).unwrap();
    assert_eq!(24, width);
    assert_eq!(5, height);
    let (width, height) = get_lms_parameters(&LmsAlgorithmType::LmsSha256N24H10).unwrap();
    assert_eq!(24, width);
    assert_eq!(10, height);
    let (width, height) = get_lms_parameters(&LmsAlgorithmType::LmsSha256N24H15).unwrap();
    assert_eq!(24, width);
    assert_eq!(15, height);
    let (width, height) = get_lms_parameters(&LmsAlgorithmType::LmsSha256N24H20).unwrap();
    assert_eq!(24, width);
    assert_eq!(20, height);
    let (width, height) = get_lms_parameters(&LmsAlgorithmType::LmsSha256N24H25).unwrap();
    assert_eq!(24, width);
    assert_eq!(25, height);
}

fn test_lmots_lookup() {
    let result = lookup_lmots_algorithm_type(0);
    assert_eq!(LmotsAlgorithmType::LmotsReserved, result.unwrap())
}

fn test_hash_message_24() {
    let mut sha256 = unsafe { Sha256::new(Sha256Reg::new()) };
    let message: [u8; 33] = [
        116, 104, 105, 115, 32, 105, 115, 32, 116, 104, 101, 32, 109, 101, 115, 115, 97, 103, 101,
        32, 73, 32, 119, 97, 110, 116, 32, 115, 105, 103, 110, 101, 100,
    ];
    let lms_identifier: [u8; 16] = [
        102, 40, 233, 90, 126, 166, 161, 73, 107, 57, 114, 28, 121, 57, 28, 123,
    ];
    let nonce: [u8; 24] = [
        108, 201, 169, 93, 130, 206, 214, 173, 223, 138, 178, 150, 192, 86, 115, 139, 157, 213,
        182, 55, 196, 22, 212, 216,
    ];
    let q: u32 = 0;
    let q_str = q.to_be_bytes();
    let expected_hash = HashValue::from([
        175, 160, 9, 71, 29, 26, 61, 20, 90, 217, 142, 152, 112, 68, 51, 17, 154, 191, 74, 150,
        161, 238, 102, 161,
    ]);
    let hash = hash_message(&mut sha256, &message, &lms_identifier, &q_str, &nonce).unwrap();
    assert_eq!(expected_hash, hash);
}

fn test_lms_24_height_15() {
    let mut sha256 = unsafe { Sha256::new(Sha256Reg::new()) };

    let message: [u8; 33] = [
        116, 104, 105, 115, 32, 105, 115, 32, 116, 104, 101, 32, 109, 101, 115, 115, 97, 103, 101,
        32, 73, 32, 119, 97, 110, 116, 32, 115, 105, 103, 110, 101, 100,
    ];
    let lms_identifier: [u8; 16] = [
        158, 20, 249, 74, 242, 177, 66, 175, 101, 91, 176, 36, 80, 31, 240, 7,
    ];
    let q: u32 = 0;
    let lmots_type = LmotsAlgorithmType::LmotsSha256N24W4;
    let lms_type = LmsAlgorithmType::LmsSha256N24H15;
    let lms_public_key = HashValue::from([
        3, 42, 162, 189, 155, 49, 233, 189, 51, 75, 70, 46, 39, 121, 32, 117, 189, 173, 221, 174,
        249, 237, 177, 36,
    ]);
    let nonce: [u8; 24] = [
        180, 36, 9, 219, 221, 74, 28, 73, 252, 121, 55, 148, 117, 233, 199, 103, 28, 127, 81, 83,
        247, 83, 90, 196,
    ];
    let y = [
        Sha192Digest::from([
            114, 83, 175, 105, 200, 90, 91, 150, 16, 85, 204, 3, 183, 225, 238, 131, 171, 176, 50,
            179, 20, 88, 250, 105,
        ]),
        Sha192Digest::from([
            0, 212, 244, 252, 218, 53, 125, 201, 169, 68, 16, 35, 61, 75, 0, 180, 185, 44, 168,
            110, 240, 248, 253, 19,
        ]),
        Sha192Digest::from([
            210, 173, 126, 3, 236, 50, 192, 89, 143, 155, 100, 253, 140, 111, 130, 121, 247, 142,
            136, 231, 123, 76, 219, 137,
        ]),
        Sha192Digest::from([
            108, 228, 158, 102, 59, 50, 107, 41, 29, 229, 201, 219, 223, 171, 5, 104, 29, 181, 134,
            104, 30, 128, 230, 175,
        ]),
        Sha192Digest::from([
            186, 149, 143, 190, 28, 131, 190, 78, 26, 210, 63, 14, 14, 151, 166, 176, 232, 0, 243,
            206, 151, 181, 252, 176,
        ]),
        Sha192Digest::from([
            148, 158, 87, 237, 101, 185, 92, 44, 185, 187, 76, 132, 78, 78, 76, 227, 31, 99, 241,
            43, 1, 93, 53, 188,
        ]),
        Sha192Digest::from([
            173, 239, 177, 238, 58, 178, 196, 109, 12, 59, 82, 77, 146, 64, 237, 241, 204, 199, 9,
            167, 249, 120, 85, 19,
        ]),
        Sha192Digest::from([
            247, 140, 243, 204, 21, 225, 185, 177, 113, 169, 47, 38, 51, 71, 89, 92, 36, 242, 213,
            190, 174, 166, 151, 147,
        ]),
        Sha192Digest::from([
            74, 82, 153, 254, 76, 126, 108, 131, 48, 159, 152, 192, 94, 195, 214, 39, 157, 51, 80,
            129, 239, 167, 72, 49,
        ]),
        Sha192Digest::from([
            107, 52, 117, 125, 249, 26, 114, 57, 175, 242, 107, 70, 94, 196, 128, 158, 34, 34, 155,
            238, 121, 172, 144, 17,
        ]),
        Sha192Digest::from([
            172, 179, 238, 166, 66, 48, 179, 212, 235, 84, 26, 173, 167, 181, 109, 68, 21, 147,
            129, 122, 28, 10, 71, 59,
        ]),
        Sha192Digest::from([
            2, 2, 149, 184, 96, 65, 100, 185, 190, 219, 17, 239, 176, 57, 67, 158, 136, 163, 14,
            93, 154, 244, 128, 105,
        ]),
        Sha192Digest::from([
            22, 202, 169, 34, 236, 93, 51, 11, 9, 84, 163, 23, 141, 28, 216, 189, 210, 140, 100,
            252, 7, 158, 216, 35,
        ]),
        Sha192Digest::from([
            188, 122, 187, 66, 118, 218, 16, 88, 162, 60, 244, 0, 8, 99, 234, 32, 4, 91, 226, 242,
            184, 220, 126, 207,
        ]),
        Sha192Digest::from([
            11, 48, 194, 18, 142, 165, 55, 185, 14, 118, 75, 58, 73, 121, 214, 109, 103, 48, 113,
            144, 144, 219, 137, 91,
        ]),
        Sha192Digest::from([
            97, 187, 195, 106, 133, 55, 105, 76, 35, 79, 90, 17, 229, 195, 13, 165, 57, 123, 127,
            124, 135, 244, 236, 220,
        ]),
        Sha192Digest::from([
            214, 99, 87, 219, 160, 8, 161, 135, 138, 137, 42, 88, 12, 90, 114, 122, 242, 3, 22, 28,
            19, 84, 20, 201,
        ]),
        Sha192Digest::from([
            62, 224, 247, 169, 52, 197, 210, 43, 245, 147, 5, 3, 170, 217, 184, 109, 121, 126, 249,
            234, 206, 13, 57, 158,
        ]),
        Sha192Digest::from([
            111, 128, 183, 62, 154, 70, 169, 35, 17, 9, 161, 84, 29, 247, 33, 54, 19, 135, 63, 115,
            182, 185, 184, 202,
        ]),
        Sha192Digest::from([
            126, 102, 196, 148, 117, 216, 193, 126, 234, 244, 162, 43, 30, 156, 15, 116, 252, 90,
            176, 226, 22, 186, 84, 117,
        ]),
        Sha192Digest::from([
            176, 130, 86, 150, 54, 220, 191, 253, 216, 234, 150, 85, 183, 139, 58, 153, 29, 50,
            215, 242, 150, 122, 216, 116,
        ]),
        Sha192Digest::from([
            213, 57, 136, 146, 251, 212, 93, 186, 102, 167, 197, 1, 70, 242, 41, 124, 60, 39, 172,
            216, 140, 224, 16, 139,
        ]),
        Sha192Digest::from([
            209, 80, 45, 106, 121, 180, 147, 197, 53, 0, 194, 54, 186, 38, 171, 173, 143, 87, 145,
            35, 230, 193, 14, 201,
        ]),
        Sha192Digest::from([
            244, 160, 96, 211, 226, 133, 43, 154, 217, 127, 228, 180, 88, 112, 51, 138, 63, 204,
            71, 177, 241, 209, 12, 210,
        ]),
        Sha192Digest::from([
            253, 40, 21, 189, 33, 221, 10, 234, 120, 172, 11, 230, 217, 177, 52, 224, 194, 80, 115,
            217, 66, 91, 234, 78,
        ]),
        Sha192Digest::from([
            142, 45, 153, 40, 242, 62, 139, 243, 237, 98, 143, 248, 136, 57, 110, 116, 158, 85,
            174, 102, 245, 154, 132, 108,
        ]),
        Sha192Digest::from([
            127, 196, 123, 139, 102, 213, 211, 220, 71, 172, 127, 40, 88, 185, 59, 160, 70, 164,
            110, 130, 107, 143, 58, 169,
        ]),
        Sha192Digest::from([
            106, 155, 152, 117, 70, 4, 234, 124, 188, 200, 185, 180, 186, 185, 67, 218, 207, 96,
            33, 156, 177, 212, 237, 103,
        ]),
        Sha192Digest::from([
            28, 50, 10, 247, 174, 132, 131, 117, 235, 156, 199, 176, 236, 48, 69, 190, 121, 253,
            17, 124, 205, 38, 151, 94,
        ]),
        Sha192Digest::from([
            60, 45, 74, 53, 46, 16, 60, 61, 118, 137, 179, 172, 242, 204, 86, 208, 237, 122, 111,
            88, 118, 236, 64, 150,
        ]),
        Sha192Digest::from([
            26, 90, 173, 140, 225, 8, 167, 203, 59, 241, 27, 1, 28, 182, 14, 71, 243, 69, 135, 243,
            247, 149, 71, 114,
        ]),
        Sha192Digest::from([
            134, 229, 36, 166, 13, 250, 239, 130, 252, 108, 141, 161, 129, 149, 133, 88, 147, 39,
            246, 41, 105, 201, 119, 183,
        ]),
        Sha192Digest::from([
            233, 74, 233, 191, 174, 66, 20, 147, 252, 183, 20, 56, 71, 47, 13, 3, 124, 130, 67,
            225, 110, 41, 117, 63,
        ]),
        Sha192Digest::from([
            212, 156, 195, 221, 197, 89, 123, 35, 135, 231, 3, 169, 154, 201, 151, 115, 19, 250,
            167, 25, 91, 65, 218, 114,
        ]),
        Sha192Digest::from([
            108, 224, 2, 164, 233, 39, 114, 244, 234, 116, 244, 233, 9, 191, 128, 40, 253, 215,
            127, 138, 9, 192, 96, 81,
        ]),
        Sha192Digest::from([
            25, 199, 185, 136, 112, 88, 213, 69, 107, 186, 60, 98, 128, 39, 200, 141, 247, 168,
            247, 169, 254, 245, 166, 65,
        ]),
        Sha192Digest::from([
            154, 91, 105, 237, 196, 172, 129, 152, 31, 235, 64, 184, 199, 169, 167, 109, 28, 90,
            129, 114, 23, 203, 168, 248,
        ]),
        Sha192Digest::from([
            92, 103, 184, 153, 111, 137, 218, 113, 32, 174, 94, 230, 44, 22, 171, 89, 28, 129, 181,
            130, 198, 136, 111, 110,
        ]),
        Sha192Digest::from([
            127, 202, 249, 32, 173, 214, 230, 13, 137, 185, 249, 161, 50, 207, 105, 187, 248, 115,
            245, 128, 201, 105, 99, 244,
        ]),
        Sha192Digest::from([
            1, 157, 13, 71, 35, 221, 198, 100, 215, 125, 204, 77, 95, 91, 109, 20, 166, 154, 226,
            43, 54, 60, 97, 72,
        ]),
        Sha192Digest::from([
            127, 226, 179, 203, 161, 35, 43, 47, 148, 46, 14, 51, 4, 64, 212, 211, 27, 104, 220,
            228, 131, 74, 215, 40,
        ]),
        Sha192Digest::from([
            240, 69, 168, 105, 145, 140, 15, 127, 17, 108, 6, 247, 3, 203, 118, 155, 106, 108, 54,
            32, 119, 207, 244, 79,
        ]),
        Sha192Digest::from([
            129, 3, 237, 227, 82, 19, 203, 115, 152, 14, 21, 217, 166, 50, 219, 205, 170, 119, 168,
            219, 113, 196, 99, 215,
        ]),
        Sha192Digest::from([
            181, 31, 8, 203, 99, 129, 24, 62, 161, 53, 19, 190, 234, 53, 106, 205, 90, 53, 196, 79,
            87, 130, 220, 191,
        ]),
        Sha192Digest::from([
            210, 242, 50, 59, 187, 92, 87, 113, 114, 253, 39, 243, 112, 150, 157, 245, 145, 10,
            158, 14, 185, 156, 208, 41,
        ]),
        Sha192Digest::from([
            59, 174, 44, 13, 235, 83, 149, 32, 113, 199, 13, 213, 25, 70, 159, 85, 36, 236, 82,
            222, 131, 225, 13, 40,
        ]),
        Sha192Digest::from([
            90, 96, 155, 203, 48, 48, 231, 221, 220, 80, 48, 182, 104, 227, 251, 132, 65, 144, 24,
            63, 213, 161, 30, 228,
        ]),
        Sha192Digest::from([
            180, 206, 62, 48, 182, 36, 174, 151, 112, 95, 172, 137, 28, 126, 34, 110, 46, 13, 253,
            211, 18, 126, 254, 125,
        ]),
        Sha192Digest::from([
            128, 81, 69, 128, 98, 253, 161, 255, 110, 129, 112, 57, 67, 245, 183, 210, 57, 162,
            252, 238, 29, 210, 192, 79,
        ]),
        Sha192Digest::from([
            67, 114, 253, 57, 242, 170, 139, 118, 218, 17, 42, 183, 40, 78, 194, 255, 206, 222, 89,
            94, 135, 216, 66, 26,
        ]),
        Sha192Digest::from([
            63, 190, 96, 11, 47, 42, 15, 68, 18, 222, 207, 100, 183, 151, 183, 29, 178, 70, 252,
            221, 70, 202, 249, 17,
        ]),
    ];
    let path = [
        Sha192Digest::from([
            190, 89, 115, 188, 231, 147, 95, 83, 64, 233, 38, 169, 252, 179, 203, 157, 45, 41, 34,
            25, 40, 211, 119, 1,
        ]),
        Sha192Digest::from([
            172, 202, 32, 47, 8, 73, 117, 153, 248, 62, 212, 36, 255, 37, 210, 168, 182, 22, 241,
            226, 72, 240, 241, 186,
        ]),
        Sha192Digest::from([
            205, 216, 22, 155, 126, 134, 186, 33, 209, 89, 170, 133, 98, 46, 157, 33, 124, 116,
            118, 213, 243, 167, 205, 251,
        ]),
        Sha192Digest::from([
            235, 68, 85, 65, 167, 165, 163, 171, 120, 146, 179, 113, 129, 67, 148, 110, 160, 193,
            228, 255, 131, 127, 176, 243,
        ]),
        Sha192Digest::from([
            104, 254, 237, 32, 201, 9, 1, 193, 218, 205, 243, 11, 144, 211, 63, 111, 75, 23, 147,
            165, 87, 6, 197, 67,
        ]),
        Sha192Digest::from([
            58, 1, 130, 70, 186, 225, 3, 231, 151, 148, 252, 31, 165, 194, 3, 253, 139, 240, 199,
            119, 180, 7, 170, 222,
        ]),
        Sha192Digest::from([
            161, 99, 130, 235, 4, 157, 69, 131, 98, 247, 182, 62, 48, 4, 249, 44, 146, 102, 14, 99,
            23, 24, 247, 96,
        ]),
        Sha192Digest::from([
            8, 66, 73, 69, 87, 172, 155, 148, 122, 33, 70, 177, 34, 210, 231, 95, 58, 61, 117, 158,
            90, 186, 238, 88,
        ]),
        Sha192Digest::from([
            28, 187, 234, 135, 188, 122, 248, 254, 120, 199, 12, 102, 0, 65, 197, 62, 218, 207, 23,
            61, 149, 122, 44, 225,
        ]),
        Sha192Digest::from([
            170, 55, 124, 140, 2, 91, 180, 152, 199, 109, 150, 7, 33, 68, 130, 6, 125, 226, 181,
            74, 14, 244, 236, 236,
        ]),
        Sha192Digest::from([
            80, 134, 106, 103, 105, 230, 239, 179, 157, 175, 158, 196, 175, 108, 233, 59, 232, 114,
            61, 140, 165, 216, 152, 7,
        ]),
        Sha192Digest::from([
            75, 227, 116, 222, 161, 154, 50, 82, 249, 197, 190, 148, 55, 151, 247, 161, 1, 183, 67,
            104, 230, 111, 47, 85,
        ]),
        Sha192Digest::from([
            30, 236, 222, 182, 222, 203, 135, 45, 112, 71, 89, 147, 80, 194, 6, 175, 54, 178, 9,
            99, 185, 126, 198, 135,
        ]),
        Sha192Digest::from([
            37, 240, 17, 120, 92, 31, 226, 45, 238, 129, 232, 31, 96, 138, 118, 183, 172, 139, 185,
            195, 241, 172, 104, 79,
        ]),
        Sha192Digest::from([
            115, 214, 39, 213, 106, 242, 110, 49, 45, 191, 246, 127, 148, 10, 131, 10, 213, 56,
            103, 75, 197, 155, 78, 57,
        ]),
    ];

    let ots = LmotsSignature {
        ots_type: lmots_type,
        nonce,
        y,
    };

    let lms_sig = LmsSignature {
        q,
        lmots_signature: ots,
        sig_type: lms_type,
        lms_path: &path,
    };

    let success =
        verify_lms_signature(&mut sha256, 15, &message, &lms_identifier, q, &lms_public_key, &lms_sig).unwrap();
    assert_eq!(success, true);

    // some negative tests, but we can't fit all of them in here before we go over the ROM limit
    let new_message = "this is a different message".as_bytes();
    let should_fail = verify_lms_signature(
        &mut sha256,
        15,
        &new_message,
        &lms_identifier,
        q,
        &lms_public_key,
        &lms_sig,
    )
    .unwrap();
    assert_eq!(should_fail, false);

    let new_lms: LmsIdentifier = [0u8; 16];
    let should_fail =
        verify_lms_signature(&mut sha256, 15, &message, &new_lms, q, &lms_public_key, &lms_sig).unwrap();
    assert_eq!(should_fail, false);

    let new_q = q + 1;
    let should_fail = verify_lms_signature(
        &mut sha256,
        15,
        &message,
        &lms_identifier,
        new_q,
        &lms_public_key,
        &lms_sig,
    )
    .unwrap();
    assert_eq!(should_fail, false);

    let new_public_key = HashValue::from([0u8; 24]);
    let should_fail =
        verify_lms_signature(&mut sha256,15, &message, &lms_identifier, q, &new_public_key, &lms_sig).unwrap();
    assert_eq!(should_fail, false);
}

fn test_lms_24_height_20() {
    let mut sha256 = unsafe { Sha256::new(Sha256Reg::new()) };

    let message: [u8; 33] = [
        116, 104, 105, 115, 32, 105, 115, 32, 116, 104, 101, 32, 109, 101, 115, 115, 97, 103, 101,
        32, 73, 32, 119, 97, 110, 116, 32, 115, 105, 103, 110, 101, 100,
    ];
    let lms_identifier: [u8; 16] = [
        69, 136, 206, 137, 163, 10, 230, 185, 177, 120, 219, 80, 34, 70, 71, 93,
    ];
    let q: u32 = 0;
    let lmots_type = LmotsAlgorithmType::LmotsSha256N24W4;
    let lms_type = LmsAlgorithmType::LmsSha256N24H20;
    let lms_public_key = HashValue::from([
        180, 158, 253, 95, 46, 160, 158, 176, 138, 132, 212, 106, 19, 251, 152, 71, 149, 125, 57,
        221, 202, 204, 143, 224,
    ]);
    let nonce: [u8; 24] = [
        212, 38, 50, 98, 221, 141, 147, 187, 22, 227, 203, 231, 132, 97, 130, 157, 22, 242, 183,
        46, 70, 120, 159, 206,
    ];
    let y = [
        Sha192Digest::from([
            23, 160, 192, 134, 4, 191, 164, 110, 108, 186, 231, 54, 220, 199, 250, 190, 52, 10,
            161, 28, 251, 82, 251, 76,
        ]),
        Sha192Digest::from([
            235, 99, 69, 32, 240, 42, 68, 226, 161, 49, 95, 109, 193, 61, 217, 223, 180, 209, 179,
            118, 137, 142, 8, 103,
        ]),
        Sha192Digest::from([
            111, 122, 243, 79, 206, 87, 232, 67, 66, 45, 232, 71, 231, 221, 102, 158, 117, 91, 179,
            126, 68, 111, 184, 69,
        ]),
        Sha192Digest::from([
            186, 110, 170, 29, 52, 13, 148, 37, 247, 139, 138, 93, 85, 213, 252, 35, 191, 171, 85,
            29, 120, 107, 47, 209,
        ]),
        Sha192Digest::from([
            168, 102, 201, 171, 37, 94, 5, 140, 199, 205, 71, 226, 141, 216, 107, 40, 169, 169,
            200, 166, 156, 68, 8, 140,
        ]),
        Sha192Digest::from([
            125, 129, 23, 182, 216, 242, 65, 5, 159, 126, 142, 57, 113, 213, 121, 191, 38, 26, 144,
            109, 14, 28, 140, 250,
        ]),
        Sha192Digest::from([
            251, 132, 26, 208, 121, 198, 137, 166, 210, 130, 129, 248, 151, 69, 191, 129, 234, 179,
            185, 3, 7, 19, 193, 172,
        ]),
        Sha192Digest::from([
            149, 96, 163, 99, 209, 204, 251, 83, 106, 89, 155, 242, 203, 89, 78, 6, 22, 85, 53,
            137, 161, 232, 105, 185,
        ]),
        Sha192Digest::from([
            219, 173, 177, 176, 90, 39, 138, 197, 51, 64, 33, 91, 155, 181, 38, 41, 238, 238, 5,
            44, 6, 115, 178, 248,
        ]),
        Sha192Digest::from([
            80, 238, 173, 176, 192, 186, 40, 60, 58, 79, 114, 219, 158, 75, 79, 122, 155, 48, 121,
            255, 127, 171, 83, 203,
        ]),
        Sha192Digest::from([
            250, 39, 3, 146, 18, 249, 38, 112, 116, 53, 124, 65, 77, 225, 52, 166, 62, 253, 221,
            175, 147, 244, 129, 207,
        ]),
        Sha192Digest::from([
            32, 192, 195, 104, 29, 123, 42, 211, 146, 56, 163, 73, 81, 88, 125, 243, 0, 137, 239,
            85, 157, 81, 254, 55,
        ]),
        Sha192Digest::from([
            124, 57, 24, 101, 117, 122, 30, 234, 31, 192, 148, 174, 144, 69, 5, 165, 143, 169, 117,
            156, 150, 187, 1, 113,
        ]),
        Sha192Digest::from([
            252, 56, 9, 248, 53, 25, 187, 70, 57, 73, 26, 219, 4, 139, 86, 113, 142, 142, 31, 103,
            174, 231, 68, 56,
        ]),
        Sha192Digest::from([
            135, 38, 77, 90, 216, 158, 13, 86, 254, 37, 143, 19, 103, 122, 86, 87, 224, 154, 142,
            240, 75, 205, 192, 184,
        ]),
        Sha192Digest::from([
            78, 210, 174, 19, 247, 196, 16, 242, 49, 160, 47, 83, 99, 5, 229, 189, 11, 80, 223,
            229, 200, 200, 157, 91,
        ]),
        Sha192Digest::from([
            211, 178, 86, 185, 56, 31, 33, 195, 202, 40, 113, 63, 224, 195, 202, 126, 72, 9, 40,
            85, 86, 51, 168, 111,
        ]),
        Sha192Digest::from([
            243, 224, 105, 8, 81, 96, 155, 248, 32, 188, 245, 138, 16, 222, 181, 216, 25, 15, 163,
            47, 115, 183, 71, 55,
        ]),
        Sha192Digest::from([
            174, 104, 219, 219, 182, 204, 128, 232, 154, 196, 139, 1, 90, 89, 4, 186, 98, 37, 1,
            141, 49, 41, 15, 253,
        ]),
        Sha192Digest::from([
            143, 116, 99, 0, 17, 225, 89, 11, 26, 197, 237, 132, 237, 51, 1, 103, 124, 13, 234,
            160, 113, 145, 197, 52,
        ]),
        Sha192Digest::from([
            119, 101, 211, 157, 200, 32, 58, 76, 41, 186, 215, 156, 47, 31, 191, 119, 138, 174,
            247, 212, 107, 182, 221, 249,
        ]),
        Sha192Digest::from([
            47, 197, 219, 248, 126, 171, 60, 4, 4, 133, 65, 96, 177, 173, 149, 217, 143, 156, 173,
            14, 175, 249, 108, 159,
        ]),
        Sha192Digest::from([
            129, 160, 29, 76, 159, 210, 128, 175, 75, 243, 136, 197, 173, 122, 242, 12, 96, 175,
            165, 122, 64, 93, 77, 234,
        ]),
        Sha192Digest::from([
            136, 69, 237, 247, 76, 229, 210, 65, 197, 120, 72, 199, 204, 206, 61, 51, 148, 10, 12,
            176, 67, 100, 17, 46,
        ]),
        Sha192Digest::from([
            255, 73, 186, 55, 124, 137, 66, 122, 7, 106, 35, 50, 236, 202, 150, 238, 246, 9, 53,
            33, 151, 244, 115, 37,
        ]),
        Sha192Digest::from([
            71, 58, 218, 52, 116, 226, 225, 191, 100, 154, 124, 119, 130, 88, 92, 32, 116, 130, 52,
            120, 147, 233, 248, 100,
        ]),
        Sha192Digest::from([
            232, 88, 112, 250, 11, 242, 213, 244, 99, 76, 222, 131, 47, 246, 103, 38, 116, 96, 172,
            60, 70, 197, 119, 252,
        ]),
        Sha192Digest::from([
            55, 142, 139, 25, 23, 233, 209, 157, 191, 37, 159, 66, 162, 65, 38, 57, 204, 135, 180,
            130, 137, 25, 253, 40,
        ]),
        Sha192Digest::from([
            93, 248, 37, 89, 123, 186, 24, 25, 125, 201, 37, 14, 62, 247, 18, 138, 249, 199, 12,
            233, 173, 224, 202, 58,
        ]),
        Sha192Digest::from([
            189, 104, 182, 236, 30, 66, 40, 219, 45, 232, 65, 9, 203, 247, 98, 45, 108, 240, 175,
            232, 202, 240, 71, 49,
        ]),
        Sha192Digest::from([
            35, 225, 109, 90, 242, 250, 145, 196, 204, 229, 53, 189, 48, 234, 194, 14, 255, 200,
            125, 62, 23, 210, 14, 69,
        ]),
        Sha192Digest::from([
            114, 128, 217, 72, 117, 245, 215, 181, 93, 17, 200, 158, 158, 194, 46, 135, 66, 217,
            161, 74, 24, 151, 250, 180,
        ]),
        Sha192Digest::from([
            45, 109, 116, 221, 107, 139, 181, 92, 231, 185, 87, 224, 191, 221, 215, 104, 162, 32,
            12, 145, 119, 79, 227, 7,
        ]),
        Sha192Digest::from([
            51, 179, 217, 176, 40, 233, 148, 179, 128, 87, 105, 93, 46, 221, 120, 175, 117, 45,
            131, 30, 253, 243, 129, 133,
        ]),
        Sha192Digest::from([
            111, 193, 94, 56, 57, 163, 83, 202, 183, 9, 145, 168, 101, 222, 72, 247, 71, 244, 176,
            168, 153, 129, 125, 94,
        ]),
        Sha192Digest::from([
            206, 127, 246, 221, 196, 141, 125, 58, 241, 236, 91, 192, 101, 235, 105, 156, 45, 14,
            122, 17, 65, 136, 132, 67,
        ]),
        Sha192Digest::from([
            1, 6, 247, 140, 202, 223, 152, 195, 88, 24, 197, 244, 184, 13, 116, 46, 45, 239, 120,
            182, 176, 92, 110, 243,
        ]),
        Sha192Digest::from([
            229, 205, 47, 49, 126, 138, 104, 104, 84, 220, 144, 245, 218, 93, 178, 22, 118, 151,
            58, 218, 63, 0, 205, 108,
        ]),
        Sha192Digest::from([
            188, 107, 111, 235, 146, 49, 25, 31, 225, 200, 64, 104, 60, 119, 240, 222, 13, 37, 218,
            179, 100, 36, 228, 56,
        ]),
        Sha192Digest::from([
            247, 40, 31, 125, 156, 183, 31, 84, 69, 87, 240, 137, 216, 182, 109, 223, 184, 152,
            185, 7, 112, 240, 3, 47,
        ]),
        Sha192Digest::from([
            11, 101, 220, 176, 247, 202, 193, 137, 85, 42, 103, 19, 0, 125, 150, 33, 47, 50, 48,
            59, 87, 133, 114, 125,
        ]),
        Sha192Digest::from([
            29, 64, 210, 159, 189, 200, 219, 113, 109, 34, 235, 211, 216, 5, 11, 181, 20, 51, 152,
            153, 51, 142, 102, 153,
        ]),
        Sha192Digest::from([
            9, 181, 87, 97, 63, 87, 11, 75, 220, 28, 223, 69, 115, 50, 54, 209, 82, 205, 253, 120,
            98, 0, 36, 138,
        ]),
        Sha192Digest::from([
            199, 211, 206, 239, 57, 18, 248, 204, 10, 125, 200, 126, 28, 26, 7, 124, 44, 236, 250,
            96, 247, 23, 108, 69,
        ]),
        Sha192Digest::from([
            84, 161, 229, 121, 202, 199, 147, 22, 38, 125, 100, 19, 216, 89, 160, 131, 118, 137,
            120, 38, 229, 11, 219, 136,
        ]),
        Sha192Digest::from([
            170, 54, 161, 94, 45, 6, 117, 108, 173, 52, 212, 152, 139, 44, 53, 28, 90, 204, 183,
            136, 232, 0, 173, 46,
        ]),
        Sha192Digest::from([
            53, 181, 197, 95, 220, 187, 103, 104, 101, 139, 103, 70, 220, 144, 250, 227, 44, 88,
            40, 169, 43, 17, 228, 132,
        ]),
        Sha192Digest::from([
            198, 5, 6, 99, 72, 41, 38, 56, 73, 180, 191, 26, 107, 180, 157, 38, 106, 56, 41, 11,
            85, 177, 40, 121,
        ]),
        Sha192Digest::from([
            82, 64, 121, 76, 108, 126, 42, 205, 249, 230, 120, 80, 100, 179, 134, 85, 12, 100, 12,
            82, 218, 105, 97, 207,
        ]),
        Sha192Digest::from([
            93, 30, 135, 26, 208, 246, 1, 13, 23, 140, 117, 213, 140, 24, 148, 90, 220, 150, 86,
            248, 89, 141, 10, 217,
        ]),
        Sha192Digest::from([
            92, 200, 225, 254, 151, 59, 166, 122, 15, 19, 6, 121, 148, 51, 172, 81, 176, 151, 38,
            101, 65, 126, 254, 167,
        ]),
    ];
    let path = [
        Sha192Digest::from([
            5, 71, 54, 208, 167, 144, 150, 170, 61, 2, 223, 28, 242, 147, 99, 217, 76, 200, 219,
            68, 65, 123, 205, 114,
        ]),
        Sha192Digest::from([
            225, 113, 19, 160, 185, 193, 223, 22, 7, 37, 8, 66, 36, 231, 43, 173, 210, 91, 211, 96,
            32, 104, 51, 238,
        ]),
        Sha192Digest::from([
            206, 74, 21, 195, 175, 17, 187, 93, 180, 0, 130, 45, 232, 36, 196, 205, 109, 38, 124,
            249, 140, 116, 198, 201,
        ]),
        Sha192Digest::from([
            224, 228, 181, 187, 72, 182, 186, 3, 227, 247, 162, 178, 162, 166, 144, 210, 209, 179,
            222, 108, 36, 54, 135, 225,
        ]),
        Sha192Digest::from([
            46, 168, 146, 50, 58, 62, 52, 23, 21, 29, 171, 109, 140, 172, 87, 161, 128, 193, 74,
            105, 252, 244, 181, 154,
        ]),
        Sha192Digest::from([
            206, 36, 161, 152, 211, 117, 253, 49, 223, 194, 163, 223, 105, 221, 254, 117, 160, 171,
            36, 96, 80, 33, 30, 220,
        ]),
        Sha192Digest::from([
            163, 203, 70, 60, 90, 99, 148, 205, 66, 205, 190, 205, 57, 11, 53, 248, 179, 243, 192,
            21, 184, 2, 194, 32,
        ]),
        Sha192Digest::from([
            54, 253, 197, 102, 138, 111, 208, 11, 179, 49, 248, 91, 82, 126, 43, 141, 215, 14, 203,
            112, 147, 9, 207, 254,
        ]),
        Sha192Digest::from([
            176, 193, 11, 180, 116, 135, 90, 6, 146, 15, 138, 127, 187, 105, 124, 25, 66, 153, 198,
            178, 117, 162, 60, 203,
        ]),
        Sha192Digest::from([
            253, 237, 188, 14, 88, 239, 110, 235, 40, 94, 96, 104, 233, 139, 167, 17, 116, 58, 238,
            140, 245, 132, 155, 52,
        ]),
        Sha192Digest::from([
            64, 192, 199, 169, 27, 211, 58, 113, 36, 223, 110, 37, 170, 225, 206, 211, 146, 134,
            46, 41, 52, 32, 57, 92,
        ]),
        Sha192Digest::from([
            68, 97, 151, 244, 43, 246, 64, 206, 130, 87, 81, 158, 43, 83, 112, 128, 168, 246, 188,
            130, 6, 215, 152, 128,
        ]),
        Sha192Digest::from([
            203, 102, 135, 137, 184, 203, 208, 177, 109, 227, 250, 249, 178, 65, 38, 169, 162, 138,
            168, 221, 51, 13, 175, 239,
        ]),
        Sha192Digest::from([
            77, 195, 47, 104, 93, 246, 33, 54, 48, 39, 193, 206, 185, 130, 106, 150, 169, 25, 64,
            50, 183, 206, 92, 31,
        ]),
        Sha192Digest::from([
            198, 96, 14, 165, 145, 110, 96, 24, 32, 226, 19, 98, 130, 202, 38, 7, 194, 4, 97, 184,
            191, 50, 103, 221,
        ]),
        Sha192Digest::from([
            168, 37, 88, 211, 230, 242, 0, 90, 238, 58, 1, 36, 122, 116, 238, 144, 112, 147, 71,
            21, 155, 16, 8, 222,
        ]),
        Sha192Digest::from([
            196, 185, 169, 76, 142, 198, 200, 148, 169, 87, 217, 205, 167, 88, 232, 166, 81, 236,
            27, 87, 59, 138, 48, 205,
        ]),
        Sha192Digest::from([
            163, 255, 13, 121, 172, 99, 152, 244, 2, 49, 9, 69, 60, 194, 234, 90, 236, 83, 12, 246,
            92, 221, 19, 126,
        ]),
        Sha192Digest::from([
            90, 135, 239, 176, 230, 215, 36, 58, 67, 50, 17, 9, 183, 98, 53, 6, 130, 212, 70, 134,
            84, 62, 198, 212,
        ]),
        Sha192Digest::from([
            118, 22, 167, 149, 221, 76, 51, 216, 183, 67, 152, 84, 49, 157, 83, 119, 164, 48, 249,
            4, 245, 16, 212, 150,
        ]),
    ];
    let ots = LmotsSignature {
        ots_type: lmots_type,
        nonce,
        y,
    };

    let lms_sig = LmsSignature {
        q,
        lmots_signature: ots,
        sig_type: lms_type,
        lms_path: &path,
    };

    let success =
        verify_lms_signature(&mut sha256, 20, &message, &lms_identifier, q, &lms_public_key, &lms_sig).unwrap();
    assert_eq!(success, true);
}

test_suite! {
    test_lms_lookup,
    test_lmots_lookup,
    test_get_lms_parameters,
    test_hash_message_24,
    test_lms_24_height_15,
    test_lms_24_height_20,
}
