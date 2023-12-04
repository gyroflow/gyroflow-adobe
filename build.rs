
use pipl::*;

const PF_PLUG_IN_VERSION: u16 = 13;
const PF_PLUG_IN_SUBVERS: u16 = 28;

fn main() {
    pipl::plugin_build(vec![
        Property::Kind(PIPLType::AEEffect),
        Property::Name("Gyroflow"),
        Property::Category("Gyroflow"),

        #[cfg(target_os = "windows")]
        Property::CodeWin64X86("EffectMain"),
        #[cfg(target_os = "macos")]
        Property::CodeMacIntel64("EffectMain"),
        #[cfg(target_os = "macos")]
        Property::CodeMacARM64("EffectMain"),

        Property::AE_PiPL_Version { major: 2, minor: 0 },
        Property::AE_Effect_Spec_Version { major: PF_PLUG_IN_VERSION, minor: PF_PLUG_IN_SUBVERS },
        Property::AE_Effect_Version((1, 0, 0, 0, 0)),
        Property::AE_Effect_Info_Flags(0),
        Property::AE_Effect_Global_OutFlags(
			OutFlags::PixIndependent |
			OutFlags::DeepColorAware |
			OutFlags::SendUpdateParamsUI |
			OutFlags::NonParamVary
			//| OutFlags::SequenceDataNeedsFlattening
		),
        Property::AE_Effect_Global_OutFlags_2(
			OutFlags2::FloatColorAware |
			OutFlags2::SupportsSmartRender |
			//OutFlags2::SupportsThreadedRendering |
			//OutFlags2::SupportsGetFlattenedSequenceData |
			OutFlags2::SupportsGpuRenderF32
		),
        Property::AE_Effect_Match_Name("ADBE Gyroflow"),
        Property::AE_Reserved_Info(0),
        Property::AE_Effect_Support_URL("https://docs.gyroflow.xyz"),
    ])
}
