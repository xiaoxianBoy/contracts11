use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Uint128};
use derive_builder::Builder;
use std::collections::BTreeMap;

/// Instantiate message
#[cw_serde]
pub struct InstantiateMsg {
    /// Limitations regarding store usage.
    pub limits: StoreLimitsInput,
}

/// Execute messages
#[cw_serde]
pub enum ExecuteMsg {
    /// # InsertData
    /// Insert the data as RDF triples in the store.
    /// For already existing triples it acts as no-op.
    ///
    /// Only the smart contract owner (i.e. the address who instantiated it) is authorized to perform
    /// this action.
    InsertData { input: DataInput },
}

/// # SelectQuery
/// Query messages
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// # Store
    ///
    /// Returns information about the triple store.
    #[returns(StoreResponse)]
    Store,

    /// # Select
    ///
    /// Returns the resources matching the criteria defined by the provided query.
    ///
    #[returns(SelectResponse)]
    Select {
        /// The query to execute.
        query: SelectQuery,
    },
}

/// # StoreLimitsInput
/// Contains requested limitations regarding store usages.
#[cw_serde]
#[derive(Default, Builder)]
#[builder(default, setter(into, strip_option))]
pub struct StoreLimitsInput {
    /// The maximum number of triples the store can contains.
    /// If `None`, the default value of [Uint128::MAX] is used, which can be considered as no limit.
    pub max_triple_count: Option<Uint128>,
    /// The maximum number of bytes the store can contains.
    /// The size of a triple is counted as the sum of the size of its subject, predicate and object,
    /// including the size of data types and language tags if any.
    /// If `None`, the default value of [Uint128::MAX] is used, which can be considered as no limit.
    pub max_byte_size: Option<Uint128>,
    /// The maximum number of bytes the store can contains for a single triple.
    /// The size of a triple is counted as the sum of the size of its subject, predicate and object,
    /// including the size of data types and language tags if any. The limit is used to prevent
    /// storing very large triples, especially literals.
    /// If `None`, the default value of [Uint128::MAX] is used, which can be considered as no limit.
    pub max_triple_byte_size: Option<Uint128>,
    /// The maximum limit of a query, i.e. the maximum number of triples returned by a select query.
    /// If `None`, the default value of 30 is used.
    pub max_query_limit: Option<u32>,
    /// The maximum number of variables a query can select.
    /// If `None`, the default value of 30 is used.
    pub max_query_variable_count: Option<u32>,
    /// The maximum number of bytes an insert data query can contains.
    /// If `None`, the default value of [Uint128::MAX] is used, which can be considered as no limit.
    pub max_insert_data_byte_size: Option<Uint128>,
    /// The maximum number of triples an insert data query can contains (after parsing).
    /// If `None`, the default value of [Uint128::MAX] is used, which can be considered as no limit.
    pub max_insert_data_triple_count: Option<Uint128>,
}

impl StoreLimitsInput {
    const DEFAULT_MAX_TRIPLE_COUNT: Uint128 = Uint128::MAX;
    const DEFAULT_MAX_BYTE_SIZE: Uint128 = Uint128::MAX;
    const DEFAULT_MAX_TRIPLE_BYTE_SIZE: Uint128 = Uint128::MAX;
    const DEFAULT_MAX_QUERY_LIMIT: u32 = 30;
    const DEFAULT_MAX_QUERY_VARIABLE_COUNT: u32 = 30;
    const DEFAULT_MAX_INSERT_DATA_BYTE_SIZE: Uint128 = Uint128::MAX;
    const DEFAULT_MAX_INSERT_DATA_TRIPLE_COUNT: Uint128 = Uint128::MAX;

    pub fn max_triple_count_or_default(&self) -> Uint128 {
        self.max_triple_count
            .unwrap_or(Self::DEFAULT_MAX_TRIPLE_COUNT)
    }
    pub fn max_byte_size_or_default(&self) -> Uint128 {
        self.max_byte_size.unwrap_or(Self::DEFAULT_MAX_BYTE_SIZE)
    }
    pub fn max_triple_byte_size_or_default(&self) -> Uint128 {
        self.max_triple_byte_size
            .unwrap_or(Self::DEFAULT_MAX_TRIPLE_BYTE_SIZE)
    }
    pub fn max_query_limit_or_default(&self) -> u32 {
        self.max_query_limit
            .unwrap_or(Self::DEFAULT_MAX_QUERY_LIMIT)
    }
    pub fn max_query_variable_count_or_default(&self) -> u32 {
        self.max_query_variable_count
            .unwrap_or(Self::DEFAULT_MAX_QUERY_VARIABLE_COUNT)
    }
    pub fn max_insert_data_byte_size_or_default(&self) -> Uint128 {
        self.max_insert_data_byte_size
            .unwrap_or(Self::DEFAULT_MAX_INSERT_DATA_BYTE_SIZE)
    }
    pub fn max_insert_data_triple_count_or_default(&self) -> Uint128 {
        self.max_insert_data_triple_count
            .unwrap_or(Self::DEFAULT_MAX_INSERT_DATA_TRIPLE_COUNT)
    }
}

