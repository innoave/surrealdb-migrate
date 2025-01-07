DEFINE TABLE $migrations_table SCHEMAFULL TYPE NORMAL COMMENT 'version:1.0'
    PERMISSIONS
        FOR select FULL
        FOR create, update, delete NONE;

DEFINE FIELD applied_rank ON migrations TYPE int;
DEFINE FIELD key ON migrations TYPE string;
DEFINE FIELD title ON migrations TYPE string;
DEFINE FIELD kind ON migrations TYPE string;
DEFINE FIELD script_path ON migrations TYPE string;
DEFINE FIELD checksum ON migrations TYPE string;
DEFINE FIELD applied_at ON migrations TYPE datetime;
DEFINE FIELD applied_by ON migrations TYPE string;
DEFINE FIELD execution_time ON migrations TYPE duration;
DEFINE FIELD successful ON migrations TYPE bool;

DEFINE INDEX rank_idx on TABLE migrations FIELDS applied_rank UNIQUE;
DEFINE INDEX key_idx ON TABLE migrations FIELDS key UNIQUE;
DEFINE INDEX successful_idx on TABLE migrations FIELDS successful;
