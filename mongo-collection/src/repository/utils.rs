use mongodb::bson::oid::ObjectId;

/// 将字符串解析为 ObjectId
///
/// 将 `ObjectId::parse_str` 的错误转换为 MongoDB 错误类型
///
/// # 参数
/// * `id` - ObjectId 字符串表示
///
/// # 返回值
/// * `Ok(ObjectId)` - 解析成功
/// * `Err(mongodb::error::Error)` - 解析失败
pub(crate) fn parse_object_id(id: &str) -> Result<ObjectId, mongodb::error::Error> {
    ObjectId::parse_str(id).map_err(|e| mongodb::error::Error::custom(e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_object_id_valid() {
        let result = parse_object_id("507f1f77bcf86cd799439011");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_object_id_invalid() {
        let result = parse_object_id("invalid");
        assert!(result.is_err());
    }
}
