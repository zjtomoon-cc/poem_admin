//! SeaORM Entity. Generated by sea-orm-codegen 0.3.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "sys_auth_rule"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub pid: String,
    pub name: String,
    pub title: String,
    pub icon: String,
    pub condition: String,
    pub remark: String,
    pub menu_type: i8,
    pub weigh: i32,
    pub status: i8,
    pub always_show: i8,
    pub path: String,
    pub jump_path: String,
    pub component: String,
    pub allow_data_scope: Option<i8>,
    pub is_data_scope: Option<i8>,
    pub is_frame: i8,
    pub module_type: String,
    pub model_id: i32,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Pid,
    Name,
    Title,
    Icon,
    Condition,
    Remark,
    MenuType,
    Weigh,
    Status,
    AlwaysShow,
    Path,
    JumpPath,
    Component,
    AllowDataScope,
    IsDataScope,
    IsFrame,
    ModuleType,
    ModelId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = String;
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::String(Some(32u32)).def(),
            Self::Pid => ColumnType::String(Some(32u32)).def(),
            Self::Name => ColumnType::String(Some(100u32)).def().unique(),
            Self::Title => ColumnType::String(Some(50u32)).def(),
            Self::Icon => ColumnType::String(Some(50u32)).def(),
            Self::Condition => ColumnType::String(Some(255u32)).def(),
            Self::Remark => ColumnType::String(Some(255u32)).def(),
            Self::MenuType => ColumnType::TinyInteger.def(),
            Self::Weigh => ColumnType::Integer.def(),
            Self::Status => ColumnType::TinyInteger.def(),
            Self::AlwaysShow => ColumnType::TinyInteger.def(),
            Self::Path => ColumnType::String(Some(100u32)).def(),
            Self::JumpPath => ColumnType::String(Some(100u32)).def(),
            Self::Component => ColumnType::String(Some(100u32)).def(),
            Self::AllowDataScope => ColumnType::TinyInteger.def().null(),
            Self::IsDataScope => ColumnType::TinyInteger.def().null(),
            Self::IsFrame => ColumnType::TinyInteger.def(),
            Self::ModuleType => ColumnType::String(Some(30u32)).def(),
            Self::ModelId => ColumnType::Integer.def(),
            Self::CreatedAt => ColumnType::DateTime.def().null(),
            Self::UpdatedAt => ColumnType::DateTime.def().null(),
            Self::DeletedAt => ColumnType::DateTime.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            _ => panic!("No RelationDef"),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
