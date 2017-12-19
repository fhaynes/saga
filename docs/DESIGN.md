# Saga
Saga is an indexing and search system similar to ElasticSearch and Lucene. It does not attempt to be as broad in scope as those tools, but instead implement a needed subset of their functionality that can be expanded upon interatively.

## Contacts
* Fletcher Haynes \<fletcher@unity3d.com\>

# Table of Contents

* [Design Principals](#design-principals)
* [Components](#components)
    * [Inverted Index](#inverted-index)
      * [Document](#document)
        * [Ingest](#ingesting-documents)
      * [Indices](#indices)
      * [Shards](#shards)
        * [Primary](#primary)
        * [Replica](#replica)
        * [Shard Placement](#shard-placement)
      * [Stores](#stores)
        * [SQLite Store](#sqlite-store)
        * [File Store](#file-store)
    * [Web](#web)
    * [Main](#main)

# Design Principals
Saga has grown out of a desire for an indexing and search solution that is not JVM-based. While powerful and scalable, the operational overhead required to manage large groups of JVM-based services at scale is significant, and our expertise is weighted more toward development. To achieve this, Saga is developed in Rust (https://www.rust-lang.org/) to leverage its speed and safety.

Our high-level design principals for this project are:

* Stability
* Security
* Speed
* Horizontally Scalable

Each of these principals will be covered in more detail in tihs document. 

# Components
This section covers the major components of Saga.

**Directory Structure**
```
saga
├── docs
├── inverted-index
│   ├── src
│   └── target
├── main
│   ├── binaries
│   ├── src
│   └── target
└── web
    ├── src
    └── target
```

Within the parent directory `saga`, components are organized into sub-crates. Each sub-crate is discussed in more detail below, and encompass a logical grouping of functionality or features. Beyond that, there is a directory for documentation. Many of these components are modeled after what is found in Elasticsearch, so will be familiar.

## Inverted Index
This component contains the core functionality of Saga. It implements the parsing, indexing, storing and searching of Documents by means of an inverted index.

### Document
A Document is some piece of text submitted that a user would like indexed. An example might be a JSON-encoded message like the following:

```json
{
    "_id": 0,
    "timestamp": "1513666331",
    "message": "A molar bear is the most sublime of all creatures"
}
```

If the user later searches for the term "molar", it should return this document.

#### Ingesting Documents
Documents are received via the JSON API that Saga's web component provides. Two primary forms of submission are accepted:

**JSON**
If a valid JSON document is submitted to Saga, all the keys are extracted and the corresponding values indexed. 

**Plain Text**
In this case, the user has submitted a string of raw text, perhaps via something like:

```bash
curl -XPOST -H "Content-Type: text/plain" http://example.com/<index_name>/document -d "Who wouldn't want to be a molar bear?"
```

Since this is not JSON, the text will be put into a field named `message`, and indexed. 

### Indices
An Index is a logical collection of Documents that has the following properties:
* Has at least 1 Primary Shard and 0 or more Replica Shards
* Can be spread across multiple Nodes
* Is the default unit of searchability

When a Document is sent to a Node for indexing, the target Index is specified as part of the URL. A `Cluster` will have many indices; by default a search query is executed only within one Index.

### Shards

#### Primary
A Shard is a unit of readability or writability. When a `Document` is indexed, the data is written to a `Primary` `Shard` somewhere in the `Cluster`. Only `Primary` `Shards` can be accept writes. Writes are distributed amongst the Primary Shards to distribute the load.

#### Replica
Each `Primary` Shard can have 0 or more `Replicas`. Once a `Document` is written to a `Primary`, it is then replicated out to all the `Replicas` of that `Primary`. This allows for redundant copies of data. If a `Primary` Shard is lost due to machine failure (or some other event), a `Replica` will be promoted.

#### Shard Placement
By default, Saga will attempt to distribute Shards such that they do not share a `Host`.

### Stores
At the lowest level, the data in a Shard must be written to disk. A `Store` is how this is accomplished. Currently, there are two `Stores` in various stages of development.

#### SQLite Store
This is the primary `Store` and has received the most development effort. When using this option, data is inserted into embedded SQLite databases that live on `Host's` filesystem. Using SQLite in this fashion offers several advantages:

* The proven reliability of SQLite
* Access to SQL semantics for querying and other operations
* Able to move `Shards` by simple moving a single file

There is one serious disadvantage, however. SQLite databases can only have one writer, though the number of readers is unlimited. There are a few simple config changes we can make to improve this:

* Enable `WAL`
* Wrap `INSERT`s in transactions
  * This effectively creates a buffer that must be flushed by committing the transaction every N seconds or when there is sufficient uncommitted data.

#### File Store
This `Store` serializes data out to local disk. This can be faster than SQLite, but a great deal of the searching, comparison, and other operations must happen in application code as opposed to the database engine.

## Web
In Progress

## Main
In Progress