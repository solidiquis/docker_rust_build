use datafusion::{common::arrow::array::RecordBatch, prelude::*};

pub async fn query_csv(csv_path: &str) -> String {
    let ctx = SessionContext::new();

    ctx.register_csv("example", csv_path, CsvReadOptions::new())
        .await
        .unwrap();

    let df = ctx
        .sql("SELECT a, MIN(b) FROM example WHERE a <= b GROUP BY a LIMIT 100")
        .await
        .unwrap();

    let results: Vec<RecordBatch> = df.collect().await.unwrap();

    arrow::util::pretty::pretty_format_batches(&results)
        .unwrap()
        .to_string()
}
