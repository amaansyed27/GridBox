use gridbox_agent::OllamaAgent;
use gridbox_fastf1_client::FastF1Client;
use gridbox_openf1::OpenF1Client;
use gridbox_storage::AppPaths;

pub async fn run(
    paths: &AppPaths,
    openf1: &OpenF1Client,
    agent: &OllamaAgent,
    fastf1: &FastF1Client,
) -> bool {
    println!("GridBox doctor\n");
    let mut healthy = true;

    match paths.ensure() {
        Ok(()) => println!("[ok] local data directory: {}", paths.data_dir.display()),
        Err(error) => {
            healthy = false;
            println!("[fail] local storage: {error}");
        }
    }

    match openf1.health().await {
        Ok(session) => println!("[ok] OpenF1: latest session is {}", session.title()),
        Err(error) => {
            healthy = false;
            println!("[fail] OpenF1: {error}");
        }
    }

    match agent.health().await {
        Ok(models) => println!(
            "[ok] Ollama: {} model(s) installed{}",
            models.len(),
            if models.is_empty() {
                " — pull a model before using Engineer"
            } else {
                ""
            }
        ),
        Err(error) => {
            healthy = false;
            println!("[fail] Ollama: {error}");
        }
    }

    match fastf1.ping().await {
        Ok(result) => {
            let available = result
                .get("fastf1_available")
                .and_then(|value| value.as_bool())
                .unwrap_or(false);
            if available {
                println!("[ok] FastF1 worker: {}", result);
            } else {
                healthy = false;
                println!("[fail] FastF1 worker started but FastF1 is not installed: {result}");
            }
        }
        Err(error) => {
            healthy = false;
            println!("[fail] FastF1 worker: {error}");
        }
    }

    healthy
}
