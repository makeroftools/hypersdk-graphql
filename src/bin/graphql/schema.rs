use async_graphql::{
    Context, InputValueError, InputValueResult, Object
};
use hypersdk::{
    hypercore::{
        HttpClient,
        PerpMarket
    }
};
use hypersdk;

#[Scalar]
impl ScalarType for hypersdk::Address {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = &value {
            // Parse the integer value
            Ok(value.parse().map(Address)?)
        } else {
            Err(InputValueError::expected_type(value))
        }
    }
}

pub struct Query;

#[Object]
impl Query {
    async fn arbitrum_id<'ctx>(&self, ctx: &Context<'ctx>) -> Result<&'static str, async_graphql::Error> {
        let client = ctx.data::<HttpClient>()?; // ? operator or .unwrap()
        let chain = client.chain();
        Ok(chain.arbitrum_id())
    }
    async fn perps<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<PerpMarket>, async_graphql::Error> {
        let client = ctx.data::<HttpClient>()?;
        let markets = client.perps().await?;
        Ok(markets)
    }
}

// 


