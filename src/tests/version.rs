use crate::{Mask, QRCode};

#[test]
fn version_format_l_mask0() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Checkerboard);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V05);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::L);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, true, true, false, true, true, true, true, true, false, false, false, true, false,
        false,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
            mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
            mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
        ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
            mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
            mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
        ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_l_mask1() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::HorizontalLines);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V03);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::L);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, true, true, false, false, true, false, true, true, true, true, false, false, true,
        true,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
            mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
            mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
        ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
            mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
            mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
        ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_l_mask2() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::VerticalLines);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V06);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::L);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, true, true, true, true, false, true, true, false, true, false, true, false, true,
        false,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
            mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
            mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
        ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
            mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
            mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
        ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_l_mask3() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::DiagonalLines);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V03);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::L);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, true, true, true, false, false, false, true, false, false, true, true, true, false,
        true,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
            mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
            mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
        ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
            mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
            mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
        ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_l_mask4() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::LargeCheckerboard);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V06);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::L);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, true, false, false, true, true, false, false, false, true, false, true, true, true,
        true,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
            mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
            mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
        ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
            mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
            mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
        ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_l_mask5() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Fields);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V06);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::L);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, true, false, false, false, true, true, false, false, false, true, true, false, false,
        false,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]

        #[rustfmt::skip]
        let tmp = [
            mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
            mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
        ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
            mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
            mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
        ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_l_mask6() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Diamonds);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V06);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::L);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, true, false, true, true, false, false, false, true, false, false, false, false,
        false, true,
    ];

    {
        let l = mat.size;

        #[rustfmt::skip]
        let tmp = [
            mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
            mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
        ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
            mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
            mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
        ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_l_mask7() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Meadow);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V05);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::L);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, true, false, true, false, false, true, false, true, true, true, false, true, true,
        false,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_m_mask0() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Checkerboard);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V01);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::M);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, false, true, false, true, false, false, false, false, false, true, false, false,
        true, false,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_m_mask1() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::HorizontalLines);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V04);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::M);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, false, true, false, false, false, true, false, false, true, false, false, true,
        false, true,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_m_mask2() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::VerticalLines);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V02);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::M);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, false, true, true, true, true, false, false, true, true, true, true, true, false,
        false,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_m_mask3() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::DiagonalLines);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V06);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::M);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, false, true, true, false, true, true, false, true, false, false, true, false, true,
        true,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_m_mask4() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::LargeCheckerboard);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V01);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::M);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, false, false, false, true, false, true, true, true, true, true, true, false, false,
        true,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_m_mask5() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Fields);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V02);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::M);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, false, false, false, false, false, false, true, true, false, false, true, true, true,
        false,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_m_mask6() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Diamonds);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V01);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::M);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, false, false, true, true, true, true, true, false, false, true, false, true, true,
        true,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_m_mask7() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Meadow);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V03);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::M);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, false, false, true, false, true, false, true, false, true, false, false, false,
        false, false,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_q_mask0() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Checkerboard);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V04);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::Q);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, true, true, false, true, false, true, false, true, false, true, true, true, true,
        true,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_q_mask1() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::HorizontalLines);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V02);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::Q);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, true, true, false, false, false, false, false, true, true, false, true, false,
        false, false,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_q_mask2() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::VerticalLines);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V04);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::Q);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, true, true, true, true, true, true, false, false, true, true, false, false, false,
        true,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_q_mask3() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::DiagonalLines);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V05);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::Q);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, true, true, true, false, true, false, false, false, false, false, false, true, true,
        false,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_q_mask4() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::LargeCheckerboard);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V01);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::Q);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, true, false, false, true, false, false, true, false, true, true, false, true, false,
        false,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_q_mask5() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Fields);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V05);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::Q);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, true, false, false, false, false, true, true, false, false, false, false, false,
        true, true,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_q_mask6() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Diamonds);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V02);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::Q);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, true, false, true, true, true, false, true, true, false, true, true, false, true,
        false,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_q_mask7() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Meadow);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V01);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::Q);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, true, false, true, false, true, true, true, true, true, false, true, true, false,
        true,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_h_mask0() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Checkerboard);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V06);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::H);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, false, true, false, true, true, false, true, false, false, false, true, false,
        false, true,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_h_mask1() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::HorizontalLines);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V02);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::H);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, false, true, false, false, true, true, true, false, true, true, true, true, true,
        false,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_h_mask2() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::VerticalLines);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V04);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::H);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, false, true, true, true, false, false, true, true, true, false, false, true, true,
        true,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_h_mask3() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::DiagonalLines);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V03);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::H);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, false, true, true, false, false, true, true, true, false, true, false, false, false,
        false,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_h_mask4() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::LargeCheckerboard);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V02);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::H);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, false, false, false, true, true, true, false, true, true, false, false, false, true,
        false,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_h_mask5() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Fields);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V04);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::H);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, false, false, false, false, true, false, false, true, false, true, false, true,
        false, true,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_h_mask6() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Diamonds);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V02);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::H);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, false, false, true, true, false, true, false, false, false, false, true, true,
        false, false,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_h_mask7() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Meadow);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V01);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::H);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, false, false, true, false, false, false, false, false, true, true, true, false,
        true, true,
    ];

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);
    }
}

