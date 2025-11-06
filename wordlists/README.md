# 📚 Wordlists Collection

This directory contains curated wordlists for web application fuzzing and security testing.

## 📋 Available Wordlists

### 🌐 Web Discovery
- **common.txt** (4,686 lines) - Common files and directories from ffuf.me
- **seclists-common.txt** (4,749 lines) - SecLists common paths
- **raft-directories.txt** (29,999 lines) - RAFT medium directories
- **raft-files.txt** (17,129 lines) - RAFT medium files
- **raft-large-directories.txt** (62,290 lines) - RAFT large directories

### 🔧 API & Parameters
- **params.txt** (2,588 lines) - Common parameter names
- **parameters.txt** (2,588 lines) - Parameter fuzzing wordlist
- **api-endpoints.txt** (5,366 lines) - API endpoint patterns

### 🌍 Subdomain Enumeration
- **subdomains.txt** (1,907 lines) - Common subdomain names

### 📄 File Extensions
- **extensions.txt** (93 lines) - Common file extensions

### 🔐 HTTP Components
- **http-headers.txt** (1,185 lines) - HTTP header names
- **http-methods.txt** (62 lines) - HTTP methods
- **content-types.txt** (690 lines) - MIME content types
- **protocols.txt** (28 lines) - Protocol schemes

### 🎯 Exploitation
- **lfi-linux.txt** (881 lines) - Linux LFI payloads
- **lfi-windows.txt** (236 lines) - Windows LFI payloads
- **special-chars.txt** (32 lines) - Special characters for fuzzing

## 📊 Total Statistics

| Category | Files | Total Lines |
|----------|-------|-------------|
| Web Discovery | 5 | 118,843 |
| Parameters | 3 | 10,542 |
| HTTP Components | 4 | 1,965 |
| Exploitation | 3 | 1,149 |
| **TOTAL** | **15** | **~132,499** |

## 🎯 Usage Examples

### Basic Directory Fuzzing
```bash
fuzzblitz -u https://example.com/FUZZ -w wordlists/common.txt
```

### Parameter Discovery
```bash
fuzzblitz -u https://example.com/?FUZZ=test -w wordlists/params.txt
```

### Subdomain Enumeration
```bash
fuzzblitz -u https://FUZZ.example.com -w wordlists/subdomains.txt
```

### File Extension Discovery
```bash
fuzzblitz -u https://example.com/config.FUZZ -w wordlists/extensions.txt
```

### API Endpoint Fuzzing
```bash
fuzzblitz -u https://api.example.com/v1/FUZZ -w wordlists/api-endpoints.txt
```

### Large-Scale Discovery
```bash
fuzzblitz -u https://example.com/FUZZ -w wordlists/raft-large-directories.txt -t 50
```

## 📥 Sources

- [ffuf.me](http://ffuf.me/wordlist/) - Community fuzzing wordlists
- [hahwul/fuzzstone](https://github.com/hahwul/fuzzstone) - Fuzzing stone wordlists
- [SecLists](https://github.com/danielmiessler/SecLists) - Security testing wordlists
- [Bo0oM/fuzz.txt](https://github.com/Bo0oM/fuzz.txt) - API fuzzing payloads
- [fuzzdb](https://github.com/fuzzdb-project/fuzzdb) - Attack pattern database

## ⚠️ Legal Disclaimer

These wordlists are provided for educational and authorized security testing purposes only. 
Always ensure you have explicit permission before testing any system you don't own.

## 📝 License

Wordlists are collected from various open-source projects. Please refer to the original sources for licensing information.
