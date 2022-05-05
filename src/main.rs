use sword::crawler::{read_uk_sanctions_list, read_un_sanctions_list};
use tokio::join;

#[tokio::main]
async fn main() {
    let uk = tokio::spawn(read_uk_sanctions_list());
    let un = tokio::spawn(read_un_sanctions_list());

    let _ = join!(uk, un);
}
