// SPDX-License-Identifier: (LGPL-2.1-only OR LGPL-3.0-only)
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AnnHandlerCfg {
    pub skip_check_chance: f32,
    pub num_workers: usize,
    pub input_queue_len: usize,
    pub public_url: String,
    pub bind_port: u16,
    pub files_to_keep: usize,
    pub block_miner_passwd: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    pub paymaker_http_password: String,
    pub master_url: String,
    pub root_workdir: String,
    pub ann_handler: HashMap<String, AnnHandlerCfg>,
}
pub fn get_ah_workdir(root_workdir: &str, ahc: &AnnHandlerCfg) -> String {
    format!("{}/ann_{}", root_workdir, ahc.bind_port)
}
