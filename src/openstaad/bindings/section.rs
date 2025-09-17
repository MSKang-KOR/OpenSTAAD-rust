#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SectionType {
    // BEAM 계열 (610-616)
    BeamSt = 610,
    BeamT = 611,
    BeamCm = 612,
    BeamTc = 613,
    BeamBc = 614,
    BeamTb = 615,
    BeamD = 616,

    // TEE 계열 (620)
    TeeSt = 620,

    // CHANNEL 계열 (630-635)
    ChannelSt = 630,
    ChannelD = 631,
    ChannelFr = 633,
    ChannelColdSt = 634,
    ChannelColdStWithLips = 635,

    // ANGLE 계열 (640-646)
    AngleSt = 640,
    AngleRa = 641,
    AngleLd = 642,
    AngleSd = 643,
    AngleColdSt = 644,
    AngleColdStWithLips = 645,
    AngleSa = 646,

    // TUBE 및 PIPE 계열 (650-656, 660)
    TubeSt = 650,
    HssRectangle = 654,
    HssRound = 655,
    CastelSt = 656,
    PipeSt = 660,

    // ZEE 및 HAT 계열 (662-664)
    ZeeColdSt = 662,
    ZeeColdStWithLips = 663,
    HatColdSt = 664,

    // PLATE 계열 (666)
    PlateStrip = 666,

    // SOLID 계열 (668)
    SolidRound = 668,

    // PRISMATIC 계열 (671-676)
    PrismaticCircle = 671,
    PrismaticRect = 672,
    PrismaticTee = 673,
    PrismaticTrap = 674,
    PrismaticGeneral = 676,

    // TAPER 계열 (675, 680)
    TaperedTube = 675,
    Taper = 680,

    // UPT 계열 (690-699)
    UptWideFlange = 690,
    UptChannel = 691,
    UptAngle = 692,
    UptDoubleAngle = 693,
    UptTee = 694,
    UptPipe = 695,
    UptTube = 696,
    UptGeneral = 697,
    UptIsection = 698,
    UptPrismatic = 699,
}

