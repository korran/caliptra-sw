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
    lookup_lmots_algorithm_type, lookup_lms_algorithm_type, HashValue, LmotsAlgorithmType,
    LmotsSignature, Lms, LmsAlgorithmType, LmsIdentifier, LmsSignature, Sha192Digest, Sha256,
};
use caliptra_registers::sha256::Sha256Reg;
use caliptra_test_harness::test_suite;

fn test_lms_lookup() {
    let result = lookup_lms_algorithm_type(0);
    assert_eq!(LmsAlgorithmType::LmsReserved, result.unwrap())
}

fn test_get_lms_parameters() {
    // Full size SHA256 hashes
    let (width, height) = Lms::default()
        .get_lms_parameters(&LmsAlgorithmType::LmsSha256N32H5)
        .unwrap();
    assert_eq!(32, width);
    assert_eq!(5, height);
    let (width, height) = Lms::default()
        .get_lms_parameters(&LmsAlgorithmType::LmsSha256N32H10)
        .unwrap();
    assert_eq!(32, width);
    assert_eq!(10, height);
    let (width, height) = Lms::default()
        .get_lms_parameters(&LmsAlgorithmType::LmsSha256N32H15)
        .unwrap();
    assert_eq!(32, width);
    assert_eq!(15, height);
    let (width, height) = Lms::default()
        .get_lms_parameters(&LmsAlgorithmType::LmsSha256N32H20)
        .unwrap();
    assert_eq!(32, width);
    assert_eq!(20, height);
    let (width, height) = Lms::default()
        .get_lms_parameters(&LmsAlgorithmType::LmsSha256N32H25)
        .unwrap();
    assert_eq!(32, width);
    assert_eq!(25, height);

    // Truncated 192 bit SHA256 hashes
    let (width, height) = Lms::default()
        .get_lms_parameters(&LmsAlgorithmType::LmsSha256N24H5)
        .unwrap();
    assert_eq!(24, width);
    assert_eq!(5, height);
    let (width, height) = Lms::default()
        .get_lms_parameters(&LmsAlgorithmType::LmsSha256N24H10)
        .unwrap();
    assert_eq!(24, width);
    assert_eq!(10, height);
    let (width, height) = Lms::default()
        .get_lms_parameters(&LmsAlgorithmType::LmsSha256N24H15)
        .unwrap();
    assert_eq!(24, width);
    assert_eq!(15, height);
    let (width, height) = Lms::default()
        .get_lms_parameters(&LmsAlgorithmType::LmsSha256N24H20)
        .unwrap();
    assert_eq!(24, width);
    assert_eq!(20, height);
    let (width, height) = Lms::default()
        .get_lms_parameters(&LmsAlgorithmType::LmsSha256N24H25)
        .unwrap();
    assert_eq!(24, width);
    assert_eq!(25, height);
}

fn test_lmots_lookup() {
    let result = lookup_lmots_algorithm_type(0);
    assert_eq!(LmotsAlgorithmType::LmotsReserved, result.unwrap())
}

// test case from https://datatracker.ietf.org/doc/html/rfc8554#section-3.1.3
fn test_coefficient() {
    let input_value = [0x12u8, 0x34u8];
    let result = Lms::default().coefficient(&input_value, 7, 1).unwrap();
    assert_eq!(result, 0);

    let result = Lms::default().coefficient(&input_value, 0, 4).unwrap();
    assert_eq!(result, 1);
}

fn test_hash_message_24() {
    let mut sha256 = unsafe { Sha256::new(Sha256Reg::new()) };
    let message: [u8; 33] = [
        116, 104, 105, 115, 32, 105, 115, 32, 116, 104, 101, 32, 109, 101, 115, 115, 97, 103, 101,
        32, 73, 32, 119, 97, 110, 116, 32, 115, 105, 103, 110, 101, 100,
    ];
    let lms_identifier: LmsIdentifier = [
        102, 40, 233, 90, 126, 166, 161, 73, 107, 57, 114, 28, 121, 57, 28, 123,
    ];
    let u8_nonce: [u8; 24] = [
        108, 201, 169, 93, 130, 206, 214, 173, 223, 138, 178, 150, 192, 86, 115, 139, 157, 213,
        182, 55, 196, 22, 212, 216,
    ];
    let mut nonce = [0u32; 6];
    for i in 0..6 {
        nonce[i] = u32::from_be_bytes([
            u8_nonce[i * 4],
            u8_nonce[i * 4 + 1],
            u8_nonce[i * 4 + 2],
            u8_nonce[i * 4 + 3],
        ]);
    }

    let q: u32 = 0;
    let q_str = q.to_be_bytes();
    let expected_hash = HashValue::from([
        175, 160, 9, 71, 29, 26, 61, 20, 90, 217, 142, 152, 112, 68, 51, 17, 154, 191, 74, 150,
        161, 238, 102, 161,
    ]);
    let hash = Lms::default()
        .hash_message(&mut sha256, &message, &lms_identifier, &q_str, &nonce)
        .unwrap();
    assert_eq!(expected_hash, hash);
}