#[test]
fn version_format_l_mask0_version23() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Checkerboard);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V23);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::L);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, true, true, false, true, true, true, true, true, false, false, false, true, false,
        false,
    ];
    let mut expected2: [bool; 18] = [
        false, true, false, true, true, true, false, true, true, true, true, true, true, false,
        true, true, false, false,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_l_mask1_version29() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::HorizontalLines);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V29);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::L);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, true, true, false, false, true, false, true, true, true, true, false, false, true,
        true,
    ];
    let mut expected2: [bool; 18] = [
        false, true, true, true, false, true, false, false, true, true, false, false, true, true,
        true, true, true, true,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_l_mask2_version40() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::VerticalLines);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V40);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::L);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, true, true, true, true, false, true, true, false, true, false, true, false, true,
        false,
    ];
    let mut expected2: [bool; 18] = [
        true, false, true, false, false, false, true, true, false, false, false, true, true, false,
        true, false, false, true,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_l_mask3_version8() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::DiagonalLines);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V08);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::L);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, true, true, true, false, false, false, true, false, false, true, true, true, false,
        true,
    ];
    let mut expected2: [bool; 18] = [
        false, false, true, false, false, false, false, true, false, true, true, false, true, true,
        true, true, false, false,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_l_mask4_version36() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::LargeCheckerboard);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V36);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::L);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, true, false, false, true, true, false, false, false, true, false, true, true, true,
        true,
    ];
    let mut expected2: [bool; 18] = [
        true, false, false, true, false, false, true, false, true, true, false, false, false,
        false, true, false, true, true,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_l_mask5_version22() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Fields);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V22);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::L);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, true, false, false, false, true, true, false, false, false, true, true, false, false,
        false,
    ];
    let mut expected2: [bool; 18] = [
        false, true, false, true, true, false, true, false, false, false, true, true, false, false,
        true, false, false, true,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_l_mask6_version10() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Diamonds);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V10);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::L);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, true, false, true, true, false, false, false, true, false, false, false, false,
        false, true,
    ];
    let mut expected2: [bool; 18] = [
        false, false, true, false, true, false, false, true, false, false, true, true, false, true,
        false, false, true, true,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_l_mask7_version17() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Meadow);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V17);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::L);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, true, false, true, false, false, true, false, true, true, true, false, true, true,
        false,
    ];
    let mut expected2: [bool; 18] = [
        false, true, false, false, false, true, false, true, false, false, false, true, false,
        true, true, true, false, true,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_m_mask0_version14() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Checkerboard);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V14);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::M);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, false, true, false, true, false, false, false, false, false, true, false, false,
        true, false,
    ];
    let mut expected2: [bool; 18] = [
        false, false, true, true, true, false, false, true, true, false, false, false, false,
        false, true, true, false, true,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_m_mask1_version30() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::HorizontalLines);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V30);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::M);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, false, true, false, false, false, true, false, false, true, false, false, true,
        false, true,
    ];
    let mut expected2: [bool; 18] = [
        false, true, true, true, true, false, true, true, false, true, false, true, true, true,
        false, true, false, true,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_m_mask2_version37() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::VerticalLines);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V37);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::M);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, false, true, true, true, true, false, false, true, true, true, true, true, false,
        false,
    ];
    let mut expected2: [bool; 18] = [
        true, false, false, true, false, true, false, true, false, false, false, false, true,
        false, true, true, true, false,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_m_mask3_version22() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::DiagonalLines);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V22);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::M);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, false, true, true, false, true, true, false, true, false, false, true, false, true,
        true,
    ];
    let mut expected2: [bool; 18] = [
        false, true, false, true, true, false, true, false, false, false, true, true, false, false,
        true, false, false, true,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_m_mask4_version31() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::LargeCheckerboard);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V31);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::M);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, false, false, false, true, false, true, true, true, true, true, true, false, false,
        true,
    ];
    let mut expected2: [bool; 18] = [
        false, true, true, true, true, true, false, false, true, false, false, true, false, true,
        false, false, false, false,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_m_mask5_version13() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Fields);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V13);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::M);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, false, false, false, false, false, false, true, true, false, false, true, true, true,
        false,
    ];
    let mut expected2: [bool; 18] = [
        false, false, true, true, false, true, true, false, false, false, false, true, false,
        false, false, true, true, true,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_m_mask6_version22() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Diamonds);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V22);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::M);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, false, false, true, true, true, true, true, false, false, true, false, true, true,
        true,
    ];
    let mut expected2: [bool; 18] = [
        false, true, false, true, true, false, true, false, false, false, true, true, false, false,
        true, false, false, true,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_m_mask7_version7() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Meadow);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V07);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::M);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        true, false, false, true, false, true, false, true, false, true, false, false, false,
        false, false,
    ];
    let mut expected2: [bool; 18] = [
        false, false, false, true, true, true, true, true, false, false, true, false, false, true,
        false, true, false, false,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_q_mask0_version20() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Checkerboard);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V20);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::Q);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, true, true, false, true, false, true, false, true, false, true, true, true, true,
        true,
    ];
    let mut expected2: [bool; 18] = [
        false, true, false, true, false, false, true, false, false, true, true, false, true, false,
        false, true, true, false,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_q_mask1_version33() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::HorizontalLines);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V33);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::Q);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, true, true, false, false, false, false, false, true, true, false, true, false,
        false, false,
    ];
    let mut expected2: [bool; 18] = [
        true, false, false, false, false, true, false, true, true, false, true, true, true, true,
        false, false, false, false,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_q_mask2_version24() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::VerticalLines);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V24);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::Q);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, true, true, true, true, true, true, false, false, true, true, false, false, false,
        true,
    ];
    let mut expected2: [bool; 18] = [
        false, true, true, false, false, false, true, true, true, false, true, true, false, false,
        false, true, false, false,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_q_mask3_version18() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::DiagonalLines);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V18);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::Q);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, true, true, true, false, true, false, false, false, false, false, false, true, true,
        false,
    ];
    let mut expected2: [bool; 18] = [
        false, true, false, false, true, false, true, false, true, false, false, false, false,
        true, false, true, true, true,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_q_mask4_version31() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::LargeCheckerboard);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V31);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::Q);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, true, false, false, true, false, false, true, false, true, true, false, true, false,
        false,
    ];
    let mut expected2: [bool; 18] = [
        false, true, true, true, true, true, false, false, true, false, false, true, false, true,
        false, false, false, false,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_q_mask5_version17() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Fields);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V17);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::Q);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, true, false, false, false, false, true, true, false, false, false, false, false,
        true, true,
    ];
    let mut expected2: [bool; 18] = [
        false, true, false, false, false, true, false, true, false, false, false, true, false,
        true, true, true, false, true,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_q_mask6_version11() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Diamonds);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V11);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::Q);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, true, false, true, true, true, false, true, true, false, true, true, false, true,
        false,
    ];
    let mut expected2: [bool; 18] = [
        false, false, true, false, true, true, true, false, true, true, true, true, true, true,
        false, true, true, false,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_q_mask7_version15() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Meadow);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V15);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::Q);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, true, false, true, false, true, true, true, true, true, false, true, true, false,
        true,
    ];
    let mut expected2: [bool; 18] = [
        false, false, true, true, true, true, true, false, false, true, false, false, true, false,
        true, false, false, false,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_h_mask0_version35() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Checkerboard);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V35);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::H);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, false, true, false, true, true, false, true, false, false, false, true, false,
        false, true,
    ];
    let mut expected2: [bool; 18] = [
        true, false, false, false, true, true, false, true, true, true, true, false, false, true,
        true, true, true, true,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_h_mask1_version15() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::HorizontalLines);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V15);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::H);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, false, true, false, false, true, true, true, false, true, true, true, true, true,
        false,
    ];
    let mut expected2: [bool; 18] = [
        false, false, true, true, true, true, true, false, false, true, false, false, true, false,
        true, false, false, false,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_h_mask2_version15() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::VerticalLines);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V15);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::H);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, false, true, true, true, false, false, true, true, true, false, false, true, true,
        true,
    ];
    let mut expected2: [bool; 18] = [
        false, false, true, true, true, true, true, false, false, true, false, false, true, false,
        true, false, false, false,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_h_mask3_version7() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::DiagonalLines);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V07);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::H);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, false, true, true, false, false, true, true, true, false, true, false, false, false,
        false,
    ];
    let mut expected2: [bool; 18] = [
        false, false, false, true, true, true, true, true, false, false, true, false, false, true,
        false, true, false, false,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_h_mask4_version7() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::LargeCheckerboard);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V07);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::H);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, false, false, false, true, true, true, false, true, true, false, false, false, true,
        false,
    ];
    let mut expected2: [bool; 18] = [
        false, false, false, true, true, true, true, true, false, false, true, false, false, true,
        false, true, false, false,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_h_mask5_version20() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Fields);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V20);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::H);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, false, false, false, false, true, false, false, true, false, true, false, true,
        false, true,
    ];
    let mut expected2: [bool; 18] = [
        false, true, false, true, false, false, true, false, false, true, true, false, true, false,
        false, true, true, false,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_h_mask6_version20() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Diamonds);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V20);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::H);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, false, false, true, true, false, true, false, false, false, false, true, true,
        false, false,
    ];
    let mut expected2: [bool; 18] = [
        false, true, false, true, false, false, true, false, false, true, true, false, true, false,
        false, true, true, false,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}

