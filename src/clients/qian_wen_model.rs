use log::error;
use serde_json::json;
use std::process::exit;

use crate::clients::Prompt;
use crate::config::consts::PROFILE;
use crate::utils::date_utils;

/// 通义千问的回复
///
/// ```json
/// {
///   "output": {
///     "finish_reason": "stop",
///     "text": "LLM Response"
///   },
///   "usage": {
///     "total_tokens": 894,
///     "output_tokens": 37,
///     "input_tokens": 857
///   },
///   "request_id": "b95ef82b b-e3b9-9f77-9a5b-12cd5a454809"
/// }
///```
#[derive(serde::Deserialize, Debug)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct LLMResponse<'a> {
    pub output: Output<'a>,
    pub usage: Usage,
    pub request_id: &'a str,
}

#[derive(serde::Deserialize, Debug)]
pub struct Usage {
    pub total_tokens: usize,
    /// 本次请求算法输出内容的 token 数目
    pub output_tokens: usize,
    /// 本次请求输入内容的 token 数目。
    /// > 在打开了搜索的情况下,需要添加搜索相关内容支持，所以会超出客户在请求中的输入。
    pub input_tokens: usize,
}

#[derive(serde::Deserialize, Debug)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct Output<'a> {
    #[serde(borrow)]
    pub text: &'a str,
    #[serde(borrow)]
    pub finish_reason: &'a str,
}

/// # 千问模型适配器
pub struct QianWenAdapter<'a> {
    url: &'a str,
    client: reqwest::blocking::Client,
}

impl QianWenAdapter<'_> {
    /// # 创建一个新的千问模型适配器
    ///
    /// 默认使用的 url 为官方的 URL `https://dashscope.aliyuncs.com/api/v1/services/aigc/text-generation/generation`
    pub fn new() -> QianWenAdapter<'static> {
        QianWenAdapter {
            //TODO 从配置文件中读取
            url: "https://dashscope.aliyuncs.com/api/v1/services/aigc/text-generation/generation",
            client: reqwest::blocking::Client::new(),
        }
    }
}

impl Prompt for QianWenAdapter<'_> {
    fn get_message(
        &self,
        year: Option<i32>,
        month: Option<u32>,
        day: Option<u32>,
        log_vec: Vec<String>,
    ) -> Option<String> {
        let date_string = date_utils::get_date_string(year, month, day);
        //TODO 从配置文件中读取 prompt
        let prompt = format!(
            "你是一个用于处理每日工作日志的程序。你可以帮助我生成每日工作日志。\
            你会根据 git 提交记录生成工作日志。需要以{}的提交为主，在此日期之前的提交记录如果与今日工作无关尽量不要体现。\
            结合提交修改的类,推断修改内容与影响范围,并体现在工作日志中。如果今日没有提交记录，那么你可以按照今天以前的提交记录，延续之前的工作撰写一条工作日志。\
            请注意，你的工作日志应该是一句话,不需要体现日期!只需要描述工作内容即可。\
            逗号少于4个,总字数要求25-30字,绝对不能少于20字!",
            date_string
        );

        // 构建请求体
        let body = json!({
            "model": "qwen-plus",
            "input":{
                "messages":[
                    {
                        "role": "system",
                        "content": prompt
                    },
                    {
                        "role": "user",
                        "content": log_vec.join(";")
                    }
                ],
            },
            "parameters": {
                "enable_search": true
            }
        });

        let authorization = match &PROFILE.authorization {
            Some(authorization) => authorization,
            None => {
                error!("未找到 Authorization 配置");
                exit(1);
            }
        };

        let res = self.client
            .post(self.url)
            .header("Content-Type", "application/json")
            //TODO 从配置文件中读取
            .header("Authorization", authorization)
            .body(body.to_string())
            .send();


        let text = match res {
            Ok(response) => {
                let result = response.text().unwrap();
                let model_response: LLMResponse = serde_json::from_str(&*result).unwrap();
                model_response.output.text.to_owned()
            }
            Err(_) => "请求失败".parse().unwrap(),
        };
        Some(text)
    }
}
