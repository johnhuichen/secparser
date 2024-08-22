use secparser_core::financial_statements::num_record::FsNum;

use crate::db::PostgresDB;

use super::fs_ingest::IngestableFsRecord;

impl IngestableFsRecord for FsNum {
    fn display_name(&self) -> String {
        format!("{}/{}/{}", self.adsh, self.tag, self.version)
    }

    fn insert(&self, db: &mut PostgresDB) -> Result<(), postgres::Error> {
        db.client.execute(
            "INSERT INTO fs_num AS t
(adsh, tag, version, ddate, qtrs, uom, dimh, iprx, value, footnote,
  -- footlen
dimn
  -- coreg TEXT,
  -- durp REAL,
  -- datp REAL,
  -- dcml REAL,
)

VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
ON CONFLICT ON CONSTRAINT fs_num_pkey DO NOTHING",
            &[
                &self.adsh,
                &self.tag,
                &self.version,
                &self.ddate,
                &(self.qtrs.unwrap_or_default() as i16),
                &self.uom,
                &self.dimh,
                &(self.iprx.unwrap_or_default() as i16),
                &self.value.unwrap_or_default(),
                &self.footnote,
                &(self.dimn.unwrap_or_default() as i16),
            ],
        )?;

        Ok(())
    }
}