impl SectionType {
    /// 섹션 타입을 문자열로 반환
    pub fn as_str(&self) -> &'static str {
        match self {
            // BEAM 계열
            SectionType::BeamSt => "BEAM ST",
            SectionType::BeamT => "BEAM T",
            SectionType::BeamCm => "BEAM CM",
            SectionType::BeamTc => "BEAM TC",
            SectionType::BeamBc => "BEAM BC",
            SectionType::BeamTb => "BEAM TB",
            SectionType::BeamD => "BEAM D",

            // TEE 계열
            SectionType::TeeSt => "TEE ST",

            // CHANNEL 계열
            SectionType::ChannelSt => "CHANNEL ST",
            SectionType::ChannelD => "CHANNEL D",
            SectionType::ChannelFr => "CHANNEL FR",
            SectionType::ChannelColdSt => "CHANNEL COLD ST",
            SectionType::ChannelColdStWithLips => "CHANNEL COLD ST WITH LIPS",

            // ANGLE 계열
            SectionType::AngleSt => "ANGLE ST",
            SectionType::AngleRa => "ANGLE RA",
            SectionType::AngleLd => "ANGLE LD",
            SectionType::AngleSd => "ANGLE SD",
            SectionType::AngleColdSt => "ANGLE COLD ST",
            SectionType::AngleColdStWithLips => "ANGLE COLD ST WITH LIPS",
            SectionType::AngleSa => "ANGLE SA",

            // TUBE 및 PIPE 계열
            SectionType::TubeSt => "TUBE ST",
            SectionType::HssRectangle => "HSS RECTANGLE",
            SectionType::HssRound => "HSS ROUND",
            SectionType::CastelSt => "CASTEL ST",
            SectionType::PipeSt => "PIPE ST",

            // ZEE 및 HAT 계열
            SectionType::ZeeColdSt => "ZEE COLD ST",
            SectionType::ZeeColdStWithLips => "ZEE COLD ST WITH LIPS",
            SectionType::HatColdSt => "HAT COLD ST",

            // PLATE 계열
            SectionType::PlateStrip => "PLATE STRIP",

            // SOLID 계열
            SectionType::SolidRound => "SOLID ROUND",

            // PRISMATIC 계열
            SectionType::PrismaticCircle => "PRISMATIC CIRCLE",
            SectionType::PrismaticRect => "PRISMATIC RECT",
            SectionType::PrismaticTee => "PRISMATIC TEE",
            SectionType::PrismaticTrap => "PRISMATIC TRAP",
            SectionType::PrismaticGeneral => "PRISMATIC GENERAL",

            // TAPER 계열
            SectionType::TaperedTube => "TAPERED TUBE",
            SectionType::Taper => "TAPER",

            // UPT 계열
            SectionType::UptWideFlange => "UPT WIDE FLANGE",
            SectionType::UptChannel => "UPT CHANNEL",
            SectionType::UptAngle => "UPT ANGLE",
            SectionType::UptDoubleAngle => "UPT DOUBLE ANGLE",
            SectionType::UptTee => "UPT TEE",
            SectionType::UptPipe => "UPT PIPE",
            SectionType::UptTube => "UPT TUBE",
            SectionType::UptGeneral => "UPT GENERAL",
            SectionType::UptIsection => "UPT ISECTION",
            SectionType::UptPrismatic => "UPT PRISMATIC",
        }
    }

    /// 프로퍼티 타입 번호를 반환
    pub fn as_number(&self) -> u16 {
        *self as u16
    }

    /// 프로퍼티 타입 번호로부터 섹션 타입을 생성
    pub fn from(number: u16) -> Option<Self> {
        match number {
            610 => Some(SectionType::BeamSt),
            611 => Some(SectionType::BeamT),
            612 => Some(SectionType::BeamCm),
            613 => Some(SectionType::BeamTc),
            614 => Some(SectionType::BeamBc),
            615 => Some(SectionType::BeamTb),
            616 => Some(SectionType::BeamD),
            620 => Some(SectionType::TeeSt),
            630 => Some(SectionType::ChannelSt),
            631 => Some(SectionType::ChannelD),
            633 => Some(SectionType::ChannelFr),
            634 => Some(SectionType::ChannelColdSt),
            635 => Some(SectionType::ChannelColdStWithLips),
            640 => Some(SectionType::AngleSt),
            641 => Some(SectionType::AngleRa),
            642 => Some(SectionType::AngleLd),
            643 => Some(SectionType::AngleSd),
            644 => Some(SectionType::AngleColdSt),
            645 => Some(SectionType::AngleColdStWithLips),
            646 => Some(SectionType::AngleSa),
            650 => Some(SectionType::TubeSt),
            654 => Some(SectionType::HssRectangle),
            655 => Some(SectionType::HssRound),
            656 => Some(SectionType::CastelSt),
            660 => Some(SectionType::PipeSt),
            662 => Some(SectionType::ZeeColdSt),
            663 => Some(SectionType::ZeeColdStWithLips),
            664 => Some(SectionType::HatColdSt),
            666 => Some(SectionType::PlateStrip),
            668 => Some(SectionType::SolidRound),
            671 => Some(SectionType::PrismaticCircle),
            672 => Some(SectionType::PrismaticRect),
            673 => Some(SectionType::PrismaticTee),
            674 => Some(SectionType::PrismaticTrap),
            675 => Some(SectionType::TaperedTube),
            676 => Some(SectionType::PrismaticGeneral),
            680 => Some(SectionType::Taper),
            690 => Some(SectionType::UptWideFlange),
            691 => Some(SectionType::UptChannel),
            692 => Some(SectionType::UptAngle),
            693 => Some(SectionType::UptDoubleAngle),
            694 => Some(SectionType::UptTee),
            695 => Some(SectionType::UptPipe),
            696 => Some(SectionType::UptTube),
            697 => Some(SectionType::UptGeneral),
            698 => Some(SectionType::UptIsection),
            699 => Some(SectionType::UptPrismatic),
            _ => None,
        }
    }

    /// 섹션 카테고리를 반환
    pub fn category(&self) -> &'static str {
        match self {
            SectionType::BeamSt
            | SectionType::BeamT
            | SectionType::BeamCm
            | SectionType::BeamTc
            | SectionType::BeamBc
            | SectionType::BeamTb
            | SectionType::BeamD => "BEAM",

            SectionType::TeeSt => "TEE",

            SectionType::ChannelSt
            | SectionType::ChannelD
            | SectionType::ChannelFr
            | SectionType::ChannelColdSt
            | SectionType::ChannelColdStWithLips => "CHANNEL",

            SectionType::AngleSt
            | SectionType::AngleRa
            | SectionType::AngleLd
            | SectionType::AngleSd
            | SectionType::AngleColdSt
            | SectionType::AngleColdStWithLips
            | SectionType::AngleSa => "ANGLE",

            SectionType::TubeSt
            | SectionType::HssRectangle
            | SectionType::HssRound
            | SectionType::CastelSt => "TUBE",

            SectionType::PipeSt => "PIPE",

            SectionType::ZeeColdSt | SectionType::ZeeColdStWithLips => "ZEE",

            SectionType::HatColdSt => "HAT",

            SectionType::PlateStrip => "PLATE",

            SectionType::SolidRound => "SOLID",

            SectionType::PrismaticCircle
            | SectionType::PrismaticRect
            | SectionType::PrismaticTee
            | SectionType::PrismaticTrap
            | SectionType::PrismaticGeneral => "PRISMATIC",

            SectionType::TaperedTube | SectionType::Taper => "TAPER",

            SectionType::UptWideFlange
            | SectionType::UptChannel
            | SectionType::UptAngle
            | SectionType::UptDoubleAngle
            | SectionType::UptTee
            | SectionType::UptPipe
            | SectionType::UptTube
            | SectionType::UptGeneral
            | SectionType::UptIsection
            | SectionType::UptPrismatic => "UPT",
        }
    }

    pub fn properties(&self) -> Vec<String> {
        match self {
            // BEAM 계열
            SectionType::BeamSt => vec!["Ax", "D", "Bf", "Tf", "Tw", "Iz", "Iy", "Ix"],
            SectionType::BeamD => vec!["D", "Bf", "Tf", "Tw", "Iz", "Iy", "Ix", "SP"],
            SectionType::BeamTc => vec!["Ax", "D", "Bf", "Tf", "Tw", "Iz", "Iy", "Ix", "WP", "TH"],
            SectionType::BeamBc => vec!["Ax", "D", "Bf", "Tf", "Tw", "Iz", "Iy", "Ix", "WP", "TH"],
            SectionType::BeamTb => vec![
                "Ax", "D", "Bf", "Tf", "Tw", "Iz", "Iy", "Ix", "WP", "TH", "BW", "BT",
            ],
            SectionType::BeamT => vec!["Ax", "D", "Bf", "Tf", "Tw", "Iz", "Iy", "Ix"],
            SectionType::BeamCm => vec![
                "Ax", "D", "Bf", "Tf", "Tw", "Iz", "Iy", "Ix", "CT", "FC", "CW", "CD",
            ],

            // TEE 계열
            SectionType::TeeSt => vec!["Ax", "D", "Bf", "Tf", "Tw", "Iz", "Iy", "Ix"],

            // CHANNEL 계열
            SectionType::ChannelSt => vec!["Ax", "D", "Bf", "Tf", "Tw", "Iz", "Iy", "Ix"],
            SectionType::ChannelD => vec!["Ax", "D", "Bf", "Tf", "Tw", "Iz", "Iy", "Ix", "SP"],
            SectionType::ChannelFr => vec!["Ax", "D", "Bf", "Tf", "Tw", "Iz", "Iy", "Ix", "FR"],
            SectionType::ChannelColdSt => {
                vec!["Ax", "D", "Bf", "T", "R", "Iz", "Iy", "Ix", "Ay", "Az"]
            }
            SectionType::ChannelColdStWithLips => vec![
                "Ax", "D", "Bf", "T", "R", "Iz", "Iy", "Ix", "LIP", "Ay", "Az",
            ],

            // ANGLE 계열
            SectionType::AngleSt => vec!["Ax", "D", "B", "T", "Iz", "Iy", "Ix"],
            SectionType::AngleRa => vec!["Ax", "D", "B", "T", "Iz", "Iy", "Ix"],
            SectionType::AngleLd => vec!["Ax", "D", "B", "T", "Iz", "Iy", "Ix", "LD"],
            SectionType::AngleSd => vec!["Ax", "D", "B", "T", "Iz", "Iy", "Ix", "SD"],
            SectionType::AngleSa => vec!["Ax", "D", "B", "T", "Iz", "Iy", "Ix"],
            SectionType::AngleColdSt => {
                vec!["Ax", "D", "B", "T", "Iz", "Iy", "Ix", "R", "Ay", "Az"]
            }
            SectionType::AngleColdStWithLips => vec![
                "Ax", "D", "B", "T", "Iz", "Iy", "Ix", "R", "LIP", "Ay", "Az",
            ],

            // TUBE 및 PIPE 계열
            SectionType::TubeSt => vec!["Ax", "D", "B", "T", "Iz", "Iy", "Ix"],
            SectionType::HssRectangle => vec!["Ax", "D", "B", "T", "Iz", "Iy", "Ix"],
            SectionType::HssRound => vec!["Ax", "OD", "Tw", "Iz", "Iy", "Ix"],
            SectionType::CastelSt => vec!["Ax", "D", "Bf", "Tf", "Tw", "Iz", "Iy", "Ix"],
            SectionType::PipeSt => vec!["Ax", "OD", "Tw", "Iz", "Iy", "Ix"],

            // ZEE 및 HAT 계열
            SectionType::ZeeColdSt => vec!["Ax", "D", "B", "T", "R", "Iz", "Iy", "Ix", "Ay", "Az"],
            SectionType::ZeeColdStWithLips => vec![
                "Ax",
                "D",
                "B",
                "T",
                "LIP",
                "LIP_Angle",
                "R",
                "Iz",
                "Iy",
                "Ix",
                "Ay",
                "Az",
            ],
            SectionType::HatColdSt => vec![
                "Ax", "D", "B", "T", "BOT_F", "R", "Iz", "Iy", "Ix", "Ay", "Az",
            ],

            // PLATE 계열
            SectionType::PlateStrip => vec!["Ax", "D", "B", "Iz", "Iy", "Ix"],

            // SOLID 계열
            SectionType::SolidRound => vec!["Ax", "OD", "Tw", "Iz", "Iy", "Ix", "Z"],

            // PRISMATIC 계열
            SectionType::PrismaticCircle => vec!["Ax", "Iz", "Iy", "Ix", "YD"],
            SectionType::PrismaticRect => vec!["Ax", "Iz", "Iy", "Ix", "YD", "ZD"],
            SectionType::PrismaticTrap => vec!["Ax", "Iz", "Iy", "Ix", "YD", "ZD", "ZB"],
            SectionType::PrismaticTee => vec!["Ax", "Iz", "Iy", "Ix", "YD", "ZD", "YB", "ZB"],
            SectionType::PrismaticGeneral => {
                vec!["Ax", "Ay", "Az", "Ix", "Iy", "Iz", "YD", "ZD", "YB", "ZB"]
            }

            // TAPER 계열
            SectionType::Taper => vec!["F1", "F2", "F3", "F4", "F5", "F6", "F7"],
            SectionType::TaperedTube => {
                vec!["Ax", "Iz", "Iy", "Ix", "D1", "D2", "TH", "SECTION_TYPE"]
            }

            // UPT 계열
            SectionType::UptWideFlange => vec![
                "Ax", "D", "Tw", "Wf", "Tf", "Iz", "Iy", "Ix", "Ay", "Az", "Wf1", "Tf1",
            ],
            SectionType::UptChannel => vec![
                "Ax", "D", "Tw", "Wf", "Tf", "Iz", "Iy", "Ix", "Cz", "Ay", "Az",
            ],
            SectionType::UptAngle => vec!["Ax", "D", "Wf", "Tf", "R", "Ay", "Az", "Iz", "Iy", "Ix"],
            SectionType::UptDoubleAngle => vec![
                "Ax", "D", "Wf", "Tf", "SP", "Iz", "Iy", "Ix", "Cy", "Ay", "Az",
            ],
            SectionType::UptTee => vec![
                "Ax", "D", "Wf", "Tf", "Tw", "Iz", "Iy", "Ix", "Cy", "Ay", "Az",
            ],
            SectionType::UptPipe => vec!["Ax", "OD", "ID", "Ay", "Az", "Iz", "Iy", "Ix"],
            SectionType::UptTube => vec!["Ax", "D", "Wf", "Tf", "Iz", "Iy", "Ix", "Ay", "Az"],
            SectionType::UptGeneral => vec![
                "Ax", "D", "Td", "B", "Tb", "Iz", "Iy", "Ix", "Sz", "Sy", "Ay", "Az", "Pz", "Py",
                "Hss", "Dee",
            ],
            SectionType::UptIsection => vec![
                "Dww", "Tww", "Dww1", "Bff", "Tff", "Bff1", "Tff1", "Ayf", "Azf", "Xif",
            ],
            SectionType::UptPrismatic => vec!["Ax", "Iz", "Iy", "Ix", "Ay", "Az", "YD", "ZD"],
        }
        .into_iter()
        .map(|s| s.to_string())
        .collect()
    }
}
