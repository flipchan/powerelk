#![allow(dead_code)]

use chrono::{DateTime, Utc};

pub struct Elk {
    pub instance: String,
    pub index: String,
}

impl Elk {
    pub fn getendpoint(self) -> String {
        let outme: String = format!("{}/{}", self.instance, self.index);
        outme
    }

    pub async fn query_elk(
        self,
        findme: String,
        limit: i64,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let new_post: serde_json::Value = reqwest::Client::new()
            .get(&self.getendpoint())
            .json(&serde_json::json!({"query": {"match": {"title": findme}}, "size": limit, "sort":[{"indexed_at": {"order": "desc"}}]  }))
            .send()
            .await?
            .json()
            .await?;

        Ok(new_post)
    }

    pub async fn time_search(
        self,
        find: String,
        datecolumn: String,
        fromtime: DateTime<Utc>,
        totime: DateTime<Utc>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let totime: String = format!("{}", totime);
        let fromtime: String = format!("{}", fromtime);

        let new_post: serde_json::Value = reqwest::Client::new()
            .post(&self.getendpoint())
            .json(&serde_json::json!({"size":1000, "query": {"bool": {"filter": [{"match": {"title": find}},{"range": { datecolumn.as_str(): { "gte": fromtime, "lt":totime}}}]}}}))
            .send()
            .await?
            .json()
            .await?;

        Ok(new_post)
    }

    pub async fn other_index_search(
        self,
        newindex: String,
        sortcolumn: String,
        find: String,
        limit: i64,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let endpoint: String = format!("{}/{}", self.instance, newindex);
        let new_post: serde_json::Value = reqwest::Client::new()
            .post(&endpoint)
            .json(&serde_json::json!({"query": {"match": {"title": find}}, "size": limit, "sort":[{ sortcolumn.as_str(): {"order": "desc"}}]  }))
            .send()
            .await?
            .json()
            .await?;

        Ok(new_post)
    }

    pub async fn get_lib(
        self,
        findme: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let out: serde_json::Value = reqwest::Client::new()
            .post(&self.getendpoint())
            .json(&serde_json::json!({ "query": { "bool": {"must": { "match": {"library": findme} }} }}))
            .send()
            .await?
            .json()
            .await?;
        Ok(out)
    }

    /// call elasticsearch's random function
    pub async fn random(self, limit: i64) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let out: serde_json::Value = reqwest::Client::new()
            .get(&self.getendpoint())
            .json(&serde_json::json!({ "size":limit,"query": { "function_score": { "functions": [ { "random_score":  {} } ],  "score_mode": "sum",} }}))
            .send()
            .await?
            .json()
            .await?;
        Ok(out)
    }

    /// return amount of items in index
    pub async fn amount_in_index(self) -> Result<i64, Box<dyn std::error::Error>> {
        let resp: serde_json::Value = reqwest::get(&self.getendpoint()).await?.json().await?;
        let am: i64 = resp.get("count").unwrap().as_i64().unwrap();
        Ok(am)
    }

    /// get index information
    pub async fn get_index_info(self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let resp: serde_json::Value = reqwest::get(&self.getendpoint()).await?.json().await?;
        Ok(resp)
    }
}
