pub fn uuid_generator(tag: &str) -> String {
    format!(
        "{}_{}",
        tag,
        uuid::Uuid::new_v4()
            .hyphenated()
            .encode_lower(&mut uuid::Uuid::encode_buffer())
    )
}
