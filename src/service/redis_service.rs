use redis::AsyncCommands;

#[derive(Clone, Debug)]
pub struct RedisService {
    url: String,
}

impl RedisService {
    pub fn new(config: &crate::config::AppConfig) -> Self {
        RedisService {
            url: config.redis_config.url.clone(),
        }
    }

    pub async fn is_exists(&self, key: &str) -> redis::RedisResult<bool> {
        let client = redis::Client::open(self.url.as_str())?;
        let mut con = client.get_multiplexed_async_connection().await?;
        let exists: bool = con.exists(key).await?;
        Ok(exists)
    }

    pub async fn store_value<T: redis::ToSingleRedisArg + Send + Sync>(
        &self,
        key: &str,
        value: T,
    ) -> redis::RedisResult<()> {
        let client = redis::Client::open(self.url.as_str())?;
        let mut con = client.get_multiplexed_async_connection().await?;
        con.set::<_, _, ()>(key, value).await?;
        Ok(())
    }

    pub async fn store_value_ex<T: redis::ToSingleRedisArg + Send + Sync>(
        &self,
        key: &str,
        value: T,
        expiration_seconds: u64,
    ) -> redis::RedisResult<()> {
        let client = redis::Client::open(self.url.as_str())?;
        let mut con = client.get_multiplexed_async_connection().await?;
        con.set_ex::<_, _, ()>(key, value, expiration_seconds)
            .await?;
        Ok(())
    }

    pub async fn store_if_not_exists<T: redis::ToSingleRedisArg + Send + Sync>(
        &self,
        key: &str,
        value: T,
        expiration_seconds: Option<u64>,
    ) -> redis::RedisResult<bool> {
        let client = redis::Client::open(self.url.as_str())?;
        let mut con = client.get_multiplexed_async_connection().await?;

        if let Some(seconds) = expiration_seconds {
            redis::cmd("SET")
                .arg(key)
                .arg(value)
                .arg("NX")
                .arg("EX")
                .arg(seconds)
                .query_async(&mut con)
                .await
        } else {
            let result: bool = con.set_nx::<_, _, bool>(key, value).await?;
            Ok(result)
        }
    }

    pub async fn get_value(&self, key: &str) -> redis::RedisResult<String> {
        let client = redis::Client::open(self.url.as_str())?;
        let mut con = client.get_multiplexed_async_connection().await?;
        con.get(key).await
    }

    pub async fn clear_key(&self, key: &str) -> redis::RedisResult<()> {
        let client = redis::Client::open(self.url.as_str())?;
        let mut con = client.get_multiplexed_async_connection().await?;
        con.del::<_, ()>(key).await?;
        Ok(())
    }
}
