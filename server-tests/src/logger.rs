use log::{Level, Log, Metadata as LogMetadata, Record as LogRecord};
use std::borrow::Cow;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

thread_local! {
    static LOG_DATA: RefCell<Option<LogData>> = RefCell::new(None);
}

#[derive(Clone)]
pub struct LogData {
    logs: Arc<Mutex<Vec<Record>>>,
}

impl LogData {
    fn empty() -> Self {
        Self {
            logs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn record(&self, record: Record) {
        let mut logs = self.logs.lock().unwrap();
        logs.push(record);
    }

    fn set_current(other: Self) {
        LOG_DATA.with(|data| {
            *data.borrow_mut() = Some(other);
        })
    }

    pub fn current() -> Self {
        LOG_DATA.with(|data| {
            let mut data = data.borrow_mut();
            if data.is_none() {
                *data = Some(Self::empty());
            }

            data.clone().unwrap()
        })
    }

    pub fn dump_to_logger<L: Log>(&self, logger: &L) {
        let logs = self.logs.lock().unwrap();

        for record in &*logs {
            let mut builder = LogRecord::builder();
            builder
                .level(record.level)
                .target(&record.target)
                .line(record.line);

            match record.module_path {
                Some(Cow::Borrowed(x)) => builder.module_path_static(Some(x)),
                Some(Cow::Owned(ref x)) => builder.module_path(Some(&x)),
                None => builder.module_path(None),
            };

            match record.file {
                Some(Cow::Borrowed(x)) => builder.file_static(Some(x)),
                Some(Cow::Owned(ref x)) => builder.file(Some(&x)),
                None => builder.file(None),
            };

            logger.log(&builder.args(format_args!("{}", record.message)).build());
        }

        logger.flush();
    }
}

#[derive(Debug)]
struct Record {
    level: Level,
    target: String,
    module_path: Option<Cow<'static, str>>,
    file: Option<Cow<'static, str>>,
    line: Option<u32>,
    message: String,
}

impl From<&'_ LogRecord<'_>> for Record {
    fn from(record: &LogRecord) -> Self {
        let module_path = record
            .module_path_static()
            .map(Cow::Borrowed)
            .or_else(|| record.module_path().map(|x| x.to_string()).map(Cow::Owned));
        let file = record
            .file_static()
            .map(Cow::Borrowed)
            .or_else(|| record.file().map(|x| x.to_string()).map(Cow::Owned));

        Self {
            level: record.level(),
            target: record.target().to_string(),
            module_path,
            file,
            line: record.line(),
            message: format!("{}", record.args()),
        }
    }
}

pub struct Logger;

impl Log for Logger {
    fn enabled(&self, _: &LogMetadata) -> bool {
        true
    }

    fn log(&self, record: &LogRecord) {
        let data = LogData::current();
        data.record(record.into());
    }

    fn flush(&self) {}
}

pub fn init() {
    let _ = log::set_boxed_logger(Box::new(Logger));
    log::set_max_level(log::LevelFilter::Trace);
    LogData::set_current(LogData::empty());
}

pub fn set_buffer(data: LogData) {
    LogData::set_current(data);
}

pub fn current() -> LogData {
    LogData::current()
}

pub fn log_recorded<L: Log>(logger: &L) {
    LogData::current().dump_to_logger(logger);
}
