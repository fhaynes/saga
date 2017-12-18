pub const PRAGMA_WAL: &'static str = "PRAGMA journal_mode=WAL";
pub const PRAGMA_SYNCHRONOUS_OFF: &'static str = "PRAGMA synchronous=OFF";

/// Node and shard related queries

/// Query to create the Nodes table
pub const QUERY_CREATE_NODES_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS nodes (
        id              TEXT PRIMARY KEY NOT NULL,
        ip              TEXT NOT NULL,
        port            INTEGER NOT NULL,
        is_master       INTEGER DEFAULT 0,
        is_data         INTEGER DEFAULT 1
    )";

/// Query to create the Indexes table
pub const QUERY_CREATE_INDEXES_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS indexes (
        name            TEXT PRIMARY KEY NOT NULL
    )";

/// Query to create the Shards table
pub const QUERY_CREATE_SHARDS_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS shards (
        id           TEXT PRIMARY KEY NOT NULL,
        index_name   TEXT NOT NULL,
        node         TEXT NOT NULL,
        FOREIGN KEY(node) REFERENCES nodes(id),
        FOREIGN KEY(index_name) REFERENCES indexes(name)
    )";

pub const QUERY_CREATE_REPLICAS_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS replicas (
        id           TEXT PRIMARY KEY NOT NULL,
        index_name   TEXT NOT NULL,
        node         TEXT NOT NULL,
        shard        TEXT NOT NULL,
        FOREIGN KEY(node) REFERENCES nodes(id),
        FOREIGN KEY(index_name) REFERENCES indexes(name),
        FOREIGN KEY(shard) REFERENCES shards(id)
    )";

pub const QUERY_CREATE_CONFIGURATION_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS
    configuration (
        id              TEXT NOT NULL
    )";

pub const QUERY_CREATE_METRICS_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS
    metrics (
        component TEXT NOT NULL,
        value REAL,
        timestamp TEXT
    )";

pub const QUERY_INSERT_CONFIG_ID: &'static str = "INSERT OR REPLACE INTO configuration (id) VALUES (?1)";
pub const QUERY_GET_CONFIG_ID: &'static str = "SELECT id FROM configuration";
pub const QUERY_INSERT_NODE: &'static str = "INSERT OR REPLACE INTO nodes (id, ip, port, is_master, is_data) VALUES (?1, ?2, ?3, ?4, ?5)";
pub const QUERY_INSERT_SHARD: &'static str = "INSERT OR REPLACE INTO shards (id, index_name, node) VALUES (?1, ?2, ?3)";
pub const QUERY_INSERT_REPLICA: &'static str = "INSERT OR REPLACE INTO replicas (id, index_name, node, shard) VALUES (?1, ?2, ?3, ?4)";
pub const QUERY_INSERT_INDEX: &'static str = "INSERT INTO indexes (name) VALUES (?1)";
pub const QUERY_INDEX_EXISTS: &'static str = "SELECT name FROM indexes WHERE name = ?1";
pub const QUERY_DELETE_NODE: &'static str = "DELETE FROM nodes WHERE id = ?1";
pub const QUERY_DELETE_ALL_NODES: &'static str = "DROP TABLE IF EXISTS nodes";
pub const QUERY_DELETE_INDEX: &'static str = "DELETE FROM indexes WHERE id = ?1";
pub const QUERY_ALL_NODES: &'static str = "SELECT id, ip, port, last_heard FROM nodes";
pub const QUERY_FIND_NODE_BY: &'static str = "SELECT id, ip, port, last_heard FROM nodes WHERE {} = :value";
pub const QUERY_FIND_NODE_BY_IP_PORT: &'static str = "SELECT id, ip, port, last_heard FROM nodes WHERE ip = :ip AND port = :port";
pub const QUERY_DELETE_NODE_BY_IP_PORT: &'static str = "DELETE FROM nodes WHERE ip = :ip AND port = :port";
pub const QUERY_GET_NODE_ID: &'static str = "SELECT id FROM nodes WHERE ip = ?1 AND port = ?2";
pub const QUERY_ALL_INDEXES: &'static str = "SELECT name FROM indexes";
pub const QUERY_GET_SHARDS_BY_INDEX: &'static str = "SELECT id, node FROM shards WHERE index_name = ?1";
pub const QUERY_GET_REPLICAS_BY_SHARD: &'static str = "SELECT id, index_name, node FROM replicas WHERE shard = ?1";

// Index related queries
pub const QUERY_CREATE_METADATA_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS metadata (
        name                TEXT PRIMARY KEY,
        keep_raw            BOOLEAN NOT NULL,
        UNIQUE(name)
    )";

