use lazy_static::lazy_static;
use rocksdb::*;

use crate::config::CONFIG;

lazy_static! {
    pub static ref ROCKSDB: DB = rocksdb::DB::open_cf(
        &rocksdb_config(),
        &CONFIG.rocksdb_path,
        &["config", "passkey", "info_hash", "reset"]
    )
    .expect("unable to connect to rocksdb");
}

fn rocksdb_config() -> Options {
    let mut opts = Options::default();
    opts.create_if_missing(true);
    opts.create_missing_column_families(true);
    opts.set_max_open_files(10000);
    opts.set_use_fsync(false);
    opts.set_bytes_per_sync(8388608);
    opts.optimize_for_point_lookup(1024);
    opts.set_table_cache_num_shard_bits(6);
    opts.set_max_write_buffer_number(16);
    opts.set_write_buffer_size(0x200000);
    opts.set_target_file_size_base(0x4000000);
    opts.set_min_write_buffer_number_to_merge(4);
    opts.set_level_zero_stop_writes_trigger(24);
    opts.set_level_zero_slowdown_writes_trigger(0);
    opts.set_compaction_style(DBCompactionStyle::Universal);
    opts.set_disable_auto_compactions(false);
    opts
}

#[inline(always)]
pub fn put_cf<K, V>(name: &str, key: K, value: V) -> Result<(), Error>
where
    K: AsRef<[u8]>,
    V: AsRef<[u8]>,
{
    let cf = ROCKSDB
        .cf_handle(name)
        .expect(&format!("Cannot open column family {}", name));
    ROCKSDB.put_cf(cf, key, value)
}
