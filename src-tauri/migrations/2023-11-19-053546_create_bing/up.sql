-- Your SQL goes here
CREATE TABLE bing
(
    id               INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name             TEXT                              NOT NULL,
    url              TEXT                              NOT NULL,
    uhd_url          TEXT                              NOT NULL,
    uhd_file_path    TEXT                              NOT NULL,
    normal_file_path TEXT                              NOT NULL,
    source           TEXT                              NOT NULL,
    created_date     date                              NOT NULL
)
