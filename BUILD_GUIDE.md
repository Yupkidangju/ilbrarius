# 빌드 가이드 (Build Guide)

## 사전 요구 사항 (Prerequisites)
* **OS:** Windows 10/11 (x64)
* **Node.js:** v18 이상
* **Rust:** v1.78 이상 (Edition 2021)
* **Build Tools:** Visual Studio C++ Build Tools (Windows)

## 설치 및 실행 (Installation & Run)

1. **의존성 설치**
   ```bash
   npm install
   ```

2. **개발 모드 실행**
   ```bash
   npm run tauri dev
   ```

3. **프로덕션 빌드**
   ```bash
   npm run tauri build
   ```

## 문제 해결 (Troubleshooting)
* **WebView2 에러:** Windows 런타임이 최신인지 확인하십시오.
* **Rust 컴파일 에러:** `rustup update`를 통해 툴체인을 최신으로 유지하십시오.
