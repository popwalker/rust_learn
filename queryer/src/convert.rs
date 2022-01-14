/// 解析出来的sql
pub struct Sql<'a> {
    pub(crate) selectio: Vec<Expr>,
    pub(crate) condition: Option<Expr>,
    pub(crate) source: &'a str,
    pub(crate) order_by: Vec<String, bool>,
    pub(crate) offser: Option<i64>,
    pub(crate) limit: Option<usize>,
}

impl<'a> TryForm<&'a Statement> for Sql<'a> {
    type Error = anyhow::Error;

    fn try_form(sql: &'a Statement) -> Result<Self, Self::Error>{
        match sql {
            Statement::Query(q) => {} 
        }
    }
}