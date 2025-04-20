/// Contains all possible generator polynomials (to compule error codewords)
pub const GENERATOR_POLYNOMIALS: [&[u8]; 31] = [
    &[0],
    &[0, 0],
    &[0, 25, 1],
    &[0, 198, 199, 3],
    &[0, 75, 249, 78, 6],
    &[0, 113, 164, 166, 119, 10],
    &[0, 166, 0, 134, 5, 176, 15],
    &[0, 87, 229, 146, 149, 238, 102, 21],
    &[0, 175, 238, 208, 249, 215, 252, 196, 28],
    &[0, 95, 246, 137, 231, 235, 149, 11, 123, 36],
    &[0, 251, 67, 46, 61, 118, 70, 64, 94, 32, 45],
    &[0, 220, 192, 91, 194, 172, 177, 209, 116, 227, 10, 55],
    &[0, 102, 43, 98, 121, 187, 113, 198, 143, 131, 87, 157, 66],
    &[
        0, 74, 152, 176, 100, 86, 100, 106, 104, 130, 218, 206, 140, 78,
    ],
    &[
        0, 199, 249, 155, 48, 190, 124, 218, 137, 216, 87, 207, 59, 22, 91,
    ],
    &[
        0, 8, 183, 61, 91, 202, 37, 51, 58, 58, 237, 140, 124, 5, 99, 105,
    ],
    &[
        0, 120, 104, 107, 109, 102, 161, 76, 3, 91, 191, 147, 169, 182, 194, 225, 120,
    ],
    &[
        0, 43, 139, 206, 78, 43, 239, 123, 206, 214, 147, 24, 99, 150, 39, 243, 163, 136,
    ],
    &[
        0, 215, 234, 158, 94, 184, 97, 118, 170, 79, 187, 152, 148, 252, 179, 5, 98, 96, 153,
    ],
    &[
        0, 67, 3, 105, 153, 52, 90, 83, 17, 150, 159, 44, 128, 153, 133, 252, 222, 138, 220, 171,
    ],
    &[
        0, 17, 60, 79, 50, 61, 163, 26, 187, 202, 180, 221, 225, 83, 239, 156, 164, 212, 212, 188,
        190,
    ],
    &[
        0, 240, 233, 104, 247, 181, 140, 67, 98, 85, 200, 210, 115, 148, 137, 230, 36, 122, 254,
        148, 175, 210,
    ],
    &[
        0, 210, 171, 247, 242, 93, 230, 14, 109, 221, 53, 200, 74, 8, 172, 98, 80, 219, 134, 160,
        105, 165, 231,
    ],
    &[
        0, 171, 102, 146, 91, 49, 103, 65, 17, 193, 150, 14, 25, 183, 248, 94, 164, 224, 192, 1,
        78, 56, 147, 253,
    ],
    &[
        0, 229, 121, 135, 48, 211, 117, 251, 126, 159, 180, 169, 152, 192, 226, 228, 218, 111, 0,
        117, 232, 87, 96, 227, 21,
    ],
    &[
        0, 231, 181, 156, 39, 170, 26, 12, 59, 15, 148, 201, 54, 66, 237, 208, 99, 167, 144, 182,
        95, 243, 129, 178, 252, 45,
    ],
    &[
        0, 173, 125, 158, 2, 103, 182, 118, 17, 145, 201, 111, 28, 165, 53, 161, 21, 245, 142, 13,
        102, 48, 227, 153, 145, 218, 70,
    ],
    &[
        0, 79, 228, 8, 165, 227, 21, 180, 29, 9, 237, 70, 99, 45, 58, 138, 135, 73, 126, 172, 94,
        216, 193, 157, 26, 17, 149, 96,
    ],
    &[
        0, 168, 223, 200, 104, 224, 234, 108, 180, 110, 190, 195, 147, 205, 27, 232, 201, 21, 43,
        245, 87, 42, 195, 212, 119, 242, 37, 9, 123,
    ],
    &[
        0, 156, 45, 183, 29, 151, 219, 54, 96, 249, 24, 136, 5, 241, 175, 189, 28, 75, 234, 150,
        148, 23, 9, 202, 162, 68, 250, 140, 24, 151,
    ],
    &[
        0, 41, 173, 145, 152, 216, 31, 179, 182, 50, 48, 110, 86, 239, 96, 222, 125, 42, 173, 226,
        193, 224, 130, 156, 37, 251, 216, 238, 40, 192, 180,
    ],
];

