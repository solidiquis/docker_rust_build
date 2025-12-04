fn main() {
    runtime::run_future(query::query_csv("foobar.csv"));
    runtime::run_future(write::query_csv("foobar.csv"));
}
