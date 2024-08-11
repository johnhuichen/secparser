use secparser_macros::FsRecordsImpl;
use serde::Deserialize;

const TSV_FILENAME: &str = "sub.tsv";

#[derive(Debug, Deserialize, FsRecordsImpl)]
pub struct FsSub {
    pub adsh: String,
    pub cik: usize,
    pub name: String,
    pub sic: String,

    pub countryba: String,
    pub stprba: String,
    pub cityba: String,
    pub zipba: String,
    pub bas1: String,
    pub bas2: String,
    pub baph: String,

    pub countryma: String,
    pub stprma: String,
    pub cityma: String,
    pub zipma: String,
    pub mas1: String,
    pub mas2: String,

    pub countryinc: String,
    pub stprinc: String,

    pub ein: String,
    pub former: String,
    pub changed: String,
    pub afs: String,
    pub wksi: Option<u8>,
    pub fye: String,
    pub form: String,
    pub period: String,
    pub fy: String,
    pub fp: String,
    pub filed: String,
    pub accepted: String,
    pub prevrpt: Option<u8>,
    pub detail: Option<u8>,
    pub instance: String,
    pub nciks: Option<u16>,
    pub aciks: String,
    pub pubfloatusd: Option<f32>,
    pub floatdate: String,
    pub floataxis: String,
    pub floatmems: Option<u8>,
}
