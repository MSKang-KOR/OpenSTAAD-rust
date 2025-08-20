use crate::openstaad::{
    new_api::utils::{MethodSignature, ParamType as ptype, ResultType as rtype},
    tools::{
        com::{get_dispatch, invoke_method},
        safe_array::{safe_array_from_vec1d, safe_array_from_vec2d},
        variant::{SafeArray, SafeArrayP, variant_with_ptr_from},
    },
};

use anyhow::{Context, Error as anyErr, Ok as anyOk, Result, bail};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ffi::c_void};
use windows::Win32::System::{
    Com::{IDispatch, SAFEARRAY},
    Ole::{SafeArrayCreateVector, SafeArrayGetElement},
    Variant::{VARIANT, VT_BSTR, VT_I4, VariantToDouble, VariantToInt32, VariantToStringAlloc},
};
use windows_core::BSTR;

#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {
    pub id: String,
    #[serde(skip)]
    pub staad: Option<IDispatch>,
    #[serde(skip)]
    pub dispatch: Option<IDispatch>,
    #[serde(skip)]
    pub methods: HashMap<String, MethodSignature>,
}

impl Geometry {
    pub fn new(staad: Option<IDispatch>) -> Self {
        let _geometry =
            unsafe { get_dispatch(staad.as_ref().unwrap(), "Geometry", &mut []).unwrap() };
        let mut methods = HashMap::new();

        methods.insert(
            "AddMultipleNodes".to_string(),
            MethodSignature {
                arguments: vec![ptype::Vec2dF64],
                indice: vec![0],
                result: vec![],
                is_scaled: true,
            },
        );
        methods.insert(
            "AddNode".to_string(),
            MethodSignature {
                arguments: vec![ptype::F64, ptype::F64, ptype::F64],
                indice: vec![0, 1, 2],
                result: vec![],
                is_scaled: true,
            },
        );
        methods.insert(
            "GetNodeCoordinates".to_string(),
            MethodSignature {
                arguments: vec![ptype::I32, ptype::MutF64, ptype::MutF64, ptype::MutF64],
                indice: vec![0],
                result: vec![rtype::Index(1), rtype::Index(2), rtype::Index(3)],
                is_scaled: true,
            },
        );
        methods.insert(
            "GetNodeCount".to_string(),
            MethodSignature {
                arguments: vec![],
                indice: vec![],
                result: vec![rtype::Integer],
                is_scaled: true,
            },
        );
        methods.insert(
            "GetNodeList".to_string(),
            MethodSignature {
                arguments: vec![ptype::MutVecI32],
                indice: vec![],
                result: vec![rtype::Index(0)],
                is_scaled: true,
            },
        );
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            staad,
            dispatch: Some(_geometry),
            methods,
        }
    }
}

unsafe impl Send for Geometry {}
unsafe impl Sync for Geometry {}

// :: Node
// AddMultipleNodes
// AddNode
// CreateMultipleNodes
// CreateNode
// DeleteNode
// GetLastNodeNo
// GetNodeCoordinates
// GetNodeCount
// GetNodeDistance
// GetNodeIncidence
// GetNodeIncidence_CIS2
// GetNodeList
// GetNodeNumber
// GetNodeUniqueId
// IsOrphanNode
// SetNodeCoordinate
// SetNodeUniqueId
// :: Beam
// AddBeam
// AddMultipleBeams
// BreakBeamsAtSpecificNodes
// CreateBeam
// CreateMultipleBeams
// DeleteBeam
// GetBeamLength
// GetBeamList
// GetBeamsConnectedAtNode
// GetCountOfBreakableBeamsAtSpecificNodes
// GetIntersectBeamsCount
// GetLastBeamNo
// GetMemberCount
// GetMemberIncidence
// GetMemberIncidence_CIS2
// GetMemberUniqueID
// GetNoOfBeamsConnectedAtNode
// IntersectBeams
// IsBeam
// IsColumn
// IsZUp
// MergeBeams
// RenumberBeam
// SetCheckForIdenticalEntity
// SetMemberUniqueID
// SplitBeam
// SplitBeamInEqlParts
// :: Group
// CreateGroupEx
// DeleteGroup
// GetGroupCount
// GetGroupCountAll
// GetGroupEntities
// GetGroupEntityCount
// GetGroupNames
// UpdateGroup