#[test]
fn generator_polynomials() {
    let poly = GENERATOR_POLYNOMIALS[7];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "α0x7 + α87x6 + α229x5 + α146x4 + α149x3 + α238x2 + α102x + α21"
    );

    let poly = GENERATOR_POLYNOMIALS[8];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "α0x8 + α175x7 + α238x6 + α208x5 + α249x4 + α215x3 + α252x2 + α196x + α28"
    );

    let poly = GENERATOR_POLYNOMIALS[9];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "α0x9 + α95x8 + α246x7 + α137x6 + α231x5 + α235x4 + α149x3 + α11x2 + α123x + α36"
    );

    let poly = GENERATOR_POLYNOMIALS[10];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "α0x10 + α251x9 + α67x8 + α46x7 + α61x6 + α118x5 + α70x4 + α64x3 + α94x2 + α32x + α45"
    );

    let poly = GENERATOR_POLYNOMIALS[11];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "α0x11 + α220x10 + α192x9 + α91x8 + α194x7 + α172x6 + α177x5 + α209x4 + α116x3 + α227x2 + α10x + α55"
    );

    let poly = GENERATOR_POLYNOMIALS[12];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "α0x12 + α102x11 + α43x10 + α98x9 + α121x8 + α187x7 + α113x6 + α198x5 + α143x4 + α131x3 + α87x2 + α157x + α66"
    );

    let poly = GENERATOR_POLYNOMIALS[13];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "α0x13 + α74x12 + α152x11 + α176x10 + α100x9 + α86x8 + α100x7 + α106x6 + α104x5 + α130x4 + α218x3 + α206x2 + α140x + α78"
    );

    let poly = GENERATOR_POLYNOMIALS[14];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "α0x14 + α199x13 + α249x12 + α155x11 + α48x10 + α190x9 + α124x8 + α218x7 + α137x6 + α216x5 + α87x4 + α207x3 + α59x2 + α22x + α91"
    );

    let poly = GENERATOR_POLYNOMIALS[15];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "α0x15 + α8x14 + α183x13 + α61x12 + α91x11 + α202x10 + α37x9 + α51x8 + α58x7 + α58x6 + α237x5 + α140x4 + α124x3 + α5x2 + α99x + α105"
    );

    let poly = GENERATOR_POLYNOMIALS[16];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "α0x16 + α120x15 + α104x14 + α107x13 + α109x12 + α102x11 + α161x10 + α76x9 + α3x8 + α91x7 + α191x6 + α147x5 + α169x4 + α182x3 + α194x2 + α225x + α120"
    );

    let poly = GENERATOR_POLYNOMIALS[17];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "α0x17 + α43x16 + α139x15 + α206x14 + α78x13 + α43x12 + α239x11 + α123x10 + α206x9 + α214x8 + α147x7 + α24x6 + α99x5 + α150x4 + α39x3 + α243x2 + α163x + α136"
    );

    let poly = GENERATOR_POLYNOMIALS[18];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "α0x18 + α215x17 + α234x16 + α158x15 + α94x14 + α184x13 + α97x12 + α118x11 + α170x10 + α79x9 + α187x8 + α152x7 + α148x6 + α252x5 + α179x4 + α5x3 + α98x2 + α96x + α153"
    );

    let poly = GENERATOR_POLYNOMIALS[19];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "α0x19 + α67x18 + α3x17 + α105x16 + α153x15 + α52x14 + α90x13 + α83x12 + α17x11 + α150x10 + α159x9 + α44x8 + α128x7 + α153x6 + α133x5 + α252x4 + α222x3 + α138x2 + α220x + α171"
    );

    let poly = GENERATOR_POLYNOMIALS[20];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "α0x20 + α17x19 + α60x18 + α79x17 + α50x16 + α61x15 + α163x14 + α26x13 + α187x12 + α202x11 + α180x10 + α221x9 + α225x8 + α83x7 + α239x6 + α156x5 + α164x4 + α212x3 + α212x2 + α188x + α190"
    );

    let poly = GENERATOR_POLYNOMIALS[21];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "α0x21 + α240x20 + α233x19 + α104x18 + α247x17 + α181x16 + α140x15 + α67x14 + α98x13 + α85x12 + α200x11 + α210x10 + α115x9 + α148x8 + α137x7 + α230x6 + α36x5 + α122x4 + α254x3 + α148x2 + α175x + α210"
    );

    let poly = GENERATOR_POLYNOMIALS[22];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "α0x22 + α210x21 + α171x20 + α247x19 + α242x18 + α93x17 + α230x16 + α14x15 + α109x14 + α221x13 + α53x12 + α200x11 + α74x10 + α8x9 + α172x8 + α98x7 + α80x6 + α219x5 + α134x4 + α160x3 + α105x2 + α165x + α231"
    );

    let poly = GENERATOR_POLYNOMIALS[23];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "α0x23 + α171x22 + α102x21 + α146x20 + α91x19 + α49x18 + α103x17 + α65x16 + α17x15 + α193x14 + α150x13 + α14x12 + α25x11 + α183x10 + α248x9 + α94x8 + α164x7 + α224x6 + α192x5 + α1x4 + α78x3 + α56x2 + α147x + α253"
    );

    let poly = GENERATOR_POLYNOMIALS[24];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "α0x24 + α229x23 + α121x22 + α135x21 + α48x20 + α211x19 + α117x18 + α251x17 + α126x16 + α159x15 + α180x14 + α169x13 + α152x12 + α192x11 + α226x10 + α228x9 + α218x8 + α111x7 + α0x6 + α117x5 + α232x4 + α87x3 + α96x2 + α227x + α21"
    );

    let poly = GENERATOR_POLYNOMIALS[25];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "α0x25 + α231x24 + α181x23 + α156x22 + α39x21 + α170x20 + α26x19 + α12x18 + α59x17 + α15x16 + α148x15 + α201x14 + α54x13 + α66x12 + α237x11 + α208x10 + α99x9 + α167x8 + α144x7 + α182x6 + α95x5 + α243x4 + α129x3 + α178x2 + α252x + α45"
    );

    let poly = GENERATOR_POLYNOMIALS[26];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "α0x26 + α173x25 + α125x24 + α158x23 + α2x22 + α103x21 + α182x20 + α118x19 + α17x18 + α145x17 + α201x16 + α111x15 + α28x14 + α165x13 + α53x12 + α161x11 + α21x10 + α245x9 + α142x8 + α13x7 + α102x6 + α48x5 + α227x4 + α153x3 + α145x2 + α218x + α70"
    );

    let poly = GENERATOR_POLYNOMIALS[27];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "α0x27 + α79x26 + α228x25 + α8x24 + α165x23 + α227x22 + α21x21 + α180x20 + α29x19 + α9x18 + α237x17 + α70x16 + α99x15 + α45x14 + α58x13 + α138x12 + α135x11 + α73x10 + α126x9 + α172x8 + α94x7 + α216x6 + α193x5 + α157x4 + α26x3 + α17x2 + α149x + α96"
    );

    let poly = GENERATOR_POLYNOMIALS[28];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "α0x28 + α168x27 + α223x26 + α200x25 + α104x24 + α224x23 + α234x22 + α108x21 + α180x20 + α110x19 + α190x18 + α195x17 + α147x16 + α205x15 + α27x14 + α232x13 + α201x12 + α21x11 + α43x10 + α245x9 + α87x8 + α42x7 + α195x6 + α212x5 + α119x4 + α242x3 + α37x2 + α9x + α123"
    );

    let poly = GENERATOR_POLYNOMIALS[29];
    let poly_string = crate::polynomials::generated_to_string(poly);
    assert_eq!(
        poly_string,
        "α0x29 + α156x28 + α45x27 + α183x26 + α29x25 + α151x24 + α219x23 + α54x22 + α96x21 + α249x20 + α24x19 + α136x18 + α5x17 + α241x16 + α175x15 + α189x14 + α28x13 + α75x12 + α234x11 + α150x10 + α148x9 + α23x8 + α9x7 + α202x6 + α162x5 + α68x4 + α250x3 + α140x2 + α24x + α151"
    )
}
mod generators {
    use super::GENERATOR_POLYNOMIALS;

