use crate::{agent::runs, error::AppError, http::routes::AppState};

fn lease_until() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 300;
    ts.to_string()
}

pub async fn run_once(state: AppState) -> Result<bool, AppError> {
    runs::reclaim_stale(&state.pool).await?;

    let worker_id = uuid::Uuid::new_v4().to_string();
    let leased = lease_until();

    let Some(run) = runs::claim_run(&state.pool, &worker_id, &leased).await? else {
        return Ok(false);
    };

    match crate::agent::orchestrator::process_run(state.clone(), &run).await {
        Ok(()) => {
            runs::mark_completed(&state.pool, &run.id).await?;
        }
        Err(e) => {
            let msg = e.to_string();
            runs::mark_failed(&state.pool, &run.id, &msg).await?;
        }
    }

    Ok(true)
}

pub async fn run_loop(state: AppState) -> Result<(), AppError> {
    loop {
        let worked = run_once(state.clone()).await?;
        if !worked {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }
}