fn test_lms_24_height_15() {
    let mut sha256 = unsafe { Sha256::new(Sha256Reg::new()) };
    const MESSAGE: [u8; 33] = [
        116, 104, 105, 115, 32, 105, 115, 32, 116, 104, 101, 32, 109, 101, 115, 115, 97, 103, 101,
        32, 73, 32, 119, 97, 110, 116, 32, 115, 105, 103, 110, 101, 100,
    ];
    const LMS_IDENTIFIER: LmsIdentifier = [
        158, 20, 249, 74, 242, 177, 66, 175, 101, 91, 176, 36, 80, 31, 240, 7,
    ];
    const Q: u32 = 0;
    const LMOTS_TYPE: LmotsAlgorithmType = LmotsAlgorithmType::LmotsSha256N24W4;
    const LMS_TYPE: LmsAlgorithmType = LmsAlgorithmType::LmsSha256N24H15;
    const LMS_PUBLIC_KEY: HashValue<6> = HashValue([
        53125821, 2603739581, 860571182, 662249589, 3182288302, 4193104164,
    ]);
    const NONCE: [u32; 6] = [
        3022260699, 3712621641, 4235802516, 1978255207, 478105939, 4149435076,
    ];
    const Y: [HashValue<6>; 51] = [
        HashValue([
            1918087017, 3361364886, 274058243, 3085037187, 2880451251, 341375593,
        ]),
        HashValue([
            13956348, 3660938697, 2839810083, 1028325556, 3106711662, 4042849555,
        ]),
        HashValue([
            3534585347, 3962749017, 2409325821, 2356118137, 4153313511, 2068634505,
        ]),
        HashValue([
            1826922086, 993159977, 501598683, 3752527208, 498435688, 511764143,
        ]),
        HashValue([
            3130363838, 478395982, 449986318, 244819632, 3892376526, 2545286320,
        ]),
        HashValue([
            2493405165, 1706646572, 3116059780, 1313754339, 526643499, 22885820,
        ]),
        HashValue([
            2918167022, 984794221, 205214285, 2453728753, 3435596199, 4185412883,
        ]),
        HashValue([
            4153209804, 367114673, 1906913062, 860313948, 619894206, 2930153363,
        ]),
        HashValue([
            1246927358, 1283353731, 815765696, 1589892647, 2637385857, 4020717617,
        ]),
        HashValue([
            1798600061, 4179259961, 2951899974, 1589936286, 572693486, 2041352209,
        ]),
        HashValue([
            2897473190, 1110488020, 3948157613, 2813685060, 361988474, 470435643,
        ]),
        HashValue([
            33723832, 1614898361, 3202028015, 2956542878, 2292387421, 2599714921,
        ]),
        HashValue([
            382380322, 3965530891, 156541719, 2367477949, 3532416252, 127850531,
        ]),
        HashValue([
            3162159938, 1994002520, 2721903616, 140765728, 73130738, 3101458127,
        ]),
        HashValue([
            187744786, 2393192377, 242633530, 1232721517, 1731228048, 2430306651,
        ]),
        HashValue([
            1639695210, 2235001164, 592402961, 3854765477, 964394876, 2280975580,
        ]),
        HashValue([
            3596834779, 2684920199, 2324245080, 207254138, 4060288540, 324277449,
        ]),
        HashValue([
            1054930857, 885379627, 4120052995, 2866395245, 2038364650, 3456973214,
        ]),
        HashValue([
            1870706494, 2588322083, 285843796, 502735158, 327630707, 3065624778,
        ]),
        HashValue([
            2120664212, 1977139582, 3941900843, 513544052, 4233801954, 381310069,
        ]),
        HashValue([
            2961331862, 920436733, 3639252565, 3079355033, 489871346, 2524633204,
        ]),
        HashValue([
            3577317522, 4224998842, 1722270977, 1190275452, 1009233112, 2363494539,
        ]),
        HashValue([
            3511692650, 2041877445, 889242166, 3123096493, 2404880675, 3871411913,
        ]),
        HashValue([
            4104151251, 3800378266, 3649037492, 1483748234, 1070352305, 4057009362,
        ]),
        HashValue([
            4247262653, 568134378, 2024541158, 3652269280, 3260052441, 1113320014,
        ]),
        HashValue([
            2385353000, 4064185331, 3982659576, 2285465204, 2656415334, 4120544364,
        ]),
        HashValue([
            2143583115, 1725289436, 1202487080, 1488534432, 1185181314, 1804548777,
        ]),
        HashValue([
            1788582005, 1174727292, 3167271348, 3132703706, 3479183772, 2983521639,
        ]),
        HashValue([
            473041655, 2927919989, 3952920496, 3962586558, 2046628220, 3441858398,
        ]),
        HashValue([
            1009601077, 772815933, 1988735916, 4073477840, 3984224088, 1995194518,
        ]),
        HashValue([
            442150284, 3775440843, 1005656833, 481693255, 4081420275, 4153755506,
        ]),
        HashValue([
            2263164070, 234549122, 4234972577, 2174059864, 2468869673, 1774811063,
        ]),
        HashValue([
            3914000831, 2923566227, 4239856696, 1194265859, 2088911841, 1848210751,
        ]),
        HashValue([
            3567043549, 3310975779, 2280063913, 2596902771, 335193881, 1531042418,
        ]),
        HashValue([
            1826620068, 3911676660, 3933533417, 163545128, 4258758538, 163602513,
        ]),
        HashValue([
            432519560, 1884869957, 1807367266, 2150090893, 4155045801, 4277511745,
        ]),
        HashValue([
            2589682157, 3299639704, 535511224, 3349784429, 475693426, 399223032,
        ]),
        HashValue([
            1550301337, 1871305329, 548298470, 739683161, 478262658, 3330830190,
        ]),
        HashValue([
            2144008480, 2916541965, 2310666657, 852453819, 4168349056, 3379127284,
        ]),
        HashValue([
            27069767, 601736804, 3615345741, 1599827220, 2795168299, 909926728,
        ]),
        HashValue([
            2145563595, 2703436591, 2486046259, 71357651, 459857124, 2202720040,
        ]),
        HashValue([
            4031096937, 2441875327, 292292343, 63665819, 1785476640, 2010117199,
        ]),
        HashValue([
            2164518371, 1377028979, 2551059929, 2788350925, 2859968731, 1908696023,
        ]),
        HashValue([
            3038709963, 1669404734, 2704610238, 3929369293, 1513473103, 1468193983,
        ]),
        HashValue([
            3539087931, 3143391089, 1929193459, 1888919029, 2433392142, 3114061865,
        ]),
        HashValue([
            1001270285, 3948123424, 1908870613, 424058709, 619467486, 2212564264,
        ]),
        HashValue([
            1516280779, 808511453, 3696242870, 1759771524, 1099962431, 3584106212,
        ]),
        HashValue([
            3033415216, 3055857303, 1885318281, 478028398, 772668883, 310312573,
        ]),
        HashValue([
            2152809856, 1660789247, 1853976633, 1140176850, 966982894, 500351055,
        ]),
        HashValue([
            1131609401, 4071263094, 3658558135, 676250367, 3470678366, 2279096858,
        ]),
        HashValue([
            1069441035, 791285572, 316591972, 3080173341, 2990996701, 1187707153,
        ]),
    ];

    const PATH: [HashValue<6>; 15] = [
        HashValue([
            3193533372, 3885195091, 1089021609, 4239641501, 757670425, 684947201,
        ]),
        HashValue([
            2898927663, 139031961, 4164867108, 4280668840, 3054957026, 1223750074,
        ]),
        HashValue([
            3453490843, 2122758689, 3512314501, 1647222049, 2088007381, 4087860731,
        ]),
        HashValue([
            3947124033, 2812650411, 2022880113, 2168689774, 2697061631, 2206183667,
        ]),
        HashValue([
            1761537312, 3372810689, 3670930187, 2429763439, 1259836325, 1460061507,
        ]),
        HashValue([
            973177414, 3135308775, 2543123487, 2780955645, 2347812727, 3020401374,
        ]),
        HashValue([
            2707653355, 77415811, 1660401214, 805632300, 2456161891, 387512160,
        ]),
        HashValue([
            138561861, 1470929812, 2049001137, 584247135, 977106334, 1522200152,
        ]),
        HashValue([
            482077319, 3162175742, 2026310758, 4310334, 3671004989, 2507812065,
        ]),
        HashValue([
            2855763084, 39564440, 3345847815, 558137862, 2112009546, 250932460,
        ]),
        HashValue([
            1350986343, 1776742323, 2645532356, 2943150395, 3899800972, 2782435335,
        ]),
        HashValue([
            1273197790, 2711237202, 4190486164, 932706209, 28787560, 3866046293,
        ]),
        HashValue([
            518839990, 3737880365, 1883724179, 1354892975, 917637475, 3112093319,
        ]),
        HashValue([
            636490104, 1545593389, 4001490975, 1619687095, 2894838211, 4054607951,
        ]),
        HashValue([
            1943414741, 1794272817, 767555199, 2483716874, 3577243467, 3315289657,
        ]),
    ];

    const OTS: LmotsSignature<6, 51> = LmotsSignature {
        ots_type: LMOTS_TYPE,
        nonce: NONCE,
        y: Y,
    };

    const LMS_SIG: LmsSignature<6, 51> = LmsSignature {
        q: Q,
        lmots_signature: OTS,
        sig_type: LMS_TYPE,
        lms_path: &PATH,
    };

    let success = Lms::default()
        .verify_lms_signature(
            &mut sha256,
            &MESSAGE,
            &LMS_IDENTIFIER,
            Q,
            &LMS_PUBLIC_KEY,
            &LMS_SIG,
        )
        .unwrap();
    assert_eq!(success, true);
}

