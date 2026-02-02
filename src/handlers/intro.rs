use std::sync::Arc;

use axum::{
  extract::State, 
  response::Html,  
};
use crate::AppState; 

pub async fn intro_handler(
  State(_state): State<Arc<AppState>>
) -> Html<&'static str> {
  Html(r#"
    <html>
      <center style="padding-top: 15%; padding-bottom: 15%;">
        <h2>Todo List HTTP Server</h2>        
        <div style="color:rgb(110, 110, 110); margin-top: -15px;">
          <h4>Written With Rust Axum Tokio</h4>
          <div style="font-size: 12px;">
            This program is just how i've implemented the simple way to use various different programming languages in addition of learning
          </div>						
          <div style="font-size: 10px; padding-top: 5px;">Author : Jefri Tambunan</div> 
        </div>
      </center>
    </html>
  "#)
}