/// # DataInput
/// Represents the input data for the [ExecuteMsg::InsertData] message as RDF triples in a specific format.
#[cw_serde]
pub enum DataInput {
    /// # RDF XML
    /// Input in [RDF/XML](https://www.w3.org/TR/rdf-syntax-grammar/) format.
    #[serde(rename = "rdf_xml")]
    RDFXml(Binary),
    /// # Turtle
    /// Input in [Turtle](https://www.w3.org/TR/turtle/) format.
    Turtle(Binary),
    /// # N-Triples
    /// Input in [N-Triples](https://www.w3.org/TR/n-triples/) format.
    NTriples(Binary),
}

/// # StoreResponse
///
/// Contains information related to triple store.
#[cw_serde]
pub struct StoreResponse {
    /// The store owner.
    pub owner: String,

    /// The store limits.
    pub limits: StoreLimits,

    /// The store current usage.
    pub stat: StoreStat,
}

/// # StoreLimits
/// Contains limitations regarding store usages.
#[cw_serde]
#[derive(Default, Builder)]
#[builder(default, setter(into, strip_option))]
pub struct StoreLimits {
    /// The maximum number of triples the store can contains.
    pub max_triple_count: Uint128,

    /// The maximum number of bytes the store can contains.
    /// The size of a triple is counted as the sum of the size of its subject, predicate and object,
    /// including the size of data types and language tags if any.
    pub max_byte_size: Uint128,

    /// The maximum number of bytes the store can contains for a single triple.
    /// The size of a triple is counted as the sum of the size of its subject, predicate and object,
    /// including the size of data types and language tags if any. The limit is used to prevent
    /// storing very large triples, especially literals.
    pub max_triple_byte_size: Uint128,

    /// The maximum limit of a query, i.e. the maximum number of triples returned by a select query.
    pub max_query_limit: u32,

    /// The maximum number of variables a query can select.
    pub max_query_variable_count: u32,

    /// The maximum number of bytes an insert data query can contains.
    pub max_insert_data_byte_size: Uint128,

    /// The maximum number of triples an insert data query can contains (after parsing).
    pub max_insert_data_triple_count: Uint128,
}

/// # StoreStat
///
/// Contains usage information about the triple store.
#[cw_serde]
pub struct StoreStat {
    /// The total number of triple present in the store.
    pub triple_count: Uint128,

    /// The total number of IRI namespace present in the store.
    pub namespace_count: Uint128,

    /// The total triple size in the store, in bytes.
    pub byte_size: Uint128,
}

/// # IRI
/// Represents an IRI.
#[cw_serde]
pub enum IRI {
    /// # Prefixed
    /// An IRI prefixed with a prefix.
    /// The prefixed IRI is expanded to a full IRI using the prefix definition specified in the query.
    /// For example, the prefixed IRI `rdf:type` is expanded to `http://www.w3.org/1999/02/22-rdf-syntax-ns#type`.
    Prefixed(String),
    /// # Full
    /// A full IRI.
    Full(String),
}

/// # SelectResponse
/// Represents the response of a [QueryMsg::Select] query.
#[cw_serde]
pub struct SelectResponse {
    /// The head of the response, i.e. the set of variables mentioned in the results.
    pub head: Head,
    /// The results of the select query.
    pub results: Results,
}

/// # Head
/// Represents the head of a [SelectResponse].
#[cw_serde]
pub struct Head {
    /// The variables selected in the query.
    pub vars: Vec<String>,
}

/// # Results
/// Represents the results of a [SelectResponse].
#[cw_serde]
pub struct Results {
    /// The bindings of the results.
    pub bindings: Vec<BTreeMap<String, Value>>,
}

/// # Value
#[cw_serde]
#[serde(tag = "type")]
pub enum Value {
    /// # URI
    /// Represents an IRI.
    URI {
        /// The value of the IRI.
        value: IRI,
    },
    /// # Literal
    /// Represents a literal S with optional language tag L or datatype IRI D.
    Literal {
        /// The value of the literal.
        value: String,
        /// The language tag of the literal.
        #[serde(rename = "xml:lang")]
        lang: Option<String>,
        /// The datatype of the literal.
        datatype: Option<IRI>,
    },
    /// # BlankNode
    /// Represents a blank node.
    BlankNode {
        /// The identifier of the blank node.
        value: String,
    },
}

