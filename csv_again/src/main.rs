fn main() {
    runtime::run_future(query::query_csv("barbar.csv"));
    runtime::run_future(write::query_csv("barbar.csv"));
}
