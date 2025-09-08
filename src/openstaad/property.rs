use serde::Serialize;
use std::collections::HashMap;
use windows::Win32::System::Com::IDispatch;

use crate::tools::value_types::{InType as itype, MethodSignature, OutType as otype};
use std::fmt::{Debug, Formatter, Result};

#[derive(Clone, Serialize)]
pub struct Property {
    pub id: String,
    #[serde(skip)]
    pub dispatch: IDispatch,
    #[serde(skip)]
    pub methods: HashMap<String, MethodSignature>,
}
impl Debug for Property {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Property Struct")
    }
}

impl Property {
    pub fn new(dispatch: IDispatch) -> Self {
        let mut methods = HashMap::new();
        let _ = set_methods(&mut methods);
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            dispatch,
            methods,
        }
    }
}

unsafe impl Send for Property {}
unsafe impl Sync for Property {}

fn set_methods(methods: &mut HashMap<String, MethodSignature>) {
    // Property::Section: Create Profile
    methods.insert(
        "CreateAnglePropertyFromTable".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::Str, itype::Int, itype::Double],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateBeamPropertyFromTable".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::Str,
                itype::Int,
                itype::Double,
                itype::Double,
            ],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateBeamPropertyFromTableComposite".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::Str, itype::Int, itype::VecDouble],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateBeamPropertyFromTableEx".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::Str, itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateBeamPropertyFromTableWithCoverPlates".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::Str, itype::Int, itype::VecDouble],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateChannelPropertyFromTable".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::Str, itype::Int, itype::Double],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreatePipePropertyFromTable".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::Str,
                itype::Int,
                itype::Double,
                itype::Double,
            ],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreatePlateThicknessProperty".to_string(),
        MethodSignature {
            inputs: vec![itype::VecDouble],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreatePrismaticCircleProperty".to_string(),
        MethodSignature {
            inputs: vec![itype::Double],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreatePrismaticGeneralProperty".to_string(),
        MethodSignature {
            inputs: vec![itype::VecDouble],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreatePrismaticRectangleProperty".to_string(),
        MethodSignature {
            inputs: vec![itype::Double, itype::Double],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreatePrismaticTeeProperty".to_string(),
        MethodSignature {
            inputs: vec![itype::Double, itype::Double, itype::Double, itype::Double],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreatePrismaticTrapezoidalProperty".to_string(),
        MethodSignature {
            inputs: vec![itype::Double, itype::Double, itype::Double],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateTaperedIProperty".to_string(),
        MethodSignature {
            inputs: vec![itype::VecDouble],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateTaperedTubeProperty".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::Double, itype::Double, itype::Double],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateTeePropertyFromTable".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::Str, itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateTubePropertyFromTable".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::Str,
                itype::Int,
                itype::Double,
                itype::Double,
                itype::Double,
            ],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateWideFlangePropertyFromTable".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::Str, itype::Int, itype::VecDouble],
            outputs: vec![otype::Int],
        },
    );

    // Property::Section: Create Profile from UPT
    methods.insert(
        "AddUPTPropertyANGLE".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::Str,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
            ],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "AddUPTPropertyCHANNEL".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::Str,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
            ],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "AddUPTPropertyDOUBLEANGLE".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::Str,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
            ],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "AddUPTPropertyGENERAL".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::Str,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
            ],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "AddUPTPropertyISECTION".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::Str,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
            ],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "AddUPTPropertyPIPE".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::Str,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
            ],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "AddUPTPropertyPRISMATIC".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::Str,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
            ],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "AddUPTPropertyTEE".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::Str,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
            ],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "AddUPTPropertyTUBE".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::Str,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
            ],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "AddUPTPropertyWIDEFLANGE".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::Str,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
            ],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "AddUPTPropertyWIDEFLANGECOMPOSITE".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::Str, itype::VecDouble],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "AddUPTPropertyWIDEFLANGEUNEQUAL".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::Str, itype::VecDouble],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreatePropertyFromUPTTable".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::Str],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateUPTTable".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetUptGeneralProfileBoundaryPoints".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::Str,
                itype::Bool,
                itype::MutVecDouble,
                itype::MutVecDouble,
            ],
            outputs: vec![otype::Int, otype::Index(3), otype::Index(4)],
        },
    );
    methods.insert(
        "GetUptGeneralProfilePointsCount".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::Str, itype::MutInt, itype::MutInt],
            outputs: vec![otype::Bool, otype::Index(2), otype::Index(3)],
        },
    );
    methods.insert(
        "GetUptGeneralStressLocationPoints".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::Str,
                itype::MutVecDouble,
                itype::MutVecDouble,
            ],
            outputs: vec![otype::Int, otype::Index(2), otype::Index(3)],
        },
    );

    // Property::Section: Beta Angle Operation
    methods.insert(
        "AssignBetaAngle".to_string(),
        MethodSignature {
            inputs: vec![itype::VecInt, itype::Double],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetBetaAngle".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Double],
        },
    );

    // Property::Section: Beta Angle Operation
    methods.insert(
        "AssignBetaAngle".to_string(),
        MethodSignature {
            inputs: vec![itype::VecInt, itype::Double],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetBetaAngle".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Double],
        },
    );

    // Property::Section: Get and Remove General Section Profile
    methods.insert(
        "GetBeamSectionDisplayName".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Str],
        },
    );
    methods.insert(
        "GetBeamSectionName".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Str],
        },
    );
    methods.insert(
        "GetBeamSectionPropertyRefNo".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetBeamSectionPropertyTypeNo".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetBeamSectionPropertyValuesEx".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::MutInt, itype::MutVecDouble],
            outputs: vec![otype::Bool, otype::Index(1), otype::Index(2)],
        },
    );
    methods.insert(
        "GetCountofSectionPropertyValuesEx".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetCountryTableNo".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetIsotropicMaterialAssignedPlateList".to_string(),
        MethodSignature {
            inputs: vec![itype::Str, itype::MutVecInt],
            outputs: vec![otype::Bool, otype::Index(1)],
        },
    );
    methods.insert(
        "GetPlateThickness".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::MutVecDouble],
            outputs: vec![otype::Int, otype::Index(1)],
        },
    );
    methods.insert(
        "GetRecordForSection".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::Str],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetSectionPropertyAssignedBeamCount".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetSectionPropertyAssignedBeamList".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::MutVecInt],
            outputs: vec![otype::Int, otype::Index(1)],
        },
    );
    methods.insert(
        "GetSectionPropertyCount".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetSectionPropertyCountry".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetSectionPropertyList".to_string(),
        MethodSignature {
            inputs: vec![itype::MutVecInt],
            outputs: vec![otype::Bool, otype::Index(0)],
        },
    );
    methods.insert(
        "GetSectionPropertyName".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::MutStr],
            outputs: vec![otype::Int, otype::Index(1)],
        },
    );
    methods.insert(
        "GetSectionPropertyType".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetSectionPropertyValues".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
            ],
            outputs: vec![
                otype::Int,
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
        "GetSectionPropertyValuesEx".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::MutInt, itype::MutVecDouble],
            outputs: vec![otype::Int, otype::Index(1), otype::Index(2)],
        },
    );
    methods.insert(
        "GetSectionTableNo".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetShapeCode".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::Str],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetThicknessPropertyAssignedPlateCount".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetThicknessPropertyAssignedPlateList".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::MutVecInt],
            outputs: vec![otype::Bool, otype::Index(1)],
        },
    );
    methods.insert(
        "GetThicknessPropertyCount".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetThicknessPropertyList".to_string(),
        MethodSignature {
            inputs: vec![itype::MutVecInt],
            outputs: vec![otype::Bool, otype::Index(0)],
        },
    );
    methods.insert(
        "GetThicknessPropertyValues".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::MutVecDouble],
            outputs: vec![otype::Int, otype::Index(1)],
        },
    );
    methods.insert(
        "RemoveMaterialFromPlate".to_string(),
        MethodSignature {
            inputs: vec![itype::VecInt],
            outputs: vec![otype::Bool],
        },
    );
    methods.insert(
        "RemoveMaterialFromSolid".to_string(),
        MethodSignature {
            inputs: vec![itype::VecInt],
            outputs: vec![otype::Bool],
        },
    );
    methods.insert(
        "RemovePropertyFromPlate".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Bool],
        },
    );

    // Property::Section: Get and Remove UPT Profile
    methods.insert(
        "CreatePropertyFromUserTable".to_string(),
        MethodSignature {
            inputs: vec![itype::Str, itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateUPTTableEx".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "FindUPTTable".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetUserProvidedTableCount".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetUserProvidedTableList".to_string(),
        MethodSignature {
            inputs: vec![itype::MutVecInt],
            outputs: vec![otype::Bool, otype::Index(0)],
        },
    );
    methods.insert(
        "GetUserProvidedTableNo".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetUserProvidedTableSectionCount".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetUserProvidedTableSectionList".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::MutVecStr],
            outputs: vec![otype::Int, otype::Index(1)],
        },
    );
    methods.insert(
        "GetUserProvidedTableSectionProperties".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::Str, itype::MutInt, itype::MutVecDouble],
            outputs: vec![otype::Bool, otype::Index(2), otype::Index(3)],
        },
    );
    methods.insert(
        "GetUserProvidedTableSectionPropertyCount".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::Str],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetUserProvidedTableSectionType".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::MutInt],
            outputs: vec![otype::Int, otype::Index(1)],
        },
    );
    methods.insert(
        "RemovePropertyFromUPTTable".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::Str],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "RemoveUPTTable".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Bool],
        },
    );

    // Property::Specification: Create Specification
    methods.insert(
        "AddControlDependentRelation".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::Int,
                itype::Int,
                itype::Int,
                itype::Int,
                itype::Int,
                itype::Int,
                itype::Int,
                itype::VecInt,
            ],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateElementIgnoreInplaneRotnSpec".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateElementLocalZOffsetSpec".to_string(),
        MethodSignature {
            inputs: vec![itype::Double, itype::Double, itype::Double, itype::Double],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateElementNodeReleaseSpec".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::VecInt],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateElementOffsetSpec".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::Int,
                itype::Double,
                itype::Double,
                itype::Double,
            ],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateElementPlaneStressSpec".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateMemberCableSpec".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::Double],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateMemberCableSpecEx".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::Double,
                itype::Int,
                itype::Double,
                itype::Double,
                itype::Double,
            ],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateMemberCompressionSpec".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateMemberFireProofingSpec".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::Double, itype::Double],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateMemberIgnoreStiffSpec".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateMemberInactiveSpec".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateMemberOffsetSpec".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::Int,
                itype::Double,
                itype::Double,
                itype::Double,
            ],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateMemberPartialReleaseSpec".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::VecInt, itype::VecDouble],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateMemberReleaseSpec".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::VecInt, itype::VecDouble],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateMemberTensionSpec".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateMemberTrussSpec".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Int],
        },
    );

    // Property::Specification: Get and Remove Specification
    methods.insert(
        "DeleteAllControlDependentRelations".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "DeleteMemberReleaseSpec".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::Int],
            outputs: vec![otype::Bool],
        },
    );
    methods.insert(
        "DeleteMemberSpec".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Bool],
        },
    );
    methods.insert(
        "DeleteProperty".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Bool],
        },
    );
    methods.insert(
        "GetAlphaAngleForSection".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::MutDouble],
            outputs: vec![otype::Int, otype::Index(1)],
        },
    );
    methods.insert(
        "GetCentroidLocationForSection".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::MutDouble, itype::MutDouble],
            outputs: vec![otype::Int, otype::Index(1), otype::Index(2)],
        },
    );
    methods.insert(
        "GetElementOffsetSpecCount".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetFireProofDataForBeam".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::MutInt,
                itype::MutDouble,
                itype::MutDouble,
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
        "GetFireProofedBeamCount".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetFireProofedBeamList".to_string(),
        MethodSignature {
            inputs: vec![itype::MutVecInt],
            outputs: vec![otype::Int, otype::Index(0)],
        },
    );
    methods.insert(
        "GetFireProofingSpecAssignedBeamCount".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetFireProofingSpecAssignedBeamList".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::MutVecInt],
            outputs: vec![otype::Int, otype::Index(1)],
        },
    );
    methods.insert(
        "GetFireProofingSpecCount".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetFireProofingSpecDetails".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::MutInt,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutInt,
            ],
            outputs: vec![
                otype::Int,
                otype::Index(1),
                otype::Index(2),
                otype::Index(3),
                otype::Index(4),
            ],
        },
    );
    methods.insert(
        "GetInactiveMemberCount".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetInactiveMemberList".to_string(),
        MethodSignature {
            inputs: vec![itype::MutVecInt],
            outputs: vec![otype::Index(0)],
        },
    );
    methods.insert(
        "GetMemberReleaseSpecEx".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::Int,
                itype::MutVecInt,
                itype::MutVecDouble,
                itype::MutDouble,
                itype::MutVecDouble,
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
        "GetMemberSpecCode".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::MutInt],
            outputs: vec![otype::Bool, otype::Index(1)],
        },
    );
    methods.insert(
        "GetPropertyUniqueID".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Str],
        },
    );
    methods.insert(
        "RemoveAllElementNodeReleaseSpec".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "RemoveAllElementOffsetSpec".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "RemoveBeamPropertyHelper".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Bool],
        },
    );
    methods.insert(
        "RemoveElementIgnoreInplaneRotnSpecFromPlate".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "RemoveElementNodeReleaseSpecFromPlate".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "RemoveElementPlaneStressSpecFromPlate".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "RemoveMemberCableSpecFromBeam".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::Int],
            outputs: vec![otype::Bool],
        },
    );
    methods.insert(
        "RemoveMemberCompressionSpecFromBeam".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "RemoveMemberFireProofingSpecFromBeam".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "RemoveMemberIgnoreStiffSpecFromBeam".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "RemoveMemberInactiveSpecFromBeam".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "RemoveMemberOffsetSpecFromBeam".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::Int],
            outputs: vec![otype::Bool],
        },
    );
    methods.insert(
        "RemoveMemberReleaseSpecFromBeam".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::Int],
            outputs: vec![otype::Bool],
        },
    );
    methods.insert(
        "RemoveMemberTensionSpecFromBeam".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "RemoveMemberTrussSpecFromBeam".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "RemovePropertyFromBeam".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "SetPropertyUniqueID".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::Str],
            outputs: vec![],
        },
    );

    // Property::Section: Assign Section to Element
    methods.insert(
        "AssignBeamProperty".to_string(),
        MethodSignature {
            inputs: vec![itype::VecInt, itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "AssignElementSpecToPlate".to_string(),
        MethodSignature {
            inputs: vec![itype::VecInt, itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "AssignMemberSpecToBeam".to_string(),
        MethodSignature {
            inputs: vec![itype::VecInt, itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "AssignPlateThickness".to_string(),
        MethodSignature {
            inputs: vec![itype::VecInt, itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateAssignProfileProperty".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "UpdatePropertiesToDesignSection".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Int],
        },
    );

    // Property::Material: Create Material Information
    methods.insert(
        "CreateIsotropicMaterialAluminum".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Str,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Bool,
            ],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateIsotropicMaterialConcrete".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Str,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Bool,
            ],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateIsotropicMaterialProperties".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Str,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
            ],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateIsotropicMaterialPropertiesEx".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Str,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
            ],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateIsotropicMaterialSteel".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Str,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Bool,
            ],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateIsotropicMaterialTimber".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Str,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Double,
                itype::Bool,
            ],
            outputs: vec![otype::Int],
        },
    );

    // Property::Material: Get and Remove Material
    methods.insert(
        "DeleteMaterial".to_string(),
        MethodSignature {
            inputs: vec![itype::Str],
            outputs: vec![otype::Bool],
        },
    );
    methods.insert(
        "GetBeamMaterialName".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Str],
        },
    );
    methods.insert(
        "GetElementMaterialName".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Str],
        },
    );
    methods.insert(
        "GetIsotropicMaterialAssignedBeamCount".to_string(),
        MethodSignature {
            inputs: vec![itype::Str],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetIsotropicMaterialAssignedBeamList".to_string(),
        MethodSignature {
            inputs: vec![itype::Str, itype::MutVecInt],
            outputs: vec![otype::Int, otype::Index(1)],
        },
    );
    methods.insert(
        "GetIsotropicMaterialAssignedPlateCount".to_string(),
        MethodSignature {
            inputs: vec![itype::Str],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetIsotropicMaterialAssignedSolidCount".to_string(),
        MethodSignature {
            inputs: vec![itype::Str],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetIsotropicMaterialAssignedSolidList".to_string(),
        MethodSignature {
            inputs: vec![itype::Str, itype::MutVecInt],
            outputs: vec![otype::Bool, otype::Index(1)],
        },
    );
    methods.insert(
        "GetIsotropicMaterialCount".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetIsotropicMaterialProperties".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
            ],
            outputs: vec![
                otype::Str,
                otype::Index(1),
                otype::Index(2),
                otype::Index(3),
                otype::Index(4),
                otype::Index(5),
                otype::Index(6),
            ],
        },
    );
    methods.insert(
        "GetIsotropicMaterialPropertiesAssigned".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutInt,
            ],
            outputs: vec![
                otype::Str,
                otype::Index(1),
                otype::Index(2),
                otype::Index(3),
                otype::Index(4),
                otype::Index(5),
                otype::Index(6),
                otype::Index(7),
            ],
        },
    );
    methods.insert(
        "GetIsotropicMaterialPropertiesEx".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
            ],
            outputs: vec![
                otype::Str,
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
                otype::Index(11),
            ],
        },
    );
    methods.insert(
        "GetMaterialProperty".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Str,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
            ],
            outputs: vec![
                otype::Int,
                otype::Index(1),
                otype::Index(2),
                otype::Index(3),
                otype::Index(4),
                otype::Index(5),
            ],
        },
    );
    methods.insert(
        "GetMaterialPropertyEx".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Str,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
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
        "GetOrthotropic2DMaterialCount".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetOrthotropic2DMaterialProperties".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::MutVecDouble,
                itype::MutVecDouble,
                itype::MutVecDouble,
                itype::MutVecDouble,
                itype::MutVecDouble,
                itype::MutVecDouble,
            ],
            outputs: vec![
                otype::Str,
                otype::Index(1),
                otype::Index(2),
                otype::Index(3),
                otype::Index(4),
                otype::Index(5),
                otype::Index(6),
            ],
        },
    );
    methods.insert(
        "GetOrthotropic3DMaterialCount".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetOrthotropic3DMaterialProperties".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::MutVecDouble,
                itype::MutVecDouble,
                itype::MutVecDouble,
                itype::MutVecDouble,
                itype::MutVecDouble,
                itype::MutVecDouble,
            ],
            outputs: vec![
                otype::Str,
                otype::Index(1),
                otype::Index(2),
                otype::Index(3),
                otype::Index(4),
                otype::Index(5),
                otype::Index(6),
            ],
        },
    );
    methods.insert(
        "GetPlateMaterialName".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Str],
        },
    );
    methods.insert(
        "GetSolidMaterialName".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Str],
        },
    );
    methods.insert(
        "GetTypeForIsotropicMaterial".to_string(),
        MethodSignature {
            inputs: vec![itype::Str, itype::MutInt],
            outputs: vec![otype::Bool, otype::Index(1)],
        },
    );
    methods.insert(
        "RemoveBeamMaterialHelper".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Bool],
        },
    );
    methods.insert(
        "RemoveMaterialFromBeam".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "SetTypeToIsotropicMaterial".to_string(),
        MethodSignature {
            inputs: vec![itype::Str, itype::Int],
            outputs: vec![otype::Bool],
        },
    );

    // Property::Material: Assign Material to Section and Element
    methods.insert(
        "AssignMaterialToMember".to_string(),
        MethodSignature {
            inputs: vec![itype::Str, itype::VecInt],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "AssignMaterialToPlate".to_string(),
        MethodSignature {
            inputs: vec![itype::Str, itype::VecInt],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "AssignMaterialToSolid".to_string(),
        MethodSignature {
            inputs: vec![itype::Str, itype::VecInt],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "SetMaterialID".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![],
        },
    );
    methods.insert(
        "SetMaterialName".to_string(),
        MethodSignature {
            inputs: vec![itype::Str],
            outputs: vec![],
        },
    );

    // Property::Attribute: Create Attribute
    methods.insert(
        "CreateElementAttribute".to_string(),
        MethodSignature {
            inputs: vec![itype::Str, itype::Str],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "CreateMemberAttribute".to_string(),
        MethodSignature {
            inputs: vec![itype::Str, itype::Str],
            outputs: vec![otype::Int],
        },
    );

    // Property::Attribute: Get and Remove Attribute
    methods.insert(
        "DeleteElementAttribute".to_string(),
        MethodSignature {
            inputs: vec![itype::Str, itype::Str],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "DeleteMemberAttribute".to_string(),
        MethodSignature {
            inputs: vec![itype::Str, itype::Str],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetAssignedAttributeByIndex".to_string(),
        MethodSignature {
            inputs: vec![itype::Int, itype::MutStr, itype::MutStr],
            outputs: vec![otype::Int, otype::Index(1), otype::Index(2)],
        },
    );
    methods.insert(
        "GetAssignedAttributeCount".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetElementCountByAttribute".to_string(),
        MethodSignature {
            inputs: vec![itype::Str, itype::Str],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetElementListByAttribute".to_string(),
        MethodSignature {
            inputs: vec![itype::Str, itype::Str, itype::MutVecInt],
            outputs: vec![otype::Int, otype::Index(2)],
        },
    );
    methods.insert(
        "GetMemberAttributeCount".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetMemberAttributeList".to_string(),
        MethodSignature {
            inputs: vec![itype::MutVecStr, itype::MutVecStr],
            outputs: vec![otype::Int, otype::Index(0), otype::Index(1)],
        },
    );
    methods.insert(
        "GetMemberCountByAttribute".to_string(),
        MethodSignature {
            inputs: vec![itype::Str, itype::Str],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "GetMemberListByAttribute".to_string(),
        MethodSignature {
            inputs: vec![itype::Str, itype::Str, itype::MutVecInt],
            outputs: vec![otype::Int, otype::Index(2)],
        },
    );
    methods.insert(
        "RemoveAttribute".to_string(),
        MethodSignature {
            inputs: vec![itype::Str, itype::Str, itype::VecInt],
            outputs: vec![otype::Int],
        },
    );

    // Property::Attribute: Assign Attribute to Element
    methods.insert(
        "AssignElementAttribute".to_string(),
        MethodSignature {
            inputs: vec![itype::Str, itype::Str, itype::VecInt],
            outputs: vec![otype::Int],
        },
    );
    methods.insert(
        "AssignMemberAttribute".to_string(),
        MethodSignature {
            inputs: vec![itype::Str, itype::Str, itype::VecInt],
            outputs: vec![otype::Int],
        },
    );

    // Property::ElementProperty: Functions Get Property information through Element
    methods.insert(
        "GetBeamConstants".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
            ],
            outputs: vec![
                otype::Bool,
                otype::Index(1),
                otype::Index(2),
                otype::Index(3),
                otype::Index(4),
                otype::Index(5),
            ],
        },
    );
    methods.insert(
        "GetBeamProperty".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
            ],
            outputs: vec![
                otype::Int,
                otype::Index(1),
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
        "GetBeamPropertyAll".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
            ],
            outputs: vec![
                otype::Int,
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
        "GetElementGlobalOffset".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::Int,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
            ],
            outputs: vec![
                otype::Int,
                otype::Index(2),
                otype::Index(3),
                otype::Index(4),
            ],
        },
    );
    methods.insert(
        "GetElementLocalOffset".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::Int,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
            ],
            outputs: vec![
                otype::Int,
                otype::Index(2),
                otype::Index(3),
                otype::Index(4),
            ],
        },
    );
    methods.insert(
        "GetElementOffsetSpec".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::Int,
                itype::MutInt,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
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
        "GetMemberGlobalOffSet".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::Int,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
            ],
            outputs: vec![
                otype::Int,
                otype::Index(2),
                otype::Index(3),
                otype::Index(4),
            ],
        },
    );
    methods.insert(
        "GetMemberLocalOffSet".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::Int,
                itype::MutDouble,
                itype::MutDouble,
                itype::MutDouble,
            ],
            outputs: vec![
                otype::Int,
                otype::Index(2),
                otype::Index(3),
                otype::Index(4),
            ],
        },
    );
    methods.insert(
        "GetMemberReleaseSpec".to_string(),
        MethodSignature {
            inputs: vec![
                itype::Int,
                itype::Int,
                itype::MutVecInt,
                itype::MutVecDouble,
            ],
            outputs: vec![otype::Int, otype::Index(2), otype::Index(3)],
        },
    );
    methods.insert(
        "GetPlateSectionPropertyRefNo".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Int],
        },
    );

    // Property::Publish: Functions Related to Publish Information
    methods.insert(
        "GetPublishedProfileName".to_string(),
        MethodSignature {
            inputs: vec![itype::Str, itype::Int],
            outputs: vec![otype::Str],
        },
    );
    methods.insert(
        "GetSTAADProfileName".to_string(),
        MethodSignature {
            inputs: vec![itype::Str, itype::Int],
            outputs: vec![otype::Str],
        },
    );

    // Property::Standard Section: Functions related to standard section
    methods.insert(
        "GetDefaultStandardProfileDBFolder".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Str],
        },
    );
    methods.insert(
        "GetStandardProfileDBFolder".to_string(),
        MethodSignature {
            inputs: vec![],
            outputs: vec![otype::Str],
        },
    );
    methods.insert(
        "GetStandardSectionDatabaseName".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Str],
        },
    );
    methods.insert(
        "GetStandardSectionName".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Str],
        },
    );
    methods.insert(
        "GetStandardSectionTableName".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Str],
        },
    );
    methods.insert(
        "IsStandardDatabaseSection".to_string(),
        MethodSignature {
            inputs: vec![itype::Int],
            outputs: vec![otype::Bool],
        },
    );
    methods.insert(
        "SetStandardProfileDBFolder".to_string(),
        MethodSignature {
            inputs: vec![itype::Str],
            outputs: vec![otype::Int],
        },
    );
}