#[test]
fn version_format_h_mask7_version17() {
    const CONTENT: &str = "4";
    const MASK: Option<Mask> = Some(Mask::Meadow);
    const VERSION: Option<crate::version::Version> = Some(crate::version::Version::V17);
    const LEVEL: Option<crate::ecl::ECL> = Some(crate::ecl::ECL::H);

    let q = QRCode::new(CONTENT.as_bytes(), LEVEL, VERSION, None, MASK);
    if q.is_err() {
        assert_eq!(true, false, "Couldn't create QR");
    };
    let mat = q.unwrap();

    const EXPECTED: [bool; 15] = [
        false, false, false, true, false, false, false, false, false, true, true, true, false,
        true, true,
    ];
    let mut expected2: [bool; 18] = [
        false, true, false, false, false, true, false, true, false, false, false, true, false,
        true, true, true, false, true,
    ];
    expected2.reverse();

    {
        let l = mat.size;
        #[rustfmt::skip]
        let tmp = [
                mat[l - 1][8], mat[l - 2][8], mat[l - 3][8], mat[l - 4][8], mat[l - 5][8], mat[l - 6][8], mat[l - 7][8],
                mat[8][l - 8], mat[8][l - 7], mat[8][l - 6], mat[8][l - 5], mat[8][l - 4], mat[8][l - 3], mat[8][l - 2], mat[8][l - 1]
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        #[rustfmt::skip]
        let tmp = [
                mat[8][0], mat[8][1], mat[8][2], mat[8][3], mat[8][4], mat[8][5], mat[8][7], mat[8][8],
                mat[7][8], mat[5][8], mat[4][8], mat[3][8], mat[2][8], mat[1][8], mat[0][8],
            ];
        assert_eq!(tmp.map(|x| x.value()), EXPECTED);

        let tmp2 = [
            mat[l - 11][0],
            mat[l - 10][0],
            mat[l - 9][0],
            mat[l - 11][1],
            mat[l - 10][1],
            mat[l - 9][1],
            mat[l - 11][2],
            mat[l - 10][2],
            mat[l - 9][2],
            mat[l - 11][3],
            mat[l - 10][3],
            mat[l - 9][3],
            mat[l - 11][4],
            mat[l - 10][4],
            mat[l - 9][4],
            mat[l - 11][5],
            mat[l - 10][5],
            mat[l - 9][5],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);

        let tmp2 = [
            mat[0][l - 11],
            mat[0][l - 10],
            mat[0][l - 9],
            mat[1][l - 11],
            mat[1][l - 10],
            mat[1][l - 9],
            mat[2][l - 11],
            mat[2][l - 10],
            mat[2][l - 9],
            mat[3][l - 11],
            mat[3][l - 10],
            mat[3][l - 9],
            mat[4][l - 11],
            mat[4][l - 10],
            mat[4][l - 9],
            mat[5][l - 11],
            mat[5][l - 10],
            mat[5][l - 9],
        ];
        assert_eq!(tmp2.map(|x| x.value()), expected2);
    }
}