fn _test_lms_24_height_20() {
    let mut sha256 = unsafe { Sha256::new(Sha256Reg::new()) };
    const MESSAGE: [u8; 33] = [
        116, 104, 105, 115, 32, 105, 115, 32, 116, 104, 101, 32, 109, 101, 115, 115, 97, 103, 101,
        32, 73, 32, 119, 97, 110, 116, 32, 115, 105, 103, 110, 101, 100,
    ];
    const LMS_IDENTIFIER: LmsIdentifier = [
        69, 136, 206, 137, 163, 10, 230, 185, 177, 120, 219, 80, 34, 70, 71, 93,
    ];
    let q: u32 = 0;
    let lmots_type = LmotsAlgorithmType::LmotsSha256N24W4;
    let lms_type = LmsAlgorithmType::LmsSha256N24H20;
    let lms_public_key: HashValue<6> = HashValue::from([
        180, 158, 253, 95, 46, 160, 158, 176, 138, 132, 212, 106, 19, 251, 152, 71, 149, 125, 57,
        221, 202, 204, 143, 224,
    ]);
    let u8_nonce: [u8; 24] = [
        212, 38, 50, 98, 221, 141, 147, 187, 22, 227, 203, 231, 132, 97, 130, 157, 22, 242, 183,
        46, 70, 120, 159, 206,
    ];
    let mut nonce = [0u32; 6];
    for i in 0..6 {
        nonce[i] = u32::from_be_bytes([
            u8_nonce[i * 4],
            u8_nonce[i * 4 + 1],
            u8_nonce[i * 4 + 2],
            u8_nonce[i * 4 + 3],
        ]);
    }

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

    let success = Lms::default()
        .verify_lms_signature(
            &mut sha256,
            &MESSAGE,
            &LMS_IDENTIFIER,
            q,
            &lms_public_key,
            &lms_sig,
        )
        .unwrap();
    assert_eq!(success, true);
}

test_suite! {
    test_coefficient,
    test_lms_lookup,
    test_lmots_lookup,
    test_get_lms_parameters,
    test_hash_message_24,
    test_lms_24_height_15,
    //test_lms_24_height_20,
}
