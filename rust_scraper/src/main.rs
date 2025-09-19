use chrono::prelude::*;
pub mod aceshigh;
pub mod complex;
pub mod deltacenter;
pub mod depot;
pub mod granary;
pub mod hogwallow;
pub mod liquidjoes;
pub mod maverik;
pub mod show;
pub mod soundwell;
pub mod stateroom;
pub mod twentyfourtix;
pub mod union;
pub mod utahfirst;
pub mod util;

const HTML_TEMPLATE: &str = r###" 
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="description" content="Listing of Salt Lake City Music Concerts Shows">
    <meta name="keywords" content="Music, Concerts, Shows, Salt, Lake">
    <meta name="author" content="keith">

    <title>slc shows</title>
    <style>
      body {
          background-color:black;
          color:silver;
          /*font-family:"Courier New";*/
          font-family:"Lucida Console", monospace;
          font-size:large; 
      }
      pre {
          background-color:black;
          color:silver;
          /*font-family:"Courier New";*/
          font-family:"Lucida Console", monospace;
          font-size:large; 
      }
      a {
          /*text-decoration: none;*/
      }
      a:link {
          color:silver;
      }
      a:visited {
          color:gray;
      }
      tr td{
          max-width: 48ch;
          overflow: hidden;
          text-overflow: ellipsis;
          white-space: nowrap;
      }
   </style>
  </head>
  <body>
    <main>
      <p>
        <a href="#about">about this page</a>
      </p>
      <table id="events_table">
          @TABLE_ROWS@
      </table>
      <div id="about"></div>
      <p id="about">
        <pre class="about"> 
          @ABOUT@
        </pre>
      </p>
    </main>
  </body>
</html>
"###;

fn generate_html_page(shows: &[show::Show]) {
    let html = HTML_TEMPLATE.to_string();

    let date_str = Local::now().naive_local().date().to_string();
    let pre_string = "\nabout this page:\n".to_string();
    let pre_string = pre_string + "\tmissing venues or feedback: slcshowsnet AT gmail DOT com\n";
    let pre_string = pre_string + &format!("\tgenerated on: {}\n---\n\n", &date_str);
    let html = html.replace("@ABOUT@", &pre_string);

    let html = html.replace("@TABLE_ROWS@", &show::generate_table_rows(shows));
    std::fs::write("index.html", &html).expect("failed to write index.html");
}

fn main() {
    println!("Scraping ...");

    let mut shows = Vec::new();
    shows.append(&mut aceshigh::scrape());
    shows.append(&mut complex::scrape());
    shows.append(&mut deltacenter::scrape());
    shows.append(&mut depot::scrape());
    shows.append(&mut granary::scrape());
    shows.append(&mut hogwallow::scrape());
    shows.append(&mut liquidjoes::scrape());
    shows.append(&mut maverik::scrape());
    shows.append(&mut soundwell::scrape());
    shows.append(&mut stateroom::scrape());
    shows.append(&mut twentyfourtix::scrape());
    shows.append(&mut union::scrape());
    shows.append(&mut utahfirst::scrape());

    generate_html_page(&shows);
}
