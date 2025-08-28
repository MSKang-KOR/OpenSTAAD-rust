use serde::Serialize;
use std::collections::HashMap;
use windows::Win32::System::Com::IDispatch;

use crate::tools::value_types::{InType as itype, MethodSignature, OutType as otype};

#[derive(Debug, Serialize)]
pub struct Support {
    pub id: String,
    #[serde(skip)]
    pub dispatch: IDispatch,
    #[serde(skip)]
    pub methods: HashMap<String, MethodSignature>,
}

impl Support {
    pub fn new(dispatch: IDispatch) -> Self {
        let mut methods = HashMap::new();

        // Support Methods (Alphabetical)
        methods.insert(
            "AssignSupportToEntityList".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::VecInt, itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "AssignSupportToNode".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "CreateElasticFooting".to_string(),
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
            "CreateElasticMat".to_string(),
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
            "CreateInclinedSupport".to_string(),
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
                ],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "CreatePlateMat".to_string(),
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
            "CreateSupportFixed".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Str],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "CreateSupportFixedBut".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::Str,
                    itype::Int,
                    itype::Int,
                    itype::Int,
                    itype::Int,
                    itype::Int,
                    itype::Int,
                ],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "CreateSupportPinned".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Str],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "CreateSupportSpring".to_string(),
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
            "DeleteSupport".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetSupportName".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Str],
            },
        );
        methods.insert(
            "GetSupportUniqueID".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::MutStr],
                outputs: vec![otype::Int, otype::Index(1)],
            },
        );
        methods.insert(
            "RemoveSupportFromAllNodes".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "RemoveSupportFromEntityList".to_string(),
            MethodSignature {
                inputs: vec![itype::VecInt, itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "RemoveSupportFromNode".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );

        // Support Information Methods (Alphabetical)
        methods.insert(
            "GetCountOfElasticFooting".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetCountOfElasticMat".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetCountOfPlateMat".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetElasticFootingAssignmentList".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::MutVecInt],
                outputs: vec![otype::Int, otype::Index(1)],
            },
        );
        methods.insert(
            "GetElasticFootingDetail".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::MutDouble,
                    itype::MutDouble,
                    itype::MutInt,
                    itype::MutDouble,
                    itype::MutInt,
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
            "GetElasticMatAssignmentList".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::MutVecInt],
                outputs: vec![otype::Bool, otype::Index(1)],
            },
        );
        methods.insert(
            "GetElasticMatDetail".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::MutInt,
                    itype::MutDouble,
                    itype::MutInt,
                    itype::MutInt,
                    itype::MutInt,
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
            "GetPlateMatAssignmentList".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::MutVecInt],
                outputs: vec![otype::Bool, otype::Index(1)],
            },
        );
        methods.insert(
            "GetPlateMatDetail".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::MutInt,
                    itype::MutDouble,
                    itype::MutDouble,
                    itype::MutDouble,
                    itype::MutInt,
                    itype::MutInt,
                    itype::MutInt,
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
                ],
            },
        );
        methods.insert(
            "GetPlateMatSupportId".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetSupportCount".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetSupportInformation".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::MutVecInt, itype::MutVecDouble],
                outputs: vec![otype::Int, otype::Index(1), otype::Index(2)],
            },
        );
        methods.insert(
            "GetSupportInformationEx".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::Int,
                    itype::Int,
                    itype::MutVecInt,
                    itype::MutVecDouble,
                ],
                outputs: vec![otype::Int, otype::Index(3), otype::Index(4)],
            },
        );
        methods.insert(
            "GetSupportNodes".to_string(),
            MethodSignature {
                inputs: vec![itype::MutVecInt],
                outputs: vec![otype::Int, otype::Index(0)],
            },
        );
        methods.insert(
            "GetSupportType".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
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

unsafe impl Send for Support {}
unsafe impl Sync for Support {}
