use std::fs::File;

use anyhow::Result;
use serde::Deserialize;

use crate::traits::FileReader;

use super::data_source::FsDataSource;

#[derive(Debug, Deserialize)]
pub struct FsSubmission {
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
    pub wksi: bool,
    pub fye: String,
    pub form: String,
    pub period: String,
    pub fy: String,
    pub fp: String,
    pub field: String,
    pub accepted: String,
    pub prevrpt: bool,
    pub detail: bool,
    pub instance: String,
    pub nciks: u32,
    pub aciks: String,
    pub pubfloatusd: f32,
    pub floatdate: String,
    pub floataxis: String,
    pub floatmems: u32,
}

pub struct FsSubmissionRecords {
    // pub count: usize,
    //
    // lines: FileLines,
}

impl FileReader for FsSubmissionRecords {}

impl FsSubmissionRecords {
    pub fn new(datasource: FsDataSource) -> Result<Self> {
        for file in datasource.zip_files {
            let file = File::open(file)?;
            println!("{:?}", file);
            let archive = zip::ZipArchive::new(file)?;
            println!("{:?}", archive.len());
        }

        Ok(FsSubmissionRecords {})
    }
}

impl Iterator for FsSubmissionRecords {
    type Item = FsSubmission;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
