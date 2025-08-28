use serde::Serialize;
use std::collections::HashMap;
use windows::Win32::System::Com::IDispatch;

use crate::tools::value_types::{InType as itype, MethodSignature, OutType as otype};

#[derive(Debug, Serialize)]
pub struct Load {
    pub id: String,
    #[serde(skip)]
    pub dispatch: IDispatch,
    #[serde(skip)]
    pub methods: HashMap<String, MethodSignature>,
}

impl Load {
    pub fn new(dispatch: IDispatch) -> Self {
        let mut methods = HashMap::new();

        // Load
        methods.insert(
            "AddRSLoad".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::VecDouble,
                    itype::Int,
                    itype::Double,
                    itype::Int,
                    itype::Double,
                    itype::Int,
                    itype::Int,
                    itype::Double,
                    itype::Int,
                    itype::Double,
                    itype::Int,
                    itype::Double,
                    itype::Int,
                    itype::Double,
                    itype::Int,
                    itype::Int,
                    itype::VecDouble,
                    itype::VecDouble,
                ],
                outputs: vec![otype::Bool],
            },
        );
        methods.insert(
            "SplitLoadsOnBeam".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int],
                outputs: vec![otype::Bool],
            },
        );

        // Load Definition: Wind Methods (Alphabetical)
        methods.insert(
            "AddWindDefinition".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Str],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "AddWindDefinitionASCE7Parameters".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::Int,
                    itype::Double,
                    itype::Double,
                    itype::Int,
                    itype::Int,
                    itype::Int,
                    itype::Int,
                    itype::Int,
                    itype::Int,
                    itype::VecDouble,
                    itype::VecDouble,
                    itype::VecInt,
                    itype::VecInt,
                    itype::VecDouble,
                ],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "AddWindExposure".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Double, itype::VecInt],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "AddWindIntensity".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::VecDouble, itype::VecDouble],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "ComputeWallWindPressureProfile".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::Double,
                    itype::Int,
                    itype::Int,
                    itype::Int,
                    itype::Bool,
                    itype::VecInt,
                    itype::VecDouble,
                    itype::VecDouble,
                    itype::Int,
                ],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "ComputeWallWindPressureProfileASCE72016".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Double,
                    itype::Double,
                    itype::Int,
                    itype::Int,
                    itype::Int,
                    itype::Bool,
                    itype::VecInt,
                    itype::VecDouble,
                    itype::VecDouble,
                    itype::Int,
                ],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "DeleteWindDefinition".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );

        // Load Definition: Seismic Methods (Alphabetical)
        methods.insert(
            "AddResponseSpectrumLoadEx".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::Int,
                    itype::VecStr,
                    itype::VecDouble,
                    itype::OptionVecStr,
                    itype::OptionVecDouble,
                    itype::VecDouble,
                ],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "AddSeismicDefinition".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int],
                outputs: vec![otype::Bool],
            },
        );
        methods.insert(
            "AddSeismicDefJointWeight".to_string(),
            MethodSignature {
                inputs: vec![itype::Double, itype::VecInt],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "AddSeismicDefMemberWeight".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::Int,
                    itype::Double,
                    itype::Double,
                    itype::Double,
                    itype::VecInt,
                ],
                outputs: vec![otype::Bool],
            },
        );
        methods.insert(
            "AddSeismicDefSelfWeight".to_string(),
            MethodSignature {
                inputs: vec![itype::Double],
                outputs: vec![otype::Bool],
            },
        );
        methods.insert(
            "AddSeismicDefWallArea".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Str, itype::VecDouble],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "ModifySeismicDefinitionParams".to_string(),
            MethodSignature {
                inputs: vec![itype::Str, itype::Double],
                outputs: vec![otype::Int],
            },
        );

        // Load Definition: Reference Load Methods (Alphabetical)
        methods.insert(
            "CreateNewReferenceLoad".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Str, itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetReferenceLoadCaseCount".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetReferenceLoadCaseNumbers".to_string(),
            MethodSignature {
                inputs: vec![itype::MutVecInt],
                outputs: vec![otype::Int, otype::Index(0)],
            },
        );
        methods.insert(
            "SetReferenceLoadActive".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );

        // Load Definition: Direct Analysis Methods (Alphabetical)
        methods.insert(
            "AddDirectAnalysisDefinitionParameter".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::VecInt, itype::Double],
                outputs: vec![otype::Bool],
            },
        );
        methods.insert(
            "DeleteDirectAnalysisDefinition".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![otype::Bool],
            },
        );
        methods.insert(
            "DeleteDirectAnalysisDefinitionParameter".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Bool],
            },
        );

        // Load Case Details
        methods.insert(
            "GetPrimaryLoadCaseNumbers".to_string(),
            MethodSignature {
                inputs: vec![itype::MutVecInt],
                outputs: vec![otype::Int, otype::Index(0)],
            },
        );
        methods.insert(
            "RemoveAttribute".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );
        // Load Case Details::Load Items: SelfWeight Load Methods (Alphabetical)
        methods.insert(
            "AddSelfWeightInXYZ".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Double],
                outputs: vec![otype::Bool],
            },
        );
        methods.insert(
            "AddSelfWeightInXYZToGeometry".to_string(),
            MethodSignature {
                inputs: vec![itype::VecInt, itype::Int, itype::Double],
                outputs: vec![otype::Bool],
            },
        );

        // Load Case Details::Load Items: Nodal Load Methods (Alphabetical)
        methods.insert(
            "AddNodalLoad".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::VecInt,
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
            "AddSupportDisplacement".to_string(),
            MethodSignature {
                inputs: vec![itype::VecInt, itype::Int, itype::Double],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetNodalLoadCount".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetNodalLoadInfo".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::MutVecDouble],
                outputs: vec![otype::Bool, otype::Index(1)],
            },
        );
        methods.insert(
            "GetNodalLoads".to_string(),
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
                    otype::Int,
                    otype::Index(1),
                    otype::Index(2),
                    otype::Index(3),
                    otype::Index(4),
                    otype::Index(5),
                    otype::Index(6),
                ],
            },
        );

        // Load Case Details::Load Items: Member Load Methods (Alphabetical)
        methods.insert(
            "AddMemberAreaLoad".to_string(),
            MethodSignature {
                inputs: vec![itype::VecInt, itype::Double],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "AddMemberConcForce".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::VecInt,
                    itype::Int,
                    itype::Double,
                    itype::Double,
                    itype::Double,
                ],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "AddMemberConcMoment".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::VecInt,
                    itype::Int,
                    itype::Double,
                    itype::Double,
                    itype::Double,
                ],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "AddMemberFixedEnd".to_string(),
            MethodSignature {
                inputs: vec![itype::VecInt, itype::VecDouble, itype::VecDouble],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "AddMemberLinearVari".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::VecInt,
                    itype::Int,
                    itype::Double,
                    itype::Double,
                    itype::Double,
                ],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "AddMemberTrapezoidal".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::VecInt,
                    itype::Int,
                    itype::Double,
                    itype::Double,
                    itype::Double,
                    itype::Double,
                ],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "AddMemberUniformForce".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::VecInt,
                    itype::Int,
                    itype::Double,
                    itype::Double,
                    itype::Double,
                    itype::Double,
                ],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "AddMemberUniformMoment".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::VecInt,
                    itype::Int,
                    itype::Double,
                    itype::Double,
                    itype::Double,
                    itype::Double,
                ],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetConcForceCount".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetConcForces".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::MutVecInt,
                    itype::MutVecDouble,
                    itype::MutVecDouble,
                    itype::MutVecDouble,
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
            "GetConcMomentCount".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetConcMoments".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::MutVecInt,
                    itype::MutVecDouble,
                    itype::MutVecDouble,
                    itype::MutVecDouble,
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
            "GetLinearVaryingLoadCount".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetLinearVaryingLoads".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::MutVecInt,
                    itype::MutVecDouble,
                    itype::MutVecDouble,
                    itype::MutVecDouble,
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
            "GetMemberLoadInfo".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::MutInt,
                    itype::MutVecDouble,
                    itype::MutVecDouble,
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
            "GetTrapLoadCount".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetTrapLoads".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::MutVecInt,
                    itype::MutVecDouble,
                    itype::MutVecDouble,
                    itype::MutVecDouble,
                    itype::MutVecDouble,
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
            "GetUDLLoadCount".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetUDLLoads".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::MutVecInt,
                    itype::MutVecDouble,
                    itype::MutVecDouble,
                    itype::MutVecDouble,
                    itype::MutVecDouble,
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
            "GetUNIMomentCount".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetUNIMoments".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::MutVecInt,
                    itype::MutVecDouble,
                    itype::MutVecDouble,
                    itype::MutVecDouble,
                    itype::MutVecDouble,
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

        // Load Case Details::Load Items - Element Load Methods (Alphabetical)
        methods.insert(
            "AddElementHydrostaticPressure".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::VecInt,
                    itype::Int,
                    itype::Int,
                    itype::Double,
                    itype::Double,
                ],
                outputs: vec![otype::Bool],
            },
        );
        methods.insert(
            "AddElementPressure".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::VecInt,
                    itype::Int,
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
            "AddElementTrapPressureEx".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::VecInt,
                    itype::Int,
                    itype::Int,
                    itype::Double,
                    itype::Double,
                    itype::Double,
                    itype::Double,
                ],
                outputs: vec![otype::Bool],
            },
        );
        methods.insert(
            "GetElementConcLoadCount".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetElementConcLoads".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::MutVecInt,
                    itype::MutVecDouble,
                    itype::MutVecDouble,
                    itype::MutVecDouble,
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
            "GetElementLoadInfo".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::MutInt,
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
            "GetElementPressureLoadCount".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetElementPressureLoads".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::MutVecInt,
                    itype::MutVecDouble,
                    itype::MutVecDouble,
                    itype::MutVecDouble,
                    itype::MutVecDouble,
                    itype::MutVecDouble,
                ],
                outputs: vec![
                    otype::Int,
                    otype::Index(1),
                    otype::Index(2),
                    otype::Index(3),
                    otype::Index(4),
                    otype::Index(5),
                    otype::Index(6),
                ],
            },
        );

        // Load Case Details::Load Items - Floor Load Methods (Alphabetical)
        methods.insert(
            "AddMemberFloorLoad".to_string(),
            MethodSignature {
                inputs: vec![
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
            "AddMemberFloorLoadEx".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::Int,
                    itype::Double,
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
            "GetBeamCountAtFloor".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Double,
                    itype::Double,
                    itype::Double,
                    itype::Double,
                    itype::Double,
                    itype::Double,
                    itype::Int,
                ],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetInfluenceArea".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Double,
                    itype::Double,
                    itype::Double,
                    itype::Double,
                    itype::Double,
                    itype::Double,
                    itype::Int,
                    itype::MutVecInt,
                    itype::MutVecDouble,
                ],
                outputs: vec![otype::Bool, otype::Index(7), otype::Index(8)],
            },
        );

        // Load Case Details::Load Items - Temperature Load Methods (Alphabetical)
        methods.insert(
            "AddStrainLoad".to_string(),
            MethodSignature {
                inputs: vec![itype::VecInt, itype::Double],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "AddTemperatureLoad".to_string(),
            MethodSignature {
                inputs: vec![itype::VecInt, itype::Double, itype::Double, itype::Double],
                outputs: vec![otype::Int],
            },
        );

        // Load Case Details::Load Items - Seismic Load Methods (Alphabetical)
        methods.insert(
            "AddSeismicLoad".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Double],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "IsDynamicLoadIncluded".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );

        // Load Case Details::Load Items - Wind and Snow Load Methods (Alphabetical)
        methods.insert(
            "AddWindLoad".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::Int,
                    itype::Double,
                    itype::Bool,
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

        // Load Case Details::Load Items - Repeat Load Methods (Alphabetical)
        methods.insert(
            "AddNotionalLoad".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::VecInt,
                    itype::VecDouble,
                    itype::VecInt,
                    itype::VecInt,
                    itype::VecDouble,
                    itype::VecInt,
                ],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "AddReferenceLoad".to_string(),
            MethodSignature {
                inputs: vec![itype::VecInt, itype::VecDouble],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "AddRepeatLoad".to_string(),
            MethodSignature {
                inputs: vec![itype::VecInt, itype::VecDouble],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "BeginLoadMerging".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![],
            },
        );
        methods.insert(
            "EndLoadMerging".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![],
            },
        );
        methods.insert(
            "GetNoLoadFactorDirectionInNotionalLoad".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetNoLoadFactorInRepeatLoad".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetNoOfSetsInReferenceLoad".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetNotionalLoadByIndex".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::MutVecInt,
                    itype::MutVecDouble,
                    itype::MutVecInt,
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
            "GetNotionalLoadCount".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetReferenceLoadByIndex".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::MutVecInt, itype::MutVecDouble],
                outputs: vec![otype::Int, otype::Index(1), otype::Index(2)],
            },
        );
        methods.insert(
            "GetReferenceLoadCaseTitle".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Str],
            },
        );
        methods.insert(
            "GetReferenceLoadCount".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetReferenceLoadType".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetRepeatLoadByIndex".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::MutVecInt, itype::MutVecDouble],
                outputs: vec![otype::Int, otype::Index(1), otype::Index(2)],
            },
        );
        methods.insert(
            "GetRepeatLoadCount".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![otype::Int],
            },
        );

        // Load Case Details::Load Combination Methods (Alphabetical)
        methods.insert(
            "AddAutoCombinationRepeat".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Str,
                    itype::Str,
                    itype::VecInt,
                    itype::MutInt,
                    itype::MutInt,
                    itype::Bool,
                    itype::Bool,
                    itype::Double,
                    itype::Bool,
                    itype::Int,
                    itype::Bool,
                    itype::Bool,
                    itype::Bool,
                    itype::Bool,
                ],
                outputs: vec![otype::Int, otype::Index(3), otype::Index(4)],
            },
        );
        methods.insert(
            "AddAutoLoadCombinations".to_string(),
            MethodSignature {
                inputs: vec![itype::Str, itype::Str, itype::VecInt, itype::MutInt],
                outputs: vec![otype::Int, otype::Index(3)],
            },
        );
        methods.insert(
            "AddLoadAndFactorToCombination".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int, itype::Double],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "CreateNewLoadCombination".to_string(),
            MethodSignature {
                inputs: vec![itype::Str, itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetLoadAndFactorForCombination".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::MutVecInt, itype::MutVecDouble],
                outputs: vec![otype::Bool, otype::Index(1), otype::Index(2)],
            },
        );
        methods.insert(
            "GetLoadCombinationCaseCount".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetLoadCombinationCaseNumbers".to_string(),
            MethodSignature {
                inputs: vec![itype::MutVecInt],
                outputs: vec![otype::Int, otype::Index(0)],
            },
        );
        methods.insert(
            "GetNoOfLoadAndFactorPairsForCombination".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "IsCombinationCase".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );

        // Load Case Operation Methods (Alphabetical)
        methods.insert(
            "ClearPrimaryLoadCase".to_string(),
            MethodSignature {
                inputs: vec![itype::VecInt, itype::Bool],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "ClearReferenceLoadCase".to_string(),
            MethodSignature {
                inputs: vec![itype::VecInt],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "CreateLoadList".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::VecInt],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "CreateNewPrimaryLoad".to_string(),
            MethodSignature {
                inputs: vec![itype::Str],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "CreateNewPrimaryLoadEx".to_string(),
            MethodSignature {
                inputs: vec![itype::Str, itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "CreateNewPrimaryLoadEx2".to_string(),
            MethodSignature {
                inputs: vec![itype::Str, itype::Int, itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "DeleteLoadList".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "DeletePrimaryLoadCases".to_string(),
            MethodSignature {
                inputs: vec![itype::VecInt, itype::Bool],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "DeleteReferenceLoadCases".to_string(),
            MethodSignature {
                inputs: vec![itype::VecInt],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetActiveLoad".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetAssignmentListForLoadType".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int, itype::MutVecInt],
                outputs: vec![otype::Int, otype::Index(2)],
            },
        );
        methods.insert(
            "GetAttribute".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetListSizeForLoadType".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetLoadCaseTitle".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Str],
            },
        );
        methods.insert(
            "GetLoadCountInLoadList".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetLoadItemsCount".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetLoadItemType".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetLoadListCount".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetLoadsInLoadList".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::MutVecInt],
                outputs: vec![otype::Bool, otype::Index(1)],
            },
        );
        methods.insert(
            "GetLoadType".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetLoadTypeCount".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetPrimaryLoadCaseCount".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "SetASDLoadAttribute".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int, itype::Bool],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "SetLoadType".to_string(),
            MethodSignature {
                inputs: vec![itype::VecInt, itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "SetLoadActive".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Bool],
            },
        );
        methods.insert(
            "SetLSDLoadAttribute".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );

        // Load Envelopes
        methods.insert(
            "AddLoadCasesToEnvelop".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::VecInt],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "CreateLoadEnvelop".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int, itype::VecInt],
                outputs: vec![otype::Bool],
            },
        );
        methods.insert(
            "DeleteLoadEnvelop".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetEnvelopeCount".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetEnvelopeIDs".to_string(),
            MethodSignature {
                inputs: vec![itype::MutVecInt],
                outputs: vec![otype::Int, otype::Index(0)],
            },
        );
        methods.insert(
            "GetLoadEnvelopeDetails".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::MutInt, itype::MutInt],
                outputs: vec![otype::Int, otype::Index(1), otype::Index(2)],
            },
        );
        methods.insert(
            "GetLoadListfromLoadEnvelope".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::MutVecInt],
                outputs: vec![otype::Int, otype::Index(1)],
            },
        );
        methods.insert(
            "RemoveLoadCasesFromEnvelop".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::VecInt],
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

unsafe impl Send for Load {}
unsafe impl Sync for Load {}
