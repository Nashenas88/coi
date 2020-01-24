use rs_docker::Docker;
use std::{
    io::{Error, ErrorKind, Result},
    process::{Child, Command, ExitStatus},
    thread,
    time::Duration,
};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    about = "Each subcommand listed below will run the subcommand before it, in this order:
build, run, init, seed

Once init or seed has been run, you can just call the run subcommand and reuse
the existing data."
)]
enum Step {
    #[structopt(about = "Build the postgres docker image")]
    Build,
    #[structopt(about = "Run the postgres docker image")]
    Run,
    #[structopt(about = "Initialize the database in the postgres docker image")]
    Init,
    #[structopt(about = "Seed dummy data to the postgres docker image")]
    Seed,
}

const DOCKER_URI: &str = "unix:///var/run/docker.sock";
const DOCKER_IMAGE_NAME: &str = "coi-actix-sample-postgres";

fn build_step() -> Result<()> {
    let mut command = build()?;
    success_check(command.wait(), "docker")
}

fn run_step() -> Result<()> {
    let mut docker = Docker::connect(DOCKER_URI)?;
    let images = docker.get_images(false)?;
    if !images.iter().any(|i| i.Id == DOCKER_IMAGE_NAME) {
        build_step()?;
    }
    let mut command = run()?;
    success_check(command.wait(), "docker")
}

fn init_step() -> Result<()> {
    let mut docker = Docker::connect(DOCKER_URI)?;
    let containers = docker.get_containers(false)?;
    let container = containers
        .iter()
        .filter(|c| c.Image == DOCKER_IMAGE_NAME)
        .next();
    if let Some(container) = container {
        let mut command = init_db()?;
        success_check(command.wait(), "psql")
    } else {
        let images = docker.get_images(false)?;
        if !images.iter().any(|i| {
            i.RepoTags
                .iter()
                .any(|t| t == &format!("{}:latest", DOCKER_IMAGE_NAME))
        }) {
            build_step()?;
        }
        let mut command = run()?;
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_secs(5));
            let mut command = init_db()?;
            success_check(command.wait(), "psql")
        });
        if let Err(e) = handle.join() {
            command.kill()?;
            Err(Error::new(ErrorKind::Other, format!("{:?}", e)))
        } else {
            Ok(())
        }
    }
}

fn seed_step() -> Result<()> {
    let mut docker = Docker::connect(DOCKER_URI)?;
    let containers = docker.get_containers(false)?;
    let container = containers
        .iter()
        .filter(|c| c.Image == DOCKER_IMAGE_NAME)
        .next();
    if let Some(container) = container {
        let mut command = init_db()?;
        success_check(command.wait(), "psql")?;
        let mut command = seed()?;
        success_check(command.wait(), "psql")
    } else {
        let images = docker.get_images(false)?;
        if !images.iter().any(|i| i.Id == DOCKER_IMAGE_NAME) {
            build_step()?;
        }
        let mut command = run()?;
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_secs(5));
            let mut command = init_db()?;
            success_check(command.wait(), "psql")?;
            let mut command = seed()?;
            success_check(command.wait(), "psql")
        });
        if let Err(e) = handle.join() {
            command.kill()?;
            Err(Error::new(ErrorKind::Other, format!("{:?}", e)))
        } else {
            Ok(())
        }
    }
}

fn main() {
    let step = Step::from_args();
    if let Err(e) = match step {
        Step::Build => build_step(),
        Step::Run => run_step(),
        Step::Init => init_step(),
        Step::Seed => seed_step(),
    } {
        eprintln!("Failed to run step: {}", e);
    }
}

fn check_not_found(command: &str) -> impl Fn(Error) -> Error + '_ {
    move |e| {
        if e.kind() == ErrorKind::NotFound {
            Error::new(
                ErrorKind::NotFound,
                format!("{} not found on this system: {}", command, e),
            )
        } else {
            e
        }
    }
}

fn success_check(res: Result<ExitStatus>, command: &str) -> Result<()> {
    let status = res?;
    if status.success() {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Other,
            format!(
                "{} could not run successfully: exit code {:?}",
                command,
                status.code()
            ),
        ))
    }
}

fn build() -> Result<Child> {
    Command::new("docker")
        .arg("build")
        .arg(".")
        .arg("-t")
        .arg(DOCKER_IMAGE_NAME)
        .spawn()
        .map_err(check_not_found("docker"))
}

fn run() -> Result<Child> {
    Command::new("docker")
        .arg("run")
        .arg("-p")
        .arg("45432:5432")
        .arg(DOCKER_IMAGE_NAME)
        .spawn()
        .map_err(check_not_found("docker"))
}

fn init_db() -> Result<Child> {
    Command::new("pwd")
        .spawn()
        .expect("huh...")
        .wait()
        .expect("really!?");
    Command::new("psql")
        .arg("host=127.0.0.1 dbname=docker port=45432 user=docker password=docker")
        .arg("-f")
        .arg("xtask/sql/init.sql")
        .spawn()
        .map_err(check_not_found("psql"))
}

fn seed() -> Result<Child> {
    Command::new("psql")
        .arg("host=127.0.0.1 dbname=docker port=45432 user=docker password=docker")
        .arg("-f")
        .arg("xtask/sql/seed.sql")
        .spawn()
        .map_err(check_not_found("psql"))
}
