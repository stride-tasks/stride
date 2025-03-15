use std::io::Write;

use wasmi_wasi::wasi_common::pipe::WritePipe;

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
        let message = String::from_utf8_lossy(buf);
        let message = message.trim();
        if self.is_error {
            log::error!("Plugin({}): {message}", self.plugin_name);
        } else {
            log::info!("Plugin({}): {message}", self.plugin_name);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
