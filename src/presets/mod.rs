use metadata::MetaDataFrame;
use std::{io::Cursor, sync::LazyLock};

macro preset($name:literal) {
    LazyLock::new(|| {
        let bytes = include_bytes!($name);
        MetaDataFrame::read_ipc(Cursor::new(bytes)).expect(concat!("preset ", $name))
    })
}

/// <https://doi.org/10.1016/j.algal.2018.11.004>
pub(crate) mod _10_1016_j_algal_2018_11_004 {
    use super::*;

    pub(crate) static CV_15: LazyLock<MetaDataFrame> =
        preset!("10.1016/j.algal.2018.11.004/CV-15.hmf.ipc");
    pub(crate) static CZ_30412: LazyLock<MetaDataFrame> =
        preset!("10.1016/j.algal.2018.11.004/CZ-30412.hmf.ipc");
    pub(crate) static CV_395: LazyLock<MetaDataFrame> =
        preset!("10.1016/j.algal.2018.11.004/CV-395.hmf.ipc");
    pub(crate) static CP_9: LazyLock<MetaDataFrame> =
        preset!("10.1016/j.algal.2018.11.004/CP-9.hmf.ipc");
    pub(crate) static SS: LazyLock<MetaDataFrame> =
        preset!("10.1016/j.algal.2018.11.004/SS.hmf.ipc");
    pub(crate) static CS: LazyLock<MetaDataFrame> =
        preset!("10.1016/j.algal.2018.11.004/CS.hmf.ipc");
    pub(crate) static NL_2047: LazyLock<MetaDataFrame> =
        preset!("10.1016/j.algal.2018.11.004/NL-2047.hmf.ipc");
    pub(crate) static PT_646: LazyLock<MetaDataFrame> =
        preset!("10.1016/j.algal.2018.11.004/PT-646.hmf.ipc");
    pub(crate) static ISO_FJ: LazyLock<MetaDataFrame> =
        preset!("10.1016/j.algal.2018.11.004/ISO-FJ.hmf.ipc");
    pub(crate) static IG_2307: LazyLock<MetaDataFrame> =
        preset!("10.1016/j.algal.2018.11.004/IG-2307.hmf.ipc");
    pub(crate) static NO_IMET1: LazyLock<MetaDataFrame> =
        preset!("10.1016/j.algal.2018.11.004/NO-IMET1.hmf.ipc");
    pub(crate) static NS_537: LazyLock<MetaDataFrame> =
        preset!("10.1016/j.algal.2018.11.004/NS-537.hmf.ipc");
}

/// <https://doi.org/10.1021/jf903048p>
pub(crate) mod _10_1021_jf903048p {
    use super::*;

    pub(crate) static MATURE_MILK: LazyLock<MetaDataFrame> =
        preset!("10.1021/jf903048p/MatureMilk.ipc");

    pub(crate) static CMF_AF: LazyLock<MetaDataFrame> = preset!("10.1021/jf903048p/CMF-AF.hmf.ipc");
    pub(crate) static CMF_AP: LazyLock<MetaDataFrame> = preset!("10.1021/jf903048p/CMF-AP.hmf.ipc");
    pub(crate) static CMF_R: LazyLock<MetaDataFrame> = preset!("10.1021/jf903048p/CMF-R.hmf.ipc");
    pub(crate) static MMF_A: LazyLock<MetaDataFrame> = preset!("10.1021/jf903048p/MMF-A.hmf.ipc");
}

/// <https://doi.org/10.1038/sj.ejcn.1601470>
pub(crate) mod _10_1038_sj_ejcn_1601470 {
    use super::*;

    pub(crate) static CMF: LazyLock<MetaDataFrame> = preset!("10.1038/sj.ejcn.1601470/CMF.hmf.ipc");
    pub(crate) static MMF: LazyLock<MetaDataFrame> = preset!("10.1038/sj.ejcn.1601470/MMF.hmf.ipc");
    pub(crate) static TMF: LazyLock<MetaDataFrame> = preset!("10.1038/sj.ejcn.1601470/TMF.hmf.ipc");
}

// IPPRAS
pub(crate) mod ippras {
    use super::*;

    pub(crate) static C70_CONTROL: LazyLock<MetaDataFrame> =
        preset!("ippras/C70_Control.2025-01-01.hmfa.ipc");
    pub(crate) static C70_H2O2: LazyLock<MetaDataFrame> = preset!("ippras/C70_H2O2.arrow");
    pub(crate) static C70_NACL: LazyLock<MetaDataFrame> = preset!("ippras/C70_NaCl.arrow");

    pub(crate) static H242_N_1: LazyLock<MetaDataFrame> = preset!("ippras/H242_-N.1.arrow");
    pub(crate) static H242_N_2: LazyLock<MetaDataFrame> = preset!("ippras/H242_-N.2.arrow");
    pub(crate) static H242_N_3: LazyLock<MetaDataFrame> = preset!("ippras/H242_-N.3.arrow");
    pub(crate) static H242_N: LazyLock<MetaDataFrame> = preset!("ippras/H242_-N.arrow");
}
