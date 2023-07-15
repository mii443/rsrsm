use std::sync::Arc;

use crate::logger::log;
use tokio::sync::Mutex;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::{config::Config, rcon::Rcon, rcon_web::WebRcon};

pub async fn create_scheduler(config: Config, web_rcon: Arc<Mutex<WebRcon>>) {
    let scheduler = JobScheduler::new().await.unwrap();
    for job in config.jobs {
        let web_rcon_clone = web_rcon.clone();
        let job_clone = job.clone();
        scheduler
            .add(
                Job::new_async(job.cron.as_str(), move |_uuid, _l| {
                    let web_rcon = web_rcon_clone.clone();
                    let job = job_clone.clone();
                    Box::pin(async move {
                        log("CRON_JOB", "cyan", &format!("Running Job: {}", job.name));
                        for command in job.commands {
                            web_rcon.lock().await.execute(&command).await;
                        }
                    })
                })
                .unwrap(),
            )
            .await
            .unwrap();
    }

    scheduler.start().await.unwrap();
}
