use std::path::PathBuf;

/// This Function will return a Vector with all Valid Include Root Pathes.
/// 
/// # Example
/// 
/// ```rust
/// let roots = get_include_root();
/// ```
pub fn get_include_root() -> Vec<PathBuf> {
    let mut res: Vec<PathBuf> = Vec::new();
    #[cfg(any(unix, target_os = "macos"))]
    {
        let mut step: Vec<&str> = Vec::new();
        step.push("/usr/include");
        step.push("/usr/local/include");
        for p in step {
            let buf = PathBuf::from(p);
            if buf.exists() {
                res.push(buf);
            }
        }
    }
    #[cfg(windows)]
    {
        let mut step: Vec<&str> = Vec::new();
        step.push("C:\\Program Files (x86)\\Windows Kits\\10\\Include");
        step.push("C:\\Program Files\\Windows Kits\\10\\Include");
        step.push("C:\\Program Files (x86)\\Microsoft SDKs\\Windows\\v7.1A\\Include");
        for p in step {
            let buf = PathBuf::from(p);
            if buf.exists() {
                res.push(buf);
            }
        }
    }
    res
}

/// This Function will return a Vector with all Valid Lib Root Pathes.
/// 
/// # Example
/// 
/// ```rust
/// let roots = get_lib_root();
/// ```
pub fn get_lib_root() -> Vec<PathBuf> {
    let mut res: Vec<PathBuf> = Vec::new();
    #[cfg(any(unix, target_os = "macos"))]
    {
        let mut step: Vec<&str> = Vec::new();
        step.push("/usr/lib");
        step.push("/usr/local/lib");
        for p in step {
            let buf = PathBuf::from(p);
            if buf.exists() {
                res.push(buf);
            }
        }
    }
    #[cfg(windows)]
    {
        let mut step: Vec<&str> = Vec::new();
        step.push("C:\\Program Files (x86)\\Windows Kits\\10\\Lib");
        step.push("C:\\Program Files\\Windows Kits\\10\\Lib");
        step.push("C:\\Program Files (x86)\\Microsoft SDKs\\Windows\\v7.1A\\Lib");
        for p in step {
            let buf = PathBuf::from(p);
            if buf.exists() {
                res.push(buf);
            }
        }
    }
    res
}