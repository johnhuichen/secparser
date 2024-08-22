use secparser_core::financial_statements::tag_record::FsTag;

use crate::db::PostgresDB;

use super::fs_ingest::IngestableFsRecord;

impl IngestableFsRecord for FsTag {
    fn display_name(&self) -> String {
        format!("{}/{}", self.tag, self.version)
    }

    fn insert(&self, db: &mut PostgresDB) -> Result<(), postgres::Error> {
        db.client.execute(
            "INSERT INTO fs_tag AS t
(tag, version,
-- custom,
abstract, datatype, iord, crdr, tlabel
-- doc
)

VALUES ($1, $2, $3, $4, $5, $6, $7)
ON CONFLICT ON CONSTRAINT fs_tag_pkey DO NOTHING",
            &[
                &self.tag,
                &self.version,
                &(self.r#abstract.unwrap_or_default() as i16),
                &self.datatype,
                &self.iord,
                &self.crdr,
                &self.tlabel,
            ],
        )?;

        Ok(())
    }
}
