use toggl_reports::{Toggl, query::Query};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let toggl = Toggl::new("00000000000000000000000000000000");
    let query = Query::new(1111111)
        .subgrouping_ids(true)
        .grouped_time_entry_ids(true);
    let res = toggl.get_summary_report(&query).await?;

    println!("{:?}", res);
    println!("TOTAL: {}s", res.total_grand.num_seconds());
    res.data
        .iter()
        .for_each(|elem| {
            let project_title = elem.title.name();
            let project_time = elem.time.num_seconds();
            println!("{}: {}s", project_title, project_time);

            elem.items.iter()
                .for_each(|item| {
                    let item_title = item.title.name();
                    let item_time = item.time.num_seconds();
                    println!("  {}: {}s", item_title, item_time);
                })
        });

    Ok(())
}
