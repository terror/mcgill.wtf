use super::*;

pub(crate) trait Spawn {
  fn spawn(options: SpawnOptions) -> Result<redis::Client>;
}

#[derive(Debug)]
pub(crate) struct SpawnOptions {
  pub(crate) local: bool,
  pub(crate) port: u16,
}

impl Spawn for redis::Client {
  fn spawn(options: SpawnOptions) -> Result<redis::Client> {
    if options.local {
      Command::new("docker")
        .args(["kill", "mcgill.wtf-redis"])
        .status()?;

      Command::new("docker")
        .args([
          "run",
          "--name",
          "mcgill.wtf-redis",
          "-d",
          "--rm",
          "-p",
          &format!("{}:6379", options.port),
          "redis/redis-stack-server:latest",
          "redis-stack-server",
          "--save",
          "",
        ])
        .status()?;
    } else {
      Command::new("redis-server")
        .args([
          "--loadmodule",
          "/opt/redis-stack/lib/redisearch.so",
          "--loadmodule",
          "/opt/redis-stack/lib/rejson.so",
          "--port",
          &format!("{}", options.port),
          "--save",
          "",
        ])
        .status()?;
    }

    Ok(Self::open(format!("redis://localhost:{}", options.port))?)
  }
}
