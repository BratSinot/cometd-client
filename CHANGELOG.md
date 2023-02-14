# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)

## [0.3.3]

### Fixed
- Replace stack array on boxed.

### Add

### Change

## [0.3.1]

### Fixed

### Add

### Change

- Rework `CometdError`.
- `AccessToken` now always set only one token to `authorization` header.
- `timeout` and `interval` sets through `core::time::Duration`.

## [0.2.0]

### Fixed

### Add

- Add access token setter for builder.
- Add support for multiple cookies.

### Change

- Builder accept `&'a str` instead of `'static`.
- Add `Sync + Send + 'static` for `dyn Error` in `CometdError`.
- Add `Sync + Send + 'static` for `trait AccessToken`.
- `endpoint` moved to `Builder::new` and it now `Url`.

## [0.1.0] - 2022-12-09

### Fixed

### Add

- Set `Advice` if some error appears.

### Change

- Refactor modules export.

## [0.0.1] 2022-12-09

- Initial release.