    #[test]
    fn generator1() {
        let version = crate::version::Version::V01;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[7]);

        let version = crate::version::Version::V01;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[10]);

        let version = crate::version::Version::V01;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[13]);

        let version = crate::version::Version::V01;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[17]);
    }

    #[test]
    fn generator2() {
        let version = crate::version::Version::V02;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[10]);

        let version = crate::version::Version::V02;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[16]);

        let version = crate::version::Version::V02;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[22]);

        let version = crate::version::Version::V02;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);
    }

    #[test]
    fn generator3() {
        let version = crate::version::Version::V03;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[15]);

        let version = crate::version::Version::V03;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[26]);

        let version = crate::version::Version::V03;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[18]);

        let version = crate::version::Version::V03;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[22]);
    }

    #[test]
    fn generator4() {
        let version = crate::version::Version::V04;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[20]);

        let version = crate::version::Version::V04;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[18]);

        let version = crate::version::Version::V04;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[26]);

        let version = crate::version::Version::V04;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[16]);
    }

    #[test]
    fn generator5() {
        let version = crate::version::Version::V05;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[26]);

        let version = crate::version::Version::V05;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[24]);

        let version = crate::version::Version::V05;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[18]);

        let version = crate::version::Version::V05;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[22]);
    }

    #[test]
    fn generator6() {
        let version = crate::version::Version::V06;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[18]);

        let version = crate::version::Version::V06;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[16]);

        let version = crate::version::Version::V06;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[24]);

        let version = crate::version::Version::V06;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);
    }

    #[test]
    fn generator7() {
        let version = crate::version::Version::V07;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[20]);

        let version = crate::version::Version::V07;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[18]);

        let version = crate::version::Version::V07;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[18]);

        let version = crate::version::Version::V07;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[26]);
    }

    #[test]
    fn generator8() {
        let version = crate::version::Version::V08;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[24]);

        let version = crate::version::Version::V08;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[22]);

        let version = crate::version::Version::V08;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[22]);

        let version = crate::version::Version::V08;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[26]);
    }

    #[test]
    fn generator9() {
        let version = crate::version::Version::V09;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V09;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[22]);

        let version = crate::version::Version::V09;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[20]);

        let version = crate::version::Version::V09;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[24]);
    }

    #[test]
    fn generator10() {
        let version = crate::version::Version::V10;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[18]);

        let version = crate::version::Version::V10;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[26]);

        let version = crate::version::Version::V10;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[24]);

        let version = crate::version::Version::V10;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);
    }

    #[test]
    fn generator11() {
        let version = crate::version::Version::V11;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[20]);

        let version = crate::version::Version::V11;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V11;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V11;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[24]);
    }

    #[test]
    fn generator12() {
        let version = crate::version::Version::V12;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[24]);

        let version = crate::version::Version::V12;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[22]);

        let version = crate::version::Version::V12;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[26]);

        let version = crate::version::Version::V12;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);
    }

    #[test]
    fn generator13() {
        let version = crate::version::Version::V13;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[26]);

        let version = crate::version::Version::V13;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[22]);

        let version = crate::version::Version::V13;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[24]);

        let version = crate::version::Version::V13;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[22]);
    }

    #[test]
    fn generator14() {
        let version = crate::version::Version::V14;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V14;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[24]);

        let version = crate::version::Version::V14;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[20]);

        let version = crate::version::Version::V14;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[24]);
    }

    #[test]
    fn generator15() {
        let version = crate::version::Version::V15;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[22]);

        let version = crate::version::Version::V15;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[24]);

        let version = crate::version::Version::V15;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V15;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[24]);
    }

    #[test]
    fn generator16() {
        let version = crate::version::Version::V16;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[24]);

        let version = crate::version::Version::V16;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V16;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[24]);

        let version = crate::version::Version::V16;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator17() {
        let version = crate::version::Version::V17;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V17;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V17;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V17;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);
    }

    #[test]
    fn generator18() {
        let version = crate::version::Version::V18;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V18;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[26]);

        let version = crate::version::Version::V18;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V18;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);
    }

    #[test]
    fn generator19() {
        let version = crate::version::Version::V19;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V19;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[26]);

        let version = crate::version::Version::V19;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[26]);

        let version = crate::version::Version::V19;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[26]);
    }

    #[test]
    fn generator20() {
        let version = crate::version::Version::V20;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V20;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[26]);

        let version = crate::version::Version::V20;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V20;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);
    }

    #[test]
    fn generator21() {
        let version = crate::version::Version::V21;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V21;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[26]);

        let version = crate::version::Version::V21;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V21;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator22() {
        let version = crate::version::Version::V22;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V22;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V22;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V22;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[24]);
    }

    #[test]
    fn generator23() {
        let version = crate::version::Version::V23;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V23;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V23;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V23;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator24() {
        let version = crate::version::Version::V24;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V24;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V24;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V24;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator25() {
        let version = crate::version::Version::V25;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[26]);

        let version = crate::version::Version::V25;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V25;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V25;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator26() {
        let version = crate::version::Version::V26;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V26;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V26;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V26;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator27() {
        let version = crate::version::Version::V27;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V27;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V27;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V27;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator28() {
        let version = crate::version::Version::V28;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V28;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V28;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V28;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator29() {
        let version = crate::version::Version::V29;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V29;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V29;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V29;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator30() {
        let version = crate::version::Version::V30;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V30;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V30;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V30;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator31() {
        let version = crate::version::Version::V31;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V31;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V31;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V31;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator32() {
        let version = crate::version::Version::V32;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V32;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V32;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V32;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator33() {
        let version = crate::version::Version::V33;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V33;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V33;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V33;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator34() {
        let version = crate::version::Version::V34;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V34;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V34;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V34;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator35() {
        let version = crate::version::Version::V35;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V35;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V35;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V35;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator36() {
        let version = crate::version::Version::V36;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V36;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V36;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V36;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator37() {
        let version = crate::version::Version::V37;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V37;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V37;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V37;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator38() {
        let version = crate::version::Version::V38;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V38;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V38;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V38;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator39() {
        let version = crate::version::Version::V39;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V39;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V39;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V39;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }

    #[test]
    fn generator40() {
        let version = crate::version::Version::V40;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::L);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V40;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::M);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[28]);

        let version = crate::version::Version::V40;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::Q);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);

        let version = crate::version::Version::V40;
        let gen = crate::hardcode::get_polynomial(version, crate::ecl::ECL::H);

        assert_eq!(gen, GENERATOR_POLYNOMIALS[30]);
    }
}
