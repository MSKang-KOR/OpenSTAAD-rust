

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0626 */
/* at Tue Jan 19 12:14:07 2038
 */
/* Compiler settings for C:\Users\kms36\AppData\Local\Temp\IDLD52F.tmp:
    Oicf, W1, Zp8, env=Win32 (32b run), target_arch=X86 8.01.0626 
    protocol : dce , ms_ext, c_ext, robust
    error checks: allocation ref bounds_check enum stub_data 
    VC __declspec() decoration level: 
         __declspec(uuid()), __declspec(selectany), __declspec(novtable)
         DECLSPEC_UUID(), MIDL_INTERFACE()
*/
/* @@MIDL_FILE_HEADING(  ) */

#pragma warning( disable: 4049 )  /* more than 64k source lines */


/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 475
#endif

#include "rpc.h"
#include "rpcndr.h"

#ifndef __RPCNDR_H_VERSION__
#error this stub requires an updated version of <rpcndr.h>
#endif /* __RPCNDR_H_VERSION__ */


#ifndef __sadfasdf_h__
#define __sadfasdf_h__

#if defined(_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif

#ifndef DECLSPEC_XFGVIRT
#if _CONTROL_FLOW_GUARD_XFG
#define DECLSPEC_XFGVIRT(base, func) __declspec(xfg_virtual(base, func))
#else
#define DECLSPEC_XFGVIRT(base, func)
#endif
#endif

/* Forward Declarations */ 

#ifndef __IOSStringList_FWD_DEFINED__
#define __IOSStringList_FWD_DEFINED__
typedef interface IOSStringList IOSStringList;

#endif 	/* __IOSStringList_FWD_DEFINED__ */


#ifndef __IOSMemberSteelDgnResults_FWD_DEFINED__
#define __IOSMemberSteelDgnResults_FWD_DEFINED__
typedef interface IOSMemberSteelDgnResults IOSMemberSteelDgnResults;

#endif 	/* __IOSMemberSteelDgnResults_FWD_DEFINED__ */


#ifndef __IOSMemberSteelDgnParams_FWD_DEFINED__
#define __IOSMemberSteelDgnParams_FWD_DEFINED__
typedef interface IOSMemberSteelDgnParams IOSMemberSteelDgnParams;

#endif 	/* __IOSMemberSteelDgnParams_FWD_DEFINED__ */


#ifndef __IOSChineseSteelDgnResults_FWD_DEFINED__
#define __IOSChineseSteelDgnResults_FWD_DEFINED__
typedef interface IOSChineseSteelDgnResults IOSChineseSteelDgnResults;

#endif 	/* __IOSChineseSteelDgnResults_FWD_DEFINED__ */


#ifndef __IOSChineseSteelDgnParameters_FWD_DEFINED__
#define __IOSChineseSteelDgnParameters_FWD_DEFINED__
typedef interface IOSChineseSteelDgnParameters IOSChineseSteelDgnParameters;

#endif 	/* __IOSChineseSteelDgnParameters_FWD_DEFINED__ */


#ifndef __IOSChineseSteelCheckOption_FWD_DEFINED__
#define __IOSChineseSteelCheckOption_FWD_DEFINED__
typedef interface IOSChineseSteelCheckOption IOSChineseSteelCheckOption;

#endif 	/* __IOSChineseSteelCheckOption_FWD_DEFINED__ */


#ifndef __IOpenSTAADUI_FWD_DEFINED__
#define __IOpenSTAADUI_FWD_DEFINED__
typedef interface IOpenSTAADUI IOpenSTAADUI;

#endif 	/* __IOpenSTAADUI_FWD_DEFINED__ */


#ifndef __IOSGeometryUI_FWD_DEFINED__
#define __IOSGeometryUI_FWD_DEFINED__
typedef interface IOSGeometryUI IOSGeometryUI;

#endif 	/* __IOSGeometryUI_FWD_DEFINED__ */


#ifndef __IStaadProWindow_FWD_DEFINED__
#define __IStaadProWindow_FWD_DEFINED__
typedef interface IStaadProWindow IStaadProWindow;

#endif 	/* __IStaadProWindow_FWD_DEFINED__ */


#ifndef __IOSViewUI_FWD_DEFINED__
#define __IOSViewUI_FWD_DEFINED__
typedef interface IOSViewUI IOSViewUI;

#endif 	/* __IOSViewUI_FWD_DEFINED__ */


#ifndef __IOSOutputUI_FWD_DEFINED__
#define __IOSOutputUI_FWD_DEFINED__
typedef interface IOSOutputUI IOSOutputUI;

#endif 	/* __IOSOutputUI_FWD_DEFINED__ */


#ifndef __IOSPropertyUI_FWD_DEFINED__
#define __IOSPropertyUI_FWD_DEFINED__
typedef interface IOSPropertyUI IOSPropertyUI;

#endif 	/* __IOSPropertyUI_FWD_DEFINED__ */


#ifndef __IOSLoadUI_FWD_DEFINED__
#define __IOSLoadUI_FWD_DEFINED__
typedef interface IOSLoadUI IOSLoadUI;

#endif 	/* __IOSLoadUI_FWD_DEFINED__ */


#ifndef __IOSTableUI_FWD_DEFINED__
#define __IOSTableUI_FWD_DEFINED__
typedef interface IOSTableUI IOSTableUI;

#endif 	/* __IOSTableUI_FWD_DEFINED__ */


#ifndef __IOSSupportUI_FWD_DEFINED__
#define __IOSSupportUI_FWD_DEFINED__
typedef interface IOSSupportUI IOSSupportUI;

#endif 	/* __IOSSupportUI_FWD_DEFINED__ */


#ifndef __IOSCommandsUI_FWD_DEFINED__
#define __IOSCommandsUI_FWD_DEFINED__
typedef interface IOSCommandsUI IOSCommandsUI;

#endif 	/* __IOSCommandsUI_FWD_DEFINED__ */


#ifndef __IOSDesignUI_FWD_DEFINED__
#define __IOSDesignUI_FWD_DEFINED__
typedef interface IOSDesignUI IOSDesignUI;

#endif 	/* __IOSDesignUI_FWD_DEFINED__ */


#ifndef __OSStringList_FWD_DEFINED__
#define __OSStringList_FWD_DEFINED__

#ifdef __cplusplus
typedef class OSStringList OSStringList;
#else
typedef struct OSStringList OSStringList;
#endif /* __cplusplus */

#endif 	/* __OSStringList_FWD_DEFINED__ */


#ifndef __OSMemberSteelDgnResults_FWD_DEFINED__
#define __OSMemberSteelDgnResults_FWD_DEFINED__

#ifdef __cplusplus
typedef class OSMemberSteelDgnResults OSMemberSteelDgnResults;
#else
typedef struct OSMemberSteelDgnResults OSMemberSteelDgnResults;
#endif /* __cplusplus */

#endif 	/* __OSMemberSteelDgnResults_FWD_DEFINED__ */


#ifndef __OSMemberSteelDgnParams_FWD_DEFINED__
#define __OSMemberSteelDgnParams_FWD_DEFINED__

#ifdef __cplusplus
typedef class OSMemberSteelDgnParams OSMemberSteelDgnParams;
#else
typedef struct OSMemberSteelDgnParams OSMemberSteelDgnParams;
#endif /* __cplusplus */

#endif 	/* __OSMemberSteelDgnParams_FWD_DEFINED__ */


#ifndef __OSChineseSteelDgnResults_FWD_DEFINED__
#define __OSChineseSteelDgnResults_FWD_DEFINED__

#ifdef __cplusplus
typedef class OSChineseSteelDgnResults OSChineseSteelDgnResults;
#else
typedef struct OSChineseSteelDgnResults OSChineseSteelDgnResults;
#endif /* __cplusplus */

#endif 	/* __OSChineseSteelDgnResults_FWD_DEFINED__ */


#ifndef __OSChineseSteelDgnParameters_FWD_DEFINED__
#define __OSChineseSteelDgnParameters_FWD_DEFINED__

#ifdef __cplusplus
typedef class OSChineseSteelDgnParameters OSChineseSteelDgnParameters;
#else
typedef struct OSChineseSteelDgnParameters OSChineseSteelDgnParameters;
#endif /* __cplusplus */

#endif 	/* __OSChineseSteelDgnParameters_FWD_DEFINED__ */


#ifndef __OSChineseSteelCheckOption_FWD_DEFINED__
#define __OSChineseSteelCheckOption_FWD_DEFINED__

#ifdef __cplusplus
typedef class OSChineseSteelCheckOption OSChineseSteelCheckOption;
#else
typedef struct OSChineseSteelCheckOption OSChineseSteelCheckOption;
#endif /* __cplusplus */

#endif 	/* __OSChineseSteelCheckOption_FWD_DEFINED__ */


#ifndef __OpenSTAAD_FWD_DEFINED__
#define __OpenSTAAD_FWD_DEFINED__

#ifdef __cplusplus
typedef class OpenSTAAD OpenSTAAD;
#else
typedef struct OpenSTAAD OpenSTAAD;
#endif /* __cplusplus */

#endif 	/* __OpenSTAAD_FWD_DEFINED__ */


#ifndef __OSGeometryUI_FWD_DEFINED__
#define __OSGeometryUI_FWD_DEFINED__

#ifdef __cplusplus
typedef class OSGeometryUI OSGeometryUI;
#else
typedef struct OSGeometryUI OSGeometryUI;
#endif /* __cplusplus */

#endif 	/* __OSGeometryUI_FWD_DEFINED__ */


#ifndef __StaadProWindow_FWD_DEFINED__
#define __StaadProWindow_FWD_DEFINED__

#ifdef __cplusplus
typedef class StaadProWindow StaadProWindow;
#else
typedef struct StaadProWindow StaadProWindow;
#endif /* __cplusplus */

#endif 	/* __StaadProWindow_FWD_DEFINED__ */


#ifndef __OSViewUI_FWD_DEFINED__
#define __OSViewUI_FWD_DEFINED__

#ifdef __cplusplus
typedef class OSViewUI OSViewUI;
#else
typedef struct OSViewUI OSViewUI;
#endif /* __cplusplus */

#endif 	/* __OSViewUI_FWD_DEFINED__ */


#ifndef __OSOutputUI_FWD_DEFINED__
#define __OSOutputUI_FWD_DEFINED__

#ifdef __cplusplus
typedef class OSOutputUI OSOutputUI;
#else
typedef struct OSOutputUI OSOutputUI;
#endif /* __cplusplus */

#endif 	/* __OSOutputUI_FWD_DEFINED__ */


#ifndef __OSPropertyUI_FWD_DEFINED__
#define __OSPropertyUI_FWD_DEFINED__

#ifdef __cplusplus
typedef class OSPropertyUI OSPropertyUI;
#else
typedef struct OSPropertyUI OSPropertyUI;
#endif /* __cplusplus */

#endif 	/* __OSPropertyUI_FWD_DEFINED__ */


#ifndef __OSLoadUI_FWD_DEFINED__
#define __OSLoadUI_FWD_DEFINED__

#ifdef __cplusplus
typedef class OSLoadUI OSLoadUI;
#else
typedef struct OSLoadUI OSLoadUI;
#endif /* __cplusplus */

#endif 	/* __OSLoadUI_FWD_DEFINED__ */


#ifndef __OSTableUI_FWD_DEFINED__
#define __OSTableUI_FWD_DEFINED__

#ifdef __cplusplus
typedef class OSTableUI OSTableUI;
#else
typedef struct OSTableUI OSTableUI;
#endif /* __cplusplus */

#endif 	/* __OSTableUI_FWD_DEFINED__ */


#ifndef __OSSupportUI_FWD_DEFINED__
#define __OSSupportUI_FWD_DEFINED__

#ifdef __cplusplus
typedef class OSSupportUI OSSupportUI;
#else
typedef struct OSSupportUI OSSupportUI;
#endif /* __cplusplus */

#endif 	/* __OSSupportUI_FWD_DEFINED__ */


#ifndef __OSCommandsUI_FWD_DEFINED__
#define __OSCommandsUI_FWD_DEFINED__

#ifdef __cplusplus
typedef class OSCommandsUI OSCommandsUI;
#else
typedef struct OSCommandsUI OSCommandsUI;
#endif /* __cplusplus */

#endif 	/* __OSCommandsUI_FWD_DEFINED__ */


#ifndef __OSDesignUI_FWD_DEFINED__
#define __OSDesignUI_FWD_DEFINED__

#ifdef __cplusplus
typedef class OSDesignUI OSDesignUI;
#else
typedef struct OSDesignUI OSDesignUI;
#endif /* __cplusplus */

#endif 	/* __OSDesignUI_FWD_DEFINED__ */


