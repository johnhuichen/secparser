use secparser_core::financial_statements::tag_record::FsTag;
use snafu::Whatever;

use crate::db::PostgresDB;

use super::ingest::ingest_fs_records;

pub fn ingest_fs_tag() -> Result<(), Whatever> {
    let get_display_name = |r: &FsTag| format!("{}/{}", r.tag, r.version);
    ingest_fs_records(insert_fs_tag, get_display_name)
}

fn insert_fs_tag(db: &mut PostgresDB, record: &FsTag) -> Result<(), postgres::Error> {
    db.client.execute(
        "INSERT INTO fs_tag AS t
(tag, version, custom, abstract, datatype, iord, crdr, tlabel, doc)

VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
ON CONFLICT ON CONSTRAINT fs_tag_pkey DO NOTHING",
        &[
            &record.tag,
            &record.version,
            &(record.custom.unwrap_or_default() as i16),
            &(record.r#abstract.unwrap_or_default() as i16),
            &record.datatype,
            &record.iord,
            &record.crdr,
            &record.tlabel,
            &record.doc,
        ],
    )?;

    Ok(())
}
