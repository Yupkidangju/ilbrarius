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

## 빌드 절차 (Build Process) [v0.1.4]

1. **프론트엔드 최적화**
   - Tailwind CSS 및 Vite를 통한 자산 번들링 수행.
2. **백엔드 컴파일**
   - Rust 릴리즈 프로필(`--release`)을 사용하여 고성능 바이너리 생성.
   - SurrealDB(RocksDB) 정적 링크 확인.
3. **패키징**
   - MSI 인스톨러 및 단일 실행 파일(.exe) 생성.

## 문제 해결 (Troubleshooting)
* **WebView2 에러:** Windows 런타임이 최신인지 확인하십시오.
* **linker link.exe not found:** Visual Studio C++ Build Tools가 설치되어 있지 않거나 PATH에 추가되지 않은 경우 발생합니다. [Visual Studio Installer](https://visualstudio.microsoft.com/downloads/)를 통해 'C++를 사용한 데스크톱 개발' 워크로드를 설치하십시오.
* **libclang.dll missing:** SurrealDB(RocksDB) 빌드 시 LLVM이 필요합니다. `winget install LLVM.LLVM`으로 설치 후 `LIBCLANG_PATH` 환경 변수를 `C:\Program Files\LLVM\bin`으로 설정하십시오.
* **Tauri v2 Capabilities 에러:** `src-tauri/capabilities/default.json` 파일이 누락된 경우 빌드 에러가 발생할 수 있습니다. 해당 파일을 생성하고 권한을 설정하십시오.
* **Windows에서 npm 실행 차단:** Windows Execution Policy에 의해 `npm` 스크립트 실행이 차단될 경우, `npm.cmd`를 사용하거나 `tauri.conf.json`의 `beforeBuildCommand`를 `npm.cmd run build`로 수정하십시오.
* **TypeScript 에러:** `tsconfig.json` 파일이 프로젝트 루트에 존재하는지 확인하십시오.
