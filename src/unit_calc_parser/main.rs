use async_trait::async_trait;
use tokio::sync::mpsc;

use crate::{query_manager::{ListEntry, QueryParser}, unit_calc_parser::lexer::{get_units, lex, lex2}};

#[derive(Clone)]
pub struct UnitCalcParser{

}
impl Default for UnitCalcParser{
    fn default() -> Self {
        Self {  }
    }
}
#[async_trait]
impl QueryParser for UnitCalcParser{
    async fn parse(&self, query: String, resopnse: mpsc::Sender<ListEntry>){
        let text=match lex2(query, &get_units()){
            Some(v)=>v.iter().map(|t| t.to_string()).collect::<Vec<String>>().join(", "),
            None=>"Error!".to_string(),
        };
        let text2=text.clone();
        resopnse.send(ListEntry { layout_fn: Box::new(move |ui|{
            ui.label(format!("{}",&text));
        }), execute: Some(Box::new(move ||{
            println!("{}",&text2);
        })), priority: 0.0 }).await.unwrap();
    }
}