use serde::Serialize;
use std::collections::HashMap;
use windows::Win32::System::Com::IDispatch;

use crate::tools::value_types::{InType as itype, MethodSignature, OutType as otype};
use std::fmt::{Debug, Formatter, Result};

#[derive(Clone, Serialize)]
pub struct Geometry {
    pub id: String,
    #[serde(skip)]
    pub dispatch: IDispatch,
    #[serde(skip)]
    pub methods: HashMap<String, MethodSignature>,
}

impl Debug for Geometry {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Geometry Struct")
    }
}

impl Geometry {
    pub fn new(dispatch: IDispatch) -> Self {
        let mut methods = HashMap::new();

        // Node Methods (Alphabetical)
        methods.insert(
            "AddMultipleNodes".to_string(),
            MethodSignature {
                inputs: vec![itype::Vec2dDouble],
                outputs: vec![],
            },
        );
        methods.insert(
            "AddNode".to_string(),
            MethodSignature {
                inputs: vec![itype::Double, itype::Double, itype::Double],
                outputs: vec![],
            },
        );
        methods.insert(
            "CreateMultipleNodes".to_string(),
            MethodSignature {
                inputs: vec![itype::VecInt, itype::Vec2dDouble],
                outputs: vec![],
            },
        );
        methods.insert(
            "CreateNode".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Double, itype::Double, itype::Double],
                outputs: vec![],
            },
        );
        methods.insert(
            "DeleteNode".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![],
            },
        );
        methods.insert(
            "GetLastNodeNo".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetNodeCoordinates".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::MutDouble,
                    itype::MutDouble,
                    itype::MutDouble,
                ],
                outputs: vec![otype::Index(1), otype::Index(2), otype::Index(3)],
            },
        );
        methods.insert(
            "GetNodeCount".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetNodeDistance".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int],
                outputs: vec![otype::Double],
            },
        );
        methods.insert(
            "GetNodeIncidence".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::MutDouble,
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
            "GetNodeIncidence_CIS2".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::Int,
                    itype::MutStr,
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
                ],
            },
        );
        methods.insert(
            "GetNodeList".to_string(),
            MethodSignature {
                inputs: vec![itype::MutVecInt],
                outputs: vec![otype::Index(0)],
            },
        );
        methods.insert(
            "GetNodeNumber".to_string(),
            MethodSignature {
                inputs: vec![itype::Double, itype::Double, itype::Double],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetNodeUniqueId".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Str],
            },
        );
        methods.insert(
            "IsOrphanNode".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Bool],
            },
        );
        methods.insert(
            "SetNodeCoordinate".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Double, itype::Double, itype::Double],
                outputs: vec![],
            },
        );
        methods.insert(
            "SetNodeUniqueId".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Str],
                outputs: vec![],
            },
        );

        // Beam Methods (Alphabetical)
        methods.insert(
            "AddBeam".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "AddMultipleBeams".to_string(),
            MethodSignature {
                inputs: vec![itype::Vec2dInt],
                outputs: vec![],
            },
        );
        methods.insert(
            "BreakBeamsAtSpecificNodes".to_string(),
            MethodSignature {
                inputs: vec![itype::VecInt, itype::MutVecInt, itype::MutVecInt],
                outputs: vec![otype::Int, otype::Index(1), otype::Index(2)],
            },
        );
        methods.insert(
            "CreateBeam".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int, itype::Int],
                outputs: vec![],
            },
        );
        methods.insert(
            "CreateMultipleBeams".to_string(),
            MethodSignature {
                inputs: vec![itype::VecInt, itype::Vec2dInt],
                outputs: vec![],
            },
        );
        methods.insert(
            "DeleteBeam".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![],
            },
        );
        methods.insert(
            "GetBeamLength".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Double],
            },
        );
        methods.insert(
            "GetBeamList".to_string(),
            MethodSignature {
                inputs: vec![itype::MutVecInt],
                outputs: vec![otype::Index(0)],
            },
        );
        methods.insert(
            "GetBeamsConnectedAtNode".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::MutVecInt],
                outputs: vec![otype::Int, otype::Index(1)],
            },
        );
        methods.insert(
            "GetCountOfBreakableBeamsAtSpecificNodes".to_string(),
            MethodSignature {
                inputs: vec![itype::VecInt],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetIntersectBeamsCount".to_string(),
            MethodSignature {
                inputs: vec![itype::VecInt, itype::Double],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetLastBeamNo".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetMemberCount".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetMemberIncidence".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::MutInt, itype::MutInt],
                outputs: vec![otype::Int, otype::Index(1), otype::Index(2)],
            },
        );
        methods.insert(
            "GetMemberIncidence_CIS2".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::MutStr, itype::MutInt, itype::MutInt],
                outputs: vec![
                    otype::Int,
                    otype::Index(1),
                    otype::Index(2),
                    otype::Index(3),
                ],
            },
        );
        methods.insert(
            "GetMemberUniqueID".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Str],
            },
        );
        methods.insert(
            "GetNoOfBeamsConnectedAtNode".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "IntersectBeams".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::VecInt, itype::Double, itype::MutVecInt],
                outputs: vec![otype::Int, otype::Index(3)],
            },
        );
        methods.insert(
            "IsBeam".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Double],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "IsColumn".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Double],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "IsZUp".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "MergeBeams".to_string(),
            MethodSignature {
                inputs: vec![
                    itype::VecInt,
                    itype::Int,
                    itype::Int,
                    itype::Double,
                    itype::Str,
                ],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "RenumberBeam".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "SetCheckForIdenticalEntity".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "SetMemberUniqueID".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Str],
                outputs: vec![],
            },
        );
        methods.insert(
            "SplitBeam".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int, itype::VecDouble],
                outputs: vec![],
            },
        );
        methods.insert(
            "SplitBeamInEqlParts".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Int],
                outputs: vec![],
            },
        );

        // Group Methods (Alphabetical)
        methods.insert(
            "CreateGroupEx".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::Str, itype::Int, itype::VecInt],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "DeleteGroup".to_string(),
            MethodSignature {
                inputs: vec![itype::Str],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetGroupCount".to_string(),
            MethodSignature {
                inputs: vec![itype::Int],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetGroupCountAll".to_string(),
            MethodSignature {
                inputs: vec![],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetGroupEntities".to_string(),
            MethodSignature {
                inputs: vec![itype::Str, itype::MutVecInt],
                outputs: vec![otype::Int, otype::Index(1)],
            },
        );
        methods.insert(
            "GetGroupEntityCount".to_string(),
            MethodSignature {
                inputs: vec![itype::Str],
                outputs: vec![otype::Int],
            },
        );
        methods.insert(
            "GetGroupNames".to_string(),
            MethodSignature {
                inputs: vec![itype::Int, itype::MutVecStr],
                outputs: vec![otype::Int, otype::Index(1)],
            },
        );
        methods.insert(
            "UpdateGroup".to_string(),
            MethodSignature {
                inputs: vec![itype::Str, itype::Int, itype::Int, itype::VecInt],
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
