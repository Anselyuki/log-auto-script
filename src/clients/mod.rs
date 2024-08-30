pub mod qian_wen_model;

/// # 对应平台的适配器枚举
///
/// 用于选择不同的平台适配器,目前使用通义千问实现日志解析
pub enum ModelAdapterEnum {
    /// 通义千问
    QianWen,
}

/// 获取大模型生成的日志解析信息
///
/// 与时间有关的参数如果为空,则以当前时间为准
pub trait Prompt {
    ///# 获取大模型生成的日志解析信息
    ///
    /// 与时间有关的参数如果为空,则以当前时间为准
    ///
    /// ## 参数
    ///
    /// * `year` - 年份
    /// * `month` - 月份
    /// * `day` - 日期
    /// * `logs` - 提交记录列表
    fn get_message(
        &self,
        year: Option<i32>,
        month: Option<u32>,
        day: Option<u32>,
        logs: Vec<String>,
    ) -> Option<String>;
}
