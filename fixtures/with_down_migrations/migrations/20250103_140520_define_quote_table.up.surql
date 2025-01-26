DEFINE TABLE quote SCHEMALESS
    PERMISSIONS
        FOR CREATE, SELECT WHERE $auth,
        FOR UPDATE, DELETE WHERE created_by = $auth;
DEFINE FIELD text ON TABLE quote TYPE string;
DEFINE FIELD created_by ON TABLE quote VALUE $auth READONLY;

DEFINE INDEX quote_index ON TABLE quote FIELDS text;