#ifdef __cplusplus
extern "C"{
#endif 



#ifndef __OpenSTAADUI_LIBRARY_DEFINED__
#define __OpenSTAADUI_LIBRARY_DEFINED__

/* library OpenSTAADUI */
/* [custom][custom][custom][version][uuid] */ 


















typedef /* [public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0001
    {
        scNull	= 0,
        scComment	= 1,
        scMaxCMCore	= 2,
        scStaadType	= 10,
        scUnitCommand	= 20,
        scJobInfoStart	= 30,
        scJobInfoJob	= 31,
        scJobInfoClient	= 32,
        scJobInfoJobNo	= 33,
        scJobInfoPart	= 34,
        scJobInfoRef	= 35,
        scJobInfoComment	= 36,
        scJobInfoEName	= 37,
        scJobInfoCName	= 38,
        scJobInfoAName	= 39,
        scJobInfoEDate	= 40,
        scJobInfoCDate	= 41,
        scJobInfoADate	= 42,
        scJobInfoRev	= 43,
        scJobCONNECTProjID	= 44,
        scJobCONNECTProjName	= 45,
        scJobInfoEnd	= 50,
        scInputWidth	= 60,
        scOutputWidth	= 61,
        scRestore	= 65,
        scSetDiv	= 67,
        scSetNl	= 68,
        scSetCo	= 69,
        scSetEchoOn	= 70,
        scSetEchoOff	= 71,
        scSetDisplacement	= 72,
        scSetZUp	= 73,
        scSetCore	= 74,
        scSetExmemory	= 75,
        scSetSolver	= 76,
        scSetTable	= 77,
        scSetCompress	= 78,
        scSetNj	= 79,
        scSetNm	= 80,
        scSetMass	= 81,
        scSetModal	= 82,
        scSetThistory	= 83,
        scSetInterpolationLin	= 84,
        scSetInterpolationLog	= 85,
        scSetDisplacementMethod	= 86,
        scSetFileExtension	= 87,
        scSetBubble	= 88,
        scSetNoSect	= 89,
        scSetTmh	= 90,
        scSetSdamp	= 91,
        scSetWarp	= 92,
        scSetIterlim	= 93,
        scSetGui	= 94,
        scSetShear	= 95,
        scSetSolutionIncore	= 96,
        scSetEndfactor	= 97,
        scSetSectionDBFolder	= 98,
        scSetNoWarn	= 99,
        scSetFloorLoadTolerance	= 100,
        scBypass	= 101,
        scSetSaveElementStiffness	= 102,
        scSetWarpingTol	= 103,
        scSetRitz	= 104,
        scSetLanczos	= 105,
        scSetFloorAngleTolerance	= 106,
        scSetParallelOpt	= 107,
        scSetMeshTolerance	= 108,
        scSetPrint	= 109,
        scSeparator	= 120,
        scPageNew	= 121,
        scPageLength	= 122,
        scPageEject	= 123,
        scIgnoreList	= 124,
        scPrecision	= 130,
        scInputNodesign	= 140,
        scInputMemory	= 141,
        scSurfaceDivision	= 142,
        scMiscLastNo	= 190,
        scJointCoordinate	= 200,
        scMemberIncidence	= 210,
        scElementIncidenceShell	= 220,
        scElementIncidenceSolid	= 230,
        scSurfaceIncidence	= 240,
        scPMemberDefine	= 250,
        scPerformRotation	= 300,
        scInactiveMember	= 310,
        scDeleteMember	= 320,
        scDeleteJoint	= 321,
        scParametricMeshModel	= 331,
        scParametricMeshModelNodeData	= 332,
        scParametricMeshModelPlateData	= 333,
        scEndParametricMeshModelInfo	= 334,
        scStartGroupDefinition	= 350,
        scGroupJoint	= 351,
        scGroupMember	= 352,
        scGroupElement	= 353,
        scGroupSolid	= 354,
        scGroupGeometry	= 355,
        scGroupFloor	= 356,
        scGroupList	= 358,
        scEndGroup	= 370,
        scIgnoreStiffnessMember	= 380,
        scStartUserTable	= 400,
        scUserTableNo	= 401,
        scUserTableUnit	= 402,
        scUserTableWideFlangeTitle	= 410,
        scUserTableWideFlangeName	= 411,
        scUserTableWideFlange	= 412,
        scUserTableChannelTitle	= 420,
        scUserTableChannelName	= 421,
        scUserTableChannel	= 422,
        scUserTableAngleTitle	= 430,
        scUserTableAngleName	= 431,
        scUserTableAngle	= 432,
        scUserTableDoubleAngleTitle	= 440,
        scUserTableDoubleAngleName	= 441,
        scUserTableDoubleAngle	= 442,
        scUserTableTeeTitle	= 450,
        scUserTableTeeName	= 451,
        scUserTableTee	= 452,
        scUserTablePipeTitle	= 460,
        scUserTablePipeName	= 461,
        scUserTablePipe	= 462,
        scUserTableTubeTitle	= 470,
        scUserTableTubeName	= 471,
        scUserTableTube	= 472,
        scUserTableGeneralTitle	= 480,
        scUserTableGeneralName	= 481,
        scUserTableGeneral	= 482,
        scUserTableIsectionTitle	= 490,
        scUserTableIsectionName	= 491,
        scUserTableIsection	= 492,
        scUserTablePrismaticTitle	= 500,
        scUserTablePrismaticName	= 501,
        scUserTablePrismatic	= 502,
        scUserTableStressLocation	= 510,
        scUserTablePofilePoints	= 511,
        scEndUserTable	= 550,
        scPMemberProperty	= 590,
        scMemberProperty	= 600,
        scTableBeamSt	= 610,
        scTableBeamT	= 611,
        scTableBeamTee	= 61100,
        scTableBeamCm	= 612,
        scTableBeamTc	= 613,
        scTableBeamBc	= 614,
        scTableBeamTb	= 615,
        scTableBeamD	= 616,
        scTableTeeSt	= 620,
        scTableChannelSt	= 630,
        scTableChannelD	= 631,
        scTableChannelBa	= 632,
        scTableChannelFr	= 633,
        scTableChannelStCold	= 634,
        scTableChannelStLips	= 635,
        scTableAngleSt	= 640,
        scTableAngleRa	= 641,
        scTableAngleLd	= 642,
        scTableAngleSd	= 643,
        scTableAngleStCold	= 644,
        scTableAngleStLips	= 645,
        scTableAngleSa	= 646,
        scTableTubeSt	= 650,
        scTableTubeDefine	= 651,
        scTableHSSRectangle	= 654,
        scTableHSSRound	= 655,
        scTableBeamCastelSt	= 656,
        scTableBulbFlatSt	= 657,
        scTableSteelJoistSt	= 658,
        scTableAITCTimber	= 659,
        scTablePipeSt	= 660,
        scTablePipeDefine	= 661,
        scTableZee	= 662,
        scTableZeeLips	= 663,
        scTableHat	= 664,
        scTableBeamCastelStComp	= 665,
        scTablePlateStrip	= 666,
        scTableSolidRect	= 667,
        scTableSolidRound	= 668,
        scTableRod	= 669,
        scTableCable	= 670,
        scPrismaticCircle	= 671,
        scPrismaticRectangle	= 672,
        scPrismaticTee	= 673,
        scPrismaticTrapezoidal	= 674,
        scTaperedTube	= 675,
        scPrismaticGeneral	= 676,
        scTimberRectangle	= 677,
        scTapered	= 680,
        scUptWideFlange	= 690,
        scUptChannel	= 691,
        scUptAngle	= 692,
        scUptDoubleAngle	= 693,
        scUptTee	= 694,
        scUptPipe	= 695,
        scUptTube	= 696,
        scUptGeneral	= 697,
        scUptIsection	= 698,
        scUptPrismatic	= 699,
        scUptAngleLd	= 700,
        scUptAngleSd	= 701,
        scUptChannelBa	= 702,
        scUptChannelFr	= 703,
        scTableChannelStColdLp	= 711,
        scTableChannelStColdBa	= 712,
        scTableChannelStColdFr	= 713,
        scTableChannelStColdLipsLp	= 714,
        scTableChannelStColdLipsBa	= 715,
        scTableChannelStColdLipsFr	= 716,
        scTableZeeLp	= 717,
        scTableZeeLipsLp	= 718,
        scAssignBeam	= 725,
        scAssignColumn	= 726,
        scAssignChannel	= 727,
        scAssignAngle	= 728,
        scAssignDoubleAngle	= 729,
        scMemberPropertyLastNo	= 750,
        scElementProperty	= 770,
        scElementPropertyThickness	= 771,
        scElementPropertyLastNo	= 780,
        scSurfaceProperty	= 781,
        scSurfacePropertyThickness	= 782,
        scSurfacePropertyLastNo	= 799,
        scMemberRelease	= 800,
        scMemberReleaseStart	= 801,
        scMemberReleaseEnd	= 802,
        scMemberReleaseBoth	= 803,
        scElementRelease	= 810,
        scElementReleaseSpec	= 811,
        scMemberTension	= 820,
        scElementIgnoreInplaneRotation	= 830,
        scElementRigidInplaneRotation	= 831,
        scMemberTruss	= 840,
        scMemberTrussTension	= 841,
        scMemberJoist	= 842,
        scMemberCompression	= 850,
        scMemberCable	= 860,
        scMemberCableTension	= 861,
        scMemberCableLength	= 862,
        scElementPlaneStress	= 870,
        scElementOffset	= 871,
        scElementOffsetSpec	= 872,
        scMemberOffset	= 880,
        scMemberOffsetStart	= 881,
        scMemberOffsetEnd	= 882,
        scMemberFireProofing	= 884,
        scMemberFireProofingData	= 885,
        scSpringTension	= 890,
        scSpringTensionData	= 891,
        scSpringCompression	= 892,
        scSpringCompressionData	= 893,
        scSpringDamping	= 894,
        scSpringDampingData	= 895,
        scMemberCurve	= 900,
        scMemberCurveData	= 901,
        scDefineImperfection	= 902,
        scCamberSpec	= 903,
        scDriftSpec	= 904,
        scMemberCracked	= 905,
        scMemberCrackedData	= 906,
        scMemberCrackedCodeSpecific	= 907,
        scMemberCrackedCodeSpecificData	= 908,
        scMemberSpecLastNo	= 910,
        scPMemberRelease	= 920,
        scPMemberReleaseStart	= 921,
        scPMemberReleaseEnd	= 922,
        scPMemberReleaseBoth	= 923,
        scPMemberOffset	= 925,
        scPMemberOffsetStart	= 926,
        scPMemberOffsetEnd	= 927,
        scPMemberTension	= 930,
        scPMemberTruss	= 931,
        scPMemberCompression	= 932,
        scPMemberSpecLastNo	= 935,
        scDefineMemberAttribute	= 940,
        scMemberAttribute	= 941,
        scEndDefineMemberAttribute	= 942,
        scDefineElementAttribute	= 943,
        scElementAttribute	= 944,
        scEndDefineElementAttribute	= 945,
        scDefineMaterialStart	= 960,
        scDefineMaterialIsotropic	= 961,
        scDefineMaterialIsotropicE	= 962,
        scDefineMaterialIsotropicPoisson	= 963,
        scDefineMaterialIsotropicDensity	= 964,
        scDefineMaterialIsotropicAlpha	= 965,
        scDefineMaterialIsotropicDamp	= 966,
        scDefineMaterialIsotropicCdamp	= 967,
        scDefineMaterialIsotropicG	= 968,
        scDefineMaterialIsotropicType	= 969,
        scDefineMaterialIsotropicDesignParam	= 970,
        scDefineMaterial2dorthotropic	= 971,
        scDefineMaterial2dorthotropicE	= 972,
        scDefineMaterial2dorthotropicPoisson	= 973,
        scDefineMaterial2dorthotropicDensity	= 974,
        scDefineMaterial2dorthotropicAlpha	= 975,
        scDefineMaterial2dorthotropicDamp	= 976,
        scDefineMaterial2dorthotropicCdamp	= 977,
        scDefineMaterial2dorthotropicG	= 978,
        scDefineMaterial3dorthotropic	= 981,
        scDefineMaterial3dorthotropicE	= 982,
        scDefineMaterial3dorthotropicPoisson	= 983,
        scDefineMaterial3dorthotropicDensity	= 984,
        scDefineMaterial3dorthotropicAlpha	= 985,
        scDefineMaterial3dorthotropicDamp	= 986,
        scDefineMaterial3dorthotropicCdamp	= 987,
        scDefineMaterial3dorthotropicG	= 988,
        scDefineMaterialEnd	= 990,
        scConstant	= 1000,
        scConstantE	= 1010,
        scConstantEBeam	= 1011,
        scConstantEPlate	= 1012,
        scConstantESolid	= 1013,
        scConstantPoisson	= 1020,
        scConstantPoissonBeam	= 1021,
        scConstantPoissonPlate	= 1022,
        scConstantPoissonSolid	= 1023,
        scConstantDensity	= 1030,
        scConstantDensityBeam	= 1031,
        scConstantDensityPlate	= 1032,
        scConstantDensitySolid	= 1033,
        scConstantBeta	= 1040,
        scConstantAlpha	= 1050,
        scConstantAlphaBeam	= 1051,
        scConstantAlphaPlate	= 1052,
        scConstantAlphaSolid	= 1053,
        scConstantCdamp	= 1060,
        scConstantCdampBeam	= 1061,
        scConstantCdampPlate	= 1062,
        scConstantCdampSolid	= 1063,
        scConstantReference	= 1070,
        scConstantReferenceVector	= 1071,
        scConstantReferenceJt	= 1080,
        scConstantMaterialName	= 1090,
        scConstantG	= 1091,
        scConstantLastNo	= 1150,
        scConstantSurface	= 1160,
        scConstantSurfaceE	= 1161,
        scConstantSurfaceE_Exp	= 1162,
        scConstantSurfacePoisson	= 1163,
        scConstantSurfacePoisson_Exp	= 1164,
        scConstantSurfaceDensity	= 1165,
        scConstantSurfaceDensity_Exp	= 1166,
        scConstantSurfaceAlpha	= 1167,
        scConstantSurfaceAlpha_Exp	= 1168,
        scConstantSurfaceCdamp	= 1169,
        scConstantSurfaceCdamp_Exp	= 1170,
        scConstantSurfaceReference	= 1171,
        scConstantSurfaceReferenceJt	= 1172,
        scConstantSurfaceMaterialName	= 1173,
        scConstantSurfaceG	= 1174,
        scConstantSurfaceLastNo	= 1180,
        scConstantPMember	= 1181,
        scConstantPMemberE	= 1182,
        scConstantPMemberE_Exp	= 1183,
        scConstantPMemberPoisson	= 1184,
        scConstantPMemberPoisson_Exp	= 1185,
        scConstantPMemberDensity	= 1186,
        scConstantPMemberDensity_Exp	= 1187,
        scConstantPMemberBeta	= 1188,
        scConstantPMemberAlpha	= 1189,
        scConstantPMemberAlpha_Exp	= 1190,
        scConstantPMemberCdamp	= 1191,
        scConstantPMemberCdamp_Exp	= 1192,
        scConstantPMemberReference	= 1193,
        scConstantPMemberReferenceVector	= 1194,
        scConstantPMemberReferenceJt	= 1195,
        scConstantPMemberMaterialName	= 1196,
        scConstantPMemberG	= 1197,
        scConstantPMemberLastNo	= 1199,
        scSupport	= 1200,
        scSupportPinned	= 1210,
        scSupportFixed	= 1220,
        scSupportFixedBut	= 1230,
        scSupportEnforced	= 1235,
        scSupportEnforcedBut	= 1236,
        scSupportInclined	= 1240,
        scSupportFooting	= 1250,
        scSupportElasticMat	= 1260,
        scSupportPlateMat	= 1261,
        scSupportMultilinearSpring	= 1270,
        scSupportMultilinearSpringSpec	= 1271,
        scSupportGenPinned	= 1272,
        scSupportGenFixed	= 1273,
        scSupportGenFixedBut	= 1274,
        scSupportLastNo	= 1350,
        scSlaveMaster	= 1400,
        scSlaveDiaphragmMaster	= 1410,
        scSlaveTieMaster	= 1420,
        scDrawPreLoad	= 1430,
        scCutOffFrequency	= 1440,
        scCutOffModeShape	= 1450,
        scCutOffTime	= 1460,
        scDefineMoving	= 1500,
        scDefineMovingTypeLoad	= 1510,
        scDefineMovingTypeDistance	= 1511,
        scDefineMovingTypeAashto	= 1520,
        scDefineMovingTypeFile	= 1530,
        scDefineMovingLastNo	= 1550,
        scDefineEnclosedZoneStart	= 1570,
        scDefineEnclosedZoneName	= 1571,
        scDefineEnclosedZoneBoundary	= 1572,
        scDefineEnclosedZoneIgnoreLoad	= 1573,
        scDefineEnclosedZoneIgnoreMember	= 1574,
        scDefineEnclosedZoneOpening	= 1575,
        scDefineEnclosedZoneEnd	= 1580,
        scDefine1893	= 1600,
        scDefineUbc	= 1610,
        scDefineUbcZone	= 1620,
        scDefine1893Zone	= 1621,
        scDefineUbcWallArea	= 1625,
        scDefineUbcSelfweight	= 1630,
        scDefineUbcFloorLoad	= 1631,
        scDefineUbcFloorXRange	= 1632,
        scDefineUbcFloorYRange	= 1633,
        scDefineUbcFloorZRange	= 1634,
        scDefineUbcFloorLoadGroup	= 1635,
        scDefineUbcEnclosedZoneWeight	= 1636,
        scDefineUbcEnclosedZoneWeightData	= 1637,
        scDefineUbcJointLoad	= 1640,
        scDefineUbcJointLoadWeight	= 1650,
        scDefineUbcMemberLoad	= 1660,
        scDefineUbcMemberLoadUni	= 1670,
        scDefineUbcMemberLoadCon	= 1680,
        scDefineUbcElementLoad	= 1681,
        scDefineUbcElementLoadPre	= 1682,
        scDefineUbcOneWayFloorLoad	= 1683,
        scDefineUbcOneWayFloorLoadYrange	= 1684,
        scDefineUbcOneWayFloorLoadZrange	= 1685,
        scDefineUbcOneWayFloorLoadXrange	= 1686,
        scDefineUbcOneWayFloorLoadGroup	= 1687,
        scDefineUbcLastNo	= 1690,
        scDefineWind	= 1700,
        scDefineWindType	= 1710,
        scDefineWindTypeParams	= 1711,
        scDefineWindTypeSNiPParams	= 1712,
        scDefineWindIntensity	= 1720,
        scDefineWindExposure	= 1730,
        scDefineWindExclude	= 1740,
        scDefineWindLastNo	= 1750,
        scDefineTimeHistory	= 1800,
        scDefineTimeHistoryType	= 1810,
        scDefineTimeHistoryRead	= 1811,
        scDefineTimeHistoryReadData	= 1812,
        scDefineTimeHistoryFunctionSine	= 1813,
        scDefineTimeHistoryFunctionCosine	= 1814,
        scDefineTimeHistoryAmplitude	= 1815,
        scDefineTimeHistorySpectrum	= 1816,
        scDefineTimeHistorySpectrumOption	= 1817,
        scDefineTimeHistorySpectrumReadData	= 1818,
        scDefineTimeHistoryArrival	= 1820,
        scDefineTimeHistoryArrivalData	= 1821,
        scDefineTimeHistoryDamp	= 1830,
        scDefineTimeHistoryCdamp	= 1840,
        scDefineTimeHistoryMdamp	= 1850,
        scDefineTimeHistoryLastNo	= 1860,
        scDefineAij	= 1900,
        scDefineAijZone	= 1910,
        scDefineAijSelfweight	= 1920,
        scDefineAijFloorLoad	= 1921,
        scDefineAijFloorXRange	= 1922,
        scDefineAijFloorYRange	= 1923,
        scDefineAijFloorZRange	= 1924,
        scDefineAijFloorLoadGroup	= 1925,
        scDefineAijJointLoad	= 1930,
        scDefineAijJointLoadWeight	= 1940,
        scDefineAijMemberLoad	= 1950,
        scDefineAijMemberLoadUni	= 1960,
        scDefineAijMemberLoadCon	= 1970,
        scDefineAijElementLoad	= 1971,
        scDefineAijElementLoadPre	= 1972,
        scDefineAijLastNo	= 1980,
        scDefineIbc	= 1981,
        scDefineIbcSds	= 1982,
        scDefineIbcSelfweight	= 1983,
        scDefineIbcJointLoad	= 1984,
        scDefineIbcJointLoadWeight	= 1985,
        scDefineIbcMemberLoad	= 1986,
        scDefineIbcMemberLoadUni	= 1987,
        scDefineIbcMemberLoadCon	= 1988,
        scDefineIbcFloorLoad	= 1989,
        scDefineIbcFloorXRange	= 1990,
        scDefineIbcFloorYRange	= 1991,
        scDefineIbcFloorZRange	= 1992,
        scDefineIbcFloorLoadGroup	= 1993,
        scDefineIbcElementLoad	= 1994,
        scDefineIbcElementLoadPre	= 1995,
        scDefineIbcLastNo	= 1996,
        scDefineReferenceLoad	= 1997,
        scDefineReferenceLoadData	= 1998,
        scDefineColombian	= 2000,
        scDefineColombianZone	= 2001,
        scDefineColombianSelfweight	= 2002,
        scDefineColombianFloorLoad	= 2003,
        scDefineColombianFloorXRange	= 2004,
        scDefineColombianFloorYRange	= 2005,
        scDefineColombianFloorZRange	= 2006,
        scDefineColombianFloorLoadGroup	= 2007,
        scDefineColombianJointLoad	= 2008,
        scDefineColombianJointLoadWeight	= 2009,
        scDefineColombianMemberLoad	= 2010,
        scDefineColombianMemberLoadUni	= 2011,
        scDefineColombianMemberLoadCon	= 2012,
        scDefineColombianElementLoad	= 2013,
        scDefineColombianElementLoadPre	= 2014,
        scDefineColombianLastNo	= 2020,
        scDefineCFE	= 2021,
        scDefineCFEZone	= 2022,
        scDefineCFESelfweight	= 2023,
        scDefineCFEFloorLoad	= 2024,
        scDefineCFEFloorXRange	= 2025,
        scDefineCFEFloorYRange	= 2026,
        scDefineCFEFloorZRange	= 2027,
        scDefineCFEFloorLoadGroup	= 2028,
        scDefineCFEJointLoad	= 2029,
        scDefineCFEJointLoadWeight	= 2030,
        scDefineCFEMemberLoad	= 2031,
        scDefineCFEMemberLoadUni	= 2032,
        scDefineCFEMemberLoadCon	= 2033,
        scDefineCFEElementLoad	= 2034,
        scDefineCFEElementLoadPre	= 2035,
        scDefineCFELastNo	= 2040,
        scDefineRPA	= 2041,
        scDefineRPAZone	= 2042,
        scDefineRPASelfweight	= 2043,
        scDefineRPAFloorLoad	= 2044,
        scDefineRPAFloorXRange	= 2045,
        scDefineRPAFloorYRange	= 2046,
        scDefineRPAFloorZRange	= 2047,
        scDefineRPAFloorLoadGroup	= 2048,
        scDefineRPAJointLoad	= 2049,
        scDefineRPAJointLoadWeight	= 2050,
        scDefineRPAMemberLoad	= 2051,
        scDefineRPAMemberLoadUni	= 2052,
        scDefineRPAMemberLoadCon	= 2053,
        scDefineRPAElementLoad	= 2054,
        scDefineRPAElementLoadPre	= 2055,
        scDefineRPALastNo	= 2060,
        scDefineNTC	= 2061,
        scDefineNTCZone	= 2062,
        scDefineNTCSelfweight	= 2063,
        scDefineNTCFloorLoad	= 2064,
        scDefineNTCFloorXRange	= 2065,
        scDefineNTCFloorYRange	= 2066,
        scDefineNTCFloorZRange	= 2067,
        scDefineNTCFloorLoadGroup	= 2068,
        scDefineNTCJointLoad	= 2069,
        scDefineNTCJointLoadWeight	= 2070,
        scDefineNTCMemberLoad	= 2071,
        scDefineNTCMemberLoadUni	= 2072,
        scDefineNTCMemberLoadCon	= 2073,
        scDefineNTCElementLoad	= 2074,
        scDefineNTCElementLoadPre	= 2075,
        scDefineNTCLastNo	= 2080,
        scDefineNRC	= 2081,
        scDefineNRCZone	= 2082,
        scDefineNRCSelfweight	= 2083,
        scDefineNRCFloorLoad	= 2084,
        scDefineNRCFloorXRange	= 2085,
        scDefineNRCFloorYRange	= 2086,
        scDefineNRCFloorZRange	= 2087,
        scDefineNRCFloorLoadGroup	= 2088,
        scDefineNRCJointLoad	= 2089,
        scDefineNRCJointLoadWeight	= 2090,
        scDefineNRCMemberLoad	= 2091,
        scDefineNRCMemberLoadUni	= 2092,
        scDefineNRCMemberLoadCon	= 2093,
        scDefineNRCElementLoad	= 2094,
        scDefineNRCElementLoadPre	= 2095,
        scDefineNRCLastNo	= 2099,
        scDefineDampingInformation	= 2100,
        scDefineDampingEvaluate	= 2110,
        scDefineDampingCalculate	= 2115,
        scDefineDampingExplicit	= 2120,
        scDefineDampingEnd	= 2130,
        scDefineDampingLastNo	= 2190,
        scDefineLoadTypes	= 2191,
        scDefineLoadTypesData	= 2192,
        scDefineLoadTypesList	= 2193,
        scDefineLoadTypesEnd	= 2194,
        scDefineSnow	= 2195,
        scDefineSnowType	= 2196,
        scDefineSnowLastNo	= 2197,
        scDefineNRC2005	= 2200,
        scDefineNRCZone2005	= 2201,
        scDefineNRCSelfweight2005	= 2202,
        scDefineNRCFloorLoad2005	= 2203,
        scDefineNRCFloorXRange2005	= 2204,
        scDefineNRCFloorYRange2005	= 2205,
        scDefineNRCFloorZRange2005	= 2206,
        scDefineNRCFloorLoadGroup2005	= 2207,
        scDefineNRCJointLoad2005	= 2208,
        scDefineNRCJointLoadWeight2005	= 2209,
        scDefineNRCMemberLoad2005	= 2210,
        scDefineNRCMemberLoadUni2005	= 2211,
        scDefineNRCMemberLoadCon2005	= 2212,
        scDefineNRCElementLoad2005	= 2213,
        scDefineNRCElementLoadPre2005	= 2214,
        scDefineNRCLastNo2005	= 2215,
        scDefineColombian2010	= 2220,
        scDefineColombian2010Aa	= 2221,
        scDefineColombian2010Selfweight	= 2222,
        scDefineColombian2010FloorLoad	= 2223,
        scDefineColombian2010FloorXRange	= 2224,
        scDefineColombian2010FloorYRange	= 2225,
        scDefineColombian2010FloorZRange	= 2226,
        scDefineColombian2010FloorLoadGroup	= 2227,
        scDefineColombian2010JointLoad	= 2228,
        scDefineColombian2010JointLoadWeight	= 2229,
        scDefineColombian2010MemberLoad	= 2230,
        scDefineColombian2010MemberLoadUni	= 2231,
        scDefineColombian2010MemberLoadCon	= 2232,
        scDefineColombian2010ElementLoad	= 2233,
        scDefineColombian2010ElementLoadPre	= 2234,
        scDefineColombian2010LastNo	= 2235,
        scDefineTurkish	= 2240,
        scDefineTurkishZone	= 2241,
        scDefineTurkishSelfweight	= 2242,
        scDefineTurkishFloorLoad	= 2243,
        scDefineTurkishFloorXRange	= 2244,
        scDefineTurkishFloorYRange	= 2245,
        scDefineTurkishFloorZRange	= 2246,
        scDefineTurkishFloorLoadGroup	= 2247,
        scDefineTurkishJointLoad	= 2248,
        scDefineTurkishJointLoadWeight	= 2249,
        scDefineTurkishMemberLoad	= 2250,
        scDefineTurkishMemberLoadUni	= 2251,
        scDefineTurkishMemberLoadCon	= 2252,
        scDefineTurkishElementLoad	= 2253,
        scDefineTurkishElementLoadPre	= 2254,
        scDefineTurkishLastNo	= 2255,
        scDefineGB50011	= 2260,
        scDefineGB50011Intensity	= 2261,
        scDefineGB50011Selfweight	= 2262,
        scDefineGB50011FloorLoad	= 2263,
        scDefineGB50011FloorXRange	= 2264,
        scDefineGB50011FloorYRange	= 2265,
        scDefineGB50011FloorZRange	= 2266,
        scDefineGB50011FloorLoadGroup	= 2267,
        scDefineGB50011JointLoad	= 2268,
        scDefineGB50011JointLoadWeight	= 2269,
        scDefineGB50011MemberLoad	= 2270,
        scDefineGB50011MemberLoadUni	= 2271,
        scDefineGB50011MemberLoadCon	= 2272,
        scDefineGB50011ElementLoad	= 2273,
        scDefineGB50011ElementLoadPre	= 2274,
        scDefineGB50011LastNo	= 2275,
        scDefineNRC2010	= 2280,
        scDefineNRCZone2010	= 2281,
        scDefineNRCSelfweight2010	= 2282,
        scDefineNRCFloorLoad2010	= 2283,
        scDefineNRCFloorXRange2010	= 2284,
        scDefineNRCFloorYRange2010	= 2285,
        scDefineNRCFloorZRange2010	= 2286,
        scDefineNRCFloorLoadGroup2010	= 2287,
        scDefineNRCJointLoad2010	= 2288,
        scDefineNRCJointLoadWeight2010	= 2289,
        scDefineNRCMemberLoad2010	= 2290,
        scDefineNRCMemberLoadUni2010	= 2291,
        scDefineNRCMemberLoadCon2010	= 2292,
        scDefineNRCElementLoad2010	= 2293,
        scDefineNRCElementLoadPre2010	= 2294,
        scDefineNRCLastNo2010	= 2295,
        scDefinePushover	= 2300,
        scDefinePushoverFrame	= 2301,
        scDefinePushoverLoadingPattern	= 2302,
        scDefinePushoverVerticalDistBaseShear	= 2303,
        scDefinePushoverTotalDistBaseShear	= 2304,
        scDefinePushoverLoadStep	= 2305,
        scDefinePushoverBaseShear	= 2306,
        scDefinePushoverDispAtControlJoint	= 2307,
        scDefinePushoverFYE	= 2321,
        scDefinePushoverHingePropMoment	= 2330,
        scDefinePushoverUserHingeType	= 2333,
        scDefinePushoverUserHingeTypeA	= 2334,
        scDefinePushoverUserHingeTypeB	= 2335,
        scDefinePushoverUserHingeTypeC	= 2336,
        scDefinePushoverUserHingeTypeD	= 2337,
        scDefinePushoverUserHingeTypeE	= 2338,
        scDefinePushoverUserHingeTypeYM	= 2339,
        scDefinePushoverUserHingeTypeYR	= 2340,
        scDefinePushoverUserHingeTypeIO	= 2341,
        scDefinePushoverUserHingeTypeLS	= 2342,
        scDefinePushoverUserHingeTypeCP	= 2343,
        scDefinePushoverHingeTypeUSER	= 2344,
        scDefinePushoverHingeTypeFEMA	= 2345,
        scDefinePushoverHingeTypeIGNORE	= 2346,
        scDefinePushoverGeomNonLinearEffect	= 2352,
        scDefinePushoverDispTolerance	= 2353,
        scDefinePushoverGeomCycles	= 2354,
        scDefinePushoverKGMatrixIter	= 2355,
        scDefinePushoverPrintResult	= 2356,
        scDefinePushoverPrintResultByChoice	= 2357,
        scDefinePushoverSaveLoadstepResult	= 2358,
        scDefinePushoverEffectiveMemLengthKY	= 2359,
        scDefinePushoverEffectiveMemLengthKZ	= 2360,
        scDefinePushoverMaxNumAnalysisCycles	= 2361,
        scDefinePushoverSpectrumParameters	= 2362,
        scDefinePushoverSpecParaDamp	= 2363,
        scDefinePushoverSpecParaSC	= 2364,
        scDefinePushoverSpecParaSS	= 2365,
        scDefinePushoverSpecParaS1	= 2366,
        scDefinePushoverParametersLastNo	= 2370,
        scDefinePushoverEndData	= 2371,
        scDefinePushLastNo	= 2375,
        scDefineReferenceLoadStart	= 2381,
        scDefineReferenceLoadEnd	= 2382,
        scFloorHeight	= 2385,
        scFloorHeightData	= 2386,
        scFloorDiaphragm	= 2387,
        scFloorDiaphragmData	= 2388,
        scBase	= 2389,
        scDefineStartingLoadStart	= 2403,
        scStartingLoadMass	= 2404,
        scStartingLoadRef	= 2405,
        scDefineStartingLoadEnd	= 2406,
        scDefineDA	= 2410,
        scDefineDACode	= 2411,
        scDefineDAFlex	= 2412,
        scDefineDAFyld	= 2413,
        scDefineDAAxial	= 2414,
        scDefineDANLFact	= 2415,
        scDefineDAEnd	= 2416,
        scDefineLastNo	= 2490,
        scStartAdvancedCommands	= 2500,
        scDefineNonlinearSpring	= 2510,
        scDefineNonlinearSpringType	= 2511,
        scDefineNonlinearSpringTypeData	= 2512,
        scDefineNonlinearSpringTypeEnd	= 2513,
        scDefineNonlinearSpringCommand	= 2520,
        scDefineNonlinearSpringEnd	= 2525,
        scStartAdvancedCommandsEnd	= 2700,
        scLoadCase	= 3000,
        scJointLoad	= 3100,
        scJointLoadSpec	= 3110,
        scJointInclinedLoadSpec	= 3120,
        scMemberLoad	= 3200,
        scMemberLoadUni	= 3210,
        scMemberLoadUniMoment	= 3220,
        scMemberLoadCon	= 3230,
        scMemberLoadConMoment	= 3240,
        scMemberLoadLinear	= 3250,
        scMemberLoadTrap	= 3260,
        scMemberLoadHydroStaic	= 3261,
        scPMemberLoad	= 3270,
        scPMemberLoadUni	= 3275,
        scPMemberLoadUniMoment	= 3280,
        scPMemberLoadCon	= 3285,
        scPMemberLoadConMoment	= 3290,
        scPMemberLoadTrap	= 3295,
        scElementLoad	= 3300,
        scElementLoadPressure	= 3310,
        scElementLoadJoint	= 3311,
        scElementLoadJtPressure	= 3312,
        scElementLoadTrap	= 3320,
        scElementLoadSolid	= 3321,
        scElementLoadSolidFacePressure	= 3322,
        scAreaLoad	= 3400,
        scAreaLoadSpec	= 3410,
        scFloorLoad	= 3500,
        scFloorLoadYrange	= 3510,
        scFloorLoadXrange	= 3511,
        scFloorLoadZrange	= 3520,
        scFloorLoadGroup	= 3530,
        scOneWayFloorLoad	= 3550,
        scOneWayFloorLoadXrange	= 3551,
        scOneWayFloorLoadYrange	= 3552,
        scOneWayFloorLoadZrange	= 3553,
        scOneWayFloorLoadGroup	= 3554,
        scEnclosedZoneLoad	= 3560,
        scEnclosedZoneLoadData	= 3561,
        scPrestressLoad	= 3600,
        scPoststressLoad	= 3610,
        scPrePoststressLoadSpec	= 3620,
        scTemperatureLoad	= 3700,
        scTemperatureLoadSpec	= 3710,
        scTemperatureLoadStrain	= 3720,
        scTemperatureLoadStrainRate	= 3721,
        scSurfaceLoad	= 3730,
        scSurfaceLoadPressure	= 3740,
        scSurfaceLoadTrap	= 3741,
        scSurfaceSelfWeight	= 3742,
        scFixedEndLoad	= 3800,
        scFixedEndLoadSpec	= 3810,
        scFxloadSpec	= 3811,
        scSupportDisplacementLoad	= 3900,
        scSupportDisplacementLoadSpec	= 3910,
        scSelfweightLoad	= 4000,
        scSpectrumSrss	= 4100,
        scSpectrumSrssData	= 4101,
        scSpectrumSrssFile	= 4102,
        scSpectrumCqc	= 4110,
        scSpectrumCqcData	= 4111,
        scSpectrumCqcFile	= 4112,
        scSpectrumAbsolute	= 4120,
        scSpectrumAbsoluteData	= 4121,
        scSpectrumAbsoluteFile	= 4122,
        scSpectrumAsce	= 4130,
        scSpectrumAsceData	= 4131,
        scSpectrumAsceFile	= 4132,
        scSpectrumTen	= 4140,
        scSpectrumTenData	= 4141,
        scSpectrumTenFile	= 4142,
        scSpectrumGrp	= 4143,
        scSpectrumGrpData	= 4144,
        scSpectrumGrpFile	= 4145,
        scSpectrumCsm	= 4150,
        scSpectrumCsmData	= 4151,
        scSpectrumCsmFile	= 4152,
        scSpectrumStartingLoad	= 4153,
        scRepeatLoad	= 4200,
        scRepeatLoadData	= 4201,
        scReferenceLoad	= 4220,
        scReferenceLoadData	= 4221,
        scNotionalLoad	= 4222,
        scNotionalLoadData	= 4223,
        scLoadGeneration	= 4300,
        scLoadGenerationType	= 4310,
        scUbcLoad	= 4400,
        scIbcLoad	= 4405,
        sc1893Load	= 4410,
        scAijLoad	= 4500,
        scColombianLoad	= 4510,
        scCFELoad	= 4520,
        scRPALoad	= 4530,
        scNTCLoad	= 4540,
        scNRCLoad	= 4550,
        scNRCLoad2005	= 4560,
        scNRCLoad2010	= 4561,
        scTurkishLoad	= 4570,
        scGB50011Load	= 4575,
        scColombian2010Load	= 4576,
        scWindLoad	= 4600,
        scWindLoadDynamic	= 4610,
        scSnowLoad	= 4650,
        scSnowLoadData	= 4651,
        scCalcNaturalFrequency	= 4700,
        scCalcRayleighFrequency	= 4701,
        scModalCalcRequested	= 4710,
        scModeSelect	= 4711,
        scTimeHistoryLoad	= 4800,
        scTimeHistoryLoadSpec	= 4810,
        scGroundMotion	= 4820,
        scLsdLoadAttribute	= 4821,
        scAsdLoadAttribute	= 4822,
        scLoadCommandLastNo	= 4950,
        scLoadCombination	= 5000,
        scLoadCombinationData	= 5001,
        scLoadLastNo	= 5500,
        scCheckSoftStory	= 5601,
        scCheckStoryDrift	= 5602,
        scCheckIrregularities	= 5603,
        scPrintProblemStatistics	= 6000,
        scPrintJointCoordinates	= 6010,
        scPrintMemberInformation	= 6020,
        scPrintElementInformation	= 6030,
        scPrintMemberProperties	= 6040,
        scPrintMaterialProperties	= 6050,
        scPrintSupportInformation	= 6060,
        scPrintAll	= 6070,
        scPrintCg	= 6080,
        scPrintElementInformationSolid	= 6090,
        scPrintPreAnalysisLastNo	= 6250,
        scDrawLoad	= 6300,
        scPerformAnalysis	= 6350,
        scPerformImperfectionAnalysis	= 6351,
        scPerformCableAnalysis	= 6352,
        scPerformSteadyStateAnalysis	= 6353,
        scPdeltaAnalysis	= 6360,
        scNonlinearAnalysis	= 6370,
        scPerformBucklingAnalysis	= 6371,
        scPerformDAAnalysis	= 6372,
        scPerformPushoverAnalysis	= 6380,
        scAnalysisCommandLastNo	= 6390,
        scStartAdvancedAnalysis	= 6400,
        scAdvancedAnalysisNonlinearTimeHistory	= 6410,
        scAdvancedAnalysisPrintTimeAccl	= 6510,
        scAdvancedAnalysisPrintTimeDisp	= 6511,
        scAdvancedAnalysisPrintTimeStress	= 6512,
        scAdvancedAnalysisEnd	= 6650,
        scChange	= 6690,
        scSave	= 6700,
        scSaveNeutralFile	= 6710,
        scLoadCombinationGeneration	= 6800,
        scLoadCombinationDead	= 6810,
        scLoadCombinationDeadData	= 6811,
        scLoadCombinationLive	= 6820,
        scLoadCombinationLiveData	= 6821,
        scAnalysisLastNo	= 6900,
        scGenerateFloorSpectrum	= 6911,
        scFloorSpectrumFloors	= 6912,
        scFloorSpectrumOptions	= 6913,
        scEndFloorSpectrum	= 6920,
        scFloorSpectrumLast	= 6920,
        scDefineEnvelop	= 6950,
        scEnvelop	= 6951,
        scEndDefineEnvelop	= 6952,
        scStartScript	= 7000,
        scScriptOpenFile	= 7010,
        scScriptForJoint	= 7020,
        scScriptForMember	= 7030,
        scScriptForSupport	= 7040,
        scScriptForLoad	= 7050,
        scScriptSortReaction	= 7060,
        scScriptSortDisplacement	= 7070,
        scScriptSortForce	= 7080,
        scScriptWriteHeader	= 7090,
        scScriptWriteCoord	= 7100,
        scScriptWriteMinc	= 7110,
        scScriptWriteMemb	= 7120,
        scScriptWriteProp	= 7130,
        scScriptWriteRelease	= 7140,
        scScriptWriteReact	= 7150,
        scScriptWriteDispl	= 7160,
        scScriptWriteForce	= 7170,
        scScriptWriteBmo	= 7180,
        scScriptWriteSdi	= 7190,
        scScriptWriteSuppInfo	= 7200,
        scScriptFormat	= 7210,
        scScriptClose	= 7220,
        scScriptExecute	= 7230,
        scScriptCall	= 7240,
        scEndScript	= 7400,
        scScriptingLanguageStart	= 7500,
        scScriptingLanguage	= 7501,
        scScriptingLanguageEnd	= 7502,
        scLoadList	= 7600,
        scSection	= 7700,
        scPrintJointDisplacement	= 8000,
        scPrintMemberForces	= 8010,
        scPrintSupportReactions	= 8020,
        scPrintAnalysisResults	= 8030,
        scPrintMemberSectionForces	= 8040,
        scPrintMemberStresses	= 8050,
        scPrintElementForces	= 8060,
        scPrintModeShapes	= 8070,
        scPrintBucklingShapes	= 8071,
        scPrintElementForceSolid	= 8080,
        scPrintEntireTable	= 8090,
        scPrintSectionDisplacement	= 8100,
        scPrintForce	= 8110,
        scPrintMaxForce	= 8120,
        scPrintStoryDrift	= 8130,
        scPrintDiaCr	= 8131,
        scPrintCabSag	= 8132,
        scPrintDuctility	= 8140,
        scPrintElementStress	= 8150,
        scPrintElementJointDisp	= 8151,
        scPrintElementStressSolid	= 8160,
        scPrintSurfaceForces	= 8161,
        scPrintStoryStiffness	= 8162,
        scPMemberPrintForces	= 8170,
        scPrintPostAnalysisLastNo	= 8350,
        scDrawPost	= 8400,
        scPlotDisplacementFile	= 8500,
        scPlotSectionFile	= 8510,
        scPlotBendingFile	= 8520,
        scPlotModeFile	= 8530,
        scPlotStressFile	= 8540,
        scPlotLastNo	= 8550,
        scSize	= 8900,
        scSteelParameter	= 9000,
        scSteelCode	= 9010,
        scSteelParameterFyld	= 9100,
        scSteelParameterSteelType	= 9101,
        scSteelParameterIsItSnugTypeBolted	= 9102,
        scSteelParameterConnectorSpacing	= 9103,
        scSteelParameterWstr	= 9110,
        scSteelParameterFsc	= 9111,
        scSteelParameterCrf	= 9112,
        scSteelParameterSc	= 9113,
        scSteelParameterRfr	= 9114,
        scSteelParameterRfs	= 9115,
        scSteelParameterCfdf	= 9116,
        scSteelParameterLb	= 9117,
        scSteelParameterLc	= 9118,
        scSteelParameterPy	= 9120,
        scSteelParameterPyRussian	= 9121,
        scSteelParameterLx	= 9125,
        scSteelParameterLy	= 9130,
        scSteelParameterSRT	= 9131,
        scSteelParameterTSL	= 9132,
        scSteelParameterE5P	= 9133,
        scSteelParameterH36	= 9134,
        scSteelParameterSTWO	= 9135,
        scSteelParameterFmain	= 9136,
        scSteelParameterBRX	= 9137,
        scSteelParameterBRY	= 9138,
        scSteelParameterLz	= 9140,
        scSteelParameterUnl	= 9150,
        scSteelParameterUnlRussian	= 9151,
        scSteelParameterDmax	= 9160,
        scSteelParameterDmin	= 9170,
        scSteelParameterWmin	= 9180,
        scSteelParameterLvv	= 9190,
        scSteelParameterStiff	= 9200,
        scSteelParameterStiffBs5400	= 9201,
        scSteelParameterStiffEC3	= 9202,
        scSteelParameterDff	= 9210,
        scSteelParameterDffRussian	= 9211,
        scSteelParameterDfh	= 9212,
        scSteelParameterMax	= 9220,
        scSteelParameterMin	= 9230,
        scSteelParameterKx	= 9235,
        scSteelParameterKy	= 9240,
        scSteelParameterKz	= 9250,
        scSteelParameterNsf	= 9260,
        scSteelParameterUnf	= 9270,
        scSteelParameterCb	= 9280,
        scSteelParameterCbRussianOrBS5950	= 9281,
        scSteelParameterSsy	= 9290,
        scSteelParameterSsyNs	= 9291,
        scSteelParameterSsz	= 9300,
        scSteelParameterSszNs	= 9301,
        scSteelParameterCmy	= 9310,
        scSteelParameterCmz	= 9320,
        scSteelParameterMain	= 9330,
        scSteelParameterMainBs5950	= 9331,
        scSteelParameterMainBs5400	= 9332,
        scSteelParameterMainRussian	= 9333,
        scSteelParameterMainIs800	= 9334,
        scSteelParameterTmainIs800	= 9335,
        scSteelParameterMainAIJ	= 9336,
        scSteelParameterTmainCSAS16	= 9337,
        scSteelParameterMainCSAS16	= 9338,
        scSteelParameterPunch	= 9340,
        scSteelParameterTrack	= 9350,
        scSteelParameterTrackNorway	= 9351,
        scSteelParameterTrackBs5400	= 9352,
        scSteelParameterTrackEc3	= 9353,
        scSteelParameterTrackDin18800	= 9354,
        scSteelParameterTrackFrench	= 9355,
        scSteelParameterTrackRussian	= 9356,
        scSteelParameterTrackBs5950	= 9357,
        scSteelParameterTrackJapanese	= 9358,
        scSteelParameterRatio	= 9360,
        scSteelParameterWeld	= 9370,
        scSteelParameterWeldBs5950	= 9371,
        scSteelParameterBeam	= 9380,
        scSteelParameterBeamBs5950	= 9381,
        scSteelParameterBeamEc3	= 9382,
        scSteelParameterBeamDin18800	= 9383,
        scSteelParameterBeamNorway	= 9384,
        scSteelParameterBeamFrench	= 9385,
        scSteelParameterDj1	= 9390,
        scSteelParameterDjy1	= 9391,
        scSteelParameterDjz1	= 9392,
        scSteelParameterDj2	= 9400,
        scSteelParameterDjy2	= 9401,
        scSteelParameterDjz2	= 9402,
        scSteelParameterCy	= 9410,
        scSteelParameterCz	= 9420,
        scSteelParameterBy	= 9430,
        scSteelParameterBz	= 9440,
        scSteelParameterMf	= 9450,
        scSteelParameterSgr	= 9460,
        scSteelParameterSgrEc3	= 9461,
        scSteelParameterSgrDin18800	= 9462,
        scSteelParameterSgrRussian	= 9463,
        scSteelParameterSgrBS5950	= 9464,
        scSteelParameterEN1993NA	= 9465,
        scSteelParameterSgrAS4100	= 9466,
        scSteelParameterSblt	= 9470,
        scSteelParameterSbltRussian	= 9471,
        scSteelParameterSbltBs5950EN1993	= 9472,
        scSteelParameterCmm	= 9480,
        scSteelParameterCmmBs5950	= 9481,
        scSteelParameterCmmEc3	= 9482,
        scSteelParameterCmmDin18800	= 9483,
        scSteelParameterCmmRussian	= 9484,
        scSteelParameterCmn	= 9490,
        scSteelParameterCmnBs5950	= 9491,
        scSteelParameterCmnEc3	= 9492,
        scSteelParameterCmnDin18800	= 9493,
        scSteelParameterCmnRussian	= 9494,
        scSteelParameterLeg	= 9500,
        scSteelParameterLegBs5950OrEC3	= 9501,
        scSteelParameterLegRussian	= 9502,
        scSteelParameterFSJApi	= 9503,
        scSteelParameterGm0EC3	= 9504,
        scSteelParameterGm1EC3	= 9505,
        scSteelParameterGm2EC3	= 9506,
        scSteelParameterZGEC3	= 9507,
        scSteelParameterFabEC3	= 9508,
        scSteelParameterLkpEC3	= 9509,
        scSteelParameterWet	= 9510,
        scSteelParameterProfile	= 9520,
        scSteelParameterTb	= 9530,
        scSteelParameterEst	= 9540,
        scSteelParameterC1	= 9550,
        scSteelParameterC2	= 9560,
        scSteelParameterC3	= 9565,
        scSteelParameterEta	= 9570,
        scSteelParameterGrade	= 9580,
        scSteelParameterCompression	= 9590,
        scSteelParameterCompressionSpanish	= 9591,
        scSteelParameterTension	= 9600,
        scSteelParameterTensionAlpha	= 9601,
        scSteelParameterPfy	= 9610,
        scSteelParameterPfz	= 9620,
        scSteelParameterSfy	= 9630,
        scSteelParameterSfz	= 9640,
        scSteelParameterSby	= 9641,
        scSteelParameterSbz	= 9642,
        scSteelParameterUnt	= 9650,
        scSteelParameterUnb	= 9660,
        scSteelParameterTorsion	= 9670,
        scSteelParameterCMT	= 9671,
        scSteelParameterTOM	= 9672,
        scSteelParameterEFT	= 9673,
        scSteelParameterALH	= 9674,
        scSteelParameterBET	= 9675,
        scSteelParameterGST	= 9676,
        scSteelParameterMthEC3	= 9677,
        scSteelParameterTmp	= 9680,
        scSteelParameterTorsionEC3	= 9688,
        scSteelParameterEstiff	= 9690,
        scSteelParameterMU	= 9695,
        scSteelParameterKc	= 9696,
        scSteelParameterElbEC3	= 9697,
        scSteelParameterPnl	= 9700,
        scSteelParameterFu	= 9705,
        scSteelParameterCmp	= 9710,
        scSteelParameterDia	= 9715,
        scSteelParameterHgt	= 9720,
        scSteelParameterCyc	= 9725,
        scSteelParameterDr1	= 9730,
        scSteelParameterDr2	= 9735,
        scSteelParameterWid	= 9740,
        scSteelParameterFpc	= 9745,
        scSteelParameterImp	= 9750,
        scSteelParameterPlt	= 9755,
        scSteelParameterPlw	= 9760,
        scSteelParameterRbh	= 9765,
        scSteelParameterRbw	= 9770,
        scSteelParameterShr	= 9775,
        scSteelParameterThk	= 9780,
        scSteelParameterFlx	= 9781,
        scSteelParameterTsa	= 9782,
        scSteelParameterCwy	= 9783,
        scSteelParameterCbCanadian	= 9785,
        scSteelParameterCmyCanadian	= 9790,
        scSteelParameterCmzCanadian	= 9795,
        scSteelParameterIst	= 9800,
        scSteelParameterPhi	= 9801,
        scSteelParameterNsc	= 9802,
        scSteelParameterAlm	= 9803,
        scSteelParameterAlb	= 9804,
        scSteelParameterKt	= 9805,
        scSteelParameterLt	= 9806,
        scSteelParameterSKt	= 9807,
        scSteelParameterSKl	= 9808,
        scSteelParameterSKr	= 9809,
        scSteelParameterGammaC1	= 9810,
        scSteelParameterGammaC2	= 9815,
        scSteelParameterGammaM	= 9816,
        scSteelParameterENMain	= 9817,
        scSteelParameterENSgr	= 9818,
        scSteelParameterMises	= 9819,
        scSteelParameterMLT	= 9820,
        scSteelParameterMLT_JL	= 9821,
        scSteelParameterPLB	= 9822,
        scSteelParameterMBG	= 9823,
        scSteelParameterYNG	= 9824,
        scSteelParameterMYX	= 9830,
        scSteelParameterMYX_JL	= 9831,
        scSteelParameterMX	= 9840,
        scSteelParameterMX_JL	= 9841,
        scSteelParameterMY	= 9850,
        scSteelParameterMY_JL	= 9851,
        scSteelParameterSway	= 9860,
        scSteelParameterEla	= 9861,
        scSteelParameterElb	= 9862,
        scSteelParameterDbl	= 9863,
        scSteelParameterFyb	= 9864,
        scSteelParameterFvb	= 9865,
        scSteelParameterNhl	= 9866,
        scSteelParameterMainAsce	= 9867,
        scSteelParameterTaper	= 9868,
        scSteelParameterSame	= 9870,
        scSteelParameterCnsf	= 9871,
        scSteelParameterDangle	= 9872,
        scSteelParameterGusset	= 9873,
        scSteelParameterLdr	= 9874,
        scSteelParameterIrr	= 9875,
        scSteelParameterIno	= 9876,
        scSteelParameterImm	= 9877,
        scSteelParameterCmb	= 9878,
        scSteelParameterDsd	= 9879,
        scSteelParameterOvr	= 9880,
        scSteelParameterWMax	= 9881,
        scSteelParameterFss	= 9882,
        scSteelParameterCan	= 9883,
        scSteelParameterSopen	= 9884,
        scSteelParameterEopen	= 9885,
        scSteelParameterCty	= 9886,
        scSteelParameterThe	= 9887,
        scSteelParameterEdi	= 9888,
        scSteelParameterDcf	= 9889,
        scSteelParameterCog	= 9890,
        scSteelParameterSpa	= 9891,
        scSteelParameterAxis	= 9892,
        scSteelParameterShear	= 9893,
        scSteelParameterStp	= 9894,
        scSteelParameterRHole	= 9895,
        scSteelParameterRDim	= 9896,
        scSteelParameterCHole	= 9897,
        scSteelParameterCDia	= 9898,
        scSteelParameterHEcc	= 9899,
        scSteelParameterElectrode	= 9900,
        scSteelParameterNT	= 9901,
        scSteelParameterAD	= 9902,
        scSteelParameterDinc	= 9903,
        scSteelParameterFinc	= 9904,
        scSteelParameterFtin	= 9905,
        scSteelParameterFbin	= 9906,
        scSteelParameterBmax	= 9907,
        scSteelParameterIncludeSeismicProvisions	= 9908,
        scSteelParameterBracedFrameCondition	= 9909,
        scSteelParameterHoleDiameter	= 9910,
        scSteelParameterBracedLocation_Major	= 9911,
        scSteelParameterBracedLocation_Minor_OuterFlange	= 9912,
        scSteelParameterBracedLocation_Minor_InnerFlange	= 9913,
        scSteelParameterBearingWidthStart	= 9914,
        scSteelParameterInsulationThickness	= 9915,
        scSteelParameterCladding	= 9916,
        scSteelParameterByPassCondition	= 9917,
        scSteelParameterSimpleSpanCondition	= 9918,
        scSteelParameterGalva	= 9919,
        scSteelParameterBeamRCeco	= 9920,
        scSteelParameterBearingWidthEnd	= 9921,
        scSteelParameterSlf	= 9922,
        scSteelParameterMethod	= 9923,
        scSteelParameterCT	= 9924,
        scSteelParameterCmx	= 9925,
        scSteelParameterAlpha	= 9926,
        scSteelParameterDBS	= 9927,
        scSteelParameterPSI	= 9928,
        scSteelParameterLAT	= 9929,
        scSteelParameterPLG	= 9930,
        scSteelParameterTST	= 9931,
        scSteelParameterTSP	= 9932,
        scSteelParameterLST	= 9933,
        scSteelParameterAVG	= 9934,
        scSteelParameterAVN	= 9935,
        scSteelParameterATG	= 9936,
        scSteelParameterATN	= 9937,
        scSteelParameterLHT	= 9938,
        scSteelParameterPBrace	= 9939,
        scSteelParameterHYD	= 9940,
        scSteelParameterPSD	= 9941,
        scSteelParameterSFC	= 9942,
        scSteelParameterSFT	= 9943,
        scSteelParameterSMZ	= 9944,
        scSteelParameterSMY	= 9945,
        scSteelParameterEqn	= 9946,
        scSteelParameterSRL	= 9947,
        scSteelParameterKS	= 9948,
        scSteelParameterKV	= 9949,
        scSteelParameterKBK	= 9950,
        scSteelParameterBiMoment	= 9951,
        scSteelParameterTbRussian2011	= 9952,
        scSteelParameterSEI	= 9953,
        scSteelParameterFRM	= 9954,
        scSteelParameterBRC	= 9955,
        scSteelParameterMTYP	= 9956,
        scSteelParameterINT	= 9957,
        scSteelParameterWLD	= 9958,
        scSteelParameterTEND	= 9959,
        scSteelParameterANG	= 9960,
        scSteelParameterNBL	= 9961,
        scSteelParameterFXTY	= 9962,
        scSteelParameterPBcRes	= 9963,
        scSteelParameterDUCT	= 9964,
        scSteelParameterGLD	= 9965,
        scSteelParameterSgrNZS3404	= 9966,
        scSteelParameterNCRCanadian	= 9967,
        scSteelParameterLBRC	= 9968,
        scSteelParameterTBRC	= 9969,
        scSteelParameterUnr	= 9970,
        scSteelParameterCanSTP	= 9971,
        scSteelParameterNBRC	= 9972,
        scSteelParameterBStiff	= 9973,
        scSteelParameterTStiff	= 9974,
        scSteelParameterSOE	= 9975,
        scSteelParameterTFA	= 9976,
        scSteelParameterRussianGammaF	= 9977,
        scSteelParameterRussian2017GammaM	= 9978,
        scSteelParameterSSD	= 9979,
        scSteelParameterLastNo	= 9980,
        scSteelCheckCode	= 9981,
        scSteelSelect	= 9982,
        scSteelSelectOptimized	= 9983,
        scSteelSelectWeld	= 9984,
        scSteelSelectWeldTruss	= 9985,
        scSteelFixedGroup	= 9986,
        scSteelGroup	= 9987,
        scSteelTakeOff	= 9988,
        scSteelMemberTakeOff	= 9989,
        scSteelLastNo	= 9999,
        scTimberParameter	= 10000,
        scTimberCode	= 10010,
        scTimberParameterGlulam	= 10100,
        scTimberParameterLz	= 10101,
        scTimberParameterLy	= 10102,
        scTimberParameterLuz	= 10103,
        scTimberParameterLuy	= 10104,
        scTimberParameterLamination	= 10105,
        scTimberParameterWet	= 10106,
        scTimberParameterNsf	= 10107,
        scTimberParameterCdt	= 10108,
        scTimberParameterCsf	= 10109,
        scTimberParameterCtm	= 10110,
        scTimberParameterCcr	= 10111,
        scTimberParameterRatio	= 10112,
        scTimberParameterBeam	= 10113,
        scTimberParameterCmc	= 10114,
        scTimberParameterCmp	= 10115,
        scTimberParameterCmv	= 10116,
        scTimberParameterCme	= 10117,
        scTimberParameterCfb	= 10118,
        scTimberParameterCft	= 10119,
        scTimberParameterCfc	= 10120,
        scTimberParameterCfu	= 10121,
        scTimberParameterCr	= 10122,
        scTimberParameterCtt	= 10123,
        scTimberParameterCh	= 10124,
        scTimberParameterCb	= 10125,
        scTimberParameterKl	= 10126,
        scTimberParameterInd	= 10127,
        scTimberParameterKbe	= 10128,
        scTimberParameterKce	= 10129,
        scTimberParameterKey	= 10130,
        scTimberParameterKez	= 10131,
        scTimberParameterKbd	= 10132,
        scTimberParameterKb	= 10133,
        scTimberParameterCmb	= 10134,
        scTimberParameterCmt	= 10135,
        scTimberParameterDMax	= 10136,
        scTimberParameterDMin	= 10137,
        scTimberParameterCv	= 10138,
        scTimberParameterCc	= 10139,
        scTimberParameterSrc	= 10140,
        scTimberParameterSrt	= 10141,
        scTimberParameterScl	= 10142,
        scTimberParameterldc	= 10143,
        scTimberParameterTsc	= 10144,
        scTimberParameterAlpha	= 10145,
        scTimberParameterKc90	= 10146,
        scTimberParameterMtyp	= 10147,
        scTimberParameterKlef	= 10148,
        scTimberParameterKy	= 10149,
        scTimberParameterKz	= 10150,
        scTimberParameterKx	= 10151,
        scTimberParameterKd	= 10152,
        scTimberParameterKh	= 10153,
        scTimberParameterKt	= 10154,
        scTimberParameterKsb	= 10155,
        scTimberParameterKsv	= 10156,
        scTimberParameterKsc	= 10157,
        scTimberParameterKse	= 10158,
        scTimberParameterKst	= 10159,
        scTimberParameterKzb	= 10160,
        scTimberParameterKzv	= 10161,
        scTimberParameterKzt	= 10162,
        scTimberParameterKzcp	= 10163,
        scTimberParameterKzc	= 10164,
        scTimberParameterKn	= 10166,
        scTimberParameterKscp	= 10167,
        scTimberParameterChix	= 10168,
        scTimberParameterType	= 10169,
        scTimberParameterSpecies	= 10170,
        scTimberParameterLx	= 10171,
        scTimberParameterDj1	= 10172,
        scTimberParameterDj2	= 10173,
        scTimberParameterTrack	= 10174,
        scTimberParameterDff	= 10175,
        scTimberParameterLastNo	= 10350,
        scTimberCheckCode	= 10400,
        scTimberSelect	= 10410,
        scTimberLastNo	= 10450,
        scStartConcDesign	= 11000,
        scConcCode	= 11010,
        scConcParameterFymain	= 11100,
        scConcParameterFymainJapan	= 11101,
        scConcParameterFysec	= 11110,
        scConcParameterFysecJapan	= 11111,
        scConcParameterFc	= 11120,
        scConcParameterClear	= 11130,
        scConcParameterMface	= 11139,
        scConcParameterSface	= 11140,
        scConcParameterSfaceRussian	= 11141,
        scConcParameterEface	= 11150,
        scConcParameterEfaceRussian	= 11151,
        scConcParameterSpsmain	= 11155,
        scConcParameterWidth	= 11160,
        scConcParameterDepth	= 11170,
        scConcParameterDepthRussian	= 11171,
        scConcParameterCrack	= 11180,
        scConcParameterCrackRussian	= 11181,
        scConcParameterMinmain	= 11190,
        scConcParameterMinmainMetric4050	= 11191,
        scConcParameterMinmainMetric2850	= 11192,
        scConcParameterMinmainMetric5060	= 11193,
        scConcParameterMinmainMetric3236	= 11194,
        scConcParameterMinmainMetric4555	= 11195,
        scConcParameterMinsec	= 11200,
        scConcParameterMinsecMetric4050	= 11201,
        scConcParameterMinsecMetric2850	= 11202,
        scConcParameterMinsecMetric5060	= 11203,
        scConcParameterMinsecMetric3236	= 11204,
        scConcParameterMinsecMetric4555	= 11205,
        scConcParameterMaxmain	= 11210,
        scConcParameterMaxmainMetric4050	= 11211,
        scConcParameterMaxmainMetric2850	= 11212,
        scConcParameterMaxmainMetric5060	= 11213,
        scConcParameterMaxmainMetric3236	= 11214,
        scConcParameterMaxmainMetric4555	= 11215,
        scConcParameterReinf	= 11220,
        scConcParameterMmag	= 11230,
        scConcParameterMmaRussian	= 11231,
        scConcParameterNsection	= 11240,
        scConcParameterNsectionEuro0321	= 11241,
        scConcParameterNsectionEuro1020	= 11242,
        scConcParameterNsectionRussian	= 11243,
        scConcParameterTrack	= 11250,
        scConcParameterTrackEuro01	= 11251,
        scConcParameterTrackEuro012	= 11252,
        scConcParameterTrackNorway	= 11253,
        scConcParameterEly	= 11260,
        scConcParameterElyRussian	= 11261,
        scConcParameterElz	= 11270,
        scConcParameterElzRussian	= 11271,
        scConcParameterUly	= 11272,
        scConcParameterUlz	= 11273,
        scConcParameterSra	= 11280,
        scConcParameterSraWood	= 11281,
        scConcParameterSraBaumann	= 11282,
        scConcParameterSraBs8007	= 11283,
        scConcParameterBrace	= 11290,
        scConcParameterScon	= 11300,
        scConcParameterTemp	= 11310,
        scConcParameterTempRussian	= 11311,
        scConcParameterServ	= 11320,
        scConcParameterMsa	= 11330,
        scConcParameterClt	= 11340,
        scConcParameterClb	= 11350,
        scConcParameterCls	= 11360,
        scConcParameterMaxsec	= 11370,
        scConcParameterMaxsecMetric5060	= 11371,
        scConcParameterRatio	= 11380,
        scConcParameterRface	= 11390,
        scConcParameterRfaceNorway	= 11391,
        scConcParameterLong	= 11400,
        scConcParameterSmag	= 11410,
        scConcParameterBiaxial	= 11420,
        scConcParameterTorsion	= 11430,
        scConcParameterTorsionIndian	= 11431,
        scConcParameterEnvir	= 11440,
        scConcParameterStirdia	= 11450,
        scConcParameterNmag	= 11460,
        scConcParameterMoy	= 11470,
        scConcParameterMoz	= 11480,
        scConcParameterRelhum	= 11490,
        scConcParameterLage	= 11500,
        scConcParameterActage	= 11510,
        scConcParameterDrycir	= 11520,
        scConcParameterStirang	= 11530,
        scConcParameterTorang	= 11540,
        scConcParameterNlt	= 11550,
        scConcParameterRcl	= 11560,
        scConcParameterUsm	= 11570,
        scConcParameterUb2	= 11580,
        scConcParameterDd1	= 11590,
        scConcParameterDd2	= 11600,
        scConcParameterBcl	= 11610,
        scConcParameterUbm	= 11620,
        scConcParameterCl1	= 11630,
        scConcParameterCl2	= 11640,
        scConcParameterWst	= 11650,
        scConcParameterWlt	= 11660,
        scConcParameterMmb	= 11670,
        scConcParameterSse	= 11680,
        scConcParameterRsh	= 11690,
        scConcParameterFwt	= 11700,
        scConcParameterFwb	= 11710,
        scConcParameterSdx	= 11720,
        scConcParameterSdy	= 11730,
        scConcParameterCl	= 11740,
        scConcParameterSta	= 11750,
        scConcParameterSelx	= 11760,
        scConcParameterSely	= 11770,
        scConcParameterReiang	= 11780,
        scConcParameterRhomn	= 11790,
        scConcParameterEnsh	= 11791,
        scConcParameterREnsh	= 11792,
        scConcParameterEudl	= 11793,
        scConcParameterPlastic	= 11794,
        scConcParameterIplm	= 11795,
        scConcParameterImb	= 11796,
        scConcParameterCombine	= 11797,
        scConcParameterBtp	= 11798,
        scConcParameterDim	= 11799,
        scConcParameterExp	= 11800,
        scConcParameterCcl	= 11801,
        scConcParameterLtc	= 11802,
        scConcParameterCfb	= 11803,
        scConcParameterDsd	= 11804,
        scConcParameterDag	= 11805,
        scConcParameterPss	= 11806,
        scConcParameterDcp	= 11807,
        scConcParameterPhi	= 11808,
        scConcParameterTeq	= 11809,
        scConcParameterMMy	= 11810,
        scConcParameterMMz	= 11811,
        scConcParameterLss	= 11812,
        scConcParameterMoe	= 11813,
        scConcParameterHlink	= 11814,
        scStartBarComb	= 11815,
        scConcParameterMd1	= 11816,
        scConcParameterMd2	= 11817,
        scEndBarComb	= 11818,
        scConcParameterGld	= 11819,
        scConcParameterMethod	= 11820,
        scConcParameterLWF	= 11821,
        scConcParameterSKZ	= 11822,
        scConcParameterSKY	= 11823,
        scConcParameterSWY	= 11824,
        scConcParameterSQZ	= 11825,
        scConcParameterSQY	= 11826,
        scConcParameterSLZ	= 11827,
        scConcParameterSLY	= 11828,
        scConcParameterBDZ	= 11829,
        scConcParameterBDY	= 11830,
        scConcParameterTRN	= 11831,
        scConcParameterRclRussian2012	= 11832,
        scConcParameterTRDO	= 11833,
        scConcParameterBRDO	= 11834,
        scConcParameterSRDO	= 11835,
        scConcParameterCRDO	= 11836,
        scConcParameterRCOAT	= 11837,
        scConcParameterMREB	= 11838,
        scConcParameterTREB	= 11839,
        scConcParameterMXMT	= 11840,
        scConcParameterMIMT	= 11841,
        scConcParameterMXMS	= 11842,
        scConcParameterMIMS	= 11843,
        scConcParameterMXMB	= 11844,
        scConcParameterMIMB	= 11845,
        scConcParameterMXMC	= 11846,
        scConcParameterMIMC	= 11847,
        scConcParameterTARS	= 11848,
        scConcParameterSTRS	= 11849,
        scConcParameterEDSP	= 11850,
        scConcParameterLATR	= 11851,
        scConcParameterXBRC	= 11852,
        scConcParameterYBRC	= 11853,
        scConcParameterSEM	= 11854,
        scConcParameterAlpha	= 11855,
        scConcParameterLastNo	= 11890,
        scConcDesignBeam	= 11900,
        scConcDesignColumn	= 11910,
        scConcDesignSlab	= 11920,
        scConcTakeOff	= 11930,
        scEndConcDesign	= 11950,
        scStartFootDesign	= 12000,
        scFootCode	= 12010,
        scFootParameterFy	= 12100,
        scFootParameterFc	= 12110,
        scFootParameterBc	= 12120,
        scFootParameterClear	= 12130,
        scFootParameterDepth	= 12140,
        scFootParameterS1	= 12150,
        scFootParameterS2	= 12160,
        scFootParameterEmbedment	= 12170,
        scFootParameterReinf	= 12180,
        scFootParameterFfac	= 12190,
        scFootParameterRatio	= 12200,
        scFootParameterTrack	= 12210,
        scFootParameterPedestal	= 12220,
        scFootParameterPedestalArea	= 12230,
        scFootParameterLastNo	= 12350,
        scFootDesign	= 12400,
        scEndFootDesign	= 12450,
        scAlumParameter	= 13000,
        scAlumCode	= 13010,
        scAlumParameterAlloy	= 13100,
        scAlumParameterProduct	= 13110,
        scAlumParameterAlclad	= 13120,
        scAlumParameterWeld	= 13130,
        scAlumParameterStructure	= 13140,
        scAlumParameterDmax	= 13150,
        scAlumParameterDmin	= 13160,
        scAlumParameterUnl	= 13170,
        scAlumParameterKy	= 13180,
        scAlumParameterKz	= 13190,
        scAlumParameterLy	= 13200,
        scAlumParameterLz	= 13210,
        scAlumParameterKt	= 13220,
        scAlumParameterLt	= 13230,
        scAlumParameterStiff	= 13240,
        scAlumParameterSsy	= 13250,
        scAlumParameterSsz	= 13260,
        scAlumParameterTrack	= 13270,
        scAlumParameterBeam	= 13280,
        scAlumParameterRatio	= 13290,
        scAlumParameterLastNo	= 13350,
        scAlumCheckCode	= 13400,
        scAlumSelect	= 13410,
        scAlumTakeOff	= 13411,
        scAlumMemberTakeOff	= 13412,
        scAlumLastNo	= 13450,
        scStartDefinePanel	= 13454,
        scPanel	= 13455,
        scEndDefinePanel	= 13456,
        scStartShearWallDesign	= 13460,
        scShearWallTwoLayered	= 13461,
        scShearWallFC	= 13462,
        scShearWallVerMin	= 13463,
        scShearWallVerMax	= 13464,
        scShearWallHorMin	= 13465,
        scShearWallHorMax	= 13466,
        scShearWallEdgeMin	= 13467,
        scShearWallEdgeMax	= 13468,
        scShearWallFyMain	= 13469,
        scShearWallClearCover	= 13470,
        scShearWallCode	= 13471,
        scShearWallDesign	= 13472,
        scShearWallTrack	= 13473,
        scShearWallReinf	= 13474,
        scShearWallLinkMin	= 13475,
        scShearWallLinkMax	= 13476,
        scShearWallKSlender	= 13477,
        scEndShearWallDesign	= 13560,
        scEqCodeParameter	= 13800,
        scSeismicCode	= 13801,
        scCheckGeometry	= 13850,
        scEndEqCodeParams	= 13899,
        scDeckRelatedData	= 16000,
        scSteadyStateBegin	= 16100,
        scSteadyGroundFreq	= 16101,
        scSteadyGroundMotion	= 16102,
        scHarmonicFreq	= 16103,
        scHarmonicForce	= 16104,
        scHarmonicGround	= 16105,
        scAmplitudeFunction	= 16106,
        scAmplitudePairs	= 16107,
        scSteadyForceFreq	= 16108,
        scSteadyForceJointLoad	= 16109,
        scSteadyForceJointLoadAssign	= 16110,
        scSteadyForceCopyLoad	= 16111,
        scSteadyForceCopyLoadData	= 16112,
        scSteadyPrintDisp	= 16699,
        scSteadyStateEnd	= 17000,
        scFinish	= 18000,
        scLockEdit	= 19000,
        scUnlockEdit	= 19001,
        scIgnoreInCalculationStart	= 19002,
        scIgnoreInCalculationEnd	= 19004
    } 	StaadCommand;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0002
    {
        cNull	= 0,
        cSteelDesignAashto	= 1001,
        cSteelDesignAisc	= 1002,
        cSteelDesignAustralian	= 1003,
        cSteelDesignBs5950	= 1004,
        cSteelDesignBs5400	= 1005,
        cSteelDesignCanadian	= 1006,
        cSteelDesignFrench	= 1007,
        cSteelDesignDin18800	= 1008,
        cSteelDesignIs800	= 1009,
        cSteelDesignJapanese	= 1010,
        cSteelDesignLrfd	= 1011,
        cSteelDesignNs3472	= 1012,
        cSteelDesignSpanish	= 1013,
        cSteelDesignNpd	= 1014,
        cSteelDesignBsk90	= 1015,
        cSteelDesignApi	= 1016,
        cSteelDesignEc3	= 1017,
        cSteelDesignChinese	= 1018,
        cSteelDesignDs412	= 1019,
        cSteelDesignAsce	= 1020,
        cSteelDesignB7	= 1021,
        cSteelDesignBsk99	= 1022,
        cSteelDesignDutch	= 1023,
        cSteelDesignCyprus	= 1024,
        cSteelDesignRussian	= 1025,
        cSteelDesignAisi1996	= 1026,
        cSteelDesignS136	= 1027,
        cSteelDesignIS801	= 1028,
        cSteelDesignIS802	= 1029,
        cSteelDesignMexicanLRFD	= 1030,
        cSteelDesignEgyptian	= 1031,
        cSteelDesignIs800Lsd	= 1032,
        cSteelDesignIs800Wsd	= 1052,
        cSteelDesignBS5950_COLD	= 1033,
        cSteelDesignSouthAfrican	= 1034,
        cSteelDesignAisc_RCECO	= 1035,
        cSteelDesignCanadianRCECO_1994	= 1036,
        cSteelDesignCanadianRCECO_2001	= 1037,
        cSteelDesignAisi_2001RCECO	= 1038,
        cSteelDesignAisi_1999RCECO	= 1039,
        cSteelDesignAisi_1996RCECO	= 1040,
        cSteelDesignS136_2001RCECO	= 1041,
        cSteelDesignS136_1999RCECO	= 1042,
        cSteelDesignS136_1994RCECO	= 1043,
        cSteelDesignAashto_Lrfd	= 1044,
        cSteelDesignAisc_Unified	= 1045,
        cSteelDesignNF3000_1989	= 1046,
        cSteelDesignNF3000_1998	= 1047,
        cSteelDesignNF3000_1974	= 1048,
        cSteelDesignNF3000_1977	= 1049,
        cSteelDesignNorsok	= 1050,
        cSteelDesignNF3000_2004	= 1051,
        cSteelDesignNF3000_2001	= 1053,
        cSteelDesignAsce52	= 1060,
        cSteelDesignAisc_Unified_2010	= 1061,
        cSteelDesignCanadianS16_09	= 1062,
        cSteelDesignRussian2011	= 1063,
        cSteelDesignSouthAfrican1993	= 1064,
        cSteelDesignCanadianS16_14	= 1065,
        cSteelDesignNZS3404_1997	= 1066,
        cSteelDesignAisc_Unified_2016	= 1067,
        cSteelDesignAisi_S100_2016	= 1068,
        cSteelDesignCanadianS16_19	= 1069,
        cSteelDesignAisc_Unified_2022	= 1070,
        cSteelDesignAustralian_2020	= 1071,
        cSteelDesignAisc_Cast	= 1102,
        cSteelDesignAsNzs4600_18	= 1103,
        cSteelDesignBs5950_1990	= 1104,
        cSteelDesignLrfd_Cast	= 1111,
        cSteelDesignAisc_N690	= 1202,
        cSteelDesignEc3British	= 1203,
        cSteelDesignAiscN690_1984	= 1204,
        cSteelDesignJapanese2005	= 1210,
        cSteelDesignEN1993	= 1220,
        cSteelDesignRussian2017	= 1221,
        cEarthquakeDesignEC8	= 1230,
        cConcreteDesignAci	= 2001,
        cConcreteDesignBs8110	= 2002,
        cConcreteDesignBs8007	= 2003,
        cConcreteDesignCanadian	= 2004,
        cConcreteDesignFrench	= 2005,
        cConcreteDesignDin1045	= 2006,
        cConcreteDesignIndian	= 2007,
        cConcreteDesignJapanese	= 2008,
        cConcreteDesignNs3473	= 2009,
        cConcreteDesignEc2	= 2010,
        cConcreteDesignChinese	= 2011,
        cConcreteDesignSpanish	= 2012,
        cConcreteDesignDutch	= 2013,
        cConcreteDesignCyprus	= 2014,
        cConcreteDesignRussian	= 2015,
        cConcreteDesignAustralian	= 2016,
        cConcreteDesignSwedish	= 2017,
        cConcreteDesignFinnish	= 2018,
        cConcreteDesignIS13920	= 2019,
        cConcreteDesignMexican	= 2020,
        cConcreteDesignDanish	= 2021,
        cConcreteDesignSouthAfrican	= 2022,
        cConcreteDesignCP65	= 2023,
        cConcreteDesignAci1999	= 2024,
        cConcreteDesignAci2002	= 2025,
        cConcreteDesignAci2005	= 2026,
        cConcreteDesignAci2008	= 2027,
        cConcreteDesignRussian2012	= 2028,
        cConcreteDesignAci2011	= 2029,
        cConcreteDesignIS13920_1993	= 2030,
        cTimberDesign	= 3001,
        cAitcTimberDesign	= 3002,
        cEC5TimberDesign	= 3003,
        cCanadianTimberDesign	= 3004,
        cAluminumDesign	= 4001,
        cCanadianAluminumDesign	= 4002,
        cFootingDesignAmerican	= 5001,
        cShearWallDesignAci	= 6001,
        cShearWallDesignBS8110	= 6002,
        cShearWallDesignIndian	= 6003
    } 	Code;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0003
    {
        smNull	= 0,
        smBugSyntax	= 2,
        smBugStaadData	= 3,
        smBugInputWidth	= 4,
        smBugOutputWidth	= 5,
        smBugGraphicPrinter	= 6,
        smBugSetNl	= 7,
        smBugSetExmemory	= 8,
        smBugSetCore	= 9,
        smBugSetCo	= 10,
        smBugSetRun	= 11,
        smBugSetDataCheck	= 12,
        smBugSeparator	= 13,
        smBugInputMemory	= 14,
        smWarningLineIgnored	= 15,
        smBugPrecision	= 16,
        smBugListData	= 17,
        smBugFixedLoad	= 18,
        smBugListType	= 19,
        smBugRangeNotDefined	= 20,
        smBugLoadListNotRange	= 21,
        smBugRangeLowGreaterHigh	= 22,
        smBugParallelUsedWithBeams	= 23,
        smWarningNodeNotFound	= 24,
        smWarningBeamNotFound	= 25,
        smWarningPlateNotFound	= 26,
        smWarningSolidNotFound	= 27,
        smWarningGeometryNotFound	= 28,
        smWarningLoadNotFound	= 29,
        smBugLineIgnoredNoList	= 30,
        smWarningListWithToIgnored	= 31,
        smBugElementThicknessProperty	= 32,
        smErrorMemberReleaseMp	= 33,
        smErrorMemberReleaseMpRange	= 34,
        smErrorMemberReleaseUnknown	= 35,
        smErrorMemberReleaseIgnored	= 36,
        smErrorElementReleaseJoint	= 37,
        smErrorElementReleaseUnknown	= 38,
        smErrorMemberReleaseEndUnknown	= 39,
        smErrorMemberOffsetEndUnknown	= 40,
        smErrorMemberCableTensionUnknown	= 41,
        smErrorConstantUnknown	= 42,
        smErrorListUnknown	= 43,
        smErrorSupportUnknown	= 44,
        smWarningFixedButUnknown	= 45,
        smErrorFixedButIgnored	= 46,
        smErrorInclinedIgnored	= 47,
        smErrorFootingIgnored	= 48,
        smErrorMatSupportIgnored	= 49,
        smErrorSlaveIgnored	= 50,
        smErrorSlaveNoJointList	= 51,
        smErrorGroupNameInvalid	= 52,
        smErrorGroupListInvalid	= 53,
        smErrorMemberPropertyCountryInvalid	= 54,
        smErrorAssignInvalid	= 55,
        smErrorTaperedInvalid	= 56,
        smErrorPrismaticUnknown	= 57,
        smErrorPrismaticInvalid	= 58,
        smErrorUserTableIgnored	= 59,
        smErrorUserTableNoInvalid	= 60,
        smErrorUserTableDataInvalid	= 61,
        smWarningUserTableIgnored	= 62,
        smErrorGroupNameNotDefined	= 63,
        smErrorUserTableNotDefined	= 64,
        smErrorUserTableNoNotDefined	= 65,
        smErrorUserTableNameNotDefined	= 66,
        smWarningJointLoadUnknown	= 67,
        smErrorJointLoadIgnored	= 68,
        smErrorMemberLoadIgnored	= 69,
        smErrorElementLoadIgnored	= 70,
        smErrorAreaLoadIgnored	= 71,
        smErrorFloorLoadIgnored	= 72,
        smErrorFloorLoadZUpIgnored	= 73,
        smErrorMemberPrestressLoadIgnored	= 74,
        smErrorMemberPoststressLoadIgnored	= 75,
        smWarningMemberPrestressLoadInvalid	= 76,
        smWarningMemberPoststressLoadInvalid	= 77,
        smErrorTemperatureLoadIgnored	= 78,
        smErrorFixedEndLoadIgnored	= 79,
        smErrorSupportDisplacementLoadIgnored	= 80,
        smErrorSelfweightIgnored	= 81,
        smErrorLoadCombinationIgnored	= 82,
        smErrorLoadIgnored	= 83,
        smErrorLoadCombinationListIgnored	= 84,
        smWarningLoadNotFoundRemoveLoadComb	= 85,
        smErrorPerformAnalysisIgnored	= 86,
        smErrorPdeltaAnalysisIgnored	= 87,
        smErrorNonlinearAnalysisIgnored	= 88,
        smErrorPrintIgnored	= 89,
        smWarningSectionIgnored	= 90,
        smErrorSectionIgnored	= 91,
        smErrorLoadListIgnored	= 92,
        smWarningSizeIgnored	= 93,
        smErrorSizeIgnored	= 94,
        smErrorCodeIgnored	= 95,
        smWarningDefaultSteelCodeAisc	= 96,
        smErrorSteelCodeNotSupported	= 97,
        smErrorSteelParameterNotAashto	= 98,
        smErrorSteelParameterNotAisc	= 99,
        smErrorSteelParameterNotAustralian	= 100,
        smErrorSteelParameterNotBs5950	= 101,
        smErrorSteelParameterNotBs5400	= 102,
        smErrorSteelParameterNotCanadian	= 103,
        smErrorSteelParameterNotFrench	= 104,
        smErrorSteelParameterNotGerman	= 105,
        smErrorSteelParameterNotIndian	= 106,
        smErrorSteelParameterNotJapanese	= 107,
        smErrorSteelParameterNotLrfd	= 108,
        smErrorSteelParameterNotNs3472	= 109,
        smErrorSteelParameterNotSpanish	= 110,
        smErrorSteelParameterNotNpd90	= 111,
        smErrorSteelParameterNotBsk90	= 112,
        smErrorSteelParameterNotApi	= 113,
        smErrorSteelParameterNotEc3	= 114,
        smErrorSteelParameterNotDin	= 115,
        smErrorSteelParameterNotChinese	= 116,
        smErrorSteelParameterNotDenmark	= 117,
        smErrorSteelParameterNotAsce	= 118,
        smErrorSteelParameterNotTimber	= 119,
        smErrorParameterNotAluminum	= 120,
        smErrorSteelParameterDataInvalid	= 121,
        smErrorSteelParameterFyldInvalid	= 122,
        smErrorSteelParameterWstrInvalid	= 123,
        smErrorSteelParameterPyInvalid	= 124,
        smErrorSteelParameterLyInvalid	= 125,
        smErrorSteelParameterLzInvalid	= 126,
        smErrorSteelParameterUnlInvalid	= 127,
        smErrorSteelParameterDmaxInvalid	= 128,
        smErrorSteelParameterDminInvalid	= 129,
        smErrorSteelParameterWminInvalid	= 130,
        smErrorSteelParameterLvvInvalid	= 131,
        smErrorSteelParameterStiffInvalid	= 132,
        smErrorSteelParameterDffInvalid	= 133,
        smErrorSteelParameterMaxInvalid	= 134,
        smErrorSteelParameterMinInvalid	= 135,
        smErrorSteelParameterKyInvalid	= 136,
        smErrorSteelParameterKzInvalid	= 137,
        smErrorSteelParameterNsfInvalid	= 138,
        smErrorSteelParameterUnfInvalid	= 139,
        smErrorSteelParameterCbInvalid	= 140,
        smErrorSteelParameterSsyInvalid	= 141,
        smErrorSteelParameterSsyData	= 142,
        smErrorSteelParameterSszInvalid	= 143,
        smErrorSteelParameterSszData	= 144,
        smErrorSteelParameterCmyInvalid	= 145,
        smErrorSteelParameterCmzInvalid	= 146,
        smErrorSteelParameterMainInvalid	= 147,
        smErrorSteelParameterMainData	= 148,
        smErrorSteelParameterPunchInvalid	= 149,
        smErrorSteelParameterPunchData	= 150,
        smErrorSteelParameterTrackInvalid	= 151,
        smErrorSteelParameterTrackDataNorway	= 152,
        smErrorSteelParameterTrackData	= 153,
        smErrorSteelParameterRatioInvalid	= 154,
        smErrorSteelParameterWeldInvalid	= 155,
        smErrorSteelParameterWeldData	= 156,
        smErrorSteelParameterBeamInvalid	= 157,
        smErrorSteelParameterBeamData	= 158,
        smErrorSteelParameterDj1Invalid	= 159,
        smErrorSteelParameterDj1Data	= 160,
        smErrorSteelParameterDj2Invalid	= 161,
        smErrorSteelParameterDj2Data	= 162,
        smErrorSteelParameterCyInvalid	= 163,
        smErrorSteelParameterCzInvalid	= 164,
        smErrorSteelParameterByInvalid	= 165,
        smErrorSteelParameterBzInvalid	= 166,
        smErrorSteelParameterMfInvalid	= 167,
        smErrorSteelParameterSgrInvalid	= 168,
        smErrorSteelParameterSgrData	= 169,
        smErrorSteelParameterSbltInvalid	= 170,
        smErrorSteelParameterSbltData	= 171,
        smErrorSteelParameterCmnInvalid	= 172,
        smErrorSteelParameterCmmInvalid	= 173,
        smErrorSteelParameterLegInvalid	= 174,
        smErrorSteelParameterLegData	= 175,
        smErrorSteelParameterWetInvalid	= 176,
        smErrorSteelParameterWetData	= 177,
        smErrorSteelParameterProfileInvalid	= 178,
        smErrorSteelParameterTbInvalid	= 179,
        smErrorSteelParameterTbData	= 180,
        smErrorSteelParameterEstInvalid	= 181,
        smErrorSteelParameterC1Invalid	= 182,
        smErrorSteelParameterC2Invalid	= 183,
        smErrorSteelParameterEtaInvalid	= 184,
        smErrorSteelParameterEtaData	= 185,
        smErrorSteelParameterGradeInvalid	= 186,
        smErrorSteelParameterGradeData	= 187,
        smErrorSteelParameterCompressionInvalid	= 188,
        smErrorSteelParameterTensionInvalid	= 189,
        smErrorSteelParameterPfyInvalid	= 190,
        smErrorSteelParameterPfzInvalid	= 191,
        smErrorSteelParameterSfyInvalid	= 192,
        smErrorSteelParameterSfzInvalid	= 193,
        smErrorSteelParameterUntInvalid	= 194,
        smErrorSteelParameterUnbInvalid	= 195,
        smErrorSteelParameterTorsionInvalid	= 196,
        smErrorSteelParameterTorsionData	= 197,
        smErrorSteelParameterTmpInvalid	= 198,
        smErrorTimberParameterGlulamInvalid	= 199,
        smErrorTimberParameterLzInvalid	= 200,
        smErrorTimberParameterLyInvalid	= 201,
        smErrorTimberParameterLuzInvalid	= 202,
        smErrorTimberParameterLuyInvalid	= 203,
        smErrorTimberParameterLaminationInvalid	= 204,
        smErrorTimberParameterLaminationData	= 205,
        smErrorTimberParameterWetInvalid	= 206,
        smErrorTimberParameterWetData	= 207,
        smErrorTimberParameterNsfInvalid	= 208,
        smErrorTimberParameterCdtInvalid	= 209,
        smErrorTimberParameterCsfInvalid	= 210,
        smErrorTimberParameterCtmInvalid	= 211,
        smErrorTimberParameterCcrInvalid	= 212,
        smErrorTimberParameterRatioInvalid	= 213,
        smErrorTimberParameterBeamInvalid	= 214,
        smErrorTimberParameterBeamData	= 215,
        smErrorAlumParameterAlloyInvalid	= 216,
        smErrorAlumParameterAlloyData	= 217,
        smErrorAlumParameterProductInvalid	= 218,
        smErrorAlumParameterProductData	= 219,
        smErrorAlumParameterAlcladInvalid	= 220,
        smErrorAlumParameterAlcladData	= 221,
        smErrorAlumParameterWeldInvalid	= 222,
        smErrorAlumParameterWeldData	= 223,
        smErrorAlumParameterStructureInvalid	= 224,
        smErrorAlumParameterStructureData	= 225,
        smErrorAlumParameterDmaxInvalid	= 226,
        smErrorAlumParameterDminInvalid	= 227,
        smErrorAlumParameterUnlInvalid	= 228,
        smErrorAlumParameterKyInvalid	= 229,
        smErrorAlumParameterKzInvalid	= 230,
        smErrorAlumParameterLyInvalid	= 231,
        smErrorAlumParameterLzInvalid	= 232,
        smErrorAlumParameterKtInvalid	= 233,
        smErrorAlumParameterLtInvalid	= 234,
        smErrorAlumParameterStiffInvalid	= 235,
        smErrorAlumParameterSsyInvalid	= 236,
        smErrorAlumParameterSsyData	= 237,
        smErrorAlumParameterSszInvalid	= 238,
        smErrorAlumParameterSszData	= 239,
        smErrorAlumParameterTrackInvalid	= 240,
        smErrorAlumParameterTrackData	= 241,
        smErrorAlumParameterBeamInvalid	= 242,
        smErrorAlumParameterBeamData	= 243,
        smWarningDefaultConcCodeAci	= 244,
        smErrorConcCodeNotSupported	= 245,
        smErrorConcParameterNotAci	= 246,
        smErrorConcParameterNotBs8110	= 247,
        smErrorConcParameterNotBs8007	= 248,
        smErrorConcParameterNotCanadian	= 249,
        smErrorConcParameterNotFrench	= 250,
        smErrorConcParameterNotGerman	= 251,
        smErrorConcParameterNotIndian	= 252,
        smErrorConcParameterNotJapanese	= 253,
        smErrorConcParameterNotNorway	= 254,
        smErrorConcParameterNotEc2	= 255,
        smErrorConcParameterNotChinese	= 256,
        smErrorConcParameterNotSpanish	= 257,
        smWarningDefaultFootCodeAmerican	= 258,
        smErrorFootParameterNotSupported	= 259,
        smErrorConcParameterFymainInvalid	= 260,
        smErrorConcParameterFysecInvalid	= 261,
        smErrorConcParameterFcInvalid	= 262,
        smErrorConcParameterClearInvalid	= 263,
        smErrorConcParameterSfaceInvalid	= 264,
        smErrorConcParameterEfaceInvalid	= 265,
        smErrorConcParameterWidthInvalid	= 266,
        smErrorConcParameterDepthInvalid	= 267,
        smErrorConcParameterCrackInvalid	= 268,
        smErrorConcParameterMinmainInvalid	= 269,
        smErrorConcParameterMinmainData	= 270,
        smErrorConcParameterMinsecInvalid	= 271,
        smErrorConcParameterMinsecData	= 272,
        smErrorConcParameterMaxmainInvalid	= 273,
        smErrorConcParameterMaxmainData	= 274,
        smErrorConcParameterReinfInvalid	= 275,
        smErrorConcParameterReinfData	= 276,
        smErrorConcParameterMmagInvalid	= 277,
        smErrorConcParameterNsectionInvalid	= 278,
        smErrorConcParameterTrackInvalid	= 279,
        smErrorConcParameterTrackData	= 280,
        smErrorConcParameterElyInvalid	= 281,
        smErrorConcParameterElzInvalid	= 282,
        smErrorConcParameterSraInvalid	= 283,
        smErrorConcParameterBraceInvalid	= 284,
        smErrorConcParameterBraceData	= 285,
        smErrorConcParameterSconInvalid	= 286,
        smErrorConcParameterSconData	= 287,
        smErrorConcParameterTempInvalid	= 288,
        smErrorConcParameterServInvalid	= 289,
        smErrorConcParameterServData	= 290,
        smErrorConcParameterMsaInvalid	= 291,
        smErrorConcParameterCltInvalid	= 292,
        smErrorConcParameterClbInvalid	= 293,
        smErrorConcParameterClsInvalid	= 294,
        smErrorConcParameterMaxsecInvalid	= 295,
        smErrorConcParameterRatioInvalid	= 296,
        smErrorConcParameterRfaceInvalid	= 297,
        smErrorConcParameterRfaceData	= 298,
        smErrorConcParameterLongInvalid	= 299,
        smErrorConcParameterLongData	= 300,
        smErrorConcParameterSmagInvalid	= 301,
        smErrorConcParameterBiaxialInvalid	= 302,
        smErrorConcParameterTorsionInvalid	= 303,
        smErrorFootParameterFyInvalid	= 304,
        smErrorFootParameterFcInvalid	= 305,
        smErrorFootParameterBcInvalid	= 306,
        smErrorFootParameterClearInvalid	= 307,
        smErrorFootParameterDepthInvalid	= 308,
        smErrorFootParameterS1Invalid	= 309,
        smErrorFootParameterS2Invalid	= 310,
        smErrorFootParameterEmbedmentInvalid	= 311,
        smErrorFootParameterReinfInvalid	= 312,
        smErrorFootParameterFfacInvalid	= 313,
        smErrorFootParameterRatioInvalid	= 314,
        smErrorFootParameterTrackInvalid	= 315,
        smErrorFootParameterTrackData	= 316,
        smErrorFootParameterPedestalInvalid	= 317,
        smErrorFootParameterPedestalData	= 318,
        smErrorConcDesignCommandIgnored	= 319,
        smErrorFootDesignCommandIgnored	= 320,
        smErrorCommandDataInvalid	= 321,
        smErrorCheckCodeInvalid	= 322,
        smErrorSelectInvalid	= 323,
        smErrorGroupInvalid	= 324,
        smErrorGroupSameAsInvalid	= 325,
        smErrorDrawInvalid	= 326,
        smErrorDrawLoadInvalid	= 327,
        smErrorDrawAnalysisInvalid	= 328,
        smErrorMemberPropertyInvalid	= 329,
        smErrorMemberPropertySpecInvalid	= 330,
        smErrorMemberPropertyAddSpecInvalid	= 331,
        smErrorMemberPropertyAddDataInvalid	= 332,
        smErrorMemberPropertyAddDataRequired	= 333,
        smErrorSteelParameterMainDataInvalid	= 334,
        smErrorSteelParameterBeamDataInvalid	= 335,
        smErrorSteelPropertyNotInTable	= 336,
        smErrorJobInformationIgnored	= 337,
        smErrorScriptIgnored	= 338,
        smErrorScriptOpenFileInvalid	= 339,
        smErrorScriptSortInvalid	= 340,
        smErrorScriptWriteInvalid	= 341,
        smErrorScriptForInvalid	= 342,
        smErrorScriptExecuteInvalid	= 343,
        smErrorScriptCallInvalid	= 344,
        smErrorDefineMovingTypeInvalid	= 345,
        smErrorDefineMovingTypeIgnored	= 346,
        smErrorDefineJointLoadIgnored	= 347,
        smErrorDefineMemberLoadIgnored	= 348,
        smErrorDefineZoneIgnored	= 349,
        smErrorDefineWindTypeIgnored	= 350,
        smErrorDefineWindIntensityList	= 351,
        smErrorDefineWindIntensityIgnored	= 352,
        smErrorDefineTimeTypeIgnored	= 353,
        smErrorDefineTimeFunctionIgnored	= 354,
        smErrorDefineTimeAmplitudeIgnored	= 355,
        smErrorLoadGenerationIgnored	= 356,
        smErrorLoadGenerationTypeIgnored	= 357,
        smErrorLoadGenerationAddLoad	= 358,
        smErrorAijLoadIgnored	= 359,
        smErrorUbcLoadIgnored	= 360,
        smErrorWindLoadIgnored	= 361,
        smErrorDefineMovingNotPresent	= 362,
        smErrorDefineUbcNotPresent	= 363,
        smErrorDefineAijNotPresent	= 364,
        smErrorDefineWindNotPresent	= 365,
        smErrorDefineTimeNotPresent	= 366,
        smErrorTimeLoadIgnored	= 367,
        smErrorGroundMotionIgnored	= 368,
        smWarningLoadNotFoundRepeatLoad	= 369,
        smErrorRepeatLoadIgnored	= 370,
        smErrorSpectrumIgnored	= 371,
        smWarningLoadNotPresentLoadCombDead	= 372,
        smErrorLoadCombDeadIgnored	= 373,
        smWarningLoadNotPresentLoadCombLive	= 374,
        smErrorLoadCombLiveIgnored	= 375,
        smErrorLoadGenerationCommandIgnored	= 376,
        smErrorGroupListTypeInvalid	= 377,
        smErrorConcParameterEnvirInvalid	= 378,
        smErrorConcParameterStirdiaInvalid	= 379,
        smErrorConcParameterNmagInvalid	= 380,
        smErrorConcParameterMoyInvalid	= 381,
        smErrorConcParameterMozInvalid	= 382,
        smErrorConcParameterRelhumInvalid	= 383,
        smErrorConcParameterLageInvalid	= 384,
        smErrorConcParameterActageInvalid	= 385,
        smErrorConcParameterDrycirInvalid	= 386,
        smErrorConcParameterStirangInvalid	= 387,
        smErrorConcParameterTorangInvalid	= 388,
        smErrorDefineMeshTypeIllegal	= 389,
        smErrorDefineMeshLineIgnored	= 390,
        smErrorDefineMeshCoordinateSystemIgnored	= 391,
        smErrorDefineMeshReferenceInvalid	= 392,
        smErrorDefineMeshJointNoInvalid	= 393,
        smErrorDefineMeshElementNodeInvalid	= 394,
        smErrorDefineMeshElementBoundaryInvalid	= 395,
        smErrorDefineMeshElementPerSideInvalid	= 396,
        smErrorDefineMeshCommandIgnored	= 397,
        smErrorRowColumnLimit40Exceeded	= 398,
        smErrorDefineMeshConnectivityInvalid	= 399,
        smWarningDefineMeshColumnLimit40	= 400,
        smWarningDefineMeshRowLimit40	= 401,
        smErrorLoadCombNoInvalid	= 402,
        smErrorMultilinearSupportIgnored	= 403,
        smCommandNotPlaceAfter	= 404,
        smDeleteAllSteelParameter	= 405,
        smDeleteAllTimberParameter	= 406,
        smDeleteAllAluminumParameter	= 407,
        smDeleteAllConcreteDesign	= 408,
        smDeleteAllFootingDesign	= 409,
        smDeleteSteelParameter	= 410,
        smDeleteTimberParameter	= 411,
        smDeleteAluminumParameter	= 412,
        smDeleteConcreteParameter	= 413,
        smDeleteFootingParameter	= 414,
        smDeleteStaadCommand	= 415,
        smDeleteFinish	= 416,
        smDeleteStaadType	= 417,
        smErrorAlumParameterRatioInvalid	= 418,
        smDeleteAnalysisCommand	= 419,
        smDeletePrintCommand	= 420,
        smErrorSteelParameterCmmBs5950Invalid	= 421,
        smErrorSteelParameterCmnBs5950Invalid	= 422,
        smErrorSteelParameterEstiffData	= 423,
        smErrorSteelParameterEstiffInvalid	= 424,
        smErrorSteelParameterLegBs5950Data	= 425,
        smErrorSteelParameterLegBs5950Invalid	= 426,
        smErrorSteelParameterWeldBs5950Data	= 427,
        smErrorSteelParameterWeldBs5950Invalid	= 428,
        smErrorSteelParameterPnlInvalid	= 429,
        smErrorUseIgnoreList	= 430,
        smErrorBugLineLimit	= 431,
        smErrorBeamNotValidByNode	= 432,
        smErrorPlateNotValidByNode	= 433,
        smErrorSolidNotValidByNode	= 434,
        smErrorColombianLoadIgnored	= 435,
        smErrorDefineColombianNotPresent	= 436,
        smCommandNotImplemented	= 437,
        smDeleteCurrentProperty	= 438,
        smASAProcessCommand	= 439,
        smASAUnProcessCommand	= 440,
        smFileHeaderBorder	= 441,
        smFileHeaderName	= 442,
        smFileHeaderASAName	= 443,
        smFileHeaderDate	= 444,
        smFileHeaderTime	= 445,
        smFileHeaderErrorName	= 446,
        smErrorPdeltaAnalysisConverge	= 447,
        smWarningRepeatLoadLimit14	= 448,
        smWarningLoadCombLimit29	= 449,
        smWarningRemoveFileData	= 450,
        smErrorSteelParameterNotDutch	= 451,
        smErrorSteelParameterNotCyprus	= 452,
        smErrorSteelParameterNotRussian	= 453,
        smErrorConcParameterNotDutch	= 454,
        smErrorConcParameterNotCyprus	= 455,
        smErrorConcParameterNotRussian	= 456,
        smErrorSteelParameterPyRussianData	= 457,
        smErrorSteelParameterUnlRussianData	= 458,
        smErrorSteelParameterSgrRussianData	= 459,
        smErrorSteelParameterCmmRussianData	= 460,
        smErrorSteelParameterCmnRussianData	= 461,
        smErrorSteelParameterLegRussianData	= 462,
        smErrorSteelParameterCbRussianData	= 463,
        smErrorSteelParameterTrack01Data	= 464,
        smErrorSteelParameterMainRussianData	= 465,
        smErrorConcParameterNltInvalid	= 466,
        smErrorConcParameterNltData	= 467,
        smErrorConcParameterRclInvalid	= 468,
        smErrorConcParameterRclData	= 469,
        smErrorConcParameterUsmInvalid	= 470,
        smErrorConcParameterUb2Invalid	= 471,
        smErrorConcParameterDd1Invalid	= 472,
        smErrorConcParameterDd2Invalid	= 473,
        smErrorConcParameterBclInvalid	= 474,
        smErrorConcParameterUbmInvalid	= 475,
        smErrorConcParameterTempRussianInvalid	= 476,
        smErrorConcParameterTempRussianData	= 477,
        smErrorConcParameterCl1Invalid	= 478,
        smErrorConcParameterCl2Invalid	= 479,
        smErrorConcParameterWstInvalid	= 480,
        smErrorConcParameterWltInvalid	= 481,
        smErrorConcParameterMmbInvalid	= 482,
        smErrorConcParameterSseInvalid	= 483,
        smErrorConcParameterSseData	= 484,
        smErrorConcParameterRshInvalid	= 485,
        smErrorConcParameterRshData	= 486,
        smErrorConcParameterFwtInvalid	= 487,
        smErrorConcParameterFwbInvalid	= 488,
        smErrorConcParameterSdxInvalid	= 489,
        smErrorConcParameterSdyInvalid	= 490,
        smErrorConcParameterClInvalid	= 491,
        smErrorConcParameterStaInvalid	= 492,
        smErrorConcParameterStaData	= 493,
        smErrorConcParameterSelxInvalid	= 494,
        smErrorConcParameterSelyInvalid	= 495,
        smErrorConcParameterReiangInvalid	= 496,
        smError1893LoadIgnored	= 497,
        smErrorDefine1893NotPresent	= 498,
        smErrorSteelParameterMainIs800Invalid	= 499,
        smErrorSteelParameterTmainIs800Invalid	= 500,
        smErrorSteelParameterTrack0124Data	= 501,
        smErrorSteelParameterLxInvalid	= 502,
        smErrorSteelParameterKxInvalid	= 503,
        smErrorSetSolverInvalid	= 504,
        smErrorDefineDampInvalid	= 505,
        smErrorSpectrumFileInvalid	= 506,
        smErrorSpectrumDataNotPresent	= 507,
        smErrorConcParameterRhomnInvalid	= 508,
        smErrorPrismaticNegativeData	= 509,
        smErrorTaperedNegativeData	= 510,
        smErrorSteelParameterFyldNegativeData	= 511,
        smErrorSteelParameterWstrNegativeData	= 512,
        smErrorSteelParameterPyNegativeData	= 513,
        smErrorSteelParameterLxNegativeData	= 514,
        smErrorSteelParameterLyNegativeData	= 515,
        smErrorSteelParameterLzNegativeData	= 516,
        smErrorSteelParameterUnlNegativeData	= 517,
        smErrorSteelParameterDmaxNegativeData	= 518,
        smErrorSteelParameterDminNegativeData	= 519,
        smErrorSteelParameterWminNegativeData	= 520,
        smErrorSteelParameterLvvNegativeData	= 521,
        smErrorSteelParameterStiffNegativeData	= 522,
        smErrorSteelParameterDffNegativeData	= 523,
        smErrorSteelParameterMaxNegativeData	= 524,
        smErrorSteelParameterMinNegativeData	= 525,
        smErrorConcParameterFymainNegativeData	= 526,
        smErrorConcParameterFysecNegativeData	= 527,
        smErrorConcParameterFcNegativeData	= 528,
        smErrorConcParameterClearNegativeData	= 529,
        smErrorConcParameterSfaceNegativeData	= 530,
        smErrorConcParameterEfaceNegativeData	= 531,
        smErrorConcParameterWidthNegativeData	= 532,
        smErrorConcParameterDepthNegativeData	= 533,
        smErrorConcParameterCrackNegativeData	= 534,
        smErrorConcParameterCltNegativeData	= 535,
        smErrorConcParameterClbNegativeData	= 536,
        smErrorConcParameterClsNegativeData	= 537,
        smErrorConcParameterStirdiaNegativeData	= 538,
        smErrorConcParameterDd1NegativeData	= 539,
        smErrorConcParameterDd2NegativeData	= 540,
        smErrorConcParameterBclNegativeData	= 541,
        smErrorConcParameterCl1NegativeData	= 542,
        smErrorConcParameterCl2NegativeData	= 543,
        smErrorConcParameterWstNegativeData	= 544,
        smErrorConcParameterWltNegativeData	= 545,
        smErrorConcParameterSdxNegativeData	= 546,
        smErrorConcParameterSdyNegativeData	= 547,
        smErrorConcParameterClNegativeData	= 548,
        smErrorConcParameterSelxNegativeData	= 549,
        smErrorConcParameterSelyNegativeData	= 550,
        smErrorNotMaterialCommand	= 551,
        smErrorDefineNonlinearSpringTypeInvalid	= 552,
        smErrorDefineNonlinearSpringDataInvalid	= 553,
        smErrorDefineNonlinearSpringNodeInvalid	= 554,
        smErrorDefineNonlinearSpringDirectionInvalid	= 555,
        smErrorDefineNonlinearSpringTypeNoDuplicate	= 556,
        smErrorPrintTimeNoNonlinearTimeHistory	= 557,
        scErrorDefineMaterialIsotropicNameInvalid	= 558,
        scErrorDefineMaterial2dorthotropicNameInvalid	= 559,
        scErrorDefineMaterial3dorthotropicNameInvalid	= 560,
        scErrorDefineMaterialDataNotComplete	= 561,
        scErrorDefineMaterialIsotropicGInvalid	= 562,
        scDefineMaterialStaadProDebuggerModeActivatedNo1	= 563,
        scDefineMaterialStaadProDebuggerModeActivatedNo2	= 564,
        scDefineMaterialStaadProDebuggerModeActivatedNo3	= 565,
        scDefineMaterialStaadProDebuggerModeActivatedNo4	= 566,
        scDefineMaterialStaadProDebuggerModeActivatedNo5	= 567,
        scDefineMaterialStaadProDebuggerModeActivatedNo6	= 568,
        scDefineMaterialStaadProDebuggerModeActivatedNo7	= 569,
        scDefineMaterialStaadProDebuggerModeActivatedNo8	= 570,
        scDefineMaterialStaadProDebuggerModeStart	= 571,
        scDefineMaterialStaadProDebuggerModeEnd	= 572,
        scMaterialStaadProDebuggerModeStart	= 573,
        scMaterialStaadProDebuggerModeEnd	= 574,
        smErrorSteelParameterFuNegativeData	= 575,
        smErrorSteelParameterFuInvalid	= 576,
        smErrorWriteCommandDataLineToLongNo1	= 577,
        smErrorWriteCommandDataLineToLongNo2	= 578,
        smErrorWriteCommandDataLineToLongNo3	= 579,
        smErrorConcParameterSpsmainNegativeData	= 580,
        smErrorConcParameterSpsmainInvalid	= 581,
        smErrorSteelParameterCmpData	= 582,
        smErrorSteelParameterCmpInvalid	= 583,
        smErrorSteelParameterDiaNegativeData	= 584,
        smErrorSteelParameterDiaInvalid	= 585,
        smErrorSteelParameterHgtNegativeData	= 586,
        smErrorSteelParameterHgtInvalid	= 587,
        smErrorSteelParameterCycNegativeData	= 588,
        smErrorSteelParameterCycInvalid	= 589,
        smErrorSteelParameterDr1NegativeData	= 590,
        smErrorSteelParameterDr1Invalid	= 591,
        smErrorSteelParameterDr2NegativeData	= 592,
        smErrorSteelParameterDr2Invalid	= 593,
        smErrorSteelParameterWidNegativeData	= 594,
        smErrorSteelParameterWidInvalid	= 595,
        smErrorSteelParameterFpcNegativeData	= 596,
        smErrorSteelParameterFpcInvalid	= 597,
        smErrorSteelParameterImpNegativeData	= 598,
        smErrorSteelParameterImpInvalid	= 599,
        smErrorSteelParameterPltNegativeData	= 600,
        smErrorSteelParameterPltInvalid	= 601,
        smErrorSteelParameterPlwNegativeData	= 602,
        smErrorSteelParameterPlwInvalid	= 603,
        smErrorSteelParameterRbhNegativeData	= 604,
        smErrorSteelParameterRbhInvalid	= 605,
        smErrorSteelParameterRbwNegativeData	= 606,
        smErrorSteelParameterRbwInvalid	= 607,
        smErrorSteelParameterShrData	= 608,
        smErrorSteelParameterShrInvalid	= 609,
        smErrorSteelParameterThkNegativeData	= 610,
        smErrorSteelParameterThkInvalid	= 611,
        smErrorDefineUbcZone1985ParameterInvalid	= 612,
        smErrorDefineUbcZone1994ParameterInvalid	= 613,
        smErrorDefineUbcZone1997ParameterInvalid	= 614,
        smErrorUniformLoadLength	= 615,
        smErrorTrapLoadLength	= 616,
        smErrorSelfweightNoFactorIgnored	= 617,
        smErrorSelfweightCanNotBeDefinedNoLoad	= 618,
        smErrorSteelParameterCbCanadianInvalid	= 619,
        smErrorSteelParameterCbCanadianData	= 620,
        smErrorSteelParameterCmyCanadianInvalid	= 621,
        smErrorSteelParameterCmyCanadianData	= 622,
        smErrorSteelParameterCmzCanadianInvalid	= 623,
        smErrorSteelParameterCmzCanadianData	= 624,
        smErrorSteelParameterIstInvalid	= 625,
        smErrorSteelParameterIstData	= 626,
        smErrorSteelParameterPhiInvalid	= 627,
        smErrorSteelParameterPhiData	= 628,
        smErrorSteelParameterNscInvalid	= 629,
        smErrorSteelParameterNscData	= 630,
        smErrorSteelParameterAlmInvalid	= 631,
        smErrorSteelParameterAlmData	= 632,
        smErrorSteelParameterAlbInvalid	= 633,
        smErrorSteelParameterAlbData	= 634,
        smErrorSteelParameterKtInvalid	= 635,
        smErrorSteelParameterKtData	= 636,
        smErrorConcParameterNotAustralian	= 637,
        smErrorSpringTensionData	= 638,
        smErrorSpringCompressionData	= 639,
        smWarningEnforcedButUnknown	= 640,
        smErrorEnforcedButIgnored	= 641,
        smErrorSetNj	= 642,
        smErrorSetNm	= 643,
        smErrorSetFileExtension	= 644,
        smErrorReferenceJtNode	= 645,
        smErrorConstantBeamMissing	= 646,
        smErrorListByOptionBug	= 647,
        smErrorElementLoadCommandMissing	= 648,
        smErrorPlotStressFileCommandDeactivated	= 649,
        smErrorPlotModeFileCommandDeactivated	= 650,
        smErrorPlotBendingFileCommandDeactivated	= 651,
        smErrorPlotDisplacementFileCommandDeactivated	= 652,
        smErrorPlotSectionFileCommandDeactivated	= 653,
        smErrorSteelParameterTrackJapanese	= 654,
        smErrorConcParameterTorsionIndianData	= 655,
        smErrorSetGui	= 656,
        smErrorSteelParameterGamC1Invalid	= 657,
        smErrorSteelParameterGamC2Invalid	= 658,
        smErrorSteelParameterMLTInvalid	= 659,
        smErrorSteelParameterMXInvalid	= 660,
        smErrorSteelParameterMYInvalid	= 661,
        smErrorSteelParameterMYXInvalid	= 662,
        smErrorSteelParameterSwayInvalid	= 663,
        smErrorSteelParameter5950Mixed	= 664,
        smErrorDefineZoneIgnoredParameter	= 665,
        smErrorWindLoadRangeOption	= 666,
        smErrorWindLoadRangeMinMax	= 667,
        smErrorSteelParameterNotAisi	= 668,
        smErrorSteelParameterFlxInvalid	= 669,
        smErrorSteelParameterTsaInvalid	= 670,
        smErrorSteelParameterCwyInvalid	= 671,
        smErrorDefineSdsIgnored	= 672,
        smErrorDefineIbcNotPresent	= 673,
        smErrorIbcLoadIgnored	= 674,
        smErrorConcParameterEnshInvalid	= 675,
        smErrorConcParameterREnshInvalid	= 676,
        smErrorConcParameterEudlInvalid	= 677,
        smErrorConcParameterPlasticInvalid	= 678,
        smErrorConcParameterIplmInvalid	= 679,
        smErrorConcParameterImbInvalid	= 680,
        smErrorConcParameterCombineInvalid	= 681,
        smErrorConcParameterNotIS13920	= 682,
        smErrorPerformCableAnalysisIgnored	= 683,
        smWarningDuplicateNodeIncidence	= 684,
        smWarningDuplicateBeamIncidence	= 685,
        smWarningDuplicatePlateIncidence	= 686,
        smWarningDuplicateSolidIncidence	= 687,
        smErrorSteelParameterElaInvalid	= 688,
        smErrorSteelParameterElbInvalid	= 689,
        smErrorSteelParameterDblInvalid	= 690,
        smErrorSteelParameterFybInvalid	= 691,
        smErrorSteelParameterFvbInvalid	= 692,
        smErrorSteelParameterNhlInvalid	= 693,
        smErrorSteelParameterTaperInvalid	= 694,
        smErrorSteelParameterTaperDataInvalid	= 695,
        smErrorSteelParameterLtInvalid	= 696,
        smErrorSteelParameterLtData	= 697,
        smErrorSteelParameterSameDataInvalid	= 698,
        smErrorSteelParameterSameInvalid	= 699,
        smErrorSteelParameterCompressionSpanishData	= 700,
        smErrorSteelParameterCmmDin18800Data	= 701,
        smErrorSpringDampingParameterInvalid	= 702,
        smErrorBeamNotValidSameStartEndNode	= 703,
        smJCHasToBeDefined_MI	= 704,
        smJCHasToBeDefined_PI	= 705,
        smJCHasToBeDefined_SI	= 706,
        smJCHasToBeDefined_SWI	= 707,
        smPIHasToBeDefinedBeforeSWI	= 708,
        smSIHasToBeDefinedBeforeSWI	= 709,
        smMIHasToBeDefinedBeforeSWI	= 710,
        smErrorGenSupportIgnored	= 711,
        smErrorSHEParamTWOData	= 712,
        smErrorSHEParamFCData	= 713,
        smErrorSHEParamVerMinData	= 714,
        smErrorSHEParamVerMaxData	= 715,
        smErrorSHEParamHorMinData	= 716,
        smErrorSHEParamHorMaxData	= 717,
        smErrorSHEParamEdgeMinData	= 718,
        smErrorSHEParamEdgeMaxData	= 719,
        smErrorSHEParamFyMainData	= 720,
        smErrorSHEParamClearCoverData	= 721,
        smSurfaceLoadIgnored	= 722,
        smErrorSteelParameterGussetInvalid	= 723,
        smErrorSteelParameterCnsfInvalid	= 724,
        smErrorSteelParameterDangleInvalid	= 725,
        smErrorOneWayFloorLoadIgnored	= 726,
        smErrorDefineElementLoadIgnored	= 727,
        smErrorElementLoadJtPressure	= 728,
        smErrorElementLoadSolid	= 729,
        smErrorElementLoadSolidFaceInvalid	= 730,
        smErrorElementLoadnofPressures	= 731,
        smErrorConflictingMemberIncidence	= 732,
        smErrorConflictingPlateIncidence	= 733,
        smErrorConflictingSolidIncidence	= 734,
        smErrorSurfaceDivisionIgnored	= 735,
        smWarningSurfaceNotFound	= 736,
        smErrorSHEParamTrack	= 737,
        smErrorSHEParamReinf	= 738,
        smErrorExternalFileNotFound	= 739,
        smErrorSteelParameterNotIS801	= 740,
        smErrorSteelParameterNotIS802	= 741,
        smErrorLoadFactorNotGood	= 742,
        smErrorUserTableNameTruncated	= 743,
        smErrorMaterialNameInvalid	= 744,
        smErrorSteelParameterLdrInvalid	= 745,
        smErrorSteelParameterIrrInvalid	= 746,
        smErrorSteelParameterInoInvalid	= 747,
        smErrorSteelParameterImmInvalid	= 748,
        smErrorSteelParameterCmbInvalid	= 749,
        smErrorSteelParameterDsdInvalid	= 750,
        smErrorConcParameterBtpInvalid	= 751,
        smErrorConcParameterDimInvalid	= 752,
        smErrorConcParameterExpInvalid	= 753,
        smErrorConcParameterCclInvalid	= 754,
        smErrorConcParameterLtcInvalid	= 755,
        smErrorConcParameterCfbInvalid	= 756,
        smErrorConcParameterDsdInvalid	= 757,
        smErrorConcParameterDagInvalid	= 758,
        smErrorConcParameterPssInvalid	= 759,
        smErrorConcParameterDcpInvalid	= 760,
        smErrorConcParameterPhiInvalid	= 761,
        smErrorConcParameterTeqInvalid	= 762,
        smErrorConcParameterMMyInvalid	= 763,
        smErrorConcParameterMMzInvalid	= 764,
        smErrorConcParameterLssInvalid	= 765,
        smErrorConcParameterMoeInvalid	= 766,
        smErrorSteelParameterOvrInvalid	= 767,
        smErrorSteelParameterWMaxInvalid	= 768,
        smErrorSteelParameterFssInvalid	= 769,
        smErrorMemberFireProofingUnknown	= 770,
        smErrorSteelParameterCanInvalid	= 771,
        smErrorSteelParameterSopenInvalid	= 772,
        smErrorSteelParameterEopenInvalid	= 773,
        smErrorTimberParameterCmcInvalid	= 774,
        smErrorTimberParameterCmpInvalid	= 775,
        smErrorTimberParameterCmvInvalid	= 776,
        smErrorTimberParameterCmeInvalid	= 777,
        smErrorTimberParameterCfbInvalid	= 778,
        smErrorTimberParameterCftInvalid	= 779,
        smErrorTimberParameterCfcInvalid	= 780,
        smErrorTimberParameterCfuInvalid	= 781,
        smErrorTimberParameterCrInvalid	= 782,
        smErrorTimberParameterCttInvalid	= 783,
        smErrorTimberParameterChInvalid	= 784,
        smErrorTimberParameterCbInvalid	= 785,
        smErrorTimberParameterKlInvalid	= 786,
        smErrorTimberParameterIndInvalid	= 787,
        smErrorTimberParameterKbeInvalid	= 788,
        smErrorTimberParameterKceInvalid	= 789,
        smErrorTimberParameterKeyInvalid	= 790,
        smErrorTimberParameterKezInvalid	= 791,
        smErrorTimberParameterKbdInvalid	= 792,
        smErrorTimberParameterKbInvalid	= 793,
        smErrorTimberParameterCmbInvalid	= 794,
        smErrorTimberParameterCmtInvalid	= 795,
        smErrorSteelParameterCtyInvalid	= 796,
        smErrorSteelParameterTheInvalid	= 797,
        smErrorSteelParameterEdiInvalid	= 798,
        smErrorSteelParameterDcfInvalid	= 799,
        smErrorSteelParameterCogInvalid	= 800,
        smErrorSteelParameterSpaInvalid	= 801,
        smErrorSteelParameterAxisInvalid	= 802,
        smErrorTimberParameterDMaxInvalid	= 803,
        smErrorTimberParameterDMinInvalid	= 804,
        smErrorDefineRPANotPresent	= 805,
        smErrorRPALoadIgnored	= 806,
        smErrorDefineCFENotPresent	= 807,
        smErrorCFELoadIgnored	= 808,
        smErrorDefineNTCNotPresent	= 809,
        smErrorNTCLoadIgnored	= 810,
        smErrorAccidentalParamIgnored	= 811,
        smErrorConcParameterHlinkInvalid	= 812,
        smErrorSHEParamLinkMinData	= 813,
        smErrorSHEParamLinkMaxData	= 814,
        smErrorSHEParamKSLData	= 815,
        smErrorConcParameterMd1Data	= 816,
        smErrorConcParameterMd2Data	= 817,
        smMD2CommandPlaceAfterMD1	= 818,
        smMD1CommandPlaceAfterMD2	= 819,
        smErrorConcParameterGLDData	= 820,
        smErrorSteelParameterShearInvalid	= 821,
        smErrorSteelParameterShearData	= 822,
        smErrorDeckNotClosed	= 823,
        smErrorDeckNotClosedXtra	= 824,
        smErrorSteelParameterStpInvalid	= 825,
        smErrorSteelParameterStpData	= 826,
        smErrorTimberParameterCvInvalid	= 827,
        smErrorTimberParameterCcInvalid	= 828,
        smErrorTimberParameterSrcInvalid	= 829,
        smErrorTimberParameterSrtInvalid	= 830,
        smErrorPerformImperfectionAnalysisIgnored	= 831,
        smErrorDefineNRCNotPresent	= 832,
        smErrorNRCLoadIgnored	= 833,
        smErrorDefineSnowNotPresent	= 834,
        smErrorSnowLoadIgnored	= 835,
        smErrorInvalidTimberParameter	= 836,
        smErrorSteelParameterData	= 837,
        smErrorSteelParameterInvalid	= 838,
        smErrorPerformPushoverAnalysisIgnored	= 839,
        smErrorEnvelopDefIgnored	= 840,
        smWarningEnvelopNotFound	= 841,
        smErrorAttributeDefIgnored	= 842,
        smErrorGenericInvalidCommand	= 843,
        smErrorGenericInvalidCommandData	= 844,
        smErrorConcParameter	= 845,
        smWarningConstantAndMaterialUsed	= 846,
        smWarningInconsistentDampList	= 847,
        smErrorNonExistentJoint	= 848,
        smErrorInvalidSiteClass	= 849,
        smErrorDefineTurkishNotPresent	= 850,
        smErrorTurkishLoadIgnored	= 851,
        smErrorInvalidDesignParamData	= 852,
        smErrorCheckSoftStoryLoadIgnored	= 853,
        smErrorPerformBucklingAnalysisIgnored	= 854,
        smErrorZIPCodeNotFound	= 855,
        smErrorLatLongNotFound	= 856,
        smErrorMaxCMCore	= 857,
        smErrorMemberCracked	= 858,
        smWarningLoadNotFoundReferenceLoad	= 859,
        smErrorRefLoadFactorNotGood	= 860,
        smErrorRefLoadIgnored	= 861,
        smErrorInvalidCommandSyntaxInSteadyStateBlock	= 862,
        smWarningObsoleteSubstCommandFound	= 863,
        smWarningObsoleteRotCommandFound	= 864,
        smErrorInvalidPushoverParameter	= 865,
        smWarningLoadNotFoundNotionalLoad	= 866,
        smDirectionNotProvidedNotionalLoad	= 867,
        smErrorNotionalLoadIgnored	= 868,
        smWarningNotionalLoadLimit	= 869,
        smErrorSpectrumSNIPA	= 870,
        smErrorSpectrumSNIPDircX	= 871,
        smErrorSpectrumSNIPDircY	= 872,
        smErrorSpectrumSNIPDircZ	= 873,
        smErrorSpectrumSNIPDirc	= 874,
        smErrorSpectrumSNIPSoilType	= 875,
        smErrorSpectrumSNIPDispSpectra	= 876,
        smErrorPerformDAAnalysisIgnored	= 877,
        smErrorInvalidColdformedRA	= 878,
        smFloorSpectrumNoFloorGroupAdded	= 879,
        smFloorSpectrumNoDirectionSpecified	= 880,
        smFloorSpectrumOptionsFLOW	= 881,
        smFloorSpectrumOptionsFHIGH	= 882,
        smFloorSpectrumOptionsFDELTA	= 883,
        smFloorSpectrumOptionsDAMP	= 884,
        smFloorSpectrumOptions	= 885,
        smTimeHistorySpectrumKeyword	= 886,
        smTimeHistoryOptionKeyword	= 887,
        smErrorInvalidNA	= 888,
        smErrorSteelParameterDBSData	= 889,
        smErrorSteelParameterLATData	= 890,
        smErrorSteelParameterPLGData	= 891,
        smErrorSteelParameterTSTData	= 892,
        smErrorSteelParameterLSTData	= 893,
        smDeleteEQCodeParameter	= 894,
        smErrorDefineGB50011Ignored	= 895,
        smErrorDefineGB50011IntensityIgnored	= 896,
        smErrorDefineGB50011InsufficinetData	= 897,
        smErrorDefineGB50011NotPresent	= 898,
        smErrorGB50011LoadIgnored	= 899,
        smErrorSteelParameterMUInvalid	= 900,
        smErrorSteelParameterC3Invalid	= 901,
        smWarningDiffMatSameProp	= 902,
        smErrorSteelParameterKcInvalid	= 903,
        smErrorIS893RSSLoadingParam	= 904,
        smErrorSteelParameterGamMinvalid	= 905,
        smErrorSteelParameterENSGRvalid	= 906,
        smErrorSteelParameterENMainvalid	= 907,
        smErrorSteelParameterMisesvalid	= 908,
        smErrorSteelParameterCMTInvalid	= 909,
        smErrorSteelParameterEFTInvalid	= 910,
        smErrorSteelParameterALHInvalid	= 911,
        smErrorSteelParameterBETInvalid	= 912,
        smWarningPossibleInvalidLoadNo	= 913,
        smWarningPMemberNotFound	= 914,
        smErrorCmdNotSupptedInPhysMode	= 915,
        smErrorSteelParameterLHTInvalid	= 916,
        smErrorSteelParameterLHTData	= 917,
        smWarningSelfWeightList	= 918,
        smErrorFloorHeight	= 919,
        smErrorElementOffset	= 920,
        smErrorFloorDiaphragm	= 921,
        smErrorFloorDiaphragmJointList	= 922,
        smErrorBase	= 923,
        smErrorSteelParameterFSJApiInvalid	= 924,
        smErrorSteelParameterGSTInvalid	= 925,
        smErrorSteelParameterMthInvalid	= 926,
        smErrorECCNotGood	= 927,
        smErrorIGNNotGood	= 928,
        smErrorSteelParameterFabInvalid	= 929,
        smWarningCodeUnknown	= 930,
        smErrorConcParameterLWFInvalid	= 931,
        smErrorSteelParameterPLBInvalid	= 932,
        smErrorInvalidSeismicParameter	= 933,
        smErrorDefineColombian2010Ignored	= 934,
        smErrorDefineColombian2010InsufficinetData	= 936,
        smErrorDefineColombian2010NotPresent	= 937,
        smErrorColombian2010LoadIgnored	= 937,
        smErrorSteelParameterEC3GammaMInvalid	= 938,
        smErrorSteelParameterRus2011TbData	= 939,
        smErrorSteelParameterTrackRussian2011Data	= 940,
        smErrorInvalidSAGMINIMUMCableCommnd	= 941,
        smErrorInvalidSTABILITYCableCommnd	= 942,
        smErrorInvalidKSMALLCableCommnd	= 943,
        smErrorInvalidSTEPSRange	= 944,
        smErrorInvalidEQITERATIONSRange	= 945,
        smErrorInvalidSAGMINIMUMRange	= 946,
        smErrorInvalidKSMALLRange	= 947,
        smErrorInvalidREFORMRange	= 948,
        smErrorInvalidKGEOMRange	= 949,
        smErrorSteelParameterENSGRRussian2011valid	= 950,
        smErrorSteelParameterENMainRussian2011valid	= 951,
        smErrorConcParameterSKZInvalid	= 952,
        smErrorConcParameterSKYInvalid	= 953,
        smErrorConcParameterSWYInvalid	= 954,
        smErrorConcParameterSQZInvalid	= 955,
        smErrorConcParameterSQYInvalid	= 956,
        smErrorConcParameterSLZInvalid	= 957,
        smErrorConcParameterSLYInvalid	= 958,
        smErrorConcParameterBDZInvalid	= 959,
        smErrorConcParameterBDYInvalid	= 960,
        smErrorConcParameterTRNInvalid	= 961,
        smErrorSteelParameterMBGInvalid	= 962,
        smErrorSteelParameterYNGInvalid	= 963,
        smErrorSteelParameterSEIInvalid	= 964,
        smErrorSteelParameterSEIData	= 965,
        smErrorSteelParameterFRMData	= 966,
        smErrorSteelParameterBRCData	= 967,
        smErrorSteelParameterMTYPData	= 968,
        smWarningInitialFrequencyShift	= 969,
        smErrorDefine1893Syntax	= 970,
        smErrorConcParameterRclInvalidRussian2012	= 971,
        smErrorSteelParameterINTData	= 972,
        smErrorCOUNotValid	= 973,
        smErrorOPPNotValid	= 974,
        smErrorSteelParameterCmzSANSInvalid	= 975,
        smErrorSteelParameterCmySANSInvalid	= 976,
        smErrorSteelParameterWTYPData	= 977,
        smErrorSteelParameterTNDData	= 978,
        smErrorSteelParameterANGData	= 979,
        smErrorSteelParameterNBLData	= 980,
        smErrorSteelParameterFXTYData	= 981,
        smWarningFloorLoadSyntax	= 982,
        smWarningOnewayLoadSyntax	= 983,
        smWarningSetMeshTolerance	= 984,
        smWarningDesignCodeNotActivated	= 985,
        smErrorCurvedMemberData	= 986,
        smErrorSteelParameterNotNZS3404	= 987,
        smErrorSteelParameterDUCTData	= 988,
        smErrorSteelParameterDUCTInvalid	= 989,
        smErrorSteelParameterGLDData	= 990,
        smErrorSteelParameterGLDInvalid	= 991,
        smErrorDominantSigned	= 992,
        smErrorSteelParameterSGRNewRange	= 993,
        smErrorConcParameterRDO	= 994,
        smErrorConcParameterRCOAT	= 995,
        smErrorConcParameterMREB	= 996,
        smErrorConcParameterTREB	= 997,
        smErrorConcParameterMXM	= 998,
        smErrorConcParameterMIM	= 999,
        smErrorConcParameterTARS	= 1000,
        smErrorConcParameterSTRS	= 1001,
        smErrorConcParameterEDSP	= 1002,
        smErrorConcParameterLATR	= 1003,
        smErrorConcParameterXBRC	= 1004,
        smErrorConcParameterYBRC	= 1005,
        smErrorConcParameterSEM	= 1006,
        smErrorStartingLoadIgnored	= 1007,
        smErrorStartingLoadNgenVal	= 1008,
        smErrorCurvedMemberIncludedAngleInvalid	= 1009,
        smErrorCurvedMemberRadiusInvalid	= 1010,
        smErrorCurvedMemberLengthToRadiusInvalid	= 1011,
        smErrorSteelParameterNCRData	= 1012,
        smErrorSteelParameterNCRInvalid	= 1013,
        smErrorSteelParameterLbrcData	= 1014,
        smErrorSteelParameterLbrcInvalid	= 1015,
        smErrorSteelParameterTbrcData	= 1016,
        smErrorSteelParameterTbrcInvalid	= 1017,
        smErrorSteelParameterUnrInvalid	= 1018,
        smErrorSteelParameterMTYPAISC16Data	= 1019,
        smErrorSteelParameterTNDAISC16Data	= 1020,
        smErrorSteelParameterSGRAISC16Data	= 1021,
        smErrorSteelParameterDUCTAISC16Data	= 1022,
        smErrorSteelParameterNBRCInvalid	= 1023,
        smErrorSteelParameterBStiffInvalid	= 1024,
        smErrorSteelParameterTStiffInvalid	= 1025,
        smErrorSteelParameterSOEInvalid	= 1026,
        smErrorSteelParameterSOEData	= 1027,
        smErrorSteelParameterTFAInvalid	= 1028,
        smErrorSteelParameterTFAData	= 1029,
        smWarningRetireSURFACE	= 1030,
        smWarningDeprecatedDesignCodes	= 1031,
        smErrorElementDesignCommandIgnored	= 1032,
        smWarningDAParameterPDITERInvalid	= 1033,
        smWarningDAParameterREDUCEDEIInvalid	= 1034,
        smWarningTooManyDispSpringPairs	= 1035,
        smErrorCheckIrregularitiesDefnInvalid	= 1036,
        smErrorSNiPWindDefnCommandInvalid	= 1037,
        smErrorSteelParameterRussianGamFinvalid	= 1038,
        smErrorConcParameterAlpha	= 1039,
        smErrorDynWindLoadDirNeg	= 1040,
        smErrorWindDefinition2011	= 1041,
        smErrorWindOblique	= 1042,
        smErrorSNiPWindClassification	= 1043,
        smErrorRussian2017Unl	= 1044,
        smWarningPdeltaConverge	= 1045,
        smErrorSteelParameterTOMInvalid	= 1046,
        smErrorSteelParameterTmainCSAS16Invalid	= 1047,
        smErrorSteelParameterMainCSAS16Invalid	= 1048,
        smErrorSteelParameterTrackRussian2017Data	= 1049,
        smErrorSteelParameterSLFData	= 1050,
        smWarningDeprecatedFYLD	= 1051,
        smErrorInvalidSTEPSRangeAdv	= 1052,
        smErrorSteelParameterSRTInvalid	= 1053,
        smErrorSteelParameterSRTData	= 1054,
        smErrorSteelParameterTSLInvalid	= 1055,
        smErrorSteelParameterTSLNegativeData	= 1056,
        smErrorNotDefineEnclosedZoneCommand	= 1057,
        smErrorExternalUserTableFileNotFound	= 1058,
        scErrorDefineMaterialFyFuInvalid	= 1059,
        scErrorDefineMaterialRyRtInvalid	= 1060,
        scErrorDefineMaterialFcuInvalid	= 1061,
        smErrorSteelParameterE5PInvalid	= 1062,
        smErrorSteelParameterE5PData	= 1063,
        smWarningDiffMatSamePropPhysical	= 1064,
        smWarningChineseWindOldVersion	= 1065,
        smErrorSeismicParameterSTNotFound	= 1066,
        smWarningWindParameterMissing	= 1067,
        smErrorSteelParameterTrack0to4Data	= 1068,
        smErrorBeamNotValidByDuplicateNode	= 1069,
        smErrorPlateNotValidByDuplicateNode	= 1070,
        smErrorSolidNotValidByDuplicateNode	= 1071,
        smWarningPMemberNotCollinear	= 1072,
        smWarningPMemberNotConnected	= 1073,
        smWarningPMemberOverlapping	= 1074,
        smWarningBeamInOtherPMember	= 1075,
        smWarningCoordinateSignificantFigures	= 1076,
        smErrorListKeywordMissing	= 1077,
        smWarningNumberOutofRangeJoint	= 1078,
        smWarningNumberOutofRangeMember	= 1079,
        smWarningNumberOutofRangePlate	= 1080,
        smWarningNumberOutofRangeSolid	= 1081,
        smErrorInvalidImrOptions	= 1082,
        smErrorInvalidImrLoadCases	= 1083,
        smWarningTimeLoadFXAssumed	= 1084,
        smWarningMultiIgnoreMemberCmdFound	= 1085,
        smWarningMultiIgnoreLoadCmdFound	= 1086,
        smErrorInvalidScaleTimeHistory	= 1087,
        smErrorWindTypeDefinitionParamsNotFound	= 1088,
        smErrorInvalidEnclosedZoneProvided	= 1089,
        smErrorEnclosedZoneNotProvided	= 1090,
        smWarningInvalidEnclosedZoneLoadDirection	= 1091,
        smErrorInvalidEnclosedZoneLoadData	= 1092,
        smWarningInvalidEnclosedZoneWeightDirection	= 1093,
        smWarningDeprecatedConnectProject	= 1094,
        smErrorEnclosedZoneNameExceedsMaxLen	= 1095,
        smErrorEnclosedZoneNameInvalidChar	= 1096,
        smErrorConcParameterLWFInvalidForACI31814	= 1097,
        smWarningMultiBoundaryCmdFound	= 1098,
        smWarningDuplicateOpeningCmdFound	= 1099,
        smErrorNotSupportedHigherVersionOfDatabase	= 1100,
        smErrorInvalidFormatOfDatabase	= 1101,
        smErrorJointCoordinateAfterChange	= 1102,
        smErrorDuplicateMemberIncidenceCmdFound	= 1103,
        smErrorSteelParameterH36Invalid	= 1104,
        smErrorSteelParameterH36Data	= 1105,
        smWarningEnclosedZoneLoadIgnoreInPlaneComponent	= 1106,
        smErrorSteelParameterLkpInvalid	= 1107,
        smWarningEnclosedZoneWeightIgnoreInPlaneComponent	= 1108,
        smErrorWindLoadNotSupportedInZup	= 1109,
        smErrorFloorGroupNonCoPlanar	= 1110,
        smWarningDuplicatePrimaryLoadInLoadComb	= 1111,
        smWarningSSAndS1ValuesNotProvidedForIBC2018	= 1112,
        smErrorSteelParameterSTWOInvalid	= 1113,
        smErrorSteelParameterFMAINInvalid	= 1114,
        smErrorSteelParameterBRXInvalid	= 1115,
        smErrorSteelParameterBRYInvalid	= 1116,
        smErrorSteelParameterFRMInvalidAISC36022	= 1117,
        smErrorSteelParameterMTHInvalidIS800LSD	= 1118,
        smErrorSteelParameter_Fsc_Invalid	= 1119,
        smErrorSteelParameter_Crf_Invalid	= 1120,
        smErrorSteelParameter_Sc_Invalid	= 1121,
        smErrorSteelParameter_Rfr_Invalid	= 1122,
        smErrorSteelParameter_Rfs_Invalid	= 1123,
        smErrorSteelParameter_Cfdf_Invalid	= 1124,
        smErrorSteelParameter_Lb_Invalid	= 1125,
        smErrorSteelParameter_Lc_Invalid	= 1126,
        smErrorSteelParameterNotAsNzs4600_18	= 1127,
        smErrorSteelParameterSGR_AsNzsData	= 1128,
        smErrorSteelParameter_Axis_AsNzs_Invalid	= 1129,
        smErrorSteelParameterTFADataAISC22	= 1130,
        smErrorInvalidCommandSyntaxInPrintHarmonicDisplacements	= 1131,
        smWarningSSAndS1ValuesNotProvidedForResponseSpectrumWithIBC2018	= 1132,
        smErrorLoadCannotReferItself	= 1133,
        smErrorSteelParameterDfhInvalid	= 1134,
        smErrorSteelParameterDfhData	= 1135,
        smErrorSteelParameterDjy1Invalid	= 1136,
        smErrorSteelParameterDjy1Data	= 1137,
        smErrorSteelParameterDjz1Invalid	= 1138,
        smErrorSteelParameterDjz1Data	= 1139,
        smErrorSteelParameterDjy2Invalid	= 1140,
        smErrorSteelParameterDjy2Data	= 1141,
        smErrorSteelParameterDjz2Invalid	= 1142,
        smErrorSteelParameterDjz2Data	= 1143,
        smErrorSteelParameterSsdInvalid	= 1144,
        smErrorSteelParameterSsdData	= 1145,
        smErrorTaubRange	= 1146
    } 	StadMess;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0004
    {
        smStarFileHeaderBorder	= 2,
        smStarFileHeaderName	= 3,
        smStarFileHeaderDate	= 4,
        smStarFileHeaderTime	= 5,
        smStarFileHeaderError	= 6,
        smStarDeckCommandIgnored	= 7,
        smStarRenumberCommandIgnored	= 8,
        smStarNodeHeadCommandIgnored	= 9,
        smStarEofCommandIgnored	= 10,
        smStarNewsysaCommandIgnored	= 11,
        smStarNewsysjCommandIgnored	= 12,
        smStarNewsysCommandIgnored	= 13,
        smStarCommandIgnored	= 14,
        smStarQuadwarnCommandIgnored	= 15,
        smStarBeamheadCommandIgnored	= 16,
        smStarBeamCreationBug	= 17,
        smStarTriaheadCommandIgnored	= 18,
        smStarTriaCreationBug	= 19,
        smStarQuadheadCommandIgnored	= 20,
        smStarQuadCreationBug	= 21,
        smStarCubeheadCommandIgnored	= 22,
        smStarCubeCreationBug	= 23,
        smStarMaddheadCommandIgnored	= 24,
        smStarMaddxincCommandIgnored	= 25,
        smStarMaddxCommandIgnored	= 26,
        smStarMaddprntCommandIgnored	= 27,
        smStarMaddfactCommandIgnored	= 28,
        smStarMaddminCommandIgnored	= 29,
        smStarMaddelCommandIgnored	= 30,
        smStarEndelCommandIgnored	= 31,
        smStarMassheadCommandIgnored	= 32,
        smStarMassxincCommandIgnored	= 33,
        smStarMassxCommandIgnored	= 34,
        smStarMassprntCommandIgnored	= 35,
        smStarMassfactCommandIgnored	= 36,
        smStarMassminCommandIgnored	= 37,
        smStarMasselCommandIgnored	= 38,
        smStarAsysheadCommandIgnored	= 39,
        smStarAsysgCommandIgnored	= 40,
        smStarBounheadCommandIgnored	= 41,
        smStarBoungaddCommandIgnored	= 42,
        smStarBoungCommandIgnored	= 43,
        smStarGuyaheadCommandIgnored	= 44,
        smStarGuyanaddCommandIgnored	= 45,
        smStarGuyanCommandIgnored	= 46,
        smStarRestgheadCommandIgnored	= 47,
        smStarSupportNodeNotFound	= 48,
        smStarWghtheadCommandIgnored	= 49,
        smStarWghtfactCommandIgnored	= 50,
        smStarWghtgenCommandIgnored	= 51,
        smStarWghtgCommandIgnored	= 52,
        smStarWghtCommandIgnored	= 53,
        smStarWghtdeckCommandIgnored	= 54,
        smStarMatlheadCommandIgnored	= 55,
        smStarMatlg3CommandIgnored	= 58,
        smStarMatlg4CommandIgnored	= 59,
        smStarMatlg5CommandIgnored	= 60
    } 	StarMess;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0005
    {
        sfStartFormat	= 2,
        sfStartFree	= 3,
        sfRenumbFormat	= 4,
        sfRenumbFree	= 5,
        sfMatlg1Format	= 6,
        sfMatlg1Free	= 7,
        sfMatlg2Format	= 8,
        sfMatlg2Free	= 9,
        sfMatlg3Format	= 10,
        sfMatlg3Free	= 11,
        sfMatlg4Format	= 12,
        sfMatlg4Free	= 13,
        sfMatlg5Format	= 14,
        sfMatlg5Free	= 15,
        sfMatlg6Format	= 16,
        sfMatlg6Free	= 17,
        sfMatlgFormat	= 18,
        sfMatlgFree	= 19,
        sfNodeFormat	= 20,
        sfNodeFree	= 21,
        sfNodefactFormat	= 22,
        sfNodefactFree	= 23,
        sfNodegFormat	= 24,
        sfNodegFree	= 25,
        sfNodegrd1Format	= 26,
        sfNodegrd1Free	= 27,
        sfNodegrd2Format	= 28,
        sfNodegrd2Free	= 29,
        sfNewsysFormat	= 30,
        sfNewsysFree	= 31,
        sfNewsysaFormat	= 32,
        sfNewsysaFree	= 33,
        sfNewsysjFormat	= 34,
        sfNewsysjFree	= 35,
        sfHeadFormat	= 36,
        sfHeadFree	= 37,
        sfBeamtprFormat	= 38,
        sfBeamtprFree	= 39,
        sfBeamgFormat	= 40,
        sfBeamgFree	= 41,
        sfBeamiFormat	= 42,
        sfBeamiFree	= 43,
        sfBrectFormat	= 44,
        sfBrectFree	= 45,
        sfElbowFormat	= 46,
        sfElbowFree	= 47,
        sfPipegFormat	= 48,
        sfPipegFree	= 49,
        sfPipetFormat	= 50,
        sfPipetFree	= 51,
        sfPiptaprFormat	= 52,
        sfPiptaprFree	= 53,
        sfTriabFormat	= 54,
        sfTriabFree	= 55,
        sfQuadFormat	= 56,
        sfQuadFree	= 57,
        sfCubegFormat	= 58,
        sfCubegFree	= 59,
        sfRestgFormat	= 60,
        sfRestgFree	= 61
    } 	StarFormat;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0006
    {
        In	= 0,
        Ft	= 1,
        foot	= 2,
        Cm	= 3,
        M	= 4,
        Mm	= 5,
        Dm	= 6,
        Km	= 7,
        Yd	= 8,
        mil	= 9
    } 	LengthUnit;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0007
    {
        Kip	= 0,
        lb	= 1,
        kg	= 2,
        MTon	= 3,
        N	= 4,
        KN	= 5,
        MN	= 6,
        DN	= 7
    } 	ForceUnit;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0008
    {
        mph	= 0,
        metersec	= 1,
        cmsec	= 2,
        mmsec	= 3,
        kmph	= 4,
        inchsec	= 5,
        ftsec	= 6,
        yardsec	= 7
    } 	VelocityUnit;

typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0009
    {
        STRENGTH_TYPE_NONE	= 0,
        NORMAL_ASD_WORKING_STRESS_FORCES_WITHOUT_P_DELTA	= 1,
        NORMAL_ASD_WORKING_STRESS_FORCES_WITH_P_DELTA	= 2,
        STRENGTH_TYPE_OF_FORCES_WITHOUT_P_DELTA	= 3,
        STRENGTH_TYPE_OF_FORCES_WITH_P_DELTA	= 4,
        COLUMN_ONLY_STRENGTH_TYPE_OF_FORCES_WITHOUT_P_DELTA	= 5,
        COLUMN_ONLY_STRENGTH_TYPE_OF_FORCES_WITH_P_DELTA	= 6
    } 	StrengthType;

typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0010
    {
        ASCE7Y95	= 0,
        ASCE7Y02	= 1,
        ASCE7Y05_10	= 2,
        AISC97	= 3,
        AISC02	= 4
    } 	LoadingCode;

typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0011
    {
        TypeI	= 0,
        TypeII	= 1,
        TypeIII	= 2,
        TypeIV	= 3
    } 	BldgClass;

typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0012
    {
        Building	= 0,
        Chimney	= 1,
        Solidsign	= 2,
        Opensign	= 3,
        Laticeframe	= 4,
        Trusstower	= 5
    } 	BldgType;

typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0013
    {
        ExpA	= 0,
        ExpB	= 1,
        ExpC	= 2,
        ExpD	= 3
    } 	ExposureCategory;

typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0014
    {
        WindWard	= 0,
        LeeWard	= 1,
        SideWall	= 2
    } 	WallType;

typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0015
    {
        ASCE7Y1995	= 0,
        ASCE7Y2002	= 1,
        ASCE7Y2010	= 2,
        ASCE7Y2016	= 3,
        ASCEY_LAST	= 4
    } 	ASCE7_Publication;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0016
    {
        Iterative	= 0,
        Eigen	= 1
    } 	BucklingAnalysisMethod;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0017
    {
        FLEX	= 0,
        FYLD	= 1,
        AXIAL	= 2
    } 	DirectAnalysisCommand;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0018
    {
        LRFD	= 1,
        ASD	= 2
    } 	DirectAnalysisOption;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0019
    {
        ASCE7_2D_RIDGE	= 0,
        ASCE7_2D_ESCARPMENT	= 1,
        ASCE7_3D_AXISYM_HILL	= 2
    } 	ASCE7_EscarpmentType;

typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0020
    {
        ASCE7_TypeI	= 0,
        ASCE7_TypeII	= 1,
        ASCE7_TypeIII	= 2,
        ASCE7_TypeIV	= 3
    } 	ASCE7_BldgClass;

typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0021
    {
        ASCE7_Building	= 0,
        ASCE7_Chimney	= 1,
        ASCE7_Solidsign	= 2,
        ASCE7_Opensign	= 3,
        ASCE7_Laticeframe	= 4,
        ASCE7_Trusstower	= 5
    } 	ASCE7_BldgType;

typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0022
    {
        ASCE7_ExpA	= 0,
        ASCE7_ExpB	= 1,
        ASCE7_ExpC	= 2,
        ASCE7_ExpD	= 3
    } 	ASCE7_ExposureCategory;

typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0023
    {
        ASCE7_WindWard	= 0,
        ASCE7_LeeWard	= 1,
        ASCE7_SideWall	= 2
    } 	ASCE7_WallType;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0024
    {
        ASCE7_Bldg_Open	= 0,
        ASCE7_Bldg_PartOpen	= 1,
        ASCE7_Bldg_PartEnclosed	= 2,
        ASCE7_Bldg_Enclosed	= 3
    } 	ASCE7_BuildingEnclosureType;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0025
    {
        ASCE7_Tank_Square	= 0,
        ASCE7_Tank_SquareDiagonal	= 1,
        ASCE7_Tank_Hexa_Octa	= 2,
        ASCE7_Tank_Round	= 3,
        ASCE7_Tank_Octa_Axs	= 4,
        ASCE7_Tank_Round_Axs	= 5
    } 	ASCE7_ChimneyCrossSectionType;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0026
    {
        ASCE7_OPLF_Flat	= 0,
        ASCE7_OPLF_Rounded	= 1
    } 	ASCE7_OPLFWOrientationType;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0027
    {
        ASCE7_TT_Triangle	= 0,
        ASCE7_TT_Square	= 1
    } 	ASCE7_TrussTowerCrossSectionType;

typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0028
    {
        DA_FLEX	= 0,
        DA_FYLD	= 1,
        DA_AXIAL	= 2
    } 	DirectAnalysisParameterTypes;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0029
    {
        NodeEntity	= 0,
        MemberEntity	= 1,
        PlateEntity	= 2,
        SolidEntity	= 3,
        SurfaceEntity	= 4,
        PhysicalMemberEntity	= 5,
        ParametricSurface	= 6,
        EndEntity	= 7
    } 	STAADEntityType;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0030
    {
        InvalidInputFile	= 0xfffffffe,
        AnalysisTerminated	= 0xffffffff,
        AnalysisRunning	= 1,
        AnalysisSuccessful	= 2,
        AnalysisCompletedWithWarningsOnly	= 3,
        AnalysisCompletedWithErrors	= 4,
        AnalysisNotPerformed	= 5
    } 	AnalysisStatusType;

typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0031
    {
        bmp	= 0,
        jpg	= 1,
        tga	= 2,
        tif	= 3,
        ImageType_LAST	= 4
    } 	ImageExportTypes;

typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0032
    {
        STEELDESIGNCODEUNKNOWN	= 0,
        STEELDESIGNAASHTO	= 1001,
        STEELDESIGNAISC	= 1002,
        STEELDESIGNAUSTRALIAN	= 1003,
        STEELDESIGNBS5950	= 1004,
        STEELDESIGNBS5400	= 1005,
        STEELDESIGNCANADIAN	= 1006,
        STEELDESIGNFRENCH	= 1007,
        STEELDESIGNDIN18800	= 1008,
        STEELDESIGNIS800	= 1009,
        STEELDESIGNJAPANESE	= 1010,
        STEELDESIGNLRFD	= 1011,
        STEELDESIGNNS3472	= 1012,
        STEELDESIGNSPANISH	= 1013,
        STEELDESIGNNPD	= 1014,
        STEELDESIGNBSK90	= 1015,
        STEELDESIGNAPI	= 1016,
        STEELDESIGNEC3	= 1017,
        STEELDESIGNCHINESE	= 1018,
        STEELDESIGNDS412	= 1019,
        STEELDESIGNASCE	= 1020,
        STEELDESIGNB7	= 1021,
        STEELDESIGNBSK99	= 1022,
        STEELDESIGNDUTCH	= 1023,
        STEELDESIGNCYPRUS	= 1024,
        STEELDESIGNRUSSIAN	= 1025,
        STEELDESIGNAISI	= 1026,
        STEELDESIGNS136	= 1027,
        STEELDESIGNIS801	= 1028,
        STEELDESIGNIS802	= 1029,
        STEELDESIGNMEXICANLRFD	= 1030,
        STEELDESIGNEGYPTIAN	= 1031,
        STEELDESIGNIS800LSD	= 1032,
        STEELDESIGNIS800WSD	= 1052,
        STEELDESIGNBS5950_COLD	= 1033,
        STEELDESIGNSOUTHAFRICAN	= 1034,
        STEELDESIGNAISC_RECO	= 1035,
        STEELDESIGNCANADIANRECO_1994	= 1036,
        STEELDESIGNCANADIANRECO_2001	= 1037,
        STEELDESIGNAISI_2001RCECO	= 1038,
        STEELDESIGNAISI_1999RCECO	= 1039,
        STEELDESIGNAISI_1996RCECO	= 1040,
        STEELDESIGNS136_2001RCECO	= 1041,
        STEELDESIGNS136_1999RCECO	= 1042,
        STEELDESIGNS136_1996RCECO	= 1043,
        STEELDESIGNAASHTO_LRFD	= 1044,
        STEELDESIGNAISC_UNIFIED	= 1045,
        STEELDESIGNNF3000_1989	= 1046,
        STEELDESIGNNF3000_1998	= 1047,
        STEELDESIGNNF3000_1974	= 1048,
        STEELDESIGNNF3000_1977	= 1049,
        STEELDESIGNNORSOK	= 1050,
        STEELDESIGNNF3000_2004	= 1051,
        STEELDESIGNNF3000_2001	= 1053,
        STEELDESIGNASCE52	= 1060,
        STEELDESIGNAISC_UNIFIED_2010	= 1061,
        STEELDESIGNCANADIANS16_09	= 1062,
        STEELDESIGNRUSSIAN2011	= 1063,
        STEELDESIGNSOUTHAFRICAN1993	= 1064,
        STEELDESIGNCANADIANS16_14	= 1065,
        STEELDESIGNNZS3404_1997	= 1066,
        STEELDESIGNAISC_UNIFIED_2016	= 1067,
        STEELDESIGNAISI_S100_2016	= 1068,
        STEELDESIGNCANADIANS16_19	= 1069,
        STEELDESIGNAISC_CAST	= 1102,
        STEELDESIGNBS5950_1990	= 1104,
        STEELDESIGNLRFD_CAST	= 1111,
        STEELDESIGNAISC_N690	= 1202,
        STEELDESIGNEC3BRITISH	= 1203,
        STEELDESIGNAISCN690_1984	= 1204,
        STEELDESIGNJAPANESE2005	= 1210,
        STEELDESIGNEN1993	= 1220,
        STEELDESIGNRUSSIAN2017	= 1221,
        EARTHQUAKEDESIGNEC8	= 1230,
        CONCRETEDESIGNACI	= 2001,
        CONCRETEDESIGNBS8110	= 2002,
        CONCRETEDESIGNBS8007	= 2003,
        CONCRETEDESIGNCANADIAN	= 2004,
        CONCRETEDESIGNFRENCH	= 2005,
        CONCRETEDESIGNDIN1045	= 2006,
        CONCRETEDESIGNINDIAN	= 2007,
        CONCRETEDESIGNJAPANESE	= 2008,
        CONCRETEDESIGNNS3473	= 2009,
        CONCRETEDESIGNEC2	= 2010,
        CONCRETEDESIGNCHINESE	= 2011,
        CONCRETEDESIGNSPANISH	= 2012,
        CONCRETEDESIGNDUTCH	= 2013,
        CONCRETEDESIGNCYPRUS	= 2014,
        CONCRETEDESIGNRUSSIAN	= 2015,
        CONCRETEDESIGNAUSTRALIAN	= 2016,
        CONCRETEDESIGNSWEDISH	= 2017,
        CONCRETEDESIGNFINNISH	= 2018,
        CONCRETEDESIGNIS13920	= 2019,
        CONCRETEDESIGNMEXICAN	= 2020,
        CONCRETEDESIGNDANISH	= 2021,
        CONCRETEDESIGNSOUTHAFRICAN	= 2022,
        CONCRETEDESIGNCP65	= 2023,
        CONCRETEDESIGNACI1999	= 2024,
        CONCRETEDESIGNACI2002	= 2025,
        CONCRETEDESIGNACI2005	= 2026,
        CONCRETEDESIGNACI2008	= 2027,
        CONCRETEDESIGNRUSSIAN2012	= 2028,
        CONCRETEDESIGNACI2011	= 2029,
        CONCRETEDESIGNIS13920_1993	= 2030,
        TIMBERDESIGN	= 3001,
        AITCTIMBERDESIGN	= 3002,
        EC5TIMBERDESIGN	= 3003,
        CANADIANTIMBERDESIGN	= 3004,
        ALUMINUMDESIGN	= 4001,
        CANADIANALUMINUMDESIGN	= 4002,
        FOOTINGDESIGNAMERICAN	= 5001,
        SHEARWALLDESIGNACI	= 6001,
        SHEARWALLDESIGNBS8110	= 6002,
        SHEARWALLDESIGNINDIAN	= 6003
    } 	DesignCodes;

typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0033
    {
        nodalLoad	= 3110,
        memberLoadUni	= 3210,
        memberLoadUniMom	= 3220,
        memberLoadCon	= 3230,
        memberLoadConMom	= 3240,
        memberLoadLinear	= 3250,
        memberLoadTrap	= 3260,
        elementLoadPressure	= 3310,
        elementLoadJoint	= 3311,
        elementLoadJtPressure	= 3312,
        elementLoadTrap	= 3320,
        elementLoadFacePressure	= 3322
    } 	LoadType;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0034
    {
        pinnedSupport	= 1,
        fixedSupport	= 2,
        fixedButSupport	= 3,
        enforcedSupport	= 4,
        enforcedButSupport	= 5
    } 	SupportType;

typedef /* [public][public][public][public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0035
    {
        dispResponse	= 0,
        velResponse	= 1,
        acclResponse	= 2
    } 	TimeHistoryResponseType;

typedef /* [public][public][public][public] */ 
enum __MIDL___MIDL_itf_IDLD52F_0000_0000_0036
    {
        Fx	= 1,
        Fy	= 2,
        Fz	= 3,
        Mx	= 4,
        My	= 5,
        Mz	= 6
    } 	DegreesOfFreedom;


EXTERN_C const IID LIBID_OpenSTAADUI;

#ifndef __IOSStringList_DISPINTERFACE_DEFINED__
#define __IOSStringList_DISPINTERFACE_DEFINED__

/* dispinterface IOSStringList */
/* [uuid] */ 


EXTERN_C const IID DIID_IOSStringList;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("A1FFC9DC-673D-4372-97DA-6685088B90E2")
    IOSStringList : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct IOSStringListVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IOSStringList * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IOSStringList * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IOSStringList * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IOSStringList * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IOSStringList * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IOSStringList * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IOSStringList * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } IOSStringListVtbl;

    interface IOSStringList
    {
        CONST_VTBL struct IOSStringListVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOSStringList_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOSStringList_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOSStringList_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOSStringList_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IOSStringList_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IOSStringList_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IOSStringList_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __IOSStringList_DISPINTERFACE_DEFINED__ */


#ifndef __IOSMemberSteelDgnResults_DISPINTERFACE_DEFINED__
#define __IOSMemberSteelDgnResults_DISPINTERFACE_DEFINED__

/* dispinterface IOSMemberSteelDgnResults */
/* [uuid] */ 


EXTERN_C const IID DIID_IOSMemberSteelDgnResults;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("39768435-3463-4403-A445-E3611AAB8A22")
    IOSMemberSteelDgnResults : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct IOSMemberSteelDgnResultsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IOSMemberSteelDgnResults * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IOSMemberSteelDgnResults * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IOSMemberSteelDgnResults * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IOSMemberSteelDgnResults * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IOSMemberSteelDgnResults * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IOSMemberSteelDgnResults * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IOSMemberSteelDgnResults * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } IOSMemberSteelDgnResultsVtbl;

    interface IOSMemberSteelDgnResults
    {
        CONST_VTBL struct IOSMemberSteelDgnResultsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOSMemberSteelDgnResults_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOSMemberSteelDgnResults_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOSMemberSteelDgnResults_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOSMemberSteelDgnResults_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IOSMemberSteelDgnResults_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IOSMemberSteelDgnResults_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IOSMemberSteelDgnResults_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __IOSMemberSteelDgnResults_DISPINTERFACE_DEFINED__ */


#ifndef __IOSMemberSteelDgnParams_DISPINTERFACE_DEFINED__
#define __IOSMemberSteelDgnParams_DISPINTERFACE_DEFINED__

/* dispinterface IOSMemberSteelDgnParams */
/* [uuid] */ 


EXTERN_C const IID DIID_IOSMemberSteelDgnParams;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("F40BDCDA-B3DE-495C-B84F-790F4456137F")
    IOSMemberSteelDgnParams : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct IOSMemberSteelDgnParamsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IOSMemberSteelDgnParams * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IOSMemberSteelDgnParams * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IOSMemberSteelDgnParams * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IOSMemberSteelDgnParams * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IOSMemberSteelDgnParams * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IOSMemberSteelDgnParams * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IOSMemberSteelDgnParams * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } IOSMemberSteelDgnParamsVtbl;

    interface IOSMemberSteelDgnParams
    {
        CONST_VTBL struct IOSMemberSteelDgnParamsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOSMemberSteelDgnParams_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOSMemberSteelDgnParams_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOSMemberSteelDgnParams_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOSMemberSteelDgnParams_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IOSMemberSteelDgnParams_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IOSMemberSteelDgnParams_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IOSMemberSteelDgnParams_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __IOSMemberSteelDgnParams_DISPINTERFACE_DEFINED__ */


#ifndef __IOSChineseSteelDgnResults_DISPINTERFACE_DEFINED__
#define __IOSChineseSteelDgnResults_DISPINTERFACE_DEFINED__

/* dispinterface IOSChineseSteelDgnResults */
/* [uuid] */ 


EXTERN_C const IID DIID_IOSChineseSteelDgnResults;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("803219FF-6FF9-4056-BDBD-ACD1D061610D")
    IOSChineseSteelDgnResults : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct IOSChineseSteelDgnResultsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IOSChineseSteelDgnResults * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IOSChineseSteelDgnResults * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IOSChineseSteelDgnResults * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IOSChineseSteelDgnResults * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IOSChineseSteelDgnResults * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IOSChineseSteelDgnResults * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IOSChineseSteelDgnResults * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } IOSChineseSteelDgnResultsVtbl;

    interface IOSChineseSteelDgnResults
    {
        CONST_VTBL struct IOSChineseSteelDgnResultsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOSChineseSteelDgnResults_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOSChineseSteelDgnResults_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOSChineseSteelDgnResults_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOSChineseSteelDgnResults_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IOSChineseSteelDgnResults_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IOSChineseSteelDgnResults_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IOSChineseSteelDgnResults_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __IOSChineseSteelDgnResults_DISPINTERFACE_DEFINED__ */


#ifndef __IOSChineseSteelDgnParameters_DISPINTERFACE_DEFINED__
#define __IOSChineseSteelDgnParameters_DISPINTERFACE_DEFINED__

/* dispinterface IOSChineseSteelDgnParameters */
/* [uuid] */ 


EXTERN_C const IID DIID_IOSChineseSteelDgnParameters;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("4BBC157D-251A-47BD-8A1D-C92710D7E74D")
    IOSChineseSteelDgnParameters : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct IOSChineseSteelDgnParametersVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IOSChineseSteelDgnParameters * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IOSChineseSteelDgnParameters * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IOSChineseSteelDgnParameters * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IOSChineseSteelDgnParameters * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IOSChineseSteelDgnParameters * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IOSChineseSteelDgnParameters * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IOSChineseSteelDgnParameters * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } IOSChineseSteelDgnParametersVtbl;

    interface IOSChineseSteelDgnParameters
    {
        CONST_VTBL struct IOSChineseSteelDgnParametersVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOSChineseSteelDgnParameters_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOSChineseSteelDgnParameters_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOSChineseSteelDgnParameters_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOSChineseSteelDgnParameters_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IOSChineseSteelDgnParameters_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IOSChineseSteelDgnParameters_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IOSChineseSteelDgnParameters_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __IOSChineseSteelDgnParameters_DISPINTERFACE_DEFINED__ */


#ifndef __IOSChineseSteelCheckOption_DISPINTERFACE_DEFINED__
#define __IOSChineseSteelCheckOption_DISPINTERFACE_DEFINED__

/* dispinterface IOSChineseSteelCheckOption */
/* [uuid] */ 


EXTERN_C const IID DIID_IOSChineseSteelCheckOption;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("B5A5F42C-16BB-4CB2-B750-5F9888E7A048")
    IOSChineseSteelCheckOption : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct IOSChineseSteelCheckOptionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IOSChineseSteelCheckOption * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IOSChineseSteelCheckOption * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IOSChineseSteelCheckOption * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IOSChineseSteelCheckOption * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IOSChineseSteelCheckOption * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IOSChineseSteelCheckOption * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IOSChineseSteelCheckOption * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } IOSChineseSteelCheckOptionVtbl;

    interface IOSChineseSteelCheckOption
    {
        CONST_VTBL struct IOSChineseSteelCheckOptionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOSChineseSteelCheckOption_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOSChineseSteelCheckOption_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOSChineseSteelCheckOption_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOSChineseSteelCheckOption_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IOSChineseSteelCheckOption_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IOSChineseSteelCheckOption_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IOSChineseSteelCheckOption_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __IOSChineseSteelCheckOption_DISPINTERFACE_DEFINED__ */


#ifndef __IOpenSTAADUI_DISPINTERFACE_DEFINED__
#define __IOpenSTAADUI_DISPINTERFACE_DEFINED__

/* dispinterface IOpenSTAADUI */
/* [uuid] */ 


EXTERN_C const IID DIID_IOpenSTAADUI;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("3F5B8055-31C6-446E-8BED-FEE43E09D4CC")
    IOpenSTAADUI : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct IOpenSTAADUIVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IOpenSTAADUI * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IOpenSTAADUI * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IOpenSTAADUI * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IOpenSTAADUI * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IOpenSTAADUI * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IOpenSTAADUI * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IOpenSTAADUI * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } IOpenSTAADUIVtbl;

    interface IOpenSTAADUI
    {
        CONST_VTBL struct IOpenSTAADUIVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpenSTAADUI_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpenSTAADUI_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpenSTAADUI_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpenSTAADUI_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IOpenSTAADUI_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IOpenSTAADUI_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IOpenSTAADUI_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __IOpenSTAADUI_DISPINTERFACE_DEFINED__ */


#ifndef __IOSGeometryUI_DISPINTERFACE_DEFINED__
#define __IOSGeometryUI_DISPINTERFACE_DEFINED__

/* dispinterface IOSGeometryUI */
/* [uuid] */ 


EXTERN_C const IID DIID_IOSGeometryUI;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("C052FED9-A2D6-42E3-A271-2C6FB8461711")
    IOSGeometryUI : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct IOSGeometryUIVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IOSGeometryUI * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IOSGeometryUI * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IOSGeometryUI * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IOSGeometryUI * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IOSGeometryUI * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IOSGeometryUI * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IOSGeometryUI * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } IOSGeometryUIVtbl;

    interface IOSGeometryUI
    {
        CONST_VTBL struct IOSGeometryUIVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOSGeometryUI_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOSGeometryUI_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOSGeometryUI_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOSGeometryUI_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IOSGeometryUI_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IOSGeometryUI_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IOSGeometryUI_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __IOSGeometryUI_DISPINTERFACE_DEFINED__ */


#ifndef __IStaadProWindow_DISPINTERFACE_DEFINED__
#define __IStaadProWindow_DISPINTERFACE_DEFINED__

/* dispinterface IStaadProWindow */
/* [uuid] */ 


EXTERN_C const IID DIID_IStaadProWindow;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("9EF2FF8C-E574-4A04-9462-2E4500C8EADB")
    IStaadProWindow : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct IStaadProWindowVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IStaadProWindow * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IStaadProWindow * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IStaadProWindow * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IStaadProWindow * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IStaadProWindow * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IStaadProWindow * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IStaadProWindow * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } IStaadProWindowVtbl;

    interface IStaadProWindow
    {
        CONST_VTBL struct IStaadProWindowVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IStaadProWindow_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IStaadProWindow_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IStaadProWindow_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IStaadProWindow_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IStaadProWindow_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IStaadProWindow_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IStaadProWindow_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __IStaadProWindow_DISPINTERFACE_DEFINED__ */


#ifndef __IOSViewUI_DISPINTERFACE_DEFINED__
#define __IOSViewUI_DISPINTERFACE_DEFINED__

/* dispinterface IOSViewUI */
/* [uuid] */ 


EXTERN_C const IID DIID_IOSViewUI;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("87B1975B-6031-487E-A0F9-FB8F69FA24E6")
    IOSViewUI : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct IOSViewUIVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IOSViewUI * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IOSViewUI * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IOSViewUI * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IOSViewUI * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IOSViewUI * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IOSViewUI * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IOSViewUI * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } IOSViewUIVtbl;

    interface IOSViewUI
    {
        CONST_VTBL struct IOSViewUIVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOSViewUI_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOSViewUI_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOSViewUI_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOSViewUI_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IOSViewUI_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IOSViewUI_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IOSViewUI_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __IOSViewUI_DISPINTERFACE_DEFINED__ */


#ifndef __IOSOutputUI_DISPINTERFACE_DEFINED__
#define __IOSOutputUI_DISPINTERFACE_DEFINED__

/* dispinterface IOSOutputUI */
/* [uuid] */ 


EXTERN_C const IID DIID_IOSOutputUI;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("824F1FC0-DC86-4CC4-A4C6-83C77D7B0496")
    IOSOutputUI : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct IOSOutputUIVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IOSOutputUI * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IOSOutputUI * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IOSOutputUI * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IOSOutputUI * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IOSOutputUI * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IOSOutputUI * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IOSOutputUI * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } IOSOutputUIVtbl;

    interface IOSOutputUI
    {
        CONST_VTBL struct IOSOutputUIVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOSOutputUI_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOSOutputUI_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOSOutputUI_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOSOutputUI_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IOSOutputUI_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IOSOutputUI_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IOSOutputUI_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __IOSOutputUI_DISPINTERFACE_DEFINED__ */


#ifndef __IOSPropertyUI_DISPINTERFACE_DEFINED__
#define __IOSPropertyUI_DISPINTERFACE_DEFINED__

/* dispinterface IOSPropertyUI */
/* [uuid] */ 


EXTERN_C const IID DIID_IOSPropertyUI;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("F919EF7D-E1DD-48CB-B3C3-B3CAE9E3B5AB")
    IOSPropertyUI : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct IOSPropertyUIVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IOSPropertyUI * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IOSPropertyUI * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IOSPropertyUI * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IOSPropertyUI * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IOSPropertyUI * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IOSPropertyUI * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IOSPropertyUI * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } IOSPropertyUIVtbl;

    interface IOSPropertyUI
    {
        CONST_VTBL struct IOSPropertyUIVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOSPropertyUI_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOSPropertyUI_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOSPropertyUI_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOSPropertyUI_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IOSPropertyUI_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IOSPropertyUI_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IOSPropertyUI_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __IOSPropertyUI_DISPINTERFACE_DEFINED__ */


#ifndef __IOSLoadUI_DISPINTERFACE_DEFINED__
#define __IOSLoadUI_DISPINTERFACE_DEFINED__

/* dispinterface IOSLoadUI */
/* [uuid] */ 


EXTERN_C const IID DIID_IOSLoadUI;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("DAA37D16-821F-4137-88EB-DA4EB7650E90")
    IOSLoadUI : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct IOSLoadUIVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IOSLoadUI * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IOSLoadUI * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IOSLoadUI * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IOSLoadUI * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IOSLoadUI * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IOSLoadUI * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IOSLoadUI * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } IOSLoadUIVtbl;

    interface IOSLoadUI
    {
        CONST_VTBL struct IOSLoadUIVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOSLoadUI_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOSLoadUI_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOSLoadUI_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOSLoadUI_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IOSLoadUI_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IOSLoadUI_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IOSLoadUI_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __IOSLoadUI_DISPINTERFACE_DEFINED__ */


#ifndef __IOSTableUI_DISPINTERFACE_DEFINED__
#define __IOSTableUI_DISPINTERFACE_DEFINED__

/* dispinterface IOSTableUI */
/* [uuid] */ 


EXTERN_C const IID DIID_IOSTableUI;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("CF1A7B89-A007-4844-A098-CBABAEBEF304")
    IOSTableUI : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct IOSTableUIVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IOSTableUI * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IOSTableUI * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IOSTableUI * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IOSTableUI * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IOSTableUI * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IOSTableUI * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IOSTableUI * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } IOSTableUIVtbl;

    interface IOSTableUI
    {
        CONST_VTBL struct IOSTableUIVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOSTableUI_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOSTableUI_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOSTableUI_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOSTableUI_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IOSTableUI_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IOSTableUI_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IOSTableUI_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __IOSTableUI_DISPINTERFACE_DEFINED__ */


#ifndef __IOSSupportUI_DISPINTERFACE_DEFINED__
#define __IOSSupportUI_DISPINTERFACE_DEFINED__

/* dispinterface IOSSupportUI */
/* [uuid] */ 


EXTERN_C const IID DIID_IOSSupportUI;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("B076EC62-9A33-4E1C-B38C-3E45090B8531")
    IOSSupportUI : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct IOSSupportUIVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IOSSupportUI * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IOSSupportUI * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IOSSupportUI * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IOSSupportUI * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IOSSupportUI * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IOSSupportUI * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IOSSupportUI * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } IOSSupportUIVtbl;

    interface IOSSupportUI
    {
        CONST_VTBL struct IOSSupportUIVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOSSupportUI_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOSSupportUI_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOSSupportUI_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOSSupportUI_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IOSSupportUI_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IOSSupportUI_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IOSSupportUI_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __IOSSupportUI_DISPINTERFACE_DEFINED__ */


#ifndef __IOSCommandsUI_DISPINTERFACE_DEFINED__
#define __IOSCommandsUI_DISPINTERFACE_DEFINED__

/* dispinterface IOSCommandsUI */
/* [uuid] */ 


EXTERN_C const IID DIID_IOSCommandsUI;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("6B994A7C-122C-4828-B42B-1B0CADF91F9D")
    IOSCommandsUI : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct IOSCommandsUIVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IOSCommandsUI * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IOSCommandsUI * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IOSCommandsUI * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IOSCommandsUI * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IOSCommandsUI * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IOSCommandsUI * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IOSCommandsUI * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } IOSCommandsUIVtbl;

    interface IOSCommandsUI
    {
        CONST_VTBL struct IOSCommandsUIVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOSCommandsUI_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOSCommandsUI_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOSCommandsUI_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOSCommandsUI_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IOSCommandsUI_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IOSCommandsUI_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IOSCommandsUI_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __IOSCommandsUI_DISPINTERFACE_DEFINED__ */


#ifndef __IOSDesignUI_DISPINTERFACE_DEFINED__
#define __IOSDesignUI_DISPINTERFACE_DEFINED__

/* dispinterface IOSDesignUI */
/* [uuid] */ 


EXTERN_C const IID DIID_IOSDesignUI;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("ED218322-40EE-411C-8552-209DB2C4F32B")
    IOSDesignUI : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct IOSDesignUIVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IOSDesignUI * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IOSDesignUI * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IOSDesignUI * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IOSDesignUI * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IOSDesignUI * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IOSDesignUI * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IOSDesignUI * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } IOSDesignUIVtbl;

    interface IOSDesignUI
    {
        CONST_VTBL struct IOSDesignUIVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOSDesignUI_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOSDesignUI_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOSDesignUI_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOSDesignUI_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IOSDesignUI_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IOSDesignUI_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IOSDesignUI_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __IOSDesignUI_DISPINTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_OSStringList;

#ifdef __cplusplus

class DECLSPEC_UUID("BD18658D-9A99-46DE-AAB4-46B308DE4CDB")
OSStringList;
#endif

EXTERN_C const CLSID CLSID_OSMemberSteelDgnResults;

#ifdef __cplusplus

class DECLSPEC_UUID("20BC5251-3CB8-4025-8D8E-B0D5F3B3E905")
OSMemberSteelDgnResults;
#endif

EXTERN_C const CLSID CLSID_OSMemberSteelDgnParams;

#ifdef __cplusplus

class DECLSPEC_UUID("D780A6BB-EE59-450C-A342-BA5E90F24A9A")
OSMemberSteelDgnParams;
#endif

EXTERN_C const CLSID CLSID_OSChineseSteelDgnResults;

#ifdef __cplusplus

class DECLSPEC_UUID("1D3341F4-4C9A-45B2-ACFF-E0C23EC8D68F")
OSChineseSteelDgnResults;
#endif

EXTERN_C const CLSID CLSID_OSChineseSteelDgnParameters;

#ifdef __cplusplus

class DECLSPEC_UUID("C3E2DBC2-B671-4FE3-84D2-F6D6218C3448")
OSChineseSteelDgnParameters;
#endif

EXTERN_C const CLSID CLSID_OSChineseSteelCheckOption;

#ifdef __cplusplus

class DECLSPEC_UUID("ADFAA55F-E4FE-47FA-B2C8-42EDE1B631BE")
OSChineseSteelCheckOption;
#endif

EXTERN_C const CLSID CLSID_OpenSTAAD;

#ifdef __cplusplus

class DECLSPEC_UUID("1797F52C-C5E2-4F41-B524-F5A107EA6453")
OpenSTAAD;
#endif

EXTERN_C const CLSID CLSID_OSGeometryUI;

#ifdef __cplusplus

class DECLSPEC_UUID("E4822980-C160-4E98-A6B4-7F5E327B465F")
OSGeometryUI;
#endif

EXTERN_C const CLSID CLSID_StaadProWindow;

#ifdef __cplusplus

class DECLSPEC_UUID("0C8AA0B7-44CF-4752-B3D3-75BEA20836A5")
StaadProWindow;
#endif

EXTERN_C const CLSID CLSID_OSViewUI;

#ifdef __cplusplus

class DECLSPEC_UUID("F1FF6204-BAA6-45BE-B4B4-5B363DFCBDEB")
OSViewUI;
#endif

EXTERN_C const CLSID CLSID_OSOutputUI;

#ifdef __cplusplus

class DECLSPEC_UUID("E177C52C-DB8A-41D1-B4AB-2CDE6060B1E1")
OSOutputUI;
#endif

EXTERN_C const CLSID CLSID_OSPropertyUI;

#ifdef __cplusplus

class DECLSPEC_UUID("56213A34-F665-4F7A-9E0C-9BCE9865616E")
OSPropertyUI;
#endif

EXTERN_C const CLSID CLSID_OSLoadUI;

#ifdef __cplusplus

class DECLSPEC_UUID("40F6DDA7-AD4A-483E-AB74-BDDD9A8FF2DB")
OSLoadUI;
#endif

EXTERN_C const CLSID CLSID_OSTableUI;

#ifdef __cplusplus

class DECLSPEC_UUID("03503F6E-1E3E-4CF3-BB0C-ABD0CFF09F1B")
OSTableUI;
#endif

EXTERN_C const CLSID CLSID_OSSupportUI;

#ifdef __cplusplus

class DECLSPEC_UUID("55ABB26D-1E09-47CC-A9E5-AB2D7397790F")
OSSupportUI;
#endif

EXTERN_C const CLSID CLSID_OSCommandsUI;

#ifdef __cplusplus

class DECLSPEC_UUID("7A3763F2-A9FC-4E13-A128-9F604642CE25")
OSCommandsUI;
#endif

EXTERN_C const CLSID CLSID_OSDesignUI;

#ifdef __cplusplus

class DECLSPEC_UUID("AF512BCB-6A77-49BE-A488-9418622BF95A")
OSDesignUI;
#endif
#endif /* __OpenSTAADUI_LIBRARY_DEFINED__ */

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


