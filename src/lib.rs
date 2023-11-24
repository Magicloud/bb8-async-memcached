use async_memcached::{ Client, Error };
use async_trait::async_trait;

#[derive(Clone)]
pub struct MemcacheConnectionManager {
    pub uri: String,
}
#[async_trait]
impl bb8::ManageConnection for MemcacheConnectionManager {
    type Connection = Client;
    type Error = Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        Self::Connection::new(&self.uri).await
    }

    async fn is_valid(
        &self,
        conn: &mut bb8::PooledConnection<'_, Self>
    ) -> Result<(), Self::Error> {
        conn.version().await.map(|_| ())
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}
