// Copyright © 2024 Pathway

#![allow(clippy::result_large_err)]

mod helpers;
mod operator_test_utils;

mod test_arrow;
mod test_bson;
mod test_bytes;
mod test_cached_object_storage;
mod test_connector_field_defaults;
mod test_connector_sync;
mod test_dd_distinct_total;
mod test_debezium;
mod test_deltalake;
mod test_dsv;
mod test_dsv_dir;
mod test_dsv_output;
mod test_file_kv;
mod test_json_output;
mod test_jsonlines;
mod test_metadata;
mod test_null_writer;
mod test_offsets_storage;
mod test_operator_persistence;
mod test_parser;
mod test_parser_errors;
mod test_prev_next;
mod test_psql_output;
mod test_psql_snapshot;
mod test_seek;
mod test_sqlite;
mod test_stream_snapshot;
mod test_time;
mod test_time_column;
mod test_types;
mod test_value_to_sql;
