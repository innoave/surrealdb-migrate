DEFINE TABLE $migrations_table SCHEMAFULL TYPE NORMAL COMMENT 'version:1.0'
    PERMISSIONS
        FOR select FULL
        FOR create, update, delete NONE;

DEFINE FIELD key ON migrations TYPE string;
DEFINE FIELD title ON migrations TYPE string;
DEFINE FIELD script_path ON migrations TYPE string;
DEFINE FIELD checksum ON migrations TYPE string;
DEFINE FIELD applied_at ON migrations TYPE datetime;
DEFINE FIELD execution_time ON migrations TYPE duration;

DEFINE INDEX key_idx ON TABLE migrations FIELDS key UNIQUE;