pub const QUERY_CREATE_TERMS_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS terms (
        term    TEXT PRIMARY KEY,
        UNIQUE(term)
    )";

pub const QUERY_CREATE_DOCUMENTS_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS documents (
        id TEXT PRIMARY KEY,
        content TEXT
    )";

pub const QUERY_CREATE_FIELDS_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS fields (
        name     TEXT,
        document TEXT,
        content  TEXT,
        FOREIGN KEY(document) REFERENCES documents(id)
    )";

pub const QUERY_CREATE_MAPPINGS_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS mappings (
        name TEXT,
        type TEXT
    )";

pub const QUERY_CREATE_MAPPINGS_INDEX: &'static str = "CREATE UNIQUE INDEX IF NOT EXISTS name_type ON mappings (name, type)";

pub const QUERY_CREATE_FIELDS_INDEX: &'static str = "CREATE INDEX IF NOT EXISTS field_index ON fields (name, document)";

pub const QUERY_CREATE_OCCURRENCES_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS occurrences (
        term TEXT,
        document TEXT,
        field TEXT,
        offset INTEGER NOT NULL,
        FOREIGN KEY(term) REFERENCES terms(term),
        FOREIGN KEY(document) REFERENCES documents(id)
    )";

pub const QUERIES_INITIALIZE_INDEX_DB: &[&str; 9] = &[
    PRAGMA_WAL,
    PRAGMA_SYNCHRONOUS_OFF,
    QUERY_CREATE_TERMS_TABLE,
    QUERY_CREATE_DOCUMENTS_TABLE,
    QUERY_CREATE_FIELDS_TABLE,
    QUERY_CREATE_MAPPINGS_TABLE,
    QUERY_CREATE_MAPPINGS_INDEX,
    QUERY_CREATE_FIELDS_INDEX,
    QUERY_CREATE_OCCURRENCES_TABLE,
];

pub const QUERY_INSERT_FIELD: &'static str = "INSERT OR IGNORE INTO fields(name, document, content) VALUES (?1, ?2, ?3)";
pub const QUERY_INSERT_TERM: &'static str = "INSERT OR IGNORE INTO terms (term) VALUES (?1)";
pub const QUERY_INSERT_OCCURRENCE: &'static str = "INSERT INTO occurrences (term, document, field, offset) VALUES (?1, ?2, ?3, ?4)";
pub const QUERY_INSERT_DOCUMENT: &'static str = "INSERT INTO documents (id, content) VALUES (?1, ?2)";
pub const QUERY_INSERT_MAPPING: &'static str = "INSERT OR IGNORE INTO mappings (name, type) VALUES (?1, ?2)";
pub const QUERY_INIT_METADATA: &'static str = "INSERT OR IGNORE INTO metadata (name, keep_raw) VALUES (?1, ?2)";
pub const QUERY_ALL_TERMS: &'static str = "SELECT term FROM terms";
pub const QUERY_OCCURRENCES_FOR_TERM: &'static str = "SELECT document, field, offset FROM occurrences WHERE term = ?1";
pub const QUERY_COUNT_TERMS: &'static str = "SELECT COUNT(*) FROM terms";
pub const QUERY_DOCUMENT_BY_ID: &'static str = "SELECT id, content FROM documents WHERE id = ?1";
pub const QUERY_DELETE_DOCUMENT_BY_ID: &'static str = "DELETE FROM documents WHERE id = ?1";
pub const QUERY_DELETE_OCCURRENCES_BY_DOCUMENT_ID: &'static str = "DELETE FROM occurrences WHERE document = ?1";
pub const QUERY_DOCUMENTS_WITH_TERM_IN_FIELD: &'static str = "SELECT document FROM occurrences WHERE field = ?1 AND term = ?2";
pub const QUERY_DOCUMENTS_WITH_TERM: &'static str = "SELECT document FROM occurrences WHERE field = ?1";
pub const QUERY_TERM_IN_DOCUMENTS: &'static str = "SELECT document FROM occurrences WHERE term = ?1 LIMIT 1";

// Partial queries, used for dynamically building up longer queries
pub const QUERY_PARTIAL_RANGE: &'static str = "SELECT document FROM occurrences WHERE field = ?1 ";

// Metrics queries
pub const QUERY_INSERT_METRIC: &'static str = "INSERT INTO metrics (component, value, timestamp) VALUES (?1, ?2, ?3)";
pub const QUERY_JOB_AVG_COMPLETION_TIME: &'static str = "SELECT AVG(value) FROM metrics WHERE component = 'job_completion_time'";
