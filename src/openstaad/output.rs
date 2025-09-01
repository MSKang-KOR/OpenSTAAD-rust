use serde::Serialize;
use std::collections::HashMap;
use windows::Win32::System::Com::IDispatch;

use crate::tools::value_types::{InType as itype, MethodSignature, OutType as otype};
use std::fmt::{Debug, Formatter, Result};

#[derive(Clone, Serialize)]
pub struct Output {
    pub id: String,
    #[serde(skip)]
    pub dispatch: IDispatch,
    #[serde(skip)]
    pub methods: HashMap<String, MethodSignature>,
}
impl Debug for Output {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Output Struct")
    }
}

impl Output {
    pub fn new(dispatch: IDispatch) -> Self {
        let mut methods = HashMap::new();

        // Output
        methods.insert(
            "AreResultsAvailable".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![otype::Bool],
            },
        );

        // Output::Nodes/Joints/Supports
        methods.insert(
            "GetBasePressures".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::VecInt,
                    itype::MutVecDouble,
                    itype::MutVecDouble,
                    itype::MutVecDouble,
                ],
                outputs: vec![
                    otype::Bool,
                    otype::Index(2),
                    otype::Index(3),
                    otype::Index(4),
                ],
            },
        );
        methods.insert(
            "GetMatInfluenceAreas".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::VecInt,
                    itype::MutVecDouble,
                    itype::MutVecDouble,
                    itype::MutVecDouble,
                ],
                outputs: vec![
                    otype::Int,
                    otype::Index(1),
                    otype::Index(2),
                    otype::Index(3),
                ],
            },
        );
        methods.insert(
            "GetNodeDisplacements".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int, itype::MutVecDouble],
                outputs: vec![otype::Bool, otype::Index(2)],
            },
        );
        methods.insert(
            "GetSupportReactions".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int, itype::MutVecDouble],
                outputs: vec![otype::Bool, otype::Index(2)],
            },
        );

        // Output::Members
        methods.insert(
            "GetIntermediateDeflectionAtDistance".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::Double,
                    itype::Int,
                    itype::MutDouble,
                    itype::MutDouble,
                ],
                outputs: vec![otype::Bool, otype::Index(3), otype::Index(4)],
            },
        );
        methods.insert(
            "GetIntermediateMemberAbsTransDisplacements".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Double, itype::Int, itype::MutVecDouble],
                outputs: vec![otype::Bool, otype::Index(3)],
            },
        );
        methods.insert(
            "GetIntermediateMemberForcesAtDistance".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Double, itype::Int, itype::MutVecDouble],
                outputs: vec![otype::Bool, otype::Index(3)],
            },
        );
        methods.insert(
            "GetIntermediateMemberTransDisplacements".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Double, itype::Int, itype::MutVecDouble],
                outputs: vec![otype::Bool, otype::Index(3)],
            },
        );
        methods.insert(
            "GetMaxSectionDisplacement".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::Double,
                    itype::Int,
                    itype::MutDouble,
                    itype::MutDouble,
                ],
                outputs: vec![otype::Bool, otype::Index(3), otype::Index(4)],
            },
        );
        methods.insert(
            "GetMemberEndDisplacements".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int, itype::Int, itype::MutVecDouble],
                outputs: vec![otype::Bool, otype::Index(3)],
            },
        );
        methods.insert(
            "GetMemberEndForces".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::Int,
                    itype::Int,
                    itype::MutVecDouble,
                    itype::Int,
                ],
                outputs: vec![otype::Bool, otype::Index(3)],
            },
        );
        methods.insert(
            "GetMinMaxAxialForce".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::Int,
                    itype::MutDouble,
                    itype::MutDouble,
                    itype::MutDouble,
                    itype::MutDouble,
                ],
                outputs: vec![
                    otype::Bool,
                    otype::Index(2),
                    otype::Index(3),
                    otype::Index(4),
                    otype::Index(5),
                ],
            },
        );
        methods.insert(
            "GetMinMaxBendingMoment".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::Double,
                    itype::Int,
                    itype::MutDouble,
                    itype::MutDouble,
                    itype::MutDouble,
                    itype::MutDouble,
                ],
                outputs: vec![
                    otype::Bool,
                    otype::Index(3),
                    otype::Index(4),
                    otype::Index(5),
                    otype::Index(6),
                ],
            },
        );
        methods.insert(
            "GetMinMaxShearForce".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::Double,
                    itype::Int,
                    itype::MutDouble,
                    itype::MutDouble,
                    itype::MutDouble,
                    itype::MutDouble,
                ],
                outputs: vec![
                    otype::Bool,
                    otype::Index(3),
                    otype::Index(4),
                    otype::Index(5),
                    otype::Index(6),
                ],
            },
        );
        methods.insert(
            "GetPMemberEndForces".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::Int,
                    itype::Int,
                    itype::MutVecDouble,
                    itype::Int,
                ],
                outputs: vec![otype::Bool, otype::Index(3)],
            },
        );
        methods.insert(
            "GetPMemberIntermediateForcesAtDistance".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Double, itype::Int, itype::MutVecDouble],
                outputs: vec![otype::Bool, otype::Index(3)],
            },
        );

        // Output::Plate
        methods.insert(
            "GetAllPlateCenterForces".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int, itype::MutVecDouble],
                outputs: vec![otype::Bool, otype::Index(2)],
            },
        );
        methods.insert(
            "GetAllPlateCenterMoments".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int, itype::MutVecDouble],
                outputs: vec![otype::Bool, otype::Index(2)],
            },
        );
        methods.insert(
            "GetAllPlateCenterPrincipalStressesAndAngles".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int, itype::MutVecDouble],
                outputs: vec![otype::Bool, otype::Index(2)],
            },
        );
        methods.insert(
            "GetAllPlateCenterPrincipalStressesAndAnglesEx".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::Int,
                    itype::MutVecDouble,
                    itype::MutVecDouble,
                ],
                outputs: vec![otype::Bool, otype::Index(2), otype::Index(3)],
            },
        );
        methods.insert(
            "GetAllPlateCenterStressesAndMoments".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int, itype::MutVecDouble],
                outputs: vec![otype::Bool, otype::Index(2)],
            },
        );
        methods.insert(
            "GetPlateCenterNormalPrincipalStresses".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::Int,
                    itype::MutDouble,
                    itype::MutDouble,
                    itype::MutDouble,
                    itype::MutDouble,
                ],
                outputs: vec![
                    otype::Bool,
                    otype::Index(2),
                    otype::Index(3),
                    otype::Index(4),
                    otype::Index(5),
                ],
            },
        );
        methods.insert(
            "GetPlateCenterVonMisesStresses".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int, itype::MutDouble, itype::MutDouble],
                outputs: vec![otype::Bool, otype::Index(2), otype::Index(3)],
            },
        );
        methods.insert(
            "GetPlateCornerForces".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int, itype::Int, itype::MutVecDouble],
                outputs: vec![otype::Bool, otype::Index(3)],
            },
        );
        methods.insert(
            "GetPlateStressAtPoint".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::Int,
                    itype::VecDouble,
                    itype::VecDouble,
                    itype::MutVecDouble,
                ],
                outputs: vec![otype::Bool, otype::Index(4)],
            },
        );
        methods.insert(
            "GetResultantForceAlongLineForParametricSurface".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Str,
                    itype::VecInt,
                    itype::VecDouble,
                    itype::VecDouble,
                    itype::Int,
                    itype::Int,
                    itype::Int,
                    itype::Int,
                    itype::MutVecDouble,
                ],
                outputs: vec![otype::Bool, otype::Index(8)],
            },
        );
        methods.insert(
            "GetResultantForceAlongLineForPlateList".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::VecInt,
                    itype::Int,
                    itype::VecInt,
                    itype::VecDouble,
                    itype::VecDouble,
                    itype::Int,
                    itype::Int,
                    itype::Int,
                    itype::Int,
                    itype::MutVecDouble,
                ],
                outputs: vec![otype::Bool, otype::Index(9)],
            },
        );

        // Output::Solids
        methods.insert(
            "GetAllSolidNormalStresses".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int, itype::Int, itype::MutVecDouble],
                outputs: vec![otype::Bool, otype::Index(3)],
            },
        );
        methods.insert(
            "GetAllSolidPrincipalStresses".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int, itype::Int, itype::MutVecDouble],
                outputs: vec![otype::Bool, otype::Index(3)],
            },
        );
        methods.insert(
            "GetAllSolidShearStresses".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int, itype::Int, itype::MutVecDouble],
                outputs: vec![otype::Bool, otype::Index(3)],
            },
        );
        methods.insert(
            "GetAllSolidVonMisesStresses".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int, itype::Int, itype::MutDouble],
                outputs: vec![otype::Bool, otype::Index(3)],
            },
        );

        // Output::Dynamic
        methods.insert(
            "GetMaxBeamStresses".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::Int,
                    itype::MutDouble,
                    itype::MutInt,
                    itype::MutDouble,
                    itype::MutInt,
                ],
                outputs: vec![
                    otype::Int,
                    otype::Index(2),
                    otype::Index(3),
                    otype::Index(4),
                    otype::Index(5),
                ],
            },
        );
        methods.insert(
            "GetMissingMassParticipationFactors".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::MutDouble,
                    itype::MutDouble,
                    itype::MutDouble,
                ],
                outputs: vec![
                    otype::Bool,
                    otype::Index(1),
                    otype::Index(2),
                    otype::Index(3),
                ],
            },
        );
        methods.insert(
            "GetModalDisplacementAtNode".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int, itype::MutVecDouble],
                outputs: vec![otype::Bool, otype::Index(2)],
            },
        );
        methods.insert(
            "GetModalMassParticipationFactors".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::MutDouble,
                    itype::MutDouble,
                    itype::MutDouble,
                ],
                outputs: vec![
                    otype::Bool,
                    otype::Index(1),
                    otype::Index(2),
                    otype::Index(3),
                ],
            },
        );
        methods.insert(
            "GetModeFrequency".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::MutDouble],
                outputs: vec![otype::Bool, otype::Index(1)],
            },
        );
        methods.insert(
            "GetNLLoadStep".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetNLNodeDisplacements".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::Int,
                    itype::Int,
                    itype::MutDouble,
                    itype::MutVecDouble,
                ],
                outputs: vec![otype::Bool, otype::Index(3), otype::Index(4)],
            },
        );
        methods.insert(
            "GetNoOfModesExtracted".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetTimeHistoryIntegrationStepInfo".to_string(),
            MethodSignature {
                inputs: vec![itype::MutDouble],
                outputs: vec![otype::Int, otype::Index(0)],
            },
        );
        methods.insert(
            "GetTimeHistoryResponse".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::Int,
                    itype::Int,
                    itype::Int,
                    itype::MutVecDouble,
                ],
                outputs: vec![otype::Int, otype::Index(4)],
            },
        );
        methods.insert(
            "GetTimeHistoryResponseAtTime".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::Int,
                    itype::Int,
                    itype::Int,
                    itype::Double,
                    itype::MutDouble,
                ],
                outputs: vec![otype::Int, otype::Index(5)],
            },
        );
        methods.insert(
            "GetTimeHistoryResponseMinMax".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::Int,
                    itype::Int,
                    itype::Int,
                    itype::MutDouble,
                    itype::MutDouble,
                    itype::MutDouble,
                    itype::MutDouble,
                ],
                outputs: vec![
                    otype::Int,
                    otype::Index(4),
                    otype::Index(5),
                    otype::Index(6),
                    otype::Index(7),
                ],
            },
        );

        // Output::Static
        methods.insert(
            "GetStaticCheckResult".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::MutVecDouble, itype::MutVecDouble],
                outputs: vec![otype::Bool, otype::Index(1), otype::Index(2)],
            },
        );

        // Output::Buckling
        methods.insert(
            "GetBucklingFactor".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::MutDouble],
                outputs: vec![otype::Bool, otype::Index(1)],
            },
        );
        methods.insert(
            "GetBucklingModeDisplacementAtNode".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int, itype::MutVecDouble],
                outputs: vec![otype::Bool, otype::Index(2)],
            },
        );
        methods.insert(
            "GetNoOfBucklingFactors".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "IsBucklingAnalysisResultsAvailable".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![otype::Int],
            },
        );

        // Design Output Methods
        methods.insert(
            "GetMemberDesignSectionName".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Str],
            },
        );
        methods.insert(
            "GetMemberSteelDesignMaxFailureRatio".to_string(),
            MethodSignature {
                inputs: vec![itype::MutDouble],
                outputs: vec![otype::Bool, otype::Index(0)],
            },
        );
        methods.insert(
            "GetMemberSteelDesignMinFailureRatio".to_string(),
            MethodSignature {
                inputs: vec![itype::MutDouble],
                outputs: vec![otype::Bool, otype::Index(0)],
            },
        );
        methods.insert(
            "GetMemberSteelDesignRatio".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::MutDouble],
                outputs: vec![otype::Bool, otype::Index(1)],
            },
        );
        methods.insert(
            "GetMemberSteelDesignResults".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::MutStr,
                    itype::MutStr,
                    itype::MutDouble,
                    itype::MutDouble,
                    itype::MutInt,
                    itype::MutDouble,
                    itype::MutStr,
                    itype::MutStr,
                    itype::MutVecDouble,
                    itype::MutDouble,
                ],
                outputs: vec![
                    otype::Bool,
                    otype::Index(1),
                    otype::Index(2),
                    otype::Index(3),
                    otype::Index(4),
                    otype::Index(5),
                    otype::Index(6),
                    otype::Index(7),
                    otype::Index(8),
                    otype::Index(9),
                    otype::Index(10),
                ],
            },
        );
        methods.insert(
            "GetMultipleMemberSteelDesignMaxRatio".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::MutDouble],
                outputs: vec![otype::Bool, otype::Index(1)],
            },
        );
        methods.insert(
            "GetMultipleMemberSteelDesignRatio".to_string(),
            MethodSignature {
                inputs: vec![itype::Str, itype::Int, itype::MutDouble],
                outputs: vec![otype::Bool, otype::Index(2)],
            },
        );
        methods.insert(
            "GetMultipleMemberSteelDesignResults".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Str,
                    itype::Int,
                    itype::MutStr,
                    itype::MutStr,
                    itype::MutDouble,
                    itype::MutDouble,
                    itype::MutInt,
                    itype::MutStr,
                    itype::MutStr,
                ],
                outputs: vec![
                    otype::Bool,
                    otype::Index(2),
                    otype::Index(3),
                    otype::Index(4),
                    otype::Index(5),
                    otype::Index(6),
                    otype::Index(7),
                    otype::Index(8),
                ],
            },
        );
        methods.insert(
            "GetSteelDesignParameterBlockCount".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetSteelDesignParameterBlockNameByIndex".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::MutStr],
                outputs: vec![otype::Bool, otype::Index(1)],
            },
        );
        methods.insert(
            "IsMultipleMemberSteelDesignResultsAvailable".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![otype::Int],
            },
        );

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            dispatch,
            methods,
        }
    }
}

unsafe impl Send for Output {}
unsafe impl Sync for Output {}
