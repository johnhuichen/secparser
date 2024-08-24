use secparser_core::financial_statements::sub_record::FsSub;

use crate::db::PostgresDB;

use super::fs_ingest::IngestableFsRecord;

impl IngestableFsRecord for FsSub {
    fn display_name(&self) -> String {
        self.name.to_string()
    }

    fn insert(&self, db: &mut PostgresDB) -> Result<(), postgres::Error> {
        db.client.execute(
            "INSERT INTO fs_sub AS t
(adsh, cik,
-- name,
-- sic,
-- countryba, stprba, cityba, zipba, bas1, bas2, baph,
-- countryma, stprma, cityma, zipma, mas1, mas2,
-- countryinc, stprinc,
ein,
-- former, changed,
afs,
-- wksi,
fye, form, period, fy, fp, filed,
-- accepted, prevrpt, detail,
instance
-- nciks, aciks, pubfloatusd, floatdate, floataxis, floatmems
)

VALUES ($1, $2,
        $3,
        $4,
        $5, $6, $7, $8, $9, $10,
        $11)
ON CONFLICT ON CONSTRAINT fs_sub_pkey DO NOTHING",
            &[
                &self.adsh,
                &(self.cik as u32),
                &self.ein,
                &self.afs,
                &self.fye,
                &self.form,
                &self.period,
                &self.fy,
                &self.fp,
                &self.filed,
                &self.instance,
            ],
        )?;

        Ok(())
    }
}
