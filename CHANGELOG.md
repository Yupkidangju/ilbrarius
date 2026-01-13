# 변경 이력 (Changelog)

이 프로젝트는 [Semantic Versioning](https://semver.org/spec/v2.0.0.html)을 따릅니다.

## [v0.1.4] - 2026-01-13
### 수정됨 (Fixed)
- SurrealDB RocksDB Connection 트레이트 불일치 문제 해결 (`Surreal<Db>` 타입 적용)
- `chromiumoxide` Page API 변경 대응 (`title()` -> `get_title()`, `get_content()` -> `content()`)
- Tauri v2 `AppHandle::path()` 사용을 위한 `tauri::Manager` 트레이트 추가
- Windows `npm` 실행 정책 차단 이슈 해결 (`npm.cmd` 사용)
- `Capabilities` 설정 누락으로 인한 런타임 권한 오류 해결

### 추가됨 (Added)
- [v0.1.4] `PDF Assembler` 구현: 수집된 모든 페이지를 목차(ToC)가 포함된 하나의 PDF로 통합 내보내기
- [v0.1.4] `export_pdf` Tauri Command 및 프론트엔드 UI 연동

## [v0.1.0] - 2026-01-13
### 추가됨 (Added)
- [v0.1.3] `Live Viewport` 구현: 크롤링 중인 브라우저 화면을 실시간으로 프론트엔드에 스트리밍
- [v0.1.3] Tauri Event (`crawl-status`, `crawl-finished`) 기반의 실시간 상태 추적 시스템
- [v0.1.3] 현대적이고 어두운 테마의 UI 리뉴얼 및 사이드바 컨트롤러 고도화
- [v0.1.2] SurrealDB(RocksDB) 임베디드 모드 연동 및 체크포인트 시스템 구현
- [v0.1.2] `chromium-oxide` 기반의 Recursive BFS 크롤러 엔진 구현 (Depth 2~4 지원)
- [v0.1.2] 실시간 데이터 영속성 처리 (탐색 즉시 DB 저장)
- [v0.1.1] URL 리다이렉션 추적 엔진 (`track_url`) 구현
