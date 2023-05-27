mod cui;
mod tasks;

pub async fn init() {
    let app = cui::Cui::new().await;
    app.process().await;
}
