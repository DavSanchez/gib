pub fn write_contents(
    mut writer: impl std::io::Write,
    content: &(&str, &[u8]),
) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(writer, "###############")?;
    writeln!(writer, "#   {}", content.0)?;
    writeln!(writer, "###############")?;
    writeln!(writer, "{}", String::from_utf8_lossy(content.1))?;
    Ok(())
}
