// This is free and unencumbered software released into the public domain.

#![allow(unused)]

use asimov_module::prelude::{FromStr, String};
use core::error::Error;

#[derive(Clone, Debug, Default)]
pub struct BlockUrl {
    pub network: String,
    pub height: u64,
}

impl FromStr for BlockUrl {
    type Err = url::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        url::Url::parse(input).map(|url| {
            let network = url.host_str().unwrap_or("testnet").to_owned();
            let path = url.path().strip_prefix('/').expect("should contain a path");
            let height: u64 = path.parse().expect("should be a valid block height");
            Self { network, height }
        })
    }
}

impl BlockUrl {
    pub fn fetch(&self) -> Result<String, Box<dyn Error>> {
        let request_url = format!(
            "https://{}.neardata.xyz/v0/block/{}",
            self.network, self.height
        );
        let mut response = ureq::get(request_url).call()?;
        let response_body = response.body_mut().read_to_string()?;
        Ok(response_body)
    }
}
