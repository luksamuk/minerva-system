//! This module contains Data Transfer Objects for the `syslog` table, which
//! represents the audit logs of the service.
//!
//! `syslog` holds information of insertions, updates and deletions from any
//! table, and also describes which service did it and which user of this given
//! tenance requested the operation.
#![allow(clippy::extra_unused_lifetimes)]

use crate::schema::syslog;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[repr(u16)]
pub enum OpType {
    /// Operation for inserting new entities.
    Insert = 1,
    /// Operation for updating entities.
    Update = 2,
    /// Operation for removing entities.
    Delete = 3,
}

/// Table name for diesel queries
#[table_name = "syslog"]
pub struct Syslog {}

/// DTO representing a single entry on the `syslog` table.
#[derive(diesel-derive-enum::sql_enum, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Syslog {
    /// ID of the entry on the table.
    pub id: i32,
    /// Name of the service that performed the operation.
    pub service: String,
    /// Username of whom requested the operation.
    pub requestor: String,
    /// Name of the entity that was manipulated, usually the same as a table.
    pub entity: String,
    /// Operation performed.
    pub operation: OpType,
    /// Date and time of the performed operation.
    pub datetime: DateTime<Utc>,
    /// Optional description of the performed operation.
    pub description: Option<String>,
}

/// DTO for adding a new entry on the `syslog` table.
#[derive(Insertable, Clone, Debug, PartialEq, Eq, AsChangeset)]
#[table_name = "syslog"]
pub struct NewLog {
    /// Name of the service performing the operation.
    pub service: String,
    /// Username of whom is requesting the operation.
    pub requestor: String,
    /// Name of the entity being manipulated, usually the same as a table.
    pub entity: String,
    /// Operation being performed.
    pub operation: OpType,
    /// Date and time of the operation being performed.
    pub datetime: DateTime<Utc>,
    /// Optional description of the operation being performed.
    pub description: Option<String>,
}
