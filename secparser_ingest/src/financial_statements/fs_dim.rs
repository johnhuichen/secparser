use secparser_core::financial_statements::dim_record::FsDim;

use crate::db::PostgresDB;

use super::fs_ingest::IngestableFsRecord;

impl IngestableFsRecord for FsDim {
    fn display_name(&self) -> String {
        self.dimhash.to_string()
    }

    fn insert(&self, db: &mut PostgresDB) -> Result<(), postgres::Error> {
        db.client.execute(
            "INSERT INTO fs_dim AS t
(dimhash, segments, segt)

VALUES ($1, $2, $3)
ON CONFLICT ON CONSTRAINT fs_dim_pkey DO NOTHING",
            &[
                &self.dimhash,
                &self.segments,
                &(self.segt.unwrap_or_default() as i16),
            ],
        )?;

        Ok(())
    }
}
