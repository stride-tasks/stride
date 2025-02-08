use std::io::Write;

use wasmi_wasi::wasi_common::pipe::WritePipe;

// use crate::api::logging::Logger;

pub(crate) struct PluginLogger {
    plugin_name: String,
    is_error: bool,
}

impl PluginLogger {
    pub(crate) fn new(plugin_name: String, is_error: bool) -> WritePipe<Self> {
        WritePipe::new(PluginLogger {
            plugin_name,
            is_error,
        })
    }
}

impl Write for PluginLogger {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let result = format!(
            "Plugin({}): {}",
            self.plugin_name.clone(),
            String::from_utf8_lossy(buf)
        );
        if self.is_error {
            // Logger::error(result.trim());
        } else {
            // Logger::info(result.trim());
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
