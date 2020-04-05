use metamod_sys::META_RES;

#[repr(u32)]
#[derive(Debug)]
pub enum CallResult {
    Ignored = 1,
    Handled = 2,
    Override = 3,
    Supercede = 4,
}

impl Into<META_RES> for CallResult {
    fn into(self) -> META_RES {
        match self {
            CallResult::Ignored => META_RES::MRES_IGNORED,
            CallResult::Handled => META_RES::MRES_HANDLED,
            CallResult::Override => META_RES::MRES_OVERRIDE,
            CallResult::Supercede => META_RES::MRES_SUPERCEDE,
        }
    }
}