/// # SelectQuery
/// Represents a SELECT query over the triple store, allowing to select variables to return
/// and to filter the results.
#[cw_serde]
pub struct SelectQuery {
    /// The prefixes used in the query.
    pub prefixes: Vec<Prefix>,
    /// The items to select.
    /// Note: the number of items to select cannot exceed the maximum query variable count defined
    /// in the store limitations.
    pub select: Vec<SelectItem>,
    /// The WHERE clause.
    /// If `None`, there is no WHERE clause, i.e. all triples are returned without filtering.
    pub r#where: WhereClause,
    /// The maximum number of results to return.
    /// If `None`, there is no limit.
    /// Note: the value of the limit cannot exceed the maximum query limit defined in the store
    /// limitations.
    pub limit: Option<u64>,
}

/// # Prefix
/// Represents a prefix in a [SelectQuery]. A prefix is a shortcut for a namespace used in the query.
#[cw_serde]
pub struct Prefix {
    /// The prefix.
    pub prefix: String,
    /// The namespace associated with the prefix.
    pub namespace: String,
}

/// # SelectItem
/// Represents an item to select in a [SelectQuery].
#[cw_serde]
pub enum SelectItem {
    /// # Variable
    /// Represents a variable.
    Variable(String),
}

/// # WhereClause
/// Represents a WHERE clause in a [SelectQuery], i.e. a set of conditions to filter the results.
pub type WhereClause = Vec<WhereCondition>;

/// # WhereCondition
/// Represents a condition in a [WhereClause].
#[cw_serde]
pub enum WhereCondition {
    /// # Simple
    /// Represents a simple condition.
    Simple(SimpleWhereCondition),
}

/// # SimpleWhereCondition
/// Represents a simple condition in a [WhereCondition].
#[cw_serde]
pub enum SimpleWhereCondition {
    /// # TriplePattern
    /// Represents a triple pattern, i.e. a condition on a triple based on its subject, predicate and
    /// object.
    TriplePattern(TriplePattern),
}

/// # TriplePattern
/// Represents a triple pattern in a [SimpleWhereCondition].
#[cw_serde]
pub struct TriplePattern {
    /// The subject of the triple pattern.
    pub subject: SubjectPattern,
    /// The predicate of the triple pattern.
    pub predicate: PredicatePattern,
    /// The object of the triple pattern.
    pub object: ObjectPattern,
}

/// # SubjectPattern
/// Represents a subject pattern in a [TriplePattern] that can be either a variable or a node.
#[cw_serde]
pub enum SubjectPattern {
    /// # Variable
    /// A variable.
    Variable(String),
    /// # Node
    /// A node, i.e. an IRI or a blank node.
    Node(Node),
}

/// # PredicatePattern
/// Represents a predicate pattern in a [TriplePattern] that can be either a variable or a node.
#[cw_serde]
pub enum PredicatePattern {
    /// # Variable
    /// A variable.
    Variable(String),
    /// # Node
    /// A node, i.e. an IRI or a blank node.
    Node(Node),
}

/// # ObjectPattern
/// Represents an object pattern in a [TriplePattern] that can be either a variable, a node or a literal.
#[cw_serde]
pub enum ObjectPattern {
    /// # Variable
    /// A variable.
    Variable(String),
    /// # Node
    /// A node, i.e. an IRI or a blank node.
    Node(Node),
    /// # Literal
    /// An RDF [literal](https://www.w3.org/TR/rdf11-concepts/#dfn-literal), i.e. a simple literal,
    /// a language-tagged string or a typed value.
    Literal(Literal),
}

/// # Literal
/// An RDF [literal](https://www.w3.org/TR/rdf11-concepts/#dfn-literal).
#[cw_serde]
pub enum Literal {
    /// # Simple
    /// A [simple literal](https://www.w3.org/TR/rdf11-concepts/#dfn-simple-literal) without datatype or language form.
    Simple(String),
    /// # LanguageTaggedString
    /// A [language-tagged string](https://www.w3.org/TR/rdf11-concepts/#dfn-language-tagged-string)
    LanguageTaggedString {
        /// The [lexical form](https://www.w3.org/TR/rdf11-concepts/#dfn-lexical-form).
        value: String,
        /// The [language tag](https://www.w3.org/TR/rdf11-concepts/#dfn-language-tag).
        language: String,
    },
    /// # TypedValue
    /// A value with a datatype.
    TypedValue {
        /// The [lexical form](https://www.w3.org/TR/rdf11-concepts/#dfn-lexical-form).
        value: String,
        /// The [datatype IRI](https://www.w3.org/TR/rdf11-concepts/#dfn-datatype-iri).
        datatype: IRI,
    },
}

/// # Node
#[cw_serde]
pub enum Node {
    /// # NamedNode
    /// An RDF [IRI](https://www.w3.org/TR/rdf11-concepts/#dfn-iri).
    NamedNode(IRI),
    /// # BlankNode
    /// An RDF [blank node](https://www.w3.org/TR/rdf11-concepts/#dfn-blank-node).
    BlankNode(String),
}