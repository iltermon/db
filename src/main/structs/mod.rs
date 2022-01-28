pub struct Column {
    pub name: String,
    pub data_type: String,
    pub comment: String,
    pub default: String,
}
pub struct Constraint {
    pub name: String,
    pub columns: Vec<Column>,
}
pub struct UniqueConstraint {
    pub constraint: Constraint,
}
pub struct PrimaryKey {
    pub unique_constraint: UniqueConstraint,
}
pub struct ForeignKey {
    pub constraint: Constraint,
    pub referenced_table: Table,
    pub referenced_columns: Vec<Column>,
}
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    //p ub// primary_key: PrimaryKey,
    pub constraints: Vec<Constraint>,
    pub comment: String,
}