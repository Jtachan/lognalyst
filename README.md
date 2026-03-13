# lognalyst

`lognalyst` is a LOG file analyzer with multithreading logic.

## Supported LOG formats

Currently only plain-text LOG files are supported.
It is expected that the provided LOG file has the following format (per line):

1. **Timestamp in ISO format**:<br>
    The timestamp must be provided as two parts: the date and the hour the log message was created.
    The date should be provided as `YYYY-MM-DD` and the hour should be logged as `HH:mm:ss`.
    Both fields should be separated by a space. For example, `2024-03-12 14:23:45` is a correct timestamp.
2. **Logging category**<br>
    Either INFO, ERROR or any other category defining what the message corresponds to.
3. **Log message**<br>
    The full message that was communicated to the LOG file.